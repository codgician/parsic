use std::marker::PhantomData;
use crate::core::*;

// Empty
#[derive(Copy, Clone, Default, Debug)]
pub struct EmptyP<S, T>(PhantomData<S>, PhantomData<fn() -> Option<T>>);

impl<S, T> EmptyP<S, T> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<S, T> Parsable<S> for EmptyP<S, T> {
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger)
        -> Option<Self::Result>
    {
        None
    }
}

/// ### Combinator: `empty`
/// A parser that consumes no item and always fails.
pub fn empty<S, T>() -> EmptyP<S, T> {
    EmptyP::new()
}

// Pure
#[derive(Clone, Copy, Debug)]
pub struct PureP<T>(T);

impl<T> PureP<T> {
    pub fn new(item: T) -> Self {
        Self(item)
    }
}

impl<S, T: Clone> Parsable<S> for PureP<T> {
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger)
        -> Option<Self::Result>
    {
        Some(self.0.clone())
    }
}

/// ### Combinator: `pure`
/// Injects a value into an identity parser.
pub fn pure<T: Copy>(item: T) -> PureP<T> {
    PureP::new(item)
}

// Compose
#[derive(Copy, Clone, Debug)]
pub struct ComposeP<P1, P2, T>(P1, P2, PhantomData<T>);

impl<P1, P2, T> ComposeP<P1, P2, T> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self(p1, p2, PhantomData)
    }
}

impl<P1, P2, F, S, T> Parsable<S> for ComposeP<P1, P2, T>
where
    F: Fn(P2::Result) -> T,
    P1: Parsable<S, Result = F>,
    P2: Parsable<S>
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        match self.0.parse(state, logger) {
            None => None,
            Some(f) => match self.1.parse(state, logger) {
                None => None,
                Some(x) => Some(f(x))
            }
        }
    }
}

pub fn compose<P1, P2, F, S, T>(p1: P1, p2: P2) -> ComposeP<P1, P2, T>
where
    F: Fn(P2::Result) -> T,
    P1: Parsable<S, Result = F>,
    P2: Parsable<S>
{
    ComposeP::new(p1, p2)
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
    /// ### Combinator: `compose`
    fn compose<P, T>(self, parser: P) -> ComposeP<Self, P, T>
    where
        Self: Sized,
        Self::Result: Fn(P::Result) -> T,
        P: Parsable<S>
    {
        ComposeP::new(self, parser)
    }

    /// ### Combinator: `many`
    fn many(self) -> ManyP<Self>
    where
        Self: Sized
    {
        ManyP::new(self)
    }

    /// ### Combinator: `some`
    fn some(self) -> SomeP<Self>
    where
        Self: Sized
    {
        SomeP::new(self)
    }
}

impl<S, P: Parsable<S>> ApplicativeExt<S> for P {}

#[cfg(test)]
mod test_empty {
    use crate::core::*;
    use crate::primitives::StrState;
    use crate::combinators::*;

    #[test]
    fn fail() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None as Option<String>,
            empty().parse(&mut st, &mut log)
        );
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, log.len());
    }
}

#[cfg(test)]
mod test_compose {
    use crate::core::*;
    use crate::primitives::*;
    use super::*;

    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(true),
            pure(|x| x == 'H')
                .compose(char('H'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn fail() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            pure(|x| x == 'H')
                .compose(char('h'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, log.len());
    }

    #[test]
    fn apply_to_empty() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            pure(|_| true)
                .compose(empty::<StrState, bool>())
                .parse(&mut st, &mut log)
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
