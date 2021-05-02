use crate::core::*;
use crate::primitives::*;

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
    
    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        match state.next() {
            Some(ch) => {
                if ch == self.0 {
                    Some(ch)
                } else {
                    logger.with(Msg::Error(MsgBody::new(
                        &format!("expecting '{}', but got '{}'.", self.0, ch)[..],
                        Some(state.pos)
                     )));
                    None
                }
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(state.pos)
                )));
                None
            }
        }
    }
}

/// ### Lexer: `char`
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
    F: Fn(&char) -> bool
{
    type Result = char;

    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        match state.next() {
            Some(ch) => {
                if self.0(&ch) {
                    Some(ch)
                } else {
                    logger.with(Msg::Error(MsgBody::new(
                        &format!("'{}' does not satisfy required conditions.", ch)[..],
                        Some(state.pos)
                    )));
                    None
                }
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(state.pos)
                )));
                None
            }
        }
    }
}

/// ### Lexer: `satisfy`
pub fn satisfy< F>(f: F) -> SatisfyP<F> 
where
    F: Fn(&char) -> bool 
{
    SatisfyP::new(f)
}

// Literal
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralP(String);

impl LiteralP {
    pub(crate) fn new<'a>(lit: &'a str) -> Self {
        Self(lit.to_owned())
    }
}

impl Parsable<StrState> for LiteralP {
    type Result = &'static str;

    fn parse(&self, state: &mut StrState, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        if state.as_stream().starts_with(&self.0[..]) {
            let ret = &state.as_stream()[0 .. self.0.len()];
            state.take(self.0.len()).for_each(|_| {});
            Some(ret)
        } else {
            logger.with(Msg::Error(MsgBody::new(
                &format!("expecting \"{}\".", self.0)[..],
                Some(state.pos)
            )));
            None
        }
    }
}

/// ### Lexer: `literal`
pub fn literal(s: &str) -> LiteralP {
    LiteralP::new(s)
}

#[cfg(test)]
mod test_char {
    use crate::core::*;
    use crate::primitives::{ StrState, char };

    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('H'),
            char('H').parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn fail() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            char('h').parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, log.len());
    }
}

#[cfg(test)]
mod test_satisfy {
    use crate::core::*;
    use crate::primitives::{ StrState, satisfy };

    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('H'),
            satisfy(|&ch| ch.is_uppercase()).parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn fail() {
        let mut st = StrState::new("hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            satisfy(|&ch| ch.is_uppercase()).parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(1, log.len());
    }
}

#[cfg(test)]
mod test_literal {
    use crate::core::*;
    use crate::primitives::{ StrState, literal };

    #[test]
    fn ok() {
        let mut st = StrState::new("Hello!");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some("Hello"),
            literal("Hello").parse(&mut st, &mut log)
        );
        assert_eq!("!", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn fail() {
        let mut st = StrState::new("Hell");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            literal("Hello").parse(&mut st, &mut log)
        );
        assert_eq!("Hell", st.as_stream());
        assert_eq!(1, log.len());
    }
}
