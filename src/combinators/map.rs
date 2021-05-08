use crate::core::{return_none, Msg, MsgBody, Parsable, Parser};

/// ## Combinator: `map` (function ver.)
pub fn map<'f, A: 'f, B: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> B + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        p.parse(stream, logger)
            .and_then(|x| Some(f(x)))
            .or_else(|| return_none(stream, &st))
    })
}

/// ## Combinator: `map_option` (function ver.)
pub fn map_option<'f, A: 'f, B: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> Option<B> + 'f,
) -> Parser<'f, B, S> {
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        p.parse(stream, logger)
            .and_then(|x| f(x))
            .and_then(|x| Some(x))
            .or_else(|| return_none(stream, &st))
    })
}

/// ## Combinator: `map_result` (function ver.)
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

/// Implements `map` for `Parsable`:
pub trait MapExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
    /// ## Combinator: `map`
    ///
    /// Maps the result of current parser to another value.
    ///
    /// ### Example
    /// ```
    /// use naive_parsec::combinators::*;
    /// use naive_parsec::core::Parsable;
    /// use naive_parsec::primitives::*;
    ///
    /// let parser = char('H').or(char('W')).map(|ch: char| ch == 'H');
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
        Self: Sized + 'f,
        S: Clone,
    {
        map(self, f)
    }

    /// ## Combinator: `map_option`
    fn map_option<B: 'f>(self, f: impl Fn(A) -> Option<B> + 'f) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        map_option(self, f)
    }

    /// ## Combinator: `map_result`
    fn map_result<B: 'f, E>(self, f: impl Fn(A) -> Result<B, E> + 'f) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        E: ToString,
        S: Clone,
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
        //! `p.map(id) ~ p`
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
