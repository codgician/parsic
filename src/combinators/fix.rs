use crate::core::{Parsable, ParseLogger};
use std::rc::Rc;

#[derive(Clone)]
pub struct FixP<'a, S, T>(
    Rc<dyn for<'b> Fn(&'b Self) -> Box<dyn Parsable<S, Result = T> + 'b> + 'a>,
);

impl<'a, S, T> FixP<'a, S, T> {
    pub fn new<F>(func: F) -> Self
    where
        F: for<'b> Fn(&'b FixP<'a, S, T>) -> Box<dyn Parsable<S, Result = T> + 'b> + 'a,
    {
        Self(Rc::new(func))
    }
}

impl<'a, S, T> Parsable<S> for FixP<'a, S, T> {
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        // fix f = f (fix f)
        (self.0)(self).parse(state, logger)
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
pub fn fix<'a, F, S, T>(func: F) -> FixP<'a, S, T>
where
    F: for<'b> Fn(&'b FixP<'a, S, T>) -> Box<dyn Parsable<S, Result = T> + 'b> + 'a,
{
    FixP::new(func)
}

#[cfg(test)]
mod test {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, StrState};

    #[test]
    fn simple_recursive_syntax() {
        // expr := '1' expr | '0'
        let parser = fix(|it| Box::new(char('1').right(it).or(char('0'))));

        let mut st = StrState::new("1110");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('0'), res);
        assert_eq!(0, logs.len());
    }

    #[test]
    fn mutual_recursive_syntax() {
        // expr     := term '+' expr | term
        // term     := factor '*' term | factor
        // factor   := '(' expr ')' | nat
        // nat      := digit { digit }
        // digit    := '0' | '1' | ... | '9'
        let expr_parser = fix(|expr_it| {
            let digit_parser = satisfy(|&ch| ch.is_digit(10));

            let nat_parser = digit_parser
                .some()
                .map(|v| v.iter().collect::<String>().parse::<i64>().unwrap());

            let factor_parser = mid(char('('), expr_it, char(')')).or(nat_parser);

            let term_parser = fix(move |term_it| {
                Box::new(
                    factor_parser
                        .clone()
                        .left(char('*'))
                        .and(term_it)
                        .map(|(v1, v2)| v1 * v2)
                        .or(factor_parser.clone()),
                )
            });

            Box::new(
                term_parser
                    .clone()
                    .left(char('+'))
                    .and(expr_it)
                    .map(|(v1, v2)| v1 + v2)
                    .or(term_parser),
            )
        });

        let mut st = StrState::new("1+2*(3+4)");
        let (res, logs) = expr_parser.exec(&mut st);

        assert_eq!(Some(15), res);
        assert_eq!(0, logs.len());
    }
}
