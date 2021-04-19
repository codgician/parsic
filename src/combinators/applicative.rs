use std::marker::PhantomData;

use crate::core::parser::{ Parser, ParseState };

// Empty
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

// Many
pub struct Many<P> {
    parser: P
}

impl<'a, P, T> Parser<ParseState<'a>> for Many<P>
    where P: Parser<ParseState<'a>, ParsedType = T>
{
    type ParsedType = Vec<T>;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        let mut res = vec![];
        let mut st = state.clone();

        while let Some(r) = self.parser.parse(state) {
            res.push(r);
            st = state.clone();
        }

        *state = st;
        Some(res)
    }
}

pub fn many<'a, P, T>(parser: P) -> Many<P> 
    where P: Parser<ParseState<'a>, ParsedType = T>
{
    Many { parser: parser }
}

// Some
pub struct Some<P> {
    parser: P
}

impl<'a, P, T> Parser<ParseState<'a>> for Some<P>
    where P: Parser<ParseState<'a>, ParsedType = T>
{
    type ParsedType = Vec<T>;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        let mut res = vec![self.parser.parse(state)?];
        let mut st = state.clone();

        while let Some(r) = self.parser.parse(state) {
            res.push(r);
            st = state.clone();
        }

        *state = st;
        Some(res)
    }
}

pub fn some<'a, P, T>(parser: P) -> Some<P> 
    where P: Parser<ParseState<'a>, ParsedType = T>
{
    Some { parser: parser }
}

#[cfg(test)]
mod test_empty {
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

#[cfg(test)]
mod test_many {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::char::char;

    #[test]
    fn ok_nonempty() {
        let mut st = ParseState::new("yyyyying");
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            super::many(char('y')).parse(&mut st)
        )
    }

    #[test]
    fn ok_empty() {
        let mut st = ParseState::new("ing");
        assert_eq!(
            Some(vec![]),
            super::many(char('y')).parse(&mut st)
        )
    }
}

#[cfg(test)]
mod test_some {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::char::char;

    #[test]
    fn ok() {
        let mut st = ParseState::new("yyyyying");
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            super::some(char('y')).parse(&mut st)
        )
    }

    #[test]
    fn fail() {
        let mut st = ParseState::new("ing");
        assert_eq!(
            None,
            super::some(char('y')).parse(&mut st)
        )
    }
}
