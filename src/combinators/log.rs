use crate::core::{ Parsable, ParseLogger, Msg, MsgBody };

#[derive(Clone, Debug)]
pub struct LogP<P>(P, Msg);

impl<S, P> Parsable<S> for LogP<P> 
    where P: Parsable<S> 
{
    type Result = P::Result;
    
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        match self.0.parse(stream, logger) {
            None => {
                logger.add(self.1.to_owned());
                None
            }
            x => x
        }
    }
}

pub fn info<S, P>(parser: P, msg: &str) -> LogP<P> 
    where P: Parsable<S> 
{
    LogP(parser, Msg::Info(MsgBody::new(msg, None)))
}

pub fn warn<S, P>(parser: P, msg: &str) -> LogP<P> 
    where P: Parsable<S> 
{
    LogP(parser, Msg::Warn(MsgBody::new(msg, None)))
}

pub fn error<S, P>(parser: P, msg: &str) -> LogP<P> 
    where P: Parsable<S> 
{
    LogP(parser, Msg::Error(MsgBody::new(msg, None)))
}

pub trait LogPExt<S> : Parsable<S> {
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

impl<S, P: Parsable<S>> LogPExt<S> for P {}
