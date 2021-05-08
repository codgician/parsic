use crate::core::{Msg, MsgBody, Parsable, Parser};

/// ## Combinator: `info` (function ver.)
fn info<'f, A: 'f, S>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    msg: &'f str,
) -> Parser<'f, A, S> {
    Parser::new(move |stream, logger| {
        p.parse(stream, logger).or_else(|| {
            logger.with(Msg::Info(MsgBody::new(msg, None)));
            None
        })
    })
}

/// ## Combinator: `warn` (function ver.)
fn warn<'f, A: 'f, S>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    msg: &'f str,
) -> Parser<'f, A, S> {
    Parser::new(move |stream, logger| {
        p.parse(stream, logger).or_else(|| {
            logger.with(Msg::Warn(MsgBody::new(msg, None)));
            None
        })
    })
}

/// ## Combinator: `error` (function ver.)
fn error<'f, A: 'f, S>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    msg: &'f str,
) -> Parser<'f, A, S> {
    Parser::new(move |stream, logger| {
        p.parse(stream, logger).or_else(|| {
            logger.with(Msg::Error(MsgBody::new(msg, None)));
            None
        })
    })
}

/// ## Combinator: `inspect` (function ver.)
fn inspect<'f, A: 'f, S: Clone + 'f>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, (Option<A>, S), S> {
    Parser::new(move |stream, logger| {
        let res = p.parse(stream, logger);
        Some((res, stream.clone()))
    })
}

/// ## Combinator: `recover` (function ver.)
fn recover<'f, A: Clone + 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    x: A,
) -> Parser<'f, A, S> {
    Parser::new(move |stream, logger| p.parse(stream, logger).or_else(|| Some(x.clone())))
}

/// Implement following method for `Parsable<S>`:
/// - `info`
/// - `warn`
/// - `error`
/// - `inspect`
/// - `recover`
pub trait LogExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// ## Combinator: `info`
    fn info(self, msg: &'f str) -> Parser<'f, A, S>
    where
        Self: Sized + 'f,
    {
        info(self, msg)
    }

    /// ## Combinator: `warn`
    fn warn(self, msg: &'f str) -> Parser<'f, A, S>
    where
        Self: Sized + 'f,
    {
        warn(self, msg)
    }

    /// ## Combinator: `error`
    fn error(self, msg: &'f str) -> Parser<'f, A, S>
    where
        Self: Sized + 'f,
    {
        error(self, msg)
    }

    /// ## Combinator: `inspect`
    fn inspect(self) -> Parser<'f, (Option<A>, S), S>
    where
        Self: Sized + 'f,
        S: Clone + 'f,
    {
        inspect(self)
    }

    /// ## Combinator: `recover`
    fn recover(self, x: A) -> Parser<'f, A, S>
    where
        Self: Sized + 'f,
        A: Clone,
        S: Clone,
    {
        recover(self, x)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> LogExt<'f, A, S> for P {}
