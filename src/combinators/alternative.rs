use crate::combinators::{map, pure};
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
/// # Properties
///
/// Should satisfy [Alternative laws](https://wiki.haskell.org/Typeclassopedia#Laws_6).
///
/// Instances of `Parser` and `or` forms a monoid:
///  
/// - **Left identity**: `or(empty(), p) ~ p`
/// - **Right identity**: `or(p, empty()) ~ p`
/// - **Associative**: `or(or(px, py), pz) ~ or(px, or(py, pz))`
///
/// Following properties exist when `empty` and `or` interacts with `pure` and `compose`:
///
/// - **Left zero**: `compose(empty(), x) ~ empty()`
/// - **Right zero**: `compose(pf, empty()) ~ empty()`
/// - **Left distribution**: `compose(or(pf, pg), px) ~ or(compose(pf, px), pg.compose(px))`
/// - **Right distribution**: `compose(pf, or(px, py)) ~ or(compose(pf, px), pf.compose(py))`
/// - **Left catch**: `or(pure(a), x) ~ pure(a)`
///
/// Check out `test_alternative` module for naive examples of above laws.
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

/// # Combinator: `optional` (function ver.)
///
/// Apply given parser **at most one time**. Denote the result
/// of the given parser `p` as `x`, then the result of `optional(p)`
/// would be `Some(x)`.
///
/// # Example
/// ```
/// use parsic::combinators::*;
/// use parsic::core::Parsable;
/// use parsic::primitives::{char, CharStream};
///
/// // Consume character 't' at most one time
/// let parser = char('t').optional();
///
/// let mut st1 = CharStream::new("tttql");
/// let mut st2 = CharStream::new("ql");
/// let (res1, logs1) = parser.exec(&mut st1);
/// let (res2, logs2) = parser.exec(&mut st2);
///
/// assert_eq!(Some(Some('t')), res1);
/// assert_eq!(Some(None), res2);
/// assert_eq!(("ttql", "ql") ,(st1.as_str(), st2.as_str()));
/// assert_eq!((0, 0), (logs1.len(), logs2.len()));
/// ```
pub fn optional<'f, A: Clone + 'f, S: Clone + 'f>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Option<A>, S> {
    or(map(p, Some), pure(None))
}

pub trait AlternativeExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// # Combinator: `or`
    ///
    /// Alternative combinator. Accepts two parsers as arguments,
    /// if the first parser succeeds then its result is returned,
    /// otherwise the result of the second parser is returned.
    ///
    /// # Properties
    ///
    /// Should satisfy [Alternative laws](https://wiki.haskell.org/Typeclassopedia#Laws_6).
    ///
    /// Instances of `Parser` and `or` forms a monoid:
    ///  
    /// - **Left identity**: `empty().or(p) ~ p`
    /// - **Right identity**: `p.or(empty()) ~ p`
    /// - **Associative**: `px.or(py).or(pz) ~ px.or(py.or(pz))`
    ///
    /// Following properties exist when `empty` and `or` interacts with `pure` and `compose`:
    ///
    /// - **Left zero**: `empty().compose(x) ~ empty()`
    /// - **Right zero**: `pf.compose(empty()) ~ empty()`
    /// - **Left distribution**: `pf.or(pg).compose(px) ~ pf.compose(px).or(pg.compose(px))`
    /// - **Right distribution**: `pf.compose(px.or(py)) ~ pf.compose(px).or(pf.compose(py))`
    /// - **Left catch**: `pure(a).or(x) ~ pure(a)`
    ///
    /// Check out `test_alternative` module for naive examples of above laws.
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

    /// # Combinator: `optional`
    ///
    /// Apply given parser **at most one time**. Denote the result
    /// of the given parser `p` as `x`, then the result of `p.optional()`
    /// would be `Some(x)`.
    ///
    /// # Example
    /// ```
    /// use parsic::combinators::*;
    /// use parsic::core::Parsable;
    /// use parsic::primitives::{char, CharStream};
    ///
    /// // Consume character 't' at most one time
    /// let parser = char('t').optional();
    ///
    /// let mut st1 = CharStream::new("tttql");
    /// let mut st2 = CharStream::new("ql");
    /// let (res1, logs1) = parser.exec(&mut st1);
    /// let (res2, logs2) = parser.exec(&mut st2);
    ///
    /// assert_eq!(Some(Some('t')), res1);
    /// assert_eq!(Some(None), res2);
    /// assert_eq!(("ttql", "ql") ,(st1.as_str(), st2.as_str()));
    /// assert_eq!((0, 0), (logs1.len(), logs2.len()));
    /// ```
    fn optional(self) -> Parser<'f, Option<A>, S>
    where
        A: Clone,
        S: Clone + 'f,
        Self: Sized + 'f,
    {
        optional(self)
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
        // Logs are meant to be different
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
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }

    #[test]
    fn right_zero() {
        //! `pf.compose(empty()) ~ empty()`
        //! Left zero law.
        let parser1 = pure(|x: u64| x + 1).compose(empty());
        let parser2 = empty();

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }

    #[test]
    fn left_distribution() {
        //! `pf.or(pg).compose(px) ~ pf.compose(px).or(pg.compose(px))`
        //! Left distribution law.
        let pf = pure::<fn(u64) -> u64, _>(|x| x + 3);
        let pg = pure::<fn(u64) -> u64, _>(|x| x * 5);
        let px = pure(2);
        let parser1 = pf.clone().or(pg.clone()).compose(px.clone());
        let parser2 = pf.compose(px.clone()).or(pg.compose(px));

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }

    #[test]
    fn right_distribution() {
        //! `pf.compose(px.or(py)) ~ pf.compose(px).or(pf.compose(py))`
        //! Right distribution law.
        let pf = pure::<fn(u64) -> u64, _>(|x| x + 3);
        let (px, py) = (pure(5), pure(7));
        let parser1 = pf.clone().compose(px.clone().or(py.clone()));
        let parser2 = pf.clone().compose(px).or(pf.compose(py));

        assert_eq!(
            parser1.exec(&mut CharStream::new("")),
            parser2.exec(&mut CharStream::new(""))
        );
    }

    #[test]
    fn left_catch() {
        //! `pure(a).or(x) ~ pure(a)`
        //! Left catch.
        let parser1 = pure('0').or(char('1'));
        let parser2 = pure('0');

        assert_eq!(
            parser1.exec(&mut CharStream::new("1")),
            parser2.exec(&mut CharStream::new("1"))
        );
    }
}
