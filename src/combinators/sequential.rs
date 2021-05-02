use crate::core::{ Parsable, ParseLogger };
use crate::combinators::MapP;

// And combinator
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
    P2: Parsable<S>
{
    type Result = (P1::Result, P2::Result);

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        match self.0.parse(state, logger) {
            None => None,
            Some(r1) => {
                match self.1.parse(state, logger) {
                    None => None,
                    Some(r2) => Some((r1, r2))
                }
            }
        }
    }
}

/// ### Combinator: `and` (function variant)
pub fn and<S, P1, P2>(p1: P1, p2: P2) -> AndP<P1, P2>
where
    P1: Parsable<S>,
    P2: Parsable<S>
{
    AndP::new(p1, p2)
}

/// ### Combinator: `left` (function variant)
pub fn left<S, P1, P2>(p1: P1, p2: P2)
    -> MapP<fn((P1::Result, P2::Result)) -> P1::Result, AndP<P1, P2>>
where
    P1: Parsable<S>,
    P2: Parsable<S>
{
    MapP::new(|(l, _)| l, AndP::new(p1, p2))
}

/// ### Combinator: `right` (function variant)
pub fn right<S, P1, P2>(p1: P1, p2: P2)
    -> MapP<fn((P1::Result, P2::Result)) -> P2::Result, AndP<P1, P2>>
where
    P1: Parsable<S>,
    P2: Parsable<S>
{
    MapP::new(|(_, r)| r, AndP::new(p1, p2))
}

/// ### Combinator: `mid` (function variant)
pub fn mid<S, P1, P2, P3>(p1: P1, p2: P2, p3: P3)
    -> MapP<fn(((P1::Result, P2::Result), P3::Result)) -> P2::Result,
        AndP<AndP<P1, P2>, P3>>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
    P3: Parsable<S>
{
    MapP::new(|((_, m), _)| m, AndP::new(AndP::new(p1, p2), p3))
}

pub trait SequentialPExt<S> : Parsable<S> {
    /// ### Combinator: `and`
    fn and<P>(self, parser: P)-> AndP<Self, P>
    where
        Self: Sized,
        P: Parsable<S>
    {
        AndP::new(self, parser)
    }

    /// ### Combinator: `left`
    fn left<P>(self, parser: P)
        -> MapP<fn((Self::Result, P::Result)) -> Self::Result, AndP<Self, P>>
    where
        Self: Sized,
        P: Parsable<S>
    {
        MapP::new(|(l, _)| l, AndP::new(self, parser))
    }

    /// ### Combinator: `right`
    fn right<P>(self, parser: P)
        -> MapP<fn((Self::Result, P::Result)) -> P::Result, AndP<Self, P>>
    where
        Self: Sized,
        P: Parsable<S>
    {
        MapP::new(|(_, r)| r, AndP::new(self, parser))
    }

    /// ### Combinator: `mid`
    fn mid<P1, P2>(self, p1: P1, p2: P2)
        -> MapP<fn(((Self::Result, P1::Result), P2::Result)) -> P1::Result,
            AndP<AndP<Self, P1>, P2>>
    where
        Self: Sized,
        P1: Parsable<S>,
        P2: Parsable<S>
    {
        MapP::new(|((_, m), _)| m, AndP::new(AndP::new(self, p1), p2))
    }
}

impl<S, P: Parsable<S>> SequentialPExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok_same_type() {
        let mut st = StrState::new("ABC");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(('A', 'B')),
            char('A')
                .and(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("C", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn ok_different_type() {
        let mut st = StrState::new("1A+");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some((1, 'A')),
            satisfy(|&ch| ch.is_digit(10))
                .bind_option(|ch| ch.to_digit(10))
                .and(char('A'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("+", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn left_fail() {
        let mut st = StrState::new("BBC");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('A')
                .and(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("BC", st.as_stream());
        assert_eq!(1, log.len());
    }

    #[test]
    fn right_fail() {
        let mut st = StrState::new("ACC");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('A')
                .and(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("C", st.as_stream());
        assert_eq!(1, log.len());
    }

    #[test]
    fn both_fail() {
        let mut st = StrState::new("CCC");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('A')
                .and(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("CC", st.as_stream());
        assert_eq!(1, log.len());
    }
}
