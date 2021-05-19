use crate::core::{return_none, Parsable, Parser};

/// # Combinator: `empty`
///
/// A parser that consumes no item and always fails.
///
/// # Example
/// ```
/// use parsic::combinators::*;
/// use parsic::core::*;
/// use parsic::primitives::CharStream;
///
/// let parser = empty::<char, CharStream>();
///
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(None, res);
/// assert_eq!("Hello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn empty<'f, A: 'f, S: 'f>() -> Parser<'f, A, S> {
    Parser::new(move |_: &mut S, _| None)
}

/// # Combinator: `or` (function ver.)
///
/// Alternative combinator. Accepts two parsers as arguments,
/// if the first parser succeeds then its result is returned,
/// otherwise the result of the second parser is returned.
///
/// # Examples
/// ```
/// use parsic::combinators::*;
/// use parsic::core::Parsable;
/// use parsic::primitives::{char, CharStream};
///
/// // Comsumes a character 'A' or a character 'B'
/// let parser = or(char('B'), char('A'));
///
/// let mut st = CharStream::new("Ahhh");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('A'), res);
/// assert_eq!("hhh", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn or<'f, A: 'f, S: Clone>(
    p1: impl Parsable<Stream = S, Result = A> + 'f,
    p2: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, A, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (st, lg) = (stream.clone(), logger.clone());
        p1.parse(stream, logger).or_else(|| {
            *stream = st.clone();
            *logger = lg;
            p2.parse(stream, logger)
                .or_else(|| return_none(stream, &st))
        })
    })
}

/// Implement `or` combinator for `Parsable<S>`.
pub trait AlternativeExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// # Combinator: `or` (function ver.)
    ///
    /// Alternative combinator. Accepts two parsers as arguments,
    /// if the first parser succeeds then its result is returned,
    /// otherwise the result of the second parser is returned.
    ///
    /// # Examples
    /// ```
    /// use parsic::combinators::*;
    /// use parsic::core::Parsable;
    /// use parsic::primitives::{char, CharStream};
    ///
    /// // Comsumes a character 'A' or a character 'B'
    /// let parser = char('B').or(char('A'));
    ///
    /// let mut st = CharStream::new("Ahhh");
    /// let (res, logs) = parser.exec(&mut st);
    ///
    /// assert_eq!(Some('A'), res);
    /// assert_eq!("hhh", st.as_str());
    /// assert_eq!(0, logs.len());
    /// ```
    fn or(self, p: impl Parsable<Stream = S, Result = A> + 'f) -> Parser<'f, A, S>
    where
        S: Clone,
        Self: Sized + 'f,
    {
        or(self, p)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> AlternativeExt<'f, A, S> for P {}

#[cfg(test)]
mod test_alternative {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::{char, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = char('B').or(char('C'));

        let mut st = CharStream::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Ahhh", st.as_str());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn monoid_left_identity() {
        //! `empty().or(p) ~ p`
        //! Left identity law for monoid.
        let parser1 = empty().or(char('0'));
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
    fn monoid_right_identity() {
        //! `p.or(empty()) ~ p`
        //! Right identity law for monoid.
        let parser1 = char('0').or(empty());
        let parser2 = char('0');

        assert_eq!(
            parser1.exec(&mut CharStream::new("01")),
            parser2.exec(&mut CharStream::new("01"))
        );
        // Parse log is designed to be different if failure occurs
        assert_eq!(
            parser1.exec(&mut CharStream::new("10")).0,
            parser2.exec(&mut CharStream::new("10")).0
        );
    }

    #[test]
    fn monoid_associative() {
        //! `px.or(py).or(pz) ~ px.or(py.or(pz))`
        //! Associative law of monoid.
        let parser1 = char('0').or(char('1')).or(char('2'));
        let parser2 = char('0').or(char('1').or(char('2')));

        assert_eq!(
            parser1.exec(&mut CharStream::new("01")),
            parser2.exec(&mut CharStream::new("01"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("10")),
            parser2.exec(&mut CharStream::new("10"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("22")),
            parser2.exec(&mut CharStream::new("22"))
        );
    }

    #[test]
    fn left_zero() {
        //! `empty().compose(x) ~ empty()`
        //! Left zero law.
        let parser1 = empty::<fn(u64) -> u64, _>().compose(pure(1));
        let parser2 = empty();

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
    }

    #[test]
    fn right_zero() {
        //! pf.compose(empty()) ~ empty()
        //! Left zero law.
        let parser1 = pure(|x| x + 1).compose(empty::<u64, _>());
        let parser2 = empty();

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
    }

    #[test]
    fn left_distribution() {
        //! `pf.or(pg).compose(px) ~ (pf.compose(px)).or(pg.compose(px))`
        //! Left distribution law.
        let pf = pure::<fn(u64) -> u64, CharStream>(|x| x + 3);
        let pg = pure::<fn(u64) -> u64, CharStream>(|x| x * 5);
        let px = pure::<u64, CharStream>(1);
        let parser1 = pf.clone().or(pg.clone()).compose(px.clone());
        let parser2 = (pf.compose(px.clone())).or(pg.compose(px));

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
    }

    #[test]
    fn right_distribution() {
        //! `pf.compose(px.or(py)) ~ (pf.compose(px)).or(pf.compose(py))`
        //! Right distribution law.
        let pf = pure::<fn(u64) -> u64, CharStream>(|x| x + 3);
        let px = pure::<u64, CharStream>(5);
        let py = pure::<u64, CharStream>(7);
        let parser1 = pf.clone().compose(px.clone().or(py.clone()));
        let parser2 = pf.clone().compose(px).or(pf.compose(py));

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
    }

    #[test]
    fn left_catch() {
        //! `pure(a).or(x) ~ pure(a)`
        //! Left catch.
        let parser1 = pure('0').or(char('1'));
        let parser2 = pure('0');

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
    }
}
