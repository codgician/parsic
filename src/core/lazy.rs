use crate::core::{Parsable, ParseLogger, Parser};

/// # Trait: `Lazy`
/// Wraps anything that implements `Parsable` to
/// support lazy evaluation.
#[derive(Clone)]
pub struct Lazy<F>(F);

impl<'f, A: 'f, S, F, P> Parsable for Lazy<F>
where
    F: Fn() -> P,
    P: Parsable<Stream = S, Result = A>,
{
    type Stream = S;
    type Result = A;
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<A> {
        (self.0)().parse(stream, logger)
    }
}

pub fn lazy<'f, A: 'f, F, S, P>(f: F) -> Parser<'f, A, S>
where
    F: Fn() -> P + 'f,
    P: Parsable<Stream = S, Result = A>,
{
    Lazy(f).into_parser()
}
