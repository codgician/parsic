use crate::core::parser::Parser;

#[derive(Clone, Copy, Debug)]
pub struct Pure<F> {
    x: F,
}

impl<F> Pure<F> {
    pub fn new(x: F) -> Pure<F> {
        Self { x: x }
    }
}

impl<F, S, T> Parser<S> for Pure<F>
    where F: Fn() -> T
{
    type ParsedType = T;

    fn parse(&self, _: &mut S) -> Option<Self::ParsedType> {
        Some((self.x)())
    }
}

pub fn pure<F, T>(x: F) -> Pure<F>
    where F: Fn() -> T
{
    Pure::new(x)
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };

    // Should construct a parser that consumes nothing
    // and returns provided parse result
    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            Some(true),
            super::pure(|| true).parse(&mut st)
        );
        assert_eq!("Hello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn empty_input() {
        let mut st = ParseState::new("");
        assert_eq!(
            Some(true),
            super::pure(|| true).parse(&mut st)
        );
        assert_eq!("", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }
}
