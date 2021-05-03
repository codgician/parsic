use std::marker::PhantomData;
use crate::core::{ Parsable, ParseLogger };

// Bind
#[derive(Clone, Copy, Debug)]
pub struct BindP<F, P, T>(F, P, PhantomData<T>);

impl<F, P, T> BindP<F, P, T> {
    pub fn new(func: F, parser: P) -> Self {
        Self(func, parser, PhantomData)
    }
}

impl<F, P1, P2, S> Parsable<S> for BindP<F, P1, P2>
where
    F: Fn(P1::Result) -> P2,
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    type Result = P2::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        match self.1.parse(state, logger) {
            Some(r1) => self.0(r1).parse(state, logger),
            _ => None
        }
    }
}

/// ### Combinator: `bind` (function variant)
/// Monadic bind operator for context sensitive parsing 
pub fn bind<F, P, S, T>(func: F, parser: P) -> BindP<F, P, T>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>
{
    BindP::new(func, parser)
}

pub trait BindPExt<S> : Parsable<S> {
    /// ### Combinator: `bind`
    fn bind<F, T>(self, func: F) -> BindP<F, Self, T>
    where
        Self: Sized,
        F: Fn(Self::Result) -> T,
    {
        BindP::new(func, self)
    }
}

impl<S, P: Parsable<S>> BindPExt<S> for P {}

#[cfg(test)]
mod test_bind {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let parser = satisfy(|_| true)
                    .bind(|ch: char| if ch.is_uppercase() {
                        char('+')
                    } else {
                        char('-')
                    });

        let mut st1 = StrState::new("A+");
        let mut st2 = StrState::new("a-");

        let (res1, logs1) = parse(parser, &mut st1);
        let (res2, logs2) = parse(parser, &mut st2);

        assert_eq!((Some('+'), Some('-')), (res1, res2));
        assert_eq!(("", ""), (st1.as_stream(), st2.as_stream()));
        assert_eq!((0, 0), (logs1.len(), logs2.len()));
    }
}
