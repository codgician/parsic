use crate::core::Parser;

/// # Combinator: `empty`
///
/// A parser that consumes no item and always fails.
///
/// # Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::*;
/// use naive_parsec::primitives::CharStream;
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
    Parser::new(|_, _| None)
}

/// # Combinator: `pure`
///
/// Injects a value into an identity parser.
///
/// # Examples
/// # Injects a value
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::*;
/// use naive_parsec::primitives::CharStream;
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
/// # Injects a function
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::*;
/// use naive_parsec::primitives::CharStream;
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
