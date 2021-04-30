use std::marker::PhantomData;
use crate::core::*;

// Empty
#[derive(Copy, Clone, Default, Debug)]
pub struct EmptyP<T>(PhantomData<fn() -> Option<T>>);

impl<S, T> Parsable<S> for EmptyP<T> {
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        None
    }
}

/// Empty Parsable Builder
pub fn empty<T>() -> EmptyP<T> {
    EmptyP(PhantomData)
}

// Many
#[derive(Copy, Clone, Debug)]
pub struct ManyP<P>(P);

impl<S, P> Parsable<S> for ManyP<P>
    where S: Clone, P: Parsable<S>
{
    type Result = Vec<P::Result>;

    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        let mut res = vec![];
        let mut st = stream.clone();
        let mut lg = logger.clone();

        while let Some(r) = self.0.parse(stream, logger) {
            res.push(r);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        *logger = lg;
        Some(res)
    }
}

// Many Combinator
pub fn many<S, P>(parser: P) -> ManyP<P>
    where P: Parsable<S> 
{
    ManyP(parser)
}

// Some Combinator
#[derive(Copy, Clone, Debug)]
pub struct SomeP<P>(P);

impl<S: Clone, P: Parsable<S>> Parsable<S> for SomeP<P> {
    type Result = Vec<P::Result>;

    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        let mut res = vec![self.0.parse(stream, logger)?];
        let mut st = stream.clone();
        let mut lg = logger.clone();

        while let Some(r) = self.0.parse(stream, logger) {
            res.push(r);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        *logger = lg;
        Some(res)
    }
}

/// Some Combinator
pub fn some<S, P: Parsable<S>>(parser: P) -> SomeP<P> {
    SomeP(parser)
}

// Implement iterator-style method for Parsable trait
pub trait ApplicativeExt<S> : Parsable<S> {
    /// Many Combinator
    fn many(self) -> ManyP<Self> where Self: Sized {
        ManyP(self)
    }

    /// Some combinator
    fn some(self) -> SomeP<Self> where Self: Sized {
        SomeP(self)
    }
}

impl<S, P: Parsable<S>> ApplicativeExt<S> for P {}

#[cfg(test)]
mod test_empty {
    use crate::core::*;
    use crate::primitives::StrState;

    #[test]
    fn fail() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None as Option<String>,
            super::empty().parse(&mut st, &mut log)
        );
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, log.len());
    }
}

#[cfg(test)]
mod test_many {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::{ StrState, char };

    #[test]
    fn ok_nonempty() {
        let mut st = StrState::new("yyyyying");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            char('y').many().parse(&mut st, &mut log)
        )
    }

    #[test]
    fn ok_empty() {
        let mut st = StrState::new("ing");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(vec![]),
            super::many(char('y')).parse(&mut st, &mut log)
        )
    }
}

#[cfg(test)]
mod test_some {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::{ StrState, char };

    #[test]
    fn ok() {
        let mut st = StrState::new("yyyyying");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            char('y').some().parse(&mut st, &mut log)
        )
    }

    #[test]
    fn fail() {
        let mut st = StrState::new("ing");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('y').some().parse(&mut st, &mut log)
        )
    }
}
