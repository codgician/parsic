use crate::core::{ Parsable, ParseLogger, Msg, MsgBody };

#[derive(Clone, Debug)]
pub struct LogP<P>(P, Msg);

impl<P> LogP<P> {
    pub fn new(parser: P, msg: Msg) -> Self {
        Self(parser, msg)
    }
}

impl<S, P> Parsable<S> for LogP<P>
where
    P: Parsable<S>
{
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        match self.0.parse(state, logger) {
            None => {
                logger.add(self.1.to_owned());
                None
            }
            x => x
        }
    }
}

/// ### Combinator: `info` (function variant)
pub fn info<S, P>(parser: P, msg: &str) -> LogP<P>
where
    P: Parsable<S>
{
    LogP::new(parser, Msg::Info(MsgBody::new(msg, None)))
}

/// ### Combinator: `warn` (function variant)
pub fn warn<S, P>(parser: P, msg: &str) -> LogP<P>
where
    P: Parsable<S>
{
    LogP::new(parser, Msg::Warn(MsgBody::new(msg, None)))
}

/// ### Combinator: `error` (function variant)
pub fn error<S, P>(parser: P, msg: &str) -> LogP<P>
where
    P: Parsable<S>
{
    LogP::new(parser, Msg::Error(MsgBody::new(msg, None)))
}

pub trait LogPExt<S> : Parsable<S> {
    /// ### Combinator: `info`
    fn info(self, msg: &str) -> LogP<Self>
    where
        Self: Sized
    {
        LogP::new(self, Msg::Info(MsgBody::new(msg, None)))
    }

    /// ### Combinator: `warn`
    fn warn(self, msg: &str) -> LogP<Self>
    where
        Self: Sized
    {
        LogP::new(self, Msg::Warn(MsgBody::new(msg, None)))
    }

    /// ### Combinator: `error`
    fn error(self, msg: &str) -> LogP<Self>
    where
        Self: Sized
    {
        LogP::new(self, Msg::Warn(MsgBody::new(msg, None)))
    }
}

impl<S, P: Parsable<S>> LogPExt<S> for P {}
