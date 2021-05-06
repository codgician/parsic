use crate::core::{Msg, MsgBody, Parsable, ParseLogger};

/// Data structure for logging combinators, including:
/// - `info`
/// - `warn`
/// - `error`
#[derive(Clone, Debug)]
pub struct LogP<P>(P, Msg);

impl<P> LogP<P> {
    pub fn new(parser: P, msg: Msg) -> Self {
        Self(parser, msg)
    }
}

impl<S, P: Parsable<S>> Parsable<S> for LogP<P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.0.parse(state, logger) {
            None => {
                logger.add(self.1.to_owned());
                None
            }
            x => x,
        }
    }
}

/// ## Combinator: `info` (function ver.)
pub fn info<S, P: Parsable<S>>(parser: P, msg: &str) -> LogP<P> {
    LogP::new(parser, Msg::Info(MsgBody::new(msg, None)))
}

/// ## Combinator: `warn` (function ver.)
pub fn warn<S, P: Parsable<S>>(parser: P, msg: &str) -> LogP<P> {
    LogP::new(parser, Msg::Warn(MsgBody::new(msg, None)))
}

/// ## Combinator: `error` (function ver.)
pub fn error<S, P: Parsable<S>>(parser: P, msg: &str) -> LogP<P> {
    LogP::new(parser, Msg::Error(MsgBody::new(msg, None)))
}

/// Data structure for `pos` combinator.
#[derive(Copy, Clone, Debug)]
pub struct InspectP<P>(P);

impl<P> InspectP<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<S: Clone, P: Parsable<S>> Parsable<S> for InspectP<P> {
    type Result = (Option<P::Result>, S);

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        Some((self.0.parse(state, logger), (*state).clone()))
    }
}

/// ## Combinator: `inspect` (function ver.)
pub fn inspect<S: Clone, P: Parsable<S>>(parser: P) -> InspectP<P> {
    InspectP::new(parser)
}

/// Data structure for `recover` combinator
#[derive(Copy, Clone, Debug)]
pub struct RecoverP<P, T>(P, T);

impl<P, T: Clone> RecoverP<P, T> {
    pub fn new(parser: P, fallback: T) -> Self {
        Self(parser, fallback)
    }
}

impl<S, P: Parsable<S>> Parsable<S> for RecoverP<P, P::Result>
where
    P::Result: Clone,
{
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.0.parse(state, logger) {
            None => Some(self.1.clone()),
            x => x,
        }
    }
}

/// ## Combinator: `recover` (function ver.)
pub fn recover<S, P: Parsable<S>>(parser: P, fallback: P::Result) -> RecoverP<P, P::Result>
where
    P::Result: Clone,
{
    RecoverP::new(parser, fallback)
}

/// Implements following method for `Parsable<S>`:
/// - `info`
/// - `warn`
/// - `error`
/// - `inspect`
/// - `recover`
pub trait LogPExt<S>: Parsable<S> {
    /// ## Combinator: `info`
    fn info(self, msg: &str) -> LogP<Self>
    where
        Self: Sized,
    {
        LogP::new(self, Msg::Info(MsgBody::new(msg, None)))
    }

    /// ## Combinator: `warn`
    fn warn(self, msg: &str) -> LogP<Self>
    where
        Self: Sized,
    {
        LogP::new(self, Msg::Warn(MsgBody::new(msg, None)))
    }

    /// ## Combinator: `error`
    fn error(self, msg: &str) -> LogP<Self>
    where
        Self: Sized,
    {
        LogP::new(self, Msg::Warn(MsgBody::new(msg, None)))
    }

    /// ## Combinator: `inspect`
    fn inspect(self) -> InspectP<Self>
    where
        Self: Sized,
        S: Clone,
    {
        InspectP::new(self)
    }

    /// ## Combinator: `recover`
    fn recover(self, fallback: Self::Result) -> RecoverP<Self, Self::Result>
    where
        Self: Sized,
        Self::Result: Clone,
    {
        RecoverP::new(self, fallback)
    }
}

impl<S, P: Parsable<S>> LogPExt<S> for P {}
