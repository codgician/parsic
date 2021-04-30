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
pub fn and<T1, T2>(p1: T1, p2: T2) -> AndP<T1, T2> {
    AndP(p1, p2)
}

pub trait SequentialPExt<S> : Parsable<S> {
    /// And combinator
    fn and<P>(self, parser: P) -> AndP<Self, P>
        where 
            Self: Sized, 
            P: Parsable<S>
    {
        AndP(self, parser)
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
