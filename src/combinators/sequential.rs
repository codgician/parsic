use crate::combinators::MapP;
use crate::core::{Parsable, ParseLogger};

/// Data structure for `and` combinator.
#[derive(Clone, Copy, Debug)]
pub struct AndP<P1, P2>(P1, P2);

impl<P1, P2> AndP<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self(p1, p2)
    }
}

impl<S, P1, P2> Parsable<S> for AndP<P1, P2>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    type Result = (P1::Result, P2::Result);

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.0.parse(state, logger) {
            None => None,
            Some(r1) => match self.1.parse(state, logger) {
                None => None,
                Some(r2) => Some((r1, r2)),
            },
        }
    }
}

/// ## Combinator: `and` (function ver.)
pub fn and<S, P1, P2>(p1: P1, p2: P2) -> AndP<P1, P2>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    AndP::new(p1, p2)
}

/// Type declaration for `left` combinator.
pub type LeftP<P1, P2, T1, T2> =
    MapP<fn((T1, T2)) -> T1, AndP<P1, P2>>;

/// ## Combinator: `left` (function ver.)
pub fn left<S, P1, P2>(
    p1: P1,
    p2: P2,
) -> LeftP<P1, P2, P1::Result, P2::Result>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    MapP::new(|(l, _)| l, AndP::new(p1, p2))
}

/// Type declaration for `right` combinator.
pub type RightP<P1, P2, T1, T2> =
    MapP<fn((T1, T2)) -> T2, AndP<P1, P2>>;

/// ## Combinator: `right` (function ver.)
pub fn right<S, P1, P2>(
    p1: P1,
    p2: P2,
) -> RightP<P1, P2, P1::Result, P2::Result>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    MapP::new(|(_, r)| r, AndP::new(p1, p2))
}

/// Type declaration for `mid` combinator.
pub type MidP<P1, P2, P3, T1, T2, T3> =
    MapP<fn(((T1, T2), T3)) -> T2, AndP<AndP<P1, P2>, P3>>;

/// ## Combinator: `mid` (function ver.)
pub fn mid<S, P1, P2, P3>(
    p1: P1,
    p2: P2,
    p3: P3,
) -> MidP<P1, P2, P3, P1::Result, P2::Result, P3::Result>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
    P3: Parsable<S>,
{
    MapP::new(|((_, m), _)| m, AndP::new(AndP::new(p1, p2), p3))
}

/// Implements following methods for `Parsable<S>`:
/// - `and`
/// - `left`
/// - `right`
/// - `mid`
pub trait SequentialPExt<S>: Parsable<S> {
    /// ## Combinator: `and`
    fn and<P>(self, parser: P) -> AndP<Self, P>
    where
        Self: Sized,
        P: Parsable<S>,
    {
        AndP::new(self, parser)
    }

    /// ## Combinator: `left`
    fn left<P>(self, parser: P) -> LeftP<Self, P, Self::Result, P::Result>
    where
        Self: Sized,
        P: Parsable<S>,
    {
        MapP::new(|(l, _)| l, AndP::new(self, parser))
    }

    /// ## Combinator: `right`
    fn right<P>(self, parser: P) -> RightP<Self, P, Self::Result, P::Result>
    where
        Self: Sized,
        P: Parsable<S>,
    {
        MapP::new(|(_, r)| r, AndP::new(self, parser))
    }

    /// ## Combinator: `mid`
    fn mid<P1, P2>(
        self,
        p1: P1,
        p2: P2,
    ) -> MidP<Self, P1, P2, Self::Result, P1::Result, P2::Result>
    where
        Self: Sized,
        P1: Parsable<S>,
        P2: Parsable<S>,
    {
        MapP::new(|((_, m), _)| m, AndP::new(AndP::new(self, p1), p2))
    }
}

impl<S, P: Parsable<S>> SequentialPExt<S> for P {}

#[cfg(test)]
mod test_and {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, StrState};

    #[test]
    fn same_type_ok() {
        let parser = char('A').and(char('B'));

        let mut st = StrState::new("ABC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(('A', 'B')), res);
        assert_eq!("C", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn different_type_ok() {
        let parser = satisfy(|&ch| ch.is_digit(10))
            .map_opt(|ch| ch.to_digit(10))
            .and(char('A'));

        let mut st = StrState::new("1A+");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some((1, 'A')), res);
        assert_eq!("+", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn left_fail() {
        let parser = char('A').and(char('B'));

        let mut st = StrState::new("BBC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("BC", st.as_stream());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn right_fail() {
        let parser = char('A').and(char('B'));

        let mut st = StrState::new("ACC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("C", st.as_stream());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn both_fail() {
        let parser = char('A').and(char('B'));

        let mut st = StrState::new("CCC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("CC", st.as_stream());
        assert_eq!(1, logs.len());
    }
}
