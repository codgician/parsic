use std::rc::Rc;
use crate::core::logger::*;

// Parsable trait
pub trait Parsable<S> {
    type Result;    // Type of parsed result

    /// Parse function
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result>;
}

// Implement Parsable trait for &Parsable<_>
impl<S, P: Parsable<S>> Parsable<S> for &P {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger)
         -> Option<Self::Result> 
    {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for &mut Parsable<_>
impl<S, P: Parsable<S>> Parsable<S>for &mut P {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for Box<Parsable<_>>
impl<S, P: Parsable<S>> Parsable<S> for Box<P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        (**self).parse(state, logger)
    }
}

// Implement Parsable trait for Rc<Parsable<_>>
impl<S, P: Parsable<S>> Parsable<S> for Rc<P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        (**self).parse(state, logger)
    }
}
