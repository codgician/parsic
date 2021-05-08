use crate::combinators::*;
use crate::core::{Msg, MsgBody, Parsable, Parser};
use crate::primitives::CharStream;

/// ## Combinator: `satisfy`
/// Consumes a single character if given condition satisifies.
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

/// ## Combinator: `char`
/// Consumes given char at from the parse stream.
pub fn char<'f>(ch: char) -> Parser<'f, char, CharStream<'f>> {
    satisfy(move |x| *x == ch)
}

/// ## Combinator: `literal`
/// Consumes given literal string.
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

/// ## Combinator: `regex`
/// Consumes a literal string that matches given regular expression.
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

/// ## Combinator: `space`
/// Consumes a single whitespace character (` `, `\n`, `\r` or `\t`).
pub fn space<'f>() -> Parser<'f, char, CharStream<'f>> {
    char(' ').or(char('\n')).or(char('\r')).or(char('\t'))
}

/// ## Combinator: `trim` (function ver.)
/// Consumes as many whitespace characters (` `, `\n`, `\r` or `\t`)
/// as possible surrounding given parser.
pub fn trim<'f, A: 'f>(
    p: impl Parsable<Stream = CharStream<'f>, Result = A> + 'f,
) -> Parser<'f, A, CharStream<'f>> {
    mid(space().many(), p, space().many())
}

pub trait PrimitiveExt<'f, A: 'f>: Parsable<Stream = CharStream<'f>, Result = A> {
    /// ## Combinator: `trim`
    /// Consumes as many whitespace characters (` `, `\n`, `\r` or `\t`)
    /// as possible surrounding given parser.
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
    fn ok() {
        let parser = char('H');

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('H'), res);
        assert_eq!("ello", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
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
    fn ok() {
        let parser = satisfy(|&ch| ch.is_uppercase());

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('H'), res);
        assert_eq!("ello", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
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
    fn ok() {
        let parser = literal("Hello");

        let mut st = CharStream::new("Hello!");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some("Hello"), res);
        assert_eq!("!", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
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
    fn ok() {
        let parser = regex(r"^\d{2}/\d{2}/\d{4}");

        let mut st = CharStream::new("10/30/2020!");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some("10/30/2020"), res);
        assert_eq!("!", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = regex(r"^\d{2}/\d{2}/\d{4}");

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(1, logs.len());
    }
}
