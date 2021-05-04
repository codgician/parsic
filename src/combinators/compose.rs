use crate::core::{Parsable, ParseLogger};
use std::marker::PhantomData;

/// Data structure for `compose` combinator.
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
    P2: Parsable<S>,
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.0.parse(state, logger) {
            None => None,
            Some(f) => match self.1.parse(state, logger) {
                None => None,
                Some(x) => Some(f(x)),
            },
        }
    }
}

/// ## Combinator: `compose` (function ver.)
/// Functional composition between parsers.
pub fn compose<P1, P2, F, S, T>(p1: P1, p2: P2) -> ComposeP<P1, P2, T>
where
    F: Fn(P2::Result) -> T,
    P1: Parsable<S, Result = F>,
    P2: Parsable<S>,
{
    ComposeP::new(p1, p2)
}

/// Implements `compose` method for `Parsable<S>`.
pub trait ComposePExt<S>: Parsable<S> {
    /// ## Combinator: `compose`
    /// Functional composition between parsers.
    fn compose<P, T>(self, parser: P) -> ComposeP<Self, P, T>
    where
        Self: Sized,
        Self::Result: Fn(P::Result) -> T,
        P: Parsable<S>,
    {
        ComposeP::new(self, parser)
    }
}

impl<S, P: Parsable<S>> ComposePExt<S> for P {}

#[cfg(test)]
mod test_compose {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, StrState};

    #[test]
    fn ok() {
        let parser = pure(|x| x == 'H').compose(char('H'));

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(true), res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = pure(|x| x == 'H').compose(char('h'));

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn compose_with_empty() {
        let parser = pure(|_| true).compose(empty::<StrState, bool>());

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, logs.len());
    }
}
