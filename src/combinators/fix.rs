use std::rc::Rc;
use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;

#[derive(Clone)]
pub struct FixP<'a, S, T>
    (Rc<dyn for<'b> Fn(&'b Self) -> Box<dyn Parsable<S, T> + 'b> + 'a>);

impl<'a, S, T> Parsable<S, T> for FixP<'a, S, T> {
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<T> {
        // Fixed-point Combinator: fix f = f (fix f)
        (self.0)(self).parse(stream, logger)
    }
}

pub fn fix<'a, S, T, F>(fix: F) -> FixP<'a, S, T>
    where
        F: for<'b> Fn(&'b FixP<'a, S, T>)
            -> Box<dyn Parsable<S, T> + 'b> + 'a,
{
    FixP(Rc::new(fix))
}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn simple_recursive_syntax() {
        // expr := '1' expr | '0'
        let expr_parser = fix(|it| Box::new(
            char('1')
            .and(it)
            .map(|(_, x)| x)
            .or(char('0'))
        ));

        let mut st = StrState::new("1110");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('0'),
            expr_parser.parse(&mut st, &mut log)
        );
        assert_eq!(0, log.len());
    }

    #[test]
    fn mutual_recursive_syntax() {
        // expr     := term '+' expr | term
        // term     := factor '*' term | factor
        // factor   := '(' expr ')' | nat
        // nat      := digit { digit }
        // digit    := '0' | '1' | ... | '9'
        let expr_parser = fix(|expr_it| {
            let digit_parser = 
                satisfy(|&ch| ch.is_digit(10));

            let nat_parser = 
                digit_parser
                    .some()
                    .map(
                        |v| v.iter()
                            .collect::<String>()
                            .parse::<i64>().unwrap()
                    );

            let parentheses_expr_parser = 
                char('(')
                    .and(expr_it)
                    .map(|(_, v)| v)
                    .and(char(')'))
                    .map(|(v, _)| v);

            let factor_parser = 
                parentheses_expr_parser
                    .or(nat_parser);

            let term_parser = fix(move |term_it| Box::new(
                factor_parser.clone()
                    .and(char('*'))
                    .map(|(v, _)| v)
                    .and(term_it)
                    .map(|(v1, v2)| v1 * v2)
                    .or(factor_parser.clone())
            ));

            Box::new(
                term_parser.clone()
                    .and(char('+'))
                    .map(|(v, _)| v)
                    .and(expr_it)
                    .map(|(v1, v2)| v1 + v2)
                    .or(term_parser)
            )
        }); 

        let mut st = StrState::new("(1+2)*(3+4)");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(21),
            expr_parser.parse(&mut st, &mut log)
        );
        assert_eq!(0, log.len());
    }
}
