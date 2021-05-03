use crate::core::logger::*;
use std::rc::Rc;

/// ## `Parsable` trait
/// Anything that is parsable should implement `Parsable` trait,
/// where `Result` indicates the result type of the parser.
/// The return types of all the lexers and combinators in this library
/// implements `Parsable` trait, meaning you can treat them as parsers
/// and call `parse()` or `exec()` from them to parse given input.
pub trait Parsable<S> {
    type Result; // Type of parsed result

    /// Parse function
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result>;

    fn exec(&self, state: &mut S) -> (Option<Self::Result>, ParseLogger) {
        let mut logger = ParseLogger::default();
        (self.parse(state, &mut logger), logger)
    }
}

// Implement Parsable trait for &Parsable<_>
impl<S, P: Parsable<S>> Parsable<S> for &P {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for &mut Parsable<_>
impl<S, P: Parsable<S>> Parsable<S> for &mut P {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for Box<Parsable<_>>
impl<S, P: Parsable<S>> Parsable<S> for Box<P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for Rc<Parsable<_>>
impl<S, P: Parsable<S>> Parsable<S> for Rc<P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        (**self).parse(state, logger)
    }
}
