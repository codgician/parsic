use crate::core::{ Parsable, ParseLogger };

// And combinator
#[derive(Clone, Copy, Debug)]
pub struct AndP<PA, PB>(PA, PB);

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

/// And combinator
pub fn and<P1, P2>(p1: P1, p2: P2) -> AndP<P1, P2> {
    AndP(p1, p2)
}

// Combinator: left
#[derive(Clone, Copy, Debug)]
pub struct LeftP<PA, PB>(PA, PB);

impl<S, P1, P2> Parsable<S> for LeftP<P1, P2>
where 
    P1: Parsable<S>, 
    P2: Parsable<S>
{
    type Result = P1::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        match self.0.parse(state, logger) {
            None => None,
            Some(r1) => {
                match self.1.parse(state, logger) {
                    None => None,
                    _ => Some(r1)
                }
            }
        }
    }
}

/// ## Combinator: `left`
pub fn left<P1, P2>(p1: P1, p2: P2) -> LeftP<P1, P2> {
    LeftP(p1, p2)
}

// Combinator: right
#[derive(Clone, Copy, Debug)]
pub struct RightP<PA, PB>(PA, PB);

impl<S, P1, P2> Parsable<S> for RightP<P1, P2>
where 
    P1: Parsable<S>, 
    P2: Parsable<S>
{
    type Result = P2::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        match self.0.parse(state, logger) {
            None => None,
            _ => {
                match self.1.parse(state, logger) {
                    None => None,
                    Some(r2) => Some(r2)
                }
            }
        }
    }
}

/// ## Combinator: `right`
pub fn right<P1, P2>(p1: P1, p2: P2) -> RightP<P1, P2> {
    RightP(p1, p2)
}

pub trait SequentialPExt<S> : Parsable<S> {
    /// ## Combinator: `and`
    fn and<P>(self, parser: P) -> AndP<Self, P>
    where 
        Self: Sized, 
        P: Parsable<S>
    {
        AndP(self, parser)
    }

    /// ## Combinator: `left`
    fn left<P>(self, parser: P) -> LeftP<Self, P>
    where
        Self: Sized,
        P: Parsable<S>
    {
        LeftP(self, parser)
    }

    /// ## Combinator: `right`
    fn right<P>(self, parser: P) -> RightP<Self, P>
    where
        Self: Sized,
        P: Parsable<S>
    {
        RightP(self, parser)
    }
}

impl<S, P: Parsable<S>> SequentialPExt<S> for P {}

#[cfg(test)]
mod test_and {
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

#[cfg(test)]
mod test_left {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok_same_type() {
        let mut st = StrState::new("ABC");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('A'),
            char('A')
                .left(char('B'))
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
            Some(1),
            satisfy(|&ch| ch.is_digit(10))
                .bind_option(|ch| ch.to_digit(10))
                .left(char('A'))
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
                .left(char('B'))
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
                .left(char('B'))
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
                .left(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("CC", st.as_stream());
        assert_eq!(1, log.len());
    }
}

#[cfg(test)]
mod test_right {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok_same_type() {
        let mut st = StrState::new("ABC");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('B'),
            char('A')
                .right(char('B'))
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
            Some('A'),
            satisfy(|&ch| ch.is_digit(10))
                .bind_option(|ch| ch.to_digit(10))
                .right(char('A'))
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
                .right(char('B'))
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
                .right(char('B'))
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
                .right(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("CC", st.as_stream());
        assert_eq!(1, log.len());
    }
}
