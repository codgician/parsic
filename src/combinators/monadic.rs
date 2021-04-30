use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;

#[derive(Clone, Copy, Debug)]
pub struct Pure<F>(pub F);

impl<F, S, T> Parsable<S, T> for Pure<F>
    where F: Fn() -> T
{
    fn parse(&self, _: &mut S, _: &mut ParseLogger) -> Option<T> {
        Some((self.0)())
    }
}

/// Pure Combinator
pub fn pure<F, T>(x: F) -> Pure<F> where F: Fn() -> T {
    Pure(x)
}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::primitives::*;

    // Should construct a parser that consumes nothing
    // and returns provided parse result
    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(true),
            super::pure(|| true).parse(&mut st, &mut log)
        );
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn empty_input() {
        let mut st = StrState::new("");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(true),
            super::pure(|| true).parse(&mut st, &mut log)
        );
        assert_eq!("", st.as_stream());
        assert_eq!(0, log.len());
    }
}
