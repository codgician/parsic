use std::marker::PhantomData;

use crate::core::parser::{ Parser, ParseState };

#[derive(Copy, Clone, Debug, Default)]
pub struct Empty<T> {
    __marker: PhantomData<fn() -> Option<T>>
}

impl<'a, T> Parser<ParseState<'a>> for Empty<T>
{
    type ParsedType = T;

    fn parse(&self, _: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        None
    }
}

pub fn empty<T>() -> Empty<T> {
    Empty { __marker: PhantomData }
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };

    // Constructs a parser that consumes no input
    // and always fails
    #[test]
    fn fail() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            None as Option<String>,
            super::empty().parse(&mut st)
        );
        assert_eq!("Hello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }
}
