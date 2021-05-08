use crate::core::{Parsable, ParseLogger, Parser};
use std::rc::Rc;

/// Data structure for `fix` combinator.
#[derive(Clone)]
pub struct Fix<'f, A, S>(Rc<dyn Fn(&Fix<'f, A, S>) -> Parser<'f, A, S> + 'f>);

impl<'f, A: 'f, S> Parsable for Fix<'f, A, S> {
    type Stream = S;
    type Result = A;
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<A> {
        //! fix f = f (fix f)
        (self.0)(self).parse(stream, logger)
    }
}

/// ## Combinator: `fix`
///
/// Fixed-point combinator (aka Y-Combinator), which is
/// introduced to support recursive synatax using closures.
///
/// ### Property
///
/// ```plain
/// fix f = f (fix f)
/// ```
pub fn fix<'f, A: 'f, F, S>(fix: F) -> Fix<'f, A, S>
where
    F: Fn(&Fix<'f, A, S>) -> Parser<'f, A, S> + 'f,
{
    Fix(Rc::new(fix))
}

#[cfg(test)]
mod test {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, CharStream};

    #[test]
    fn simple_recursive_syntax() {
        // expr := '1' expr | '0'
        let parser = fix(|parser| char('1').right(parser.clone()).or(char('0')));

        let mut st = CharStream::new("1110");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('0'), res);
        assert_eq!(0, logs.len());
    }

    #[test]
    fn mutual_recursive_syntax() {
        // expr     := term '+' expr | term
        // term     := factor '*' term | factor
        // factor   := '(' expr ')' | uint
        // uint      := digit { digit }
        // digit    := '0' | '1' | ... | '9'
        let digit = satisfy(|&ch| ch.is_digit(10));
        let uint = digit
            .some()
            .map_result(|v| v.iter().collect::<String>().parse::<u64>());
        let expr = fix(move |expr| {
            let factor = char('(').mid(expr.clone(), char(')')).or(uint.clone());

            let term = fix(move |term| {
                factor
                    .clone()
                    .left(char('*'))
                    .and(term.clone())
                    .map(|(v1, v2)| v1 * v2)
                    .or(factor.clone())
            });

            term.clone()
                .left(char('+'))
                .and(expr.clone())
                .map(|(v1, v2)| v1 + v2)
                .or(term.clone())
        });

        let mut st = CharStream::new("1+2*(3+4)");
        let (res, logs) = expr.exec(&mut st);

        assert_eq!(Some(15), res);
        assert_eq!(0, logs.len());
    }
}
