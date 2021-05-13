use crate::core::{return_none, Msg, MsgBody, Parsable, Parser};

/// # Combinator: `map` (function ver.)
///
/// Maps the result of a parser to another value using the given function.
///
/// # Properties
///
/// Should satisfy [Functor laws](https://wiki.haskell.org/Typeclassopedia#Laws):
///
/// - **Identity**: `map(p, |x| x) ~ p`
/// - **Composition**: `map(p, |x| f(g(x))) ~ map(map(p, f), g)`
///
/// Check out `test_map` module in the source code for naive examples of above laws.
///
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = map(or(char('H'), char('W')), |ch: char| ch == 'H');
///
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(true), res);
/// assert_eq!("ello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn map<'f, A: 'f, B: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> B + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        p.parse(stream, logger)
            .map(|x| f(x))
            .or_else(|| return_none(stream, &st))
    })
}

/// # Combinator: `map_option` (function ver.)
///
/// Maps the result of a parser to another value using the given function that
/// produces an `Option<T>`. The only difference with `map` is that `map_option`
/// will automatically try to unwrap the `Option<T>` and will fail if the result
/// is `None`.
///
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{CharStream, satisfy};
///
/// let parser = map_option(satisfy(|_| true), |ch: char| ch.to_digit(10));
///
/// let mut st = CharStream::new("817");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(8), res);
/// assert_eq!("17", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn map_option<'f, A: 'f, B: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> Option<B> + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        p.parse(stream, logger).and_then(|x| f(x)).or_else(|| {
            logger.add(Msg::Error(MsgBody::new(
                "map_option recieved a function that yielded None.",
                None,
            )));
            return_none(stream, &st)
        })
    })
}

/// # Combinator: `map_result` (function ver.)
///
/// Maps the result of a parser to another value using the given function that
/// produces an `Result<T, E>`. The only difference with `map` is that `map_result`
/// will automatically try to unwrap the `Result<T, E>`. If an `Err` is yeilded,
/// `map_result` will log down the error message. Therefore, it requires `E` from
/// `Result<T, E>` to implement `ToString` trait.
///
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{CharStream, satisfy};
///
/// // A parser that consumes a natural number
/// let parser = map_result(
///                 some(satisfy(|&ch| ch.is_digit(10))),
///                 |v| v.into_iter().collect::<String>().parse::<i64>()
///              );
///
/// let mut st = CharStream::new("12345");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(12345), res);
/// assert_eq!("", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn map_result<'f, A: 'f, B: 'f, E: ToString, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> Result<B, E> + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        p.parse(stream, logger).and_then(|x| match f(x) {
            Ok(r) => Some(r),
            Err(e) => {
                logger.add(Msg::Error(MsgBody::new(&e.to_string()[..], None)));
                return_none(stream, &st)
            }
        })
    })
}

/// Implement `map` and related combinators for `Parsable`.
pub trait MapExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// # Combinator: `map`
    ///
    /// Maps the result of current parser to another value.
    ///
    /// # Properties
    ///
    /// Should satisfy [Functor laws](https://wiki.haskell.org/Typeclassopedia#Laws):
    ///
    /// - **Identity**: `p.map(|x| x) ~ p`
    /// - **Composition**: `p.map(|x| f(g(x))) ~ p.map(f).map(g)`
    ///
    /// Check out `test_map` module in the source code for naive examples of above laws.
    ///
    /// # Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::*;
    ///
    /// let parser = char('H').or(char('W'))
    ///                       .map(|ch: char| ch == 'H');
    ///
    /// let mut st = CharStream::new("Hello");
    /// let (res, logs) = parser.exec(&mut st);
    ///
    /// assert_eq!(Some(true), res);
    /// assert_eq!("ello", st.as_str());
    /// assert_eq!(0, logs.len());
    /// ```
    fn map<B: 'f>(self, f: impl Fn(A) -> B + 'f) -> Parser<'f, B, S>
    where
        S: Clone,
        Self: Sized + 'f,
    {
        map(self, f)
    }

    /// # Combinator: `map_option`
    ///
    /// Maps the result of a parser to another value using the given function that
    /// produces an `Option<T>`. The only difference with `map` is that `map_option`
    /// will automatically try to unwrap the `Option<T>` and will fail if the result
    /// is `None`.
    ///
    /// # Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::{CharStream, satisfy};
    ///
    /// let parser = satisfy(|_| true).map_option(|ch: char| ch.to_digit(10));
    ///
    /// let mut st = CharStream::new("817");
    /// let (res, logs) = parser.exec(&mut st);
    ///
    /// assert_eq!(Some(8), res);
    /// assert_eq!("17", st.as_str());
    /// assert_eq!(0, logs.len());
    /// ```
    fn map_option<B: 'f>(self, f: impl Fn(A) -> Option<B> + 'f) -> Parser<'f, B, S>
    where
        S: Clone,
        Self: Sized + 'f,
    {
        map_option(self, f)
    }

    /// # Combinator: `map_result`
    ///
    /// Maps the result of a parser to another value using the given function that
    /// produces an `Result<T, E>`. The only difference with `map` is that `map_result`
    /// will automatically try to unwrap the `Result<T, E>`. If an `Err` is yeilded,
    /// `map_result` will log down the error message. Therefore, it requires `E` from
    /// `Result<T, E>` to implement `ToString` trait.
    ///
    /// # Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::{CharStream, satisfy};
    ///
    /// // A parser that consumes a natural number
    /// let parser = satisfy(|&ch| ch.is_digit(10)).some()
    ///              .map_result(|v| v.into_iter().collect::<String>().parse::<i64>());
    ///
    /// let mut st = CharStream::new("12345");
    /// let (res, logs) = parser.exec(&mut st);
    ///
    /// assert_eq!(Some(12345), res);
    /// assert_eq!("", st.as_str());
    /// assert_eq!(0, logs.len());
    /// ```
    fn map_result<B: 'f, E>(self, f: impl Fn(A) -> Result<B, E> + 'f) -> Parser<'f, B, S>
    where
        E: ToString,
        S: Clone,
        Self: Sized + 'f,
    {
        map_result(self, f)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> MapExt<'f, A, S> for P {}

#[cfg(test)]
mod test_map {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::*;

    #[test]
    fn fail_with_grace() {
        let parser = char('-').and(char('1')).map(|(_, x)| x);

        let mut st = CharStream::new("+1");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("+1", st.as_str());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn identity() {
        //! `p.map(|x| x) ~ p`
        //! Preserves identity function.
        let parser1 = char('0').map(|x| x);
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
    fn composition() {
        //! `p.map(|x| f(g(x))) ~ p.map(f).map(g)`
        //! Preserves function composition.
        let f = |ch: char| if ch == '0' { 'a' } else { 'b' };
        let g = |ch: char| if ch == 'a' { 'A' } else { 'B' };
        let parser1 = char('0').map(|x| g(f(x)));
        let parser2 = char('0').map(f).map(g);

        assert_eq!(
            parser1.exec(&mut CharStream::new("01")),
            parser2.exec(&mut CharStream::new("01"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("10")),
            parser2.exec(&mut CharStream::new("10"))
        );
    }
}

#[cfg(test)]
mod test_map_option {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{satisfy, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = satisfy(|_| true).map_option(|ch: char| ch.to_digit(10));

        let mut st = CharStream::new("naive");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("naive", st.as_str());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_map_result {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{satisfy, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_result(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = CharStream::new("naive");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("naive", st.as_str());
        assert_eq!(1, logs.len());
    }
}
