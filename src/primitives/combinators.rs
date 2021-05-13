use crate::combinators::*;
use crate::core::{Msg, MsgBody, Parsable, Parser};
use crate::primitives::CharStream;

/// # Combinator: `satisfy`
///
/// Consume a single character if given function applied
/// to the next character from the parse stream yields `true`.
///
/// # Example
/// ```
/// use parsic::core::Parsable;
/// use parsic::primitives::{CharStream, satisfy};
///
/// // Consume a uppercase letter
/// let parser = satisfy(|&ch| ch.is_uppercase());
///
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('H'), res);
/// assert_eq!("ello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn satisfy<'f>(f: impl Fn(&char) -> bool + 'f) -> Parser<'f, char, CharStream<'f>> {
    Parser::new(move |stream: &mut CharStream<'f>, logger| {
        let st = stream.clone();
        match stream.next() {
            Some(ch) if f(&ch) => Some(ch),
            Some(ch) => {
                *stream = st;
                logger.with(Msg::Error(MsgBody::new(
                    &format!("'{}' does not satisfy required conditions.", ch)[..],
                    Some(stream.pos()),
                )));
                None
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(stream.pos()),
                )));
                None
            }
        }
    })
}

/// # Combinator: `char`
///
/// Consume the given char from the parse stream.
/// `char(x)` is equivalent to `satisfy(|x: &char| *x == ch)`
///
/// # Example
/// ```
/// use parsic::core::Parsable;
/// use parsic::primitives::{char, CharStream};
///
/// // Consume a single character 'H'
/// let parser = char('H');
///
/// let mut st = CharStream::new("Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('H'), res);
/// assert_eq!("ello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn char<'f>(ch: char) -> Parser<'f, char, CharStream<'f>> {
    satisfy(move |x| *x == ch)
}

/// # Combinator: `literal`
///
/// Consume given literal string from the parse stream.test
///
/// # Example
/// ```
/// use parsic::core::Parsable;
/// use parsic::primitives::{CharStream, literal};
///
/// // Consume literal string "Hello"
/// let parser = literal("Hello");
///
/// let mut st = CharStream::new("Hello!");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some("Hello"), res);
/// assert_eq!("!", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn literal<'f>(s: &'f str) -> Parser<'f, &'f str, CharStream> {
    Parser::new(move |stream: &mut CharStream<'f>, logger| {
        if stream.as_str().starts_with(s) {
            let ret = &stream.as_str()[0..s.len()];
            stream.take(s.len()).for_each(|_| {});
            Some(ret)
        } else {
            logger.with(Msg::Error(MsgBody::new(
                &format!("expecting \"{}\".", s)[..],
                Some(stream.pos()),
            )));
            None
        }
    })
}

/// # Combinator: `regex`
///
/// Consume a literal string that matches given regular expression.
///
/// # Example
/// ```
/// use parsic::core::Parsable;
/// use parsic::primitives::{CharStream, regex};
///
/// // Consume a date string
/// let parser = regex(r"^\d{2}/\d{2}/\d{4}");
///
/// let mut st = CharStream::new("10/30/2020!");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some("10/30/2020"), res);
/// assert_eq!("!", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn regex<'f>(re: &'f str) -> Parser<'f, &'f str, CharStream> {
    Parser::new(move |stream: &mut CharStream<'f>, logger| {
        let regex = regex::Regex::new(re).unwrap();
        let s = stream.as_str();
        match regex.find(s) {
            Some(m) if m.start() == 0 => {
                stream.take(m.end()).for_each(|_| {});
                Some(&s[0..m.end()])
            }
            _ => {
                logger.with(Msg::Error(MsgBody::new(
                    &format!("expecting \"{}\".", regex.as_str())[..],
                    Some(stream.pos()),
                )));
                None
            }
        }
    })
}

/// # Combinator: `space`
///
/// Consume a single whitespace character (` `, `\n`, `\r` or `\t`).
/// Equivalant to `char(' ').or(char('\n')).or(char('\r')).or(char('\t'))`.
///
/// # Example
/// ```
/// use parsic::core::Parsable;
/// use parsic::primitives::{CharStream, space};
///
/// // Consume a whitespace character
/// let parser = space();
///
/// let mut st = CharStream::new(" Hello");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(' '), res);
/// assert_eq!("Hello", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn space<'f>() -> Parser<'f, char, CharStream<'f>> {
    char(' ').or(char('\n')).or(char('\r')).or(char('\t'))
}

/// # Combinator: `trim` (function ver.)
///
/// Consume as many whitespace characters (` `, `\n`, `\r` or `\t`)
/// as possible surrounding given parser. `trim(p)` is equivalant to
/// `mid(space().many(), p, space().many())`.
///
/// # Example
/// ```
/// use parsic::core::Parsable;
/// use parsic::primitives::{CharStream, literal, trim};
///
/// // Consume a whitespace character
/// let parser = trim(literal("Hello"));
///
/// let mut st = CharStream::new("   Hello   ");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some("Hello"), res);
/// assert_eq!("", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
pub fn trim<'f, A: 'f>(
    p: impl Parsable<Stream = CharStream<'f>, Result = A> + 'f,
) -> Parser<'f, A, CharStream<'f>> {
    mid(space().many(), p, space().many())
}

/// Implement `trim` method for `Parsable<CharStream>`:
pub trait PrimitiveExt<'f, A: 'f>: Parsable<Stream = CharStream<'f>, Result = A> {
    /// # Combinator: `trim`
    ///
    /// Consume as many whitespace characters (` `, `\n`, `\r` or `\t`)
    /// as possible surrounding given parser. `trim(p)` is equivalant to
    /// `mid(space().many(), p, space().many())`.
    ///
    /// # Example
    /// ```
    /// use parsic::core::Parsable;
    /// use parsic::primitives::{CharStream, literal, PrimitiveExt, trim};
    ///
    /// // Consume a whitespace character
    /// let parser = literal("Hello").trim();
    ///
    /// let mut st = CharStream::new("   Hello   ");
    /// let (res, logs) = parser.exec(&mut st);
    ///
    /// assert_eq!(Some("Hello"), res);
    /// assert_eq!("", st.as_str());
    /// assert_eq!(0, logs.len());
    /// ```
    fn trim(self) -> Parser<'f, A, CharStream<'f>>
    where
        Self: Sized + 'f,
    {
        trim(self)
    }
}

impl<'f, A: 'f, P: Parsable<Stream = CharStream<'f>, Result = A>> PrimitiveExt<'f, A> for P {}

#[cfg(test)]
mod test_char {
    use crate::core::Parsable;
    use crate::primitives::{char, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = char('h');

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_satisfy {
    use crate::core::Parsable;
    use crate::primitives::{satisfy, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = satisfy(|&ch| ch.is_uppercase());

        let mut st = CharStream::new("hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("hello", st.as_str());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_literal {
    use crate::core::Parsable;
    use crate::primitives::{literal, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = literal("Hello");

        let mut st = CharStream::new("Hell");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hell", st.as_str());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_regex {
    use crate::core::Parsable;
    use crate::primitives::{regex, CharStream};

    #[test]
    fn fail_with_grace() {
        let parser = regex(r"^\d{2}/\d{2}/\d{4}");

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(1, logs.len());
    }
}
