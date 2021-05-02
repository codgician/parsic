use std::marker::PhantomData;
use crate::core::*;

// Empty
#[derive(Copy, Clone, Default, Debug)]
pub struct EmptyP<T>(PhantomData<fn() -> Option<T>>);

impl<T> EmptyP<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<S, T> Parsable<S> for EmptyP<T> {
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger)
        -> Option<Self::Result>
    {
        None
    }
}

/// ### Combinator: `empty`
pub fn empty<T>() -> EmptyP<T> {
    EmptyP::new()
}

// Many
#[derive(Copy, Clone, Debug)]
pub struct ManyP<P>(P);

impl<P> ManyP<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<S, P> Parsable<S> for ManyP<P>
where
    S: Clone,
    P: Parsable<S>
{
    type Result = Vec<P::Result>;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        let mut res = vec![];
        let mut st = state.clone();
        let mut lg = logger.clone();

        while let Some(r) = self.0.parse(state, logger) {
            res.push(r);
            st = state.clone();
            lg = logger.clone();
        }

        *state = st;
        *logger = lg;
        Some(res)
    }
}

/// ### Combinator: `many` (function variant)
pub fn many<S, P>(parser: P) -> ManyP<P>
where
    P: Parsable<S>
{
    ManyP::new(parser)
}

// Some Combinator
#[derive(Copy, Clone, Debug)]
pub struct SomeP<P>(P);

impl<P> SomeP<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<S: Clone, P: Parsable<S>> Parsable<S> for SomeP<P> {
    type Result = Vec<P::Result>;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        let mut res = vec![self.0.parse(state, logger)?];
        let mut st = state.clone();
        let mut lg = logger.clone();

        while let Some(r) = self.0.parse(state, logger) {
            res.push(r);
            st = state.clone();
            lg = logger.clone();
        }

        *state = st;
        *logger = lg;
        Some(res)
    }
}

/// ### Combinator: `some` (function variant)
pub fn some<S, P: Parsable<S>>(parser: P) -> SomeP<P> {
    SomeP::new(parser)
}

pub trait ApplicativeExt<S> : Parsable<S> {
    /// ### Combinator: `many`
    fn many(self) -> ManyP<Self> where Self: Sized {
        ManyP::new(self)
    }

    /// ### Combinator: `some`
    fn some(self) -> SomeP<Self> where Self: Sized {
        SomeP::new(self)
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
