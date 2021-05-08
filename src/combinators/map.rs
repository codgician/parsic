use crate::core::{Msg, MsgBody, Parsable, Parser};

/// ## Combinator: `map` (function ver.)
pub fn map<'f, A: 'f, B: 'f, S>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> B + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream, logger| {
        p.parse(stream, logger).and_then(|x| Some(f(x)))
    })
}

/// ## Combinator: `map_option` (function ver.)
pub fn map_option<'f, A: 'f, B: 'f, S>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> Option<B> + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream, logger| {
        p.parse(stream, logger)
            .and_then(|x| f(x))
            .and_then(|x| Some(x))
    })
}

/// ## Combinator: `map_result` (function ver.)
pub fn map_result<'f, A: 'f, B: 'f, E: ToString, S>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> Result<B, E> + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream, logger| {
        p.parse(stream, logger).and_then(|x| match f(x) {
            Ok(r) => Some(r),
            Err(e) => {
                logger.add(Msg::Error(MsgBody::new(&e.to_string()[..], None)));
                None
            }
        })
    })
}

/// Implements `map` for `Parsable`:
pub trait MapExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// ## Combinator: `map`
    fn map<B: 'f>(self, f: impl Fn(A) -> B + 'f) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
    {
        map(self, f)
    }

    /// ## Combinator: `map_option`
    fn map_option<B: 'f>(
        self,
        f: impl Fn(A) -> Option<B> + 'f,
    ) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
    {
        map_option(self, f)
    }

    /// ## Combinator: `map_result`
    fn map_result<B: 'f, E>(
        self,
        f: impl Fn(A) -> Result<B, E> + 'f,
    ) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        E: ToString,
    {
        map_result(self, f)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> MapExt<'f, A, S> for P {}

#[cfg(test)]
mod test {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let parser = char('H').or(char('W')).map(|ch: char| ch == 'H');

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(true), res);
        assert_eq!("ello", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn select_ok() {
        let parser = char('-').and(char('1')).map(|(_, x)| x);

        let mut st = CharStream::new("-1");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('1'), res);
        assert_eq!("", st.as_str());
        assert_eq!(0, logs.len());
    }
}

#[cfg(test)]
mod test_map_result {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{satisfy, CharStream};

    #[test]
    fn ok_fully_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_result(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = CharStream::new("12345");
        let (res, logs) = nat_parser.exec(&mut st);

        assert_eq!(Some(12345), res);
        assert_eq!("", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_partially_consumed() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_result(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = CharStream::new("123de");
        let (res, logs) = nat_parser.exec(&mut st);

        assert_eq!(Some(123), res);
        assert_eq!("de", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let nat_parser = satisfy(|&ch| ch.is_digit(10))
            .some()
            .map_result(|v| v.into_iter().collect::<String>().parse::<i64>());

        let mut st = CharStream::new("abcde");
        let (res, logs) = nat_parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("abcde", st.as_str());
        assert_eq!(1, logs.len());
    }
}
