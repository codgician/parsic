use crate::core::parser::Parsable;
use crate::core::logger::*;

#[derive(Clone, Debug)]
pub struct LogP<P>(P, Msg);

impl<S, T, P> Parsable<S, T> for LogP<P> where P: Parsable<S, T> {
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<T> {
        match self.0.parse(stream, logger) {
            None => {
                logger.add(self.1.to_owned());
                None
            }
            x => x
        }
    }
}

pub fn info<S, T, P>(parser: P, msg: &str) -> LogP<P> where P: Parsable<S, T> {
    LogP(parser, Msg::Info(MsgBody::new(msg, None)))
}

pub fn warn<S, T, P>(parser: P, msg: &str) -> LogP<P> where P: Parsable<S, T> {
    LogP(parser, Msg::Warn(MsgBody::new(msg, None)))
}

pub fn error<S, T, P>(parser: P, msg: &str) -> LogP<P> where P: Parsable<S, T> {
    LogP(parser, Msg::Error(MsgBody::new(msg, None)))
}

pub trait LogPExt<S, T> : Parsable<S, T> {
    fn info(self, msg: &str) -> LogP<Self> 
        where Self: Sized 
    {
        info(self, msg)
    }

    fn warn(self, msg: &str) -> LogP<Self> 
        where Self: Sized 
    {
        warn(self, msg)
    }

    fn error(self, msg: &str) -> LogP<Self> 
        where Self: Sized 
    {
        error(self, msg)
    }
}

impl<S, T, P: Parsable<S, T>> LogPExt<S, T> for P {}
