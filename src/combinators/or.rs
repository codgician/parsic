use crate::core::{return_none, Parsable, Parser};

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
        match p1.parse(stream, logger) {
            Some(x) => Some(x),
            None => {
                *stream = st.clone();
                *logger = lg;
                match p2.parse(stream, logger) {
                    Some(x) => Some(x),
                    None => return_none(stream, &st),
                }
            }
        }
    })
}

/// Implement `or` combinator for `Parsable<S>`.
pub trait OrExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
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

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> OrExt<'f, A, S> for P {}

#[cfg(test)]
mod test_or {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, CharStream};

    #[test]
    fn left_ok() {
        let parser = char('A').or(char('B'));

        let mut st = CharStream::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('A'), res);
        assert_eq!("hhh", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn right_ok() {
        let parser = char('B').or(char('A'));

        let mut st = CharStream::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('A'), res);
        assert_eq!("hhh", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn both_ok() {
        let parser = char('A').or(char('A'));

        let mut st = CharStream::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('A'), res);
        assert_eq!("hhh", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn both_fail() {
        let parser = char('B').or(char('C'));

        let mut st = CharStream::new("Ahhh");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Ahhh", st.as_str());
        assert_eq!(1, logs.len());
    }
}
