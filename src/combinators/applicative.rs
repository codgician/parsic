use crate::core::{return_none, Parsable, Parser};

/// # Combinator: `pure`
///
/// Injects a value into an identity parser.
///
/// # Examples
/// ## Injects a value
/// ```
/// use parsic::combinators::*;
/// use parsic::core::*;
/// use parsic::primitives::CharStream;
/// let parser = pure(true);
///
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(true), res);
/// assert_eq!("Hello", st.as_str());
/// assert_eq!(0, logs.len());
///
/// ```
/// ## Injects a function
/// ```
/// use parsic::combinators::*;
/// use parsic::core::*;
/// use parsic::primitives::CharStream;
///
/// let parser = pure(|_| true);
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(true, res.unwrap()(1));
/// assert_eq!("Hello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn pure<'f, A: Clone + 'f, S: 'f>(x: A) -> Parser<'f, A, S> {
    Parser::new(move |_, _| Some(x.clone()))
}

/// # Combinator: `compose` (function ver.)
///
/// Functional composition between parsers.
///
/// # Properties
///
/// Should satisfy [Applicative functor laws](https://wiki.haskell.org/Typeclassopedia#Laws_2).
///
/// - **Identity**: `compose(pure(id), p) ~ p`
/// - **Homomorphism**: `compose(pure(f), pure(g)) ~ pure(|x| f(g(x)))`
/// - **Interchange**: `compose(pf, pure(x)) ~ compose(pure(|f| f(x)), pf)`
/// - **Composition**: `compose(pf, pg.compose(px)) ~ compose(pure(|f| |g| |x| f(g(x))), px)`
///
/// Check out `test_applicative` module for naive examples of above laws.
///
/// # Example
/// ```
/// use parsic::combinators::*;
/// use parsic::core::Parsable;
/// use parsic::primitives::{char, CharStream};
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
        pf.parse(stream, logger)
            .and_then(|f| px.parse(stream, logger).map(|x| f(x)))
            .or_else(|| return_none(stream, &st))
    })
}

pub trait ApplicativeExt<'f, F: 'f, S>: Parsable<Stream = S, Result = F> {
    /// # Combinator: `compose`
    ///
    /// Functional composition between parsers.
    ///
    /// # Properties
    ///
    /// Should satisfy [Applicative functor laws](https://wiki.haskell.org/Typeclassopedia#Laws_2):
    ///
    /// - **Identity**: `pure(id).compose(p) ~ p`
    /// - **Homomorphism**: `pure(f).compose(pure(g)) ~ pure(|x| f(g(x)))`
    /// - **Interchange**: `pf.compose(pure(x)) ~ pure(|f| f(x)).compose(pf)`
    /// - **Composition**: `pf.compose(pg.compose(px)) ~ pure(|f| |g| |x| f(g(x))).compose(px)`
    ///
    /// Check out `test_applicative` module for naive examples of above laws.
    ///
    /// # Example
    /// ```
    /// use parsic::combinators::*;
    /// use parsic::core::Parsable;
    /// use parsic::primitives::{char, CharStream};
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
        F: Fn(A) -> B,
        S: Clone,
        Self: Sized + 'f,
    {
        compose(self, px)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> ApplicativeExt<'f, A, S> for P {}

#[cfg(test)]
mod test_applicative {
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
        //! `pure(|x| x).compose(p) ~ p`
        //! Identity law.
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
        //! Homomorphism, function application order does not matter.
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
        //! `pf.compose(pure(x)) ~ pure(|f| f(x)).compose(pf)`
        //! Interchange law.
        let (pf, x) = (pure::<fn(u64) -> u64, _>(|x| x + 1), 1);
        let parser1 = pf.clone().compose(pure(x));
        let parser2 = pure(|f: fn(u64) -> u64| f(x)).compose(pf);

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }

    #[test]
    fn composition() {
        //! `pf.compose(pg.compose(px)) ~ pure(|f| |g| |x| f(g(x))).compose(px)`
        //! Composition law
        let pf = pure::<fn(u64) -> u64, _>(|x| x + 3);
        let pg = pure::<fn(u64) -> u64, _>(|x| x * 5);
        let px = pure(2);
        let parser1 = pf.clone().compose(pg.clone().compose(px.clone()));
        let parser2 = (pure(|f: fn(u64) -> u64| move |g: fn(u64) -> u64| move |x: u64| f(g(x)))
            .compose(pf)
            .compose(pg))
        .compose(px);

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }
}
