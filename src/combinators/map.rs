use crate::core::{Msg, MsgBody, Parsable, ParseLogger};
use std::marker::PhantomData;

/// Data structure for `map` combinator.
#[derive(Clone, Copy, Debug)]
pub struct MapP<F, P>(F, P);

impl<F, P> MapP<F, P> {
    pub fn new(f: F, p: P) -> Self {
        Self(f, p)
    }
}

impl<F, P, S, T> Parsable<S> for MapP<F, P>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>,
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        self.1.parse(state, logger).map(&self.0)
    }
}

/// ## Combinator: `map` (function ver.)
pub fn map<F, P, S, T>(func: F, parser: P) -> MapP<F, P>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>,
{
    MapP::new(func, parser)
}

/// Data structure for `map_opt` combinator.
#[derive(Clone, Copy, Debug)]
pub struct MapOptP<F, P, T>(F, P, PhantomData<T>);

impl<F, P, T> MapOptP<F, P, T> {
    pub fn new(func: F, parser: P) -> Self {
        Self(func, parser, PhantomData)
    }
}

impl<F, P, S, T> Parsable<S> for MapOptP<F, P, Option<T>>
where
    F: Fn(P::Result) -> Option<T>,
    P: Parsable<S>,
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.1.parse(state, logger).map(&self.0) {
            Some(Some(x)) => Some(x),
            _ => None,
        }
    }
}

impl<F, P, S, T, E> Parsable<S> for MapOptP<F, P, Result<T, E>>
where
    F: Fn(P::Result) -> Result<T, E>,
    P: Parsable<S>,
    E: ToString,
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.1.parse(state, logger).map(&self.0) {
            Some(Ok(x)) => Some(x),
            Some(Err(e)) => {
                logger.add(Msg::Error(MsgBody::new(&e.to_string()[..], None)));
                None
            }
            _ => None,
        }
    }
}

/// ## Combinator: `map_opt` (function ver.)
/// **Only supports `Option<T>` and `Result<T, E>`**.
pub fn map_opt<F, P, S, T>(func: F, parser: P) -> MapOptP<F, P, T>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>,
{
    MapOptP::new(func, parser)
}

/// Implements following method for `Parsable<S>`:
/// - `map`
/// - `map_opt`
pub trait MapExt<S>: Parsable<S> {
    /// ## Combinator: `map`
    fn map<T, F>(self, func: F) -> MapP<F, Self>
    where
        Self: Sized,
        F: Fn(Self::Result) -> T,
    {
        MapP::new(func, self)
    }

    /// ## Combinator: `map_opt`
    fn map_opt<F, T>(self, func: F) -> MapOptP<F, Self, T>
    where
        Self: Sized,
        F: Fn(Self::Result) -> T,
    {
        MapOptP::new(func, self)
    }
}

impl<S, P: Parsable<S>> MapExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let parser = char('H').or(char('W')).map(|ch: char| ch == 'H');

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(true), res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn select_ok() {
        let parser = char('-').and(char('1')).map(|(_, x)| x);

        let mut st = StrState::new("-1");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('1'), res);
        assert_eq!("", st.as_stream());
        assert_eq!(0, logs.len());
    }
}

#[cfg(test)]
mod test_map_opt {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{satisfy, StrState};

    #[test]
    fn ok_fully_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_opt(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = StrState::new("12345");
        let (res, logs) = nat_parser.exec(&mut st);

        assert_eq!(Some(12345), res);
        assert_eq!("", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_partially_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_opt(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = StrState::new("123de");
        let (res, logs) = nat_parser.exec(&mut st);

        assert_eq!(Some(123), res);
        assert_eq!("de", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_opt(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = StrState::new("abcde");
        let (res, logs) = nat_parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("abcde", st.as_stream());
        assert_eq!(1, logs.len());
    }
}
