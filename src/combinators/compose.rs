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

pub trait ComposePExt<S> : Parsable<S> {
    /// ### Combinator: `compose`
    fn compose<P, T>(self, parser: P) -> ComposeP<Self, P, T>
    where
        Self: Sized,
        Self::Result: Fn(P::Result) -> T,
        P: Parsable<S>
    {
        ComposeP::new(self, parser)
    }
}

impl<S, P: Parsable<S>> ComposePExt<S> for P {}

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
        let parser = pure(|x| x == 'H')
                    .compose(char('H'));

        let mut st = StrState::new("Hello");
        let (res, logs) = parse(parser, &mut st);

        assert_eq!(Some(true), res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = pure(|x| x == 'H')
                    .compose(char('h'));

        let mut st = StrState::new("Hello");
        let (res, logs) = parse(parser, &mut st);

        assert_eq!(None, res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn apply_to_empty() {
        let parser = pure(|_| true)
                    .compose(empty::<StrState, bool>());

        let mut st = StrState::new("Hello");
        let (res, logs) = parse(parser, &mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, logs.len());
    }
}
