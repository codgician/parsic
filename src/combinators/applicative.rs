use std::marker::PhantomData;
use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;

// Empty
#[derive(Copy, Clone, Default, Debug)]
pub struct EmptyP<T>(PhantomData<fn() -> Option<T>>);

impl<S, T> Parsable<S, T> for EmptyP<T> {
    fn parse(&self, _: &mut S, _: &mut ParseLogger) -> Option<T> {
        None
    }
}

/// Empty Parsable Builder
pub fn empty<T>() -> EmptyP<T> {
    EmptyP(PhantomData)
}

// Many
pub struct ManyP<P>(P);

impl<S, T, P> Parsable<S, Vec<T>> for ManyP<P>
    where S: Clone, P: Parsable<S, T>
{
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<Vec<T>> {
        let mut res = vec![];
        let mut st = stream.clone();

        while let Some(r) = self.0.parse(stream, logger) {
            res.push(r);
            st = stream.clone();
        }

        *stream = st;
        Some(res)
    }
}

// Many Combinator
pub fn many<S, T, P>(parser: P) -> ManyP<P>
    where P: Parsable<S, T> 
{
    ManyP(parser)
}

// Some Combinator
pub struct SomeP<P>(P);

impl<S: Clone, T, P: Parsable<S, T>> Parsable<S, Vec<T>> for SomeP<P> {
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<Vec<T>> {
        let mut res = vec![self.0.parse(stream, logger)?];
        let mut st = stream.clone();
        let mut lg = logger.clone();

        while let Some(r) = self.0.parse(stream, logger) {
            res.push(r);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        *logger = lg;
        Some(res)
    }
}

/// Some Combinator
pub fn some<S, T, P: Parsable<S, T>>(parser: P) -> SomeP<P> {
    SomeP(parser)
}

// Implement iterator-style method for Parsable trait
pub trait ApplicativeExt<S, T> : Parsable<S, T> {
    /// Many Combinator
    fn many(self) -> ManyP<Self> where Self: Sized {
        ManyP(self)
    }

    /// Some combinator
    fn some(self) -> SomeP<Self> where Self: Sized {
        SomeP(self)
    }
}

impl<S, T, P: Parsable<S, T>> ApplicativeExt<S, T> for P {}

#[cfg(test)]
mod test_empty {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::primitives::*;

    #[test]
    fn fail() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None as Option<String>,
            super::empty().parse(&mut st, &mut log)
        );
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, log.len());
    }
}

#[cfg(test)]
mod test_many {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok_nonempty() {
        let mut st = StrState::new("yyyyying");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            char('y').many().parse(&mut st, &mut log)
        )
    }

    #[test]
    fn ok_empty() {
        let mut st = StrState::new("ing");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(vec![]),
            super::many(char('y')).parse(&mut st, &mut log)
        )
    }
}

#[cfg(test)]
mod test_some {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let mut st = StrState::new("yyyyying");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(vec!['y', 'y', 'y', 'y', 'y']),
            char('y').some().parse(&mut st, &mut log)
        )
    }

    #[test]
    fn fail() {
        let mut st = StrState::new("ing");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('y').some().parse(&mut st, &mut log)
        )
    }
}
