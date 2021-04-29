use std::rc::Rc;
use crate::core::logger::*;

// Parsable trait
// S: Type for input stream
// T: Type for parse result
pub trait Parsable<S, T> {
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<T>;
}

// Implement Parsable trait for &Parsable<_>
impl<S, T, P: Parsable<S, T>> Parsable<S, T> for &P {
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<T> {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for &mut Parsable<_>
impl<S, T, P: Parsable<S, T>> Parsable<S, T>for &mut P {
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<T> {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for Box<Parsable<_>>
impl<S, T, P: Parsable<S, T>> Parsable<S, T> for Box<P> {
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<T> {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for Rc<Parsable<_>>
impl<S, T, P: Parsable<S, T>> Parsable<S, T> for Rc<P> {
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<T> {
        (**self).parse(state, logger)
    }
}
