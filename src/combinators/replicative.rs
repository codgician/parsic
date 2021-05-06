use crate::core::{Parsable, ParseLogger};

/// Data structure for `many` combinator.
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
        let (mut st, mut lg) = (state.clone(), logger.clone());

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

/// Data structure for `some` combinator.
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
        let (mut st, mut lg) = (state.clone(), logger.clone());
        let mut res = vec![];

        while let Some(r) = self.0.parse(state, logger) {
            res.push(r);
            st = state.clone();
            lg = logger.clone();
        }

        *state = st;
        match res {
            v if v.is_empty() => None,
            _ => {
                *logger = lg; 
                Some(res)
            }
        }
    }
}

/// ## Combinator: `some` (function ver.)
pub fn some<S: Clone, P: Parsable<S>>(parser: P) -> SomeP<P> {
    SomeP::new(parser)
}

/// Data structure for `optional` combinator.
#[derive(Copy, Clone, Debug)]
pub struct OptionalP<P>(P);

impl<P> OptionalP<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<S: Clone, P: Parsable<S>> Parsable<S> for OptionalP<P> {
    type Result = Option<P::Result>;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        let st = state.clone();
        let lg = logger.clone();

        match self.0.parse(state, logger) {
            None => {
                *state = st;
                *logger = lg;
                Some(None)
            }
            x => Some(x),
        }
    }
}

/// ## Combinator: `optional` (function ver.)
pub fn optional<P>(parser: P) -> OptionalP<P> {
    OptionalP::new(parser)
}

/// Implements following method for `Parsable<S>`:
/// - `many`
/// - `some`
/// - `optional`
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

    /// ## Combinator: `optional`
    fn optional(self) -> OptionalP<Self>
    where
        Self: Sized,
        S: Clone,
    {
        OptionalP::new(self)
    }
}

impl<S, P: Parsable<S>> ReplicativeExt<S> for P {}

#[cfg(test)]
mod test_many {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, StrState};

    #[test]
    fn ok_nonempty() {
        let parser = char('y').many();

        let mut st = StrState::new("yyyyying");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec!['y', 'y', 'y', 'y', 'y']), res);
        assert_eq!("ing", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_empty() {
        let parser = many(char('y'));

        let mut st = StrState::new("ing");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec![]), res);
        assert_eq!("ing", st.as_stream());
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
        assert_eq!("cpnb", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = char('y').some();

        let mut st = StrState::new("cpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("cpnb", st.as_stream());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_optional {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::{char, StrState};

    #[test]
    fn ok_one() {
        let parser = char('y').optional();

        let mut st = StrState::new("yyyyycpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(Some('y')), res);
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_zero() {
        let parser = char('y').optional();

        let mut st = StrState::new("cpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(None), res);
        assert_eq!(0, logs.len());
    }
}
