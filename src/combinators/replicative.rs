use crate::core::{Parsable, ParseLogger};

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
    P: Parsable<S>,
{
    type Result = Vec<P::Result>;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
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

/// ## Combinator: `many` (function ver.)
pub fn many<S, P>(parser: P) -> ManyP<P>
where
    P: Parsable<S>,
{
    ManyP::new(parser)
}

// Some
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

/// ## Combinator: `some` (function ver.)
pub fn some<S: Clone, P: Parsable<S>>(parser: P) -> SomeP<P> {
    SomeP::new(parser)
}

pub trait ReplicativeExt<S>: Parsable<S> {
    /// ## Combinator: `many`
    fn many(self) -> ManyP<Self>
    where
        Self: Sized,
        S: Clone,
    {
        ManyP::new(self)
    }

    /// ## Combinator: `some`
    fn some(self) -> SomeP<Self>
    where
        Self: Sized,
        S: Clone,
    {
        SomeP::new(self)
    }
}

impl<S, P: Parsable<S>> ReplicativeExt<S> for P {}

#[cfg(test)]
mod test_many {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::{char, StrState};

    #[test]
    fn ok_nonempty() {
        let parser = char('y').many();

        let mut st = StrState::new("yyyyying");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec!['y', 'y', 'y', 'y', 'y']), res);
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_empty() {
        let parser = many(char('y'));

        let mut st = StrState::new("ing");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec![]), res);
        assert_eq!(0, logs.len());
    }
}

#[cfg(test)]
mod test_some {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::{char, StrState};

    #[test]
    fn ok() {
        let parser = Parser::new(char('y').some());

        let mut st = StrState::new("yyyyycpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec!['y', 'y', 'y', 'y', 'y']), res);
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = char('y').some();

        let mut st = StrState::new("cpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!(1, logs.len());
    }
}
