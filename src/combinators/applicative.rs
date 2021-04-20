use std::marker::PhantomData;

use crate::core::parser::Parser;

// Empty
#[derive(Copy, Clone, Debug)]
pub struct Empty<T> {
    __marker: PhantomData<fn() -> Option<T>>
}

impl<T> Empty<T> {
    pub fn new() -> Self {
        Self { __marker: PhantomData }
    }
}

impl<T> Default for Empty<T> {
    fn default() -> Self {
        Self::new()
    }
} 

impl<S, T> Parser<S> for Empty<T> {
    type ParsedType = T;

    fn parse(&self, _: &mut S) -> Option<Self::ParsedType> {
        None
    }
}

/// Empty Parser Builder
pub fn empty<T>() -> Empty<T> {
    Empty::new()
}

// Many
pub struct Many<P> {
    parser: P
}

impl<P> Many<P> {
    pub fn new(parser: P) -> Self {
        Self { parser: parser }
    }
}

impl<S: Clone, P, T> Parser<S> for Many<P>
    where P: Parser<S, ParsedType = T>
{
    type ParsedType = Vec<T>;

    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
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

// Many Combinator
pub fn many<S, P, T>(parser: P) -> Many<P> 
    where P: Parser<S, ParsedType = T>
{
    Many::new(parser)
}

pub struct Some<P> {
    parser: P
}

impl<P> Some<P> {
    pub fn new(parser: P) -> Self {
        Self { parser: parser }
    }
}

impl<S: Clone, P, T> Parser<S> for Some<P>
    where P: Parser<S, ParsedType = T>
{
    type ParsedType = Vec<T>;

    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
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

/// Some Combinator
pub fn some<S, P, T>(parser: P) -> Some<P> 
    where P: Parser<S, ParsedType = T>
{
    Some::new(parser)
}

// Implement iterator-style method for Parser trait
pub trait ApplicativeExt<S> : Parser<S> {
    /// Many Combinator
    fn many(self) -> Many<Self>
        where Self: Sized
    {
        Many::new(self)
    }

    /// Some combinator
    fn some(self) -> Some<Self>
        where Self: Sized
    {
        Some::new(self)
    }
}

impl<S, P: Parser<S>> ApplicativeExt<S> for P {}

#[cfg(test)]
mod test_empty {
    use crate::core::parser::{ Parser, ParseState };

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
    use crate::combinators::*;

    #[test]
    fn ok_nonempty() {
        let mut st = ParseState::new("yyyyying");
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            char('y').many().parse(&mut st)
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
    use crate::combinators::*;

    #[test]
    fn ok() {
        let mut st = ParseState::new("yyyyying");
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            char('y').some().parse(&mut st)
        )
    }

    #[test]
    fn fail() {
        let mut st = ParseState::new("ing");
        assert_eq!(
            None,
            char('y').some().parse(&mut st)
        )
    }
}
