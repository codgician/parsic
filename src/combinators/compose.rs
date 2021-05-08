use crate::core::{Parsable, Parser};

/// ## Combinator: `compose` (function ver.)
/// Functional composition between parsers.
pub fn compose<'f, A: 'f, B: 'f, F, S>(
    pf: impl Parsable<Stream = S, Result = F> + 'f,
    px: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, B, S>
where
    F: Fn(A) -> B + 'f,
    S: Clone,
{
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        match pf.parse(stream, logger) {
            Some(f) => match px.parse(stream, logger) {
                Some(x) => Some(f(x)),
                _ => {
                    *stream = st;
                    None
                }
            },
            _ => {
                *stream = st;
                None
            }
        }
    })
}

/// Implements `compose` method for `Parsable<S>`.
pub trait ComposeExt<'f, F: 'f, S>: Parsable<Stream = S, Result = F> {
    /// ## Combinator: `compose`
    /// Functional composition between parsers.
    fn compose<A: 'f, B: 'f>(
        self,
        px: impl Parsable<Stream = S, Result = A> + 'f,
    ) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        F: Fn(A) -> B,
        S: Clone,
    {
        compose(self, px)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> ComposeExt<'f, A, S> for P {}

#[cfg(test)]
mod test_compose {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, CharStream};

    #[test]
    fn ok() {
        let parser = pure(|x| x == 'H').compose(char('H'));

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(true), res);
        assert_eq!("ello", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = pure(|x| x == 'H').compose(char('h'));

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn compose_with_empty() {
        let parser = pure(|_| true).compose(empty::<bool, CharStream>());

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(0, logs.len());
    }
}
