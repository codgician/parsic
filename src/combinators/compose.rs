use crate::core::{return_none, Parsable, Parser};

/// ## Combinator: `compose` (function ver.)
///
/// Functional composition between parsers.
///
/// ### Properties
///
/// Should satisfy [Applicative functor laws](https://wiki.haskell.org/Typeclassopedia#Laws_2).
///
/// - **Identity**: `compose(pure(id), p) ~ p`
/// - **Homomorphism**: `compose(pure(f), pure(g)) ~ pure(|x| f(g(x)))`
/// - **Interchange**: `compose(x, pure(y)) ~ compose(pure(|g| g(y)), x)`
/// - **Composition**: `compose(x, compose(y, z)) ~ compose(pure(|f| |g| |x| f(g(x))), z)`
///
/// Check out `test_bind` module in the source code for naive examples of above laws.
///
/// ### Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = compose(pure(|x| x == 'H'), char('H'));
///
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(true), res);
/// assert_eq!("ello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn compose<'f, A: 'f, B: 'f, F, S: Clone>(
    pf: impl Parsable<Stream = S, Result = F> + 'f,
    px: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, B, S>
where
    F: Fn(A) -> B + 'f,
{
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        match pf.parse(stream, logger) {
            Some(f) => match px.parse(stream, logger) {
                Some(x) => Some(f(x)),
                None => return_none(stream, &st),
            },
            None => return_none(stream, &st),
        }
    })
}

/// Implement `compose` combinator for `Parsable<S>`.
pub trait ComposeExt<'f, F: 'f, S>: Parsable<Stream = S, Result = F> {
    /// ## Combinator: `compose`
    ///
    /// Functional composition between parsers.
    ///
    /// ### Properties
    ///
    /// Should satisfy [Applicative functor laws](https://wiki.haskell.org/Typeclassopedia#Laws_2):
    ///
    /// - **Identity**: `pure(id).compose(p) ~ p`
    /// - **Homomorphism**: `pure(f).compose(pure(g)) ~ pure(|x| f(g(x)))`
    /// - **Interchange**: `x.compose(pure(y)) ~ pure(|g| g(y)).compose(x)`
    /// - **Composition**: `x.compose(y.compose(z)) ~ pure(|f| |g| |x| f(g(x))).compose(z)`
    ///
    /// Check out `test_bind` module in the source code for naive examples of above laws.
    ///
    /// ### Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::{char, CharStream};
    ///
    /// let parser = pure(|x| x == 'H').compose(char('H'));
    ///
    /// let mut st = CharStream::new("Hello");
    /// let (res, logs) = parser.exec(&mut st);
    ///
    /// assert_eq!(Some(true), res);
    /// assert_eq!("ello", st.as_str());
    /// assert_eq!(0, logs.len());
    /// ```
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
    fn fail_with_grace() {
        let parser = pure(|x| x == 'H').compose(char('h'));

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn identity() {
        //! `pure(id).compose(p) ~ p`
        //! Note: `id` is the identity function `|x| x`.
        let parser1 = pure(|x| x).compose(char('0'));
        let parser2 = char('0');

        assert_eq!(
            parser1.exec(&mut CharStream::new("01")),
            parser2.exec(&mut CharStream::new("01"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("10")),
            parser2.exec(&mut CharStream::new("10"))
        );
    }

    #[test]
    fn homomorphism() {
        //! `pure(f).compose(pure(g)) ~ pure(|x| f(g(x)))`
        //! Function application order does not matter.
        let f = |ch| if ch == '0' { 'a' } else { 'b' };
        let g = |ch| if ch == 'a' { 'A' } else { 'B' };
        let parser1 = pure(f).compose(pure(g).compose(char('0')));
        let parser2 = pure(|x| f(g(x))).compose(char('0'));

        assert_eq!(
            parser1.exec(&mut CharStream::new("01")),
            parser2.exec(&mut CharStream::new("01"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("10")),
            parser2.exec(&mut CharStream::new("10"))
        );
    }

    #[test]
    fn interchange() {
        //! `x.compose(pure(y)) ~ pure(|g| g(y)).compose(x)`
        //! Interchange law.
        let (x, y) = (pure::<fn(u64) -> u64, _>(|x| x + 1), 1);
        let parser1 = x.clone().compose(pure(y));
        let parser2 = pure::<_, _>(|f: fn(u64) -> u64| f(y)).compose(x);

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }

    #[test]
    fn composition() {
        //! `x.compose(y.compose(z)) ~ pure(|f| |g| |x| f(g(x))).compose(z)`
        //! Composition law
        let x = pure::<fn(u64) -> u64, _>(|x| x + 3);
        let y = pure::<fn(u64) -> u64, _>(|x| x * 5);
        let z = pure(2);
        let parser1 = x.clone().compose(y.clone().compose(z.clone()));
        let parser2 = (pure(|f: fn(u64) -> u64| move |g: fn(u64) -> u64| move |x: u64| f(g(x)))
            .compose(x)
            .compose(y))
        .compose(z);

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }
}
