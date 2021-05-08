use crate::core::{Parsable, ParseLogger};

/// ## Trait: `Lazy`
/// Wraps anything that implements `Parsable` to support lazy evaluation.
#[derive(Clone)]
pub struct Lazy<F>(F);

impl<'f, A: 'f, S, F, P> Parsable for Lazy<F>
where
    F: Fn() -> P,
    P: Parsable<Stream = S, Result = A>,
{
    type Stream = S;
    type Result = A;
    fn parse<'s>(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<A> {
        (self.0)().parse(stream, logger)
    }
}

pub fn lazy<'f, A: 'f, S, F, P>(f: F) -> Lazy<F>
where
    F: Fn() -> P,
    P: Parsable<Stream = S, Result = A>,
{
    Lazy(f)
}
