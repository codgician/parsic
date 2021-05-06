use crate::core::{Parsable, ParseLogger};

/// Data structure for `or` combinator.
#[derive(Clone, Copy, Debug)]
pub struct OrP<P1, P2>(P1, P2);

impl<P1, P2> OrP<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self(p1, p2)
    }
}

impl<S, P1, P2> Parsable<S> for OrP<P1, P2>
where
    S: Clone,
    P1: Parsable<S>,
    P2: Parsable<S, Result = P1::Result>,
{
    type Result = P1::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        let (st, lg) = (state.clone(), logger.clone());
        match self.0.parse(state, logger) {
            None => {
                *state = st.clone();
                *logger = lg;
                match self.1.parse(state, logger) {
                    None => {
                        *state = st;
                        None
                    }
                    x => x
                }
            }
            x => x,
        }
    }
}

/// ## Combinator: `or` (function ver.)
/// Alternative combinator.
pub fn or<P1, P2>(p1: P1, p2: P2) -> OrP<P1, P2> {
    OrP::new(p1, p2)
}

/// Implements `or` method for `Parsable<S>`.
pub trait OrExt<S>: Parsable<S> {
    /// ## Combinator: `or`
    /// Alternative combinator.
    fn or<P>(self, parser: P) -> OrP<Self, P>
    where
        Self: Sized,
        P: Parsable<S>,
        S: Clone,
    {
        OrP::new(self, parser)
    }
}

impl<S, P: Parsable<S>> OrExt<S> for P {}

#[cfg(test)]
mod test_or {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, StrState};

    #[test]
    fn left_ok() {
        let parser = char('A').or(char('B'));

        let mut st = StrState::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('A'), res);
        assert_eq!("hhh", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn right_ok() {
        let parser = char('B').or(char('A'));

        let mut st = StrState::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('A'), res);
        assert_eq!("hhh", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn both_ok() {
        let parser = char('A').or(char('A'));

        let mut st = StrState::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('A'), res);
        assert_eq!("hhh", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn both_fail() {
        let parser = char('B').or(char('C'));

        let mut st = StrState::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Ahhh", st.as_stream());
        assert_eq!(1, logs.len());
    }
}
