use std::marker::PhantomData;
use crate::core::{ Parsable, ParseLogger, Msg, MsgBody };

// Pure combinator
#[derive(Clone, Copy, Debug)]
pub struct Pure<F>(pub F);

impl<F, S, T> Parsable<S> for Pure<F>
    where F: Fn() -> T
{
    type Result = T;

    fn parse(&self, _: &mut S, _: &mut ParseLogger) 
        -> Option<T> 
    {
        Some((self.0)())
    }
}

/// Pure Combinator
pub fn pure<F, T>(x: F) -> Pure<F> where F: Fn() -> T {
    Pure(x)
}

// BindP combinator
#[derive(Clone, Copy, Debug)]
pub struct BindP<F, P, T>(F, P, PhantomData<T>);

impl<F, P, S, T> Parsable<S> for BindP<F, P, Option<T>> 
    where 
        F: Fn(P::Result) -> Option<T>, 
        P: Parsable<S>
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        match self.1.parse(state, logger).map(&self.0) {
            Some(Some(x)) => Some(x),
            _ => None
        }
    }
}

impl<F, P, S, T, E> Parsable<S> for BindP<F, P, Result<T, E>> 
    where 
        F: Fn(P::Result) -> Result<T, E>, 
        P: Parsable<S>, 
        E: ToString
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result>
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

pub fn bind_option<F, P, S, T>(func: F, parser: P) 
    -> BindP<F, P, Option<T>>
    where 
        F: Fn(P::Result) -> Option<T>, 
        P: Parsable<S>
{
    BindP(func, parser, PhantomData)
}

pub fn bind_result<F, P, S, T, E>(func: F, parser: P) 
    -> BindP<F, P, Result<T, E>>
    where 
        F: Fn(P::Result) -> Result<T, E>, 
        P: Parsable<S>
{
    BindP(func, parser, PhantomData)
}

pub trait MonadicExt<S> : Parsable<S> {
    /// BindP Combinator (Option)
    fn bind_option<F, T>(self, func: F) -> BindP<F, Self, Option<T>>
        where 
            Self: Sized, 
            F: Fn(Self::Result) -> Option<T>,
    {
        BindP(func, self, PhantomData)
    }

    /// BindP Combinator (Result)
    fn bind_result<F, T, E>(self, func: F) -> BindP<F, Self, Result<T, E>>
        where 
            Self: Sized, 
            F: Fn(Self::Result) -> Result<T, E>
    {
        BindP(func, self, PhantomData)
    }
}

impl<S, P: Parsable<S>> MonadicExt<S> for P {}

#[cfg(test)]
mod test_pure {
    use crate::core::*;
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
    use crate::core::*;
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
