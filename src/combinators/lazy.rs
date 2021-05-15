use crate::core::{Parsable, ParseLogger, Parser};

/// Data structure for `lazy` combinator.
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

/// # Combinator: `lazy`
///
/// `lazy` accepts an instande of `Fn() -> Parser`, usually a fn pointer, and
/// wraps it into an instance of `Parsable`. The function will not be evaluated
/// until the actual parsing happens, which avoids infinite recursing when
/// writing parsers for recursive syntax using functions (see the example below).
///
/// # Example
/// ```
/// use parsic::combinators::*;
/// use parsic::core::{Parsable, Parser};
/// use parsic::primitives::{char, CharStream};
///
/// // expr := '1' expr | '0'
/// fn parser<'f>() -> Parser<'f, char, CharStream<'f>> {
///     char('1').right(lazy(parser)).or(char('0'))
/// }
///
/// let mut st = CharStream::new("1110");
/// let (res, logs) = parser().exec(&mut st);
///
/// assert_eq!(Some('0'), res);
/// assert_eq!("", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn lazy<'f, A: 'f, F, S, P>(f: F) -> Parser<'f, A, S>
where
    F: Fn() -> P + 'f,
    P: Parsable<Stream = S, Result = A>,
{
    Lazy(f).into_parser()
}
