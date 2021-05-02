use crate::core::*;

#[derive(Copy, Clone, Debug)]
pub struct ParseFn<F>(F);

impl<F> ParseFn<F> {
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<S, T, F> Parsable<S> for ParseFn<F>
where
    F: for<'a> Fn(&'a mut S, &mut ParseLogger) -> Option<T>,
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        (self.0)(state, logger)
    }
}
