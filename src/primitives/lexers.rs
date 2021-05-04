use crate::core::{Msg, MsgBody, Parsable, ParseLogger};
use crate::primitives::StrState;

// Char
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharP(char);

impl CharP {
    pub(crate) fn new(ch: char) -> Self {
        Self(ch)
    }
}

impl Parsable<StrState> for CharP {
    type Result = char;

    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger) -> Option<Self::Result> {
        match state.next() {
            Some(ch) => {
                if ch == self.0 {
                    Some(ch)
                } else {
                    logger.with(Msg::Error(MsgBody::new(
                        &format!("expecting '{}', but got '{}'.", self.0, ch)[..],
                        Some(state.pos),
                    )));
                    None
                }
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(state.pos),
                )));
                None
            }
        }
    }
}

/// ### Lexer: `char`
/// Consumes one char at a time from parse stream.
pub fn char(ch: char) -> CharP {
    CharP::new(ch)
}

// Satisfy
#[derive(Clone, Copy, Debug)]
pub struct SatisfyP<F>(F);

impl<F> SatisfyP<F> {
    pub(crate) fn new(func: F) -> Self {
        Self(func)
    }
}

impl<'a, F> Parsable<StrState> for SatisfyP<F>
where
    F: Fn(&char) -> bool,
{
    type Result = char;

    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger) -> Option<Self::Result> {
        match state.next() {
            Some(ch) => {
                if self.0(&ch) {
                    Some(ch)
                } else {
                    logger.with(Msg::Error(MsgBody::new(
                        &format!("'{}' does not satisfy required conditions.", ch)[..],
                        Some(state.pos),
                    )));
                    None
                }
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(state.pos),
                )));
                None
            }
        }
    }
}

/// ### Lexer: `satisfy`
/// Consumes a single character if given condition satisifies.
pub fn satisfy<F>(f: F) -> SatisfyP<F>
where
    F: Fn(&char) -> bool,
{
    SatisfyP::new(f)
}

// Literal
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralP(String);

impl LiteralP {
    pub(crate) fn new(lit: &str) -> Self {
        Self(lit.to_owned())
    }
}

impl Parsable<StrState> for LiteralP {
    type Result = &'static str;

    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger) -> Option<Self::Result> {
        if state.as_stream().starts_with(&self.0[..]) {
            let ret = &state.as_stream()[0..self.0.len()];
            state.take(self.0.len()).for_each(|_| {});
            Some(ret)
        } else {
            logger.with(Msg::Error(MsgBody::new(
                &format!("expecting \"{}\".", self.0)[..],
                Some(state.pos),
            )));
            None
        }
    }
}

/// ### Lexer: `literal`
/// Consumes given literal string.
pub fn literal(s: &str) -> LiteralP {
    LiteralP::new(s)
}

// Regex
#[derive(Clone, Debug)]
pub struct RegexP(regex::Regex);

impl RegexP {
    pub fn new(re: &str) -> Result<Self, regex::Error> {
        regex::Regex::new(re).map(|r| Self(r))
    }

    pub fn unwrap(self) -> regex::Regex {
        self.0
    }

    pub fn inspect(&self) -> &regex::Regex {
        &self.0
    }
}

impl From<regex::Regex> for RegexP {
    fn from(re: regex::Regex) -> Self {
        Self(re)
    }
}

impl Parsable<StrState> for RegexP {
    type Result = &'static str;

    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger)
        -> Option<Self::Result>
    {
        let stream = state.as_stream();
        match self.0.find(stream) {
            Some(m) if m.start() == 0 => {
                state.take(m.end()).for_each(|_| {});
                Some(&stream[0 .. m.end()])
            }
            _ => {
                logger.with(Msg::Error(MsgBody::new(
                    &format!("expecting \"{}\".", self.0.as_str())[..],
                    Some(state.pos),
                )));
                None
            }
        }
    }
}

/// ### Combinator: `regex`
/// Consumes a literal string that matches given regular expression.
pub fn regex(re: &str) -> RegexP {
    RegexP::new(re).unwrap()
}

#[cfg(test)]
mod test_char {
    use crate::core::Parsable;
    use crate::primitives::{char, StrState};

    #[test]
    fn ok() {
        let parser = char('H');

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('H'), res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = char('h');

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_satisfy {
    use crate::core::Parsable;
    use crate::primitives::{satisfy, StrState};

    #[test]
    fn ok() {
        let parser = satisfy(|&ch| ch.is_uppercase());

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some('H'), res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = satisfy(|&ch| ch.is_uppercase());

        let mut st = StrState::new("hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_literal {
    use crate::core::Parsable;
    use crate::primitives::{literal, StrState};

    #[test]
    fn ok() {
        let parser = literal("Hello");

        let mut st = StrState::new("Hello!");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some("Hello"), res);
        assert_eq!("!", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = literal("Hello");

        let mut st = StrState::new("Hell");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hell", st.as_stream());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_regex {
    use crate::core::Parsable;
    use crate::primitives::{regex, StrState};

    #[test]
    fn ok() {
        let parser = regex(r"^\d{2}/\d{2}/\d{4}");

        let mut st = StrState::new("10/30/2020!");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some("10/30/2020"), res);
        assert_eq!("!", st.as_stream());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = regex(r"^\d{2}/\d{2}/\d{4}");

        let mut st = StrState::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_stream());
        assert_eq!(1, logs.len());
    }
}
