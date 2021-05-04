use crate::core::{Parsable, ParseLogger};
use std::marker::PhantomData;

/// Data structure for `empty` combinator.
#[derive(Copy, Clone, Default, Debug)]
pub struct EmptyP<S, T>(PhantomData<(S, T)>);

impl<S, T> EmptyP<S, T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<S, T> Parsable<S> for EmptyP<S, T> {
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger) -> Option<Self::Result> {
        None
    }
}

/// ### Combinator: `empty`
/// A parser that consumes no item and always fails.
pub fn empty<S, T>() -> EmptyP<S, T> {
    EmptyP::new()
}

/// Data structure for `pure` combinator.
#[derive(Clone, Copy, Debug)]
pub struct PureP<S, T>(T, PhantomData<S>);

impl<S, T> PureP<S, T> {
    pub fn new(item: T) -> Self {
        Self(item, PhantomData)
    }
}

impl<S, T: Clone> Parsable<S> for PureP<S, T> {
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger) -> Option<Self::Result> {
        Some(self.0.clone())
    }
}

/// ### Combinator: `pure`
/// Injects a value into an identity parser.
pub fn pure<S, T: Copy>(item: T) -> PureP<S, T> {
    PureP::new(item)
}

#[cfg(test)]
mod test_empty {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::StrState;

    #[test]
    fn should_always_fail() {
        let parser = empty::<StrState, char>();

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, logs.len());
    }
}

#[cfg(test)]
mod test_pure {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::StrState;

    #[test]
    fn injects_value() {
        let parser = pure(true);

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(true), res);
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn injects_function() {
        let parser = pure(|_| true);

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(true, res.unwrap()(1));
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, logs.len());
    }
}
