use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;

#[derive(Clone, Copy, Debug)]
pub struct OrP<P1, P2>(P1, P2);

impl<S, T, P1, P2> Parsable<S, T> for OrP<P1, P2>
    where S: Clone, P1: Parsable<S, T>, P2: Parsable<S, T>
{
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<T> {
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

pub fn or<P1, P2>(p1: P1, p2: P2) -> OrP<P1, P2> {
    OrP(p1, p2)
}

pub trait OrExt<S, T> : Parsable<S, T> {
    /// Or Combinator
    fn or<P>(self, parser: P) -> OrP<Self, P>
        where Self: Sized, P: Parsable<S, T>
    {
        OrP(self, parser)
    }
}

impl<S, T, P: Parsable<S, T>> OrExt<S, T> for P {}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
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
