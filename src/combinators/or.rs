use crate::core::{ Parsable, ParseLogger };

// Or
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
    P2: Parsable<S, Result = P1::Result>
{
    type Result = P1::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        let st0 = state.clone();
        let lg0 = logger.clone();
        match self.0.parse(state, logger) {
            None => {
                *state = st0;
                *logger = lg0;
                self.1.parse(state, logger)
            },
            x => x
        }
    }
}

/// ### Combinator: `or` (function variant)
pub fn or<P1, P2>(p1: P1, p2: P2) -> OrP<P1, P2> {
    OrP::new(p1, p2)
}

pub trait OrExt<S> : Parsable<S> {
    /// ### Combinator: `or`
    fn or<P>(self, parser: P) -> OrP<Self, P>
    where
        Self: Sized,
        P: Parsable<S>,
        S: Clone
    {
        OrP::new(self, parser)
    }
}

impl<S, P: Parsable<S>> OrExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn left_ok() {
        let mut st = StrState::new("Ahhh");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('A'),
            char('A')
                .or(char('B'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("hhh", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn right_ok() {
        let mut st = StrState::new("Ahhh");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('A'),
            char('B')
                .or(char('A'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("hhh", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn both_ok() {
        let mut st = StrState::new("Ahhh");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('A'),
            char('A')
                .or(char('A'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("hhh", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn both_fail() {
        let mut st = StrState::new("Ahhh");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('B')
                .or(char('C'))
                .parse(&mut st, &mut log)
        );
        assert_eq!("hhh", st.as_stream());
        assert_eq!(1, log.len());
    }
}
