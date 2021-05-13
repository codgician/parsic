use crate::core::{IntoParser, Parsable, ParseLogger, Parser};
use std::rc::Rc;

/// Data structure for `fix` combinator.
pub struct Fix<'f, A, S>(Rc<dyn Fn(Parser<'f, A, S>) -> Parser<'f, A, S> + 'f>);

impl<'f, A, S> Clone for Fix<'f, A, S> {
    fn clone(&self) -> Self {
        Fix(self.0.clone())
    }
}

impl<'f, A: 'f, S: 'f> Parsable for Fix<'f, A, S> {
    type Stream = S;
    type Result = A;
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<A> {
        //! fix f = f (fix f)
        (self.0)(self.clone().into_parser()).parse(stream, logger)
    }
}

/// # Combinator: `fix`
///
/// In Rust, closures are anonymous functions, so there is no name for us to call
/// when we want to make it recursive. Therefore, a Y-Combinator, or
/// [fixed-point combinator](https://en.wikipedia.org/wiki/Fixed-point_combinator) `fix`
/// is introduced to address this issue, making it possible to write parsers that
/// support recursive syntax using closures.
///
/// # Property
/// ```plain
/// fix f = f (fix f)
/// ```
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{char, CharStream};
///
/// // expr := '1' expr | '0'
/// let parser = fix(|parser| char('1').right(parser.clone()).or(char('0')));
///
/// let mut st = CharStream::new("1110");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('0'), res);
/// assert_eq!("", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn fix<'f, A: 'f, F, S: 'f>(fix: F) -> Parser<'f, A, S>
where
    F: Fn(Parser<'f, A, S>) -> Parser<'f, A, S> + 'f,
{
    Fix(Rc::new(fix)).into_parser()
}

#[cfg(test)]
mod test {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, CharStream};

    #[test]
    fn mutual_recursive_syntax() {
        // expr     := term '+' expr | term
        // term     := factor '*' term | factor
        // factor   := '(' expr ')' | uint
        // uint     := digit { digit }
        // digit    := '0' | '1' | ... | '9'
        let expr = fix(move |expr| {
            let digit = satisfy(|&ch| ch.is_digit(10));
            let uint = digit
                .some()
                .map_result(|v| v.iter().collect::<String>().parse::<u64>());
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
                .and(expr)
                .map(|(v1, v2)| v1 + v2)
                .or(term)
        });

        let mut st = CharStream::new("1+2*(3+4)");
        let (res, logs) = expr.exec(&mut st);

        assert_eq!(Some(15), res);
        assert_eq!(0, logs.len());
    }
}
