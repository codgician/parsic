use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;

// And combinator
#[derive(Clone, Copy, Debug)]
pub struct AndP<PA, PB>(PA, PB);

impl<S, T1, T2, P1, P2> Parsable<S, (T1, T2)> for AndP<P1, P2>
    where P1: Parsable<S, T1>, P2: Parsable<S, T2>
{
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger)
        -> Option<(T1, T2)>
    {
        match self.0.parse(stream, logger) {
            None => None,
            Some(r1) => {
                match self.1.parse(stream, logger) {
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

pub trait SequentialPExt<S, T> : Parsable<S, T> {
    /// And combinator
    fn and<P>(self, parser: P) -> AndP<Self, P>
        where Self: Sized, P: Parsable<S, T>
    {
        AndP(self, parser)
    }
}

impl<S, T, P: Parsable<S, T>> SequentialPExt<S, T> for P {}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::core::stream::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let mut st = CharStream::new("ABC");
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
    fn left_fail() {
        let mut st = CharStream::new("BBC");
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
        let mut st = CharStream::new("ACC");
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
        let mut st = CharStream::new("CCC");
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
