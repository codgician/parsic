use std::marker::PhantomData;

use crate::core::parser::Parsable;
use crate::core::logger::*;

// Pure combinator
#[derive(Clone, Copy, Debug)]
pub struct Pure<F>(pub F);

impl<F, S, T> Parsable<S, T> for Pure<F>
    where F: Fn() -> T
{
    fn parse(&self, _: &mut S, _: &mut ParseLogger) -> Option<T> {
        Some((self.0)())
    }
}

/// Pure Combinator
pub fn pure<F, T>(x: F) -> Pure<F> where F: Fn() -> T {
    Pure(x)
}

// Bind combinator
#[derive(Clone, Copy, Debug)]
pub struct Bind<F, P, T1, T2>(F, P, PhantomData<T1>, PhantomData<T2>);

impl<S, T1, T2, F, P> Parsable<S, T2> for Bind<F, P, T1, Option<T2>> 
    where 
        F: Fn(T1) -> Option<T2>, 
        P: Parsable<S, T1>
{
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<T2> 
    {
        match self.1.parse(state, logger).map(&self.0) {
            Some(Some(x)) => Some(x),
            _ => None
        }
    }
}

impl<S, T1, T2, F, P, E> Parsable<S, T2> for Bind<F, P, T1, Result<T2, E>> 
    where 
        F: Fn(T1) -> Result<T2, E>, 
        P: Parsable<S, T1>, 
        E: ToString
{
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<T2> 
    {
        match self.1.parse(state, logger).map(&self.0) {
            Some(Ok(x)) => Some(x),
            Some(Err(e)) => {
                logger.add(Msg::Error(MsgBody::new(&e.to_string()[..], None)));
                None
            }
            _ => None
        }
    }
}

pub fn bind_option<S, T1, T2, F, P>(func: F, parser: P) -> Bind<F, P, T1, Option<T2>>
    where 
        F: Fn(T1) -> Option<T2>, 
        P: Parsable<S, T1>
{
    Bind(func, parser, PhantomData, PhantomData)
}

pub fn bind_result<S, T1, T2, F, P, E>(func: F, parser: P) -> Bind<F, P, T1, Result<T2, E>>
    where 
        F: Fn(T1) -> Result<T2, E>, 
        P: Parsable<S, T1>
{
    Bind(func, parser, PhantomData, PhantomData)
}

pub trait MonadicExt<S, T1> : Parsable<S, T1> {
    /// Bind Combinator (Option)
    fn bind_option<T2, F>(self, func: F) -> Bind<F, Self, T1, Option<T2>>
        where 
            Self: Sized, 
            F: Fn(T1) -> Option<T2>,
    {
        Bind(func, self, PhantomData, PhantomData)
    }

    /// Bind Combinator (Result)
    fn bind_result<T2, F, E>(self, func: F) -> Bind<F, Self, T1, Result<T2, E>>
        where 
            Self: Sized, 
            F: Fn(T1) -> Result<T2, E>
    {
        Bind(func, self, PhantomData, PhantomData)
    }
}

impl<S, T, P: Parsable<S, T>> MonadicExt<S, T> for P {}

#[cfg(test)]
mod test_pure {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::primitives::*;
    
    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(true),
            super::pure(|| true).parse(&mut st, &mut log)
        );
        assert_eq!("Hello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn empty_input() {
        let mut st = StrState::new("");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(true),
            super::pure(|| true).parse(&mut st, &mut log)
        );
        assert_eq!("", st.as_stream());
        assert_eq!(0, log.len());
    }
}

#[cfg(test)]
mod test_bind {
    use crate::core::parser::*;
    use crate::core::logger::ParseLogger;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok_fully_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
                            .some()
                            .bind_result(|v| v.into_iter()
                                .collect::<String>()
                                .parse::<i64>()
                            );

        let mut st = StrState::new("12345");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(12345),
            nat_parser.parse(&mut st, &mut log)
        );
        assert_eq!("", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn ok_partially_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
                            .some()
                            .bind_result(|v| v.into_iter()
                                .collect::<String>()
                                .parse::<i64>()
                            );

        let mut st = StrState::new("123de");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some(123),
            nat_parser.parse(&mut st, &mut log)
        );
        assert_eq!("de", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn fail() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
                            .some()
                            .bind_result(|v| v.into_iter()
                                .collect::<String>()
                                .parse::<i64>()
                            );

        let mut st = StrState::new("abcde");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            nat_parser.parse(&mut st, &mut log)
        );
        assert_eq!("bcde", st.as_stream());
        assert_eq!(1, log.len());
    }
}
