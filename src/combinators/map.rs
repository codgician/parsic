use std::marker::PhantomData;
use crate::core::{ Parsable, ParseLogger, Msg, MsgBody };

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
    P: Parsable<S>
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        self.1.parse(state, logger).map(&self.0)
    }
}

/// ### Combinator: `map` (function variant)
pub fn map<F, P, S, T>(func: F, parser: P) -> MapP<F, P>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>
{
    MapP::new(func, parser)
}

// MapOpt
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

impl<F, P, S, T, E> Parsable<S> for MapOptP<F, P, Result<T, E>>
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

/// ### Combinator: `map_opt` (function variant)
/// **Only supports `Option<T>` and `Result<T, E>`**.
pub fn map_opt<F, P, S, T>(func: F, parser: P) -> MapOptP<F, P, T>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>
{
    MapOptP::new(func, parser)
}

pub trait MapExt<S> : Parsable<S> {
    /// ### Combinator: `map`
    fn map<T, F>(self, func: F) -> MapP<F, Self>
    where
        Self: Sized,
        F: Fn(Self::Result) -> T
    {
        MapP::new(func, self)
    }

    /// ### Combinator: `map_opt`
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
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        let parser = char('H')
                    .or(char('W'))
                    .map(|ch: char| ch == 'H');
        assert_eq!(
            Some(true),
            parser.parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn select_ok() {
        let mut st = StrState::new("-1");
        let mut log = ParseLogger::default();
        let parser = char('-')
                    .and(char('1'))
                    .map(|(_, x)| x);
        assert_eq!(
            Some('1'),
            parser.parse(&mut st, &mut log)
        );
        assert_eq!("", st.as_stream());
        assert_eq!(0, log.len());
    }
}

#[cfg(test)]
mod test_map_opt {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok_fully_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
                            .some()
                            .map_opt(|v| v.into_iter()
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
                            .map_opt(|v| v.into_iter()
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
                            .map_opt(|v| v.into_iter()
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
