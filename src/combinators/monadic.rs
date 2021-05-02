use std::marker::PhantomData;
use crate::core::{ Parsable, ParseLogger, Msg, MsgBody };

// Bind
#[derive(Clone, Copy, Debug)]
pub struct BindP<F, P, T>(F, P, PhantomData<T>);

impl<F, P, T> BindP<F, P, T> {
    pub fn new(func: F, parser: P) -> Self {
        Self(func, parser, PhantomData)
    }
}

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

/// ### Combinator: `bind` (`Option<T>`, function variant)
pub fn bind_option<F, P, S, T>(func: F, parser: P)
    -> BindP<F, P, Option<T>>
where
    F: Fn(P::Result) -> Option<T>,
    P: Parsable<S>
{
    BindP::new(func, parser)
}

/// ### Combinator: `bind` (`Result<T, E>`, function variant)
pub fn bind_result<F, P, S, T, E>(func: F, parser: P)
    -> BindP<F, P, Result<T, E>>
where
    F: Fn(P::Result) -> Result<T, E>,
    P: Parsable<S>
{
    BindP::new(func, parser)
}

pub trait MonadicExt<S> : Parsable<S> {
    /// ### Combinator: `bind` (`Option<T>`)
    fn bind_option<F, T>(self, func: F) -> BindP<F, Self, Option<T>>
    where
        Self: Sized,
        F: Fn(Self::Result) -> Option<T>,
    {
        BindP::new(func, self)
    }

    /// ### Combinator: `bind` (`Result<T, E>`)
    fn bind_result<F, T, E>(self, func: F) -> BindP<F, Self, Result<T, E>>
    where
        Self: Sized,
        F: Fn(Self::Result) -> Result<T, E>
    {
        BindP::new(func, self)
    }
}

impl<S, P: Parsable<S>> MonadicExt<S> for P {}

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
