use crate::core::{return_none, Parsable, Parser};

/// # Combinator: `many` (function ver.)
///
/// Apply given parser as many times as possible (**zero** or more times),
/// and returns a vector `Vec<T>` containg all the parse results. The
/// combinator always succeeds.
///
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{char, CharStream};
///
/// // Consume character 't' zero or more times
/// let parser = many(char('t'));
///
/// let mut st1 = CharStream::new("tttql");
/// let mut st2 = CharStream::new("ql");
/// let (res1, logs1) = parser.exec(&mut st1);
/// let (res2, logs2) = parser.exec(&mut st2);
///
/// assert_eq!(Some(vec!['t', 't', 't']), res1);
/// assert_eq!(Some(vec![]), res2);
/// assert_eq!(("ql", "ql"), (st1.as_str(), st2.as_str()));
/// assert_eq!((0, 0), (logs1.len(), logs2.len()));
/// ```
pub fn many<'f, A: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Vec<A>, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (mut st, mut lg) = (stream.clone(), logger.clone());
        let mut res = vec![];
        while let Some(x) = p.parse(stream, logger) {
            res.push(x);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        *logger = lg;
        Some(res)
    })
}

/// # Combinator: `some` (function ver.)
///
/// Apply given parser as many times as possible (**one** or more times),
/// and returns a vector `Vec<T>` containg all the parse results. The
/// combinator fails if the parser fails at the first attempt.
///
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{char, CharStream};
///
/// // Consume character 't' one or more times
/// let parser = some(char('t'));
///
/// let mut st1 = CharStream::new("tttql");
/// let mut st2 = CharStream::new("ql");
/// let (res1, logs1) = parser.exec(&mut st1);
/// let (res2, logs2) = parser.exec(&mut st2);
///
/// assert_eq!(Some(vec!['t', 't', 't']), res1);
/// assert_eq!(None, res2);
/// assert_eq!(("ql", "ql"), (st1.as_str(), st2.as_str()));
/// assert_eq!((0, 1), (logs1.len(), logs2.len()));
/// ```
pub fn some<'f, A: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Vec<A>, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (mut st, mut lg) = (stream.clone(), logger.clone());
        let mut res = vec![];
        while let Some(x) = p.parse(stream, logger) {
            res.push(x);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        match res {
            v if v.is_empty() => None,
            _ => {
                *logger = lg;
                Some(res)
            }
        }
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
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::Parsable;
/// use naive_parsec::primitives::{char, CharStream};
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
pub fn optional<'f, A: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Option<A>, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (st, lg) = (stream.clone(), logger.clone());
        match p.parse(stream, logger) {
            None => {
                *logger = lg;
                Some(return_none(stream, &st))
            }
            x => Some(x),
        }
    })
}

/// Implement replicative combinators for `Parsable<S>`.
pub trait ReplicativeExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// # Combinator: `many`
    ///
    /// Apply given parser as many times as possible (zero or more times),
    /// and returns a vector `Vec<T>` containg all the parse results. The
    /// combinator always succeeds.
    ///
    /// # Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::{char, CharStream};
    ///
    /// // Consume character 't' zero or more times
    /// let parser = char('t').many();
    ///
    /// let mut st1 = CharStream::new("tttql");
    /// let mut st2 = CharStream::new("ql");
    /// let (res1, logs1) = parser.exec(&mut st1);
    /// let (res2, logs2) = parser.exec(&mut st2);
    ///
    /// assert_eq!(Some(vec!['t', 't', 't']), res1);
    /// assert_eq!(Some(vec![]), res2);
    /// assert_eq!(("ql", "ql"), (st1.as_str(), st2.as_str()));
    /// assert_eq!((0, 0), (logs1.len(), logs2.len()));
    /// ```
    fn many(self) -> Parser<'f, Vec<A>, S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        many(self)
    }

    /// # Combinator: `some` (function ver.)
    ///
    /// Apply given parser as many times as possible (**one** or more times),
    /// and returns a vector `Vec<T>` containg all the parse results. The
    /// combinator fails if the parser fails at the first attempt.
    ///
    /// # Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::{char, CharStream};
    ///
    /// // Consume character 't' one or more time
    /// let parser = char('t').some();
    ///
    /// let mut st1 = CharStream::new("tttql");
    /// let mut st2 = CharStream::new("ql");
    /// let (res1, logs1) = parser.exec(&mut st1);
    /// let (res2, logs2) = parser.exec(&mut st2);
    ///
    /// assert_eq!(Some(vec!['t', 't', 't']), res1);
    /// assert_eq!(None, res2);
    /// assert_eq!(("ql", "ql"), (st1.as_str(), st2.as_str()));
    /// assert_eq!((0, 1), (logs1.len(), logs2.len()));
    /// ```
    fn some(self) -> Parser<'f, Vec<A>, S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        some(self)
    }

    /// # Combinator: `optional`
    ///
    /// Apply given parser **at most one time**. Denote the result
    /// of the given parser `p` as `x`, then the result of `p.optional()`
    /// would be `Some(x)`.
    ///
    /// # Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::{char, CharStream};
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
        Self: Sized + 'f,
        S: Clone,
    {
        optional(self)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> ReplicativeExt<'f, A, S> for P {}
