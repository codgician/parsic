use crate::core::logger::*;
use std::rc::Rc;

/// # `Parser` struct
/// Wraps the parser function.
#[derive(Clone)]
pub struct Parser<'f, A, S>(Rc<dyn Fn(&mut S, &mut ParseLogger) -> Option<A> + 'f>);

impl<'f, A: 'f, S> Parser<'f, A, S> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&mut S, &mut ParseLogger) -> Option<A> + 'f,
    {
        Self(Rc::new(f))
    }
}

/// # `Parsable` trait
/// Anything that is parsable should implement `Parsable` trait,
/// The return types of all the combinators and combinators in this library
/// Implement `Parsable` trait, meaning you can treat them as parsers
/// and call `parse()` or `exec()` from them to parse given input.
pub trait Parsable {
    type Stream;
    type Result;

    /// Parse function
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result>;

    /// Wrapper for parse function
    fn exec(&self, stream: &mut Self::Stream) -> (Option<Self::Result>, ParseLogger) {
        let mut logger = ParseLogger::default();
        (self.parse(stream, &mut logger), logger)
    }

    /// Convert into a Parser
    fn into_parser<'f>(self) -> Parser<'f, Self::Result, Self::Stream> 
    where
        Self: Sized + 'f
    {
        Parser::new(move |stream: &mut Self::Stream, logger| self.parse(stream, logger))
    }
}

/// Implement `Parsable` trait for `Parser`
impl<'f, A: 'f, S> Parsable for Parser<'f, A, S> {
    type Stream = S;
    type Result = A;
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result> {
        (*self).0(stream, logger)
    }
}

/// Implement `Parsable` trait for any `&P` where `P: Parsable`
impl<P: Parsable> Parsable for &P {
    type Stream = P::Stream;
    type Result = P::Result;
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(stream, logger)
    }
}

/// Implement `Parsable` trait for any `&mut P` where `P: Parsable`
impl<P: Parsable> Parsable for &mut P {
    type Stream = P::Stream;
    type Result = P::Result;
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(stream, logger)
    }
}

/// Implement `Parsable` trait for any `Box<P>` where `P: Parsable`
impl<P: Parsable> Parsable for Box<P> {
    type Stream = P::Stream;
    type Result = P::Result;
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(stream, logger)
    }
}

/// Implement `Parsable` trait for any `Rc<P>` where `P: Parsable`
impl<P: Parsable> Parsable for Rc<P> {
    type Stream = P::Stream;
    type Result = P::Result;
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(stream, logger)
    }
}

/// Implement `Parsable` trait for any `fn() -> P` where `P: Parsable`
/// This enables lazy evaluation when defining recursive parsers.
impl<P: Parsable> Parsable for fn() -> P {
    type Stream = P::Stream;
    type Result = P::Result;
    fn parse(&self, stream: &mut Self::Stream, logger: &mut ParseLogger) -> Option<Self::Result> {
        (*self)().parse(stream, logger)
    }
}
