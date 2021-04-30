use crate::core::*;
use crate::primitives::*;

// Char parser builder
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharP(char);

impl Parsable<StrState> for CharP {
    type Result = char;
    
    fn parse(&self, stream: &mut StrState, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        match stream.inp.next() {
            Some(ch) => {
                if ch == self.0 {
                    Some(ch)
                } else {
                    logger.with(Msg::Error(MsgBody::new(
                        &format!("expecting '{}', but got '{}'.", self.0, ch)[..],
                        Some(stream.pos)
                     )));
                    None
                }
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(stream.pos)
                )));
                None
            }
        }
    }
}

pub fn char(ch: char) -> CharP {
    CharP(ch)
}

// Satisfy parser builder
#[derive(Clone, Copy, Debug)]
pub struct SatisfyP<F>(F);

impl<'a, F> Parsable<StrState> for SatisfyP<F>
    where F: Fn(&char) -> bool
{
    type Result = char;

    fn parse(&self, stream: &mut StrState, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        match stream.inp.next() {
            Some(ch) => {
                if self.0(&ch) {
                    Some(ch)
                } else {
                    logger.with(Msg::Error(MsgBody::new(
                        &format!("'{}' does not satisfy required conditions.", ch)[..],
                        Some(stream.pos)
                    )));
                    None
                }
            }
            None => {
                logger.with(Msg::Error(MsgBody::new(
                    "unexpected end of input.",
                    Some(stream.pos)
                )));
                None
            }
        }
    }
}

pub fn satisfy< F>(f: F) -> SatisfyP<F> 
    where F: Fn(&char) -> bool 
{
    SatisfyP(f)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralP(String);

impl Parsable<StrState> for LiteralP {
    type Result = &'static str;

    fn parse(&self, stream: &mut StrState, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        if stream.as_stream().starts_with(&self.0[..]) {
            let ret = &stream.as_stream()[0 .. self.0.len()];
            stream.take(self.0.len()).for_each(|_| {});
            Some(ret)
        } else {
            logger.with(Msg::Error(MsgBody::new(
                &format!("expecting \"{}\".", self.0)[..],
                Some(stream.pos)
            )));
            None
        }
    }
}

pub fn literal(s: &str) -> LiteralP {
    LiteralP(s.to_owned())
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
