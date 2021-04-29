use crate::core::parser::*;
use crate::core::stream::*;
use crate::core::logger::*;

// Char parser builder
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharP(char);

impl<'a> Parsable<CharStream<'a>, char> for CharP {
    fn parse(&self, stream: &mut CharStream<'a>, logger: &mut ParseLogger) -> Option<char> {
        match stream.inp.next() {
            Some(ch) => {
                if ch == self.0 {
                    Some(ch)
                } else {
                    logger.with(Msg::Err(
                        MsgBody {
                            pos: stream.pos,
                            msg: format!("expecting '{}', but got '{}'.", self.0, ch)
                        }
                    ));
                    None
                }
            }
            None => {
                logger.with(Msg::Err(
                    MsgBody {
                        pos: stream.pos,
                        msg: format!("unexpected end of input.")
                    }
                ));
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
impl<'a, F> Parsable<CharStream<'a>, char> for SatisfyP<F>
    where F: Fn(&char) -> bool
{
    fn parse(&self, stream: &mut CharStream<'a>, logger: &mut ParseLogger) -> Option<char> {
        match stream.inp.next() {
            Some(ch) => {
                if self.0(&ch) {
                    Some(ch)
                } else {
                    logger.with(Msg::Err(
                        MsgBody {
                            pos: stream.pos,
                            msg: format!("'{}' does not satisfy required conditions.", ch)
                        }
                    ));
                    None
                }
            }
            None => {
                logger.with(Msg::Err(
                    MsgBody {
                        pos: stream.pos,
                        msg: format!("unexpected end of input.")
                    }
                ));
                None
            }
        }
    }
}

pub fn satisfy< F>(f: F) -> SatisfyP<F> where F: Fn(&char) -> bool {
    SatisfyP(f)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralP(String);

impl<'a> Parsable<CharStream<'a>, &'a str> for LiteralP {
    fn parse(&self, stream: &mut CharStream<'a>, logger: &mut ParseLogger) -> Option<&'a str> {
        if stream.as_stream().starts_with(&self.0[..]) {
            let ret = &stream.as_stream()[0 .. self.0.len()];
            stream.take(self.0.len()).for_each(|_| {});
            Some(ret)
        } else {
            logger.with(Msg::Err(
                MsgBody {
                    pos: stream.pos,
                    msg: format!("expecting \"{}\".", self.0)
                }
            ));
            None
        }
    }
}

pub fn literal(s: &str) -> LiteralP {
    LiteralP(s.to_owned())
}

#[cfg(test)]
mod test_char {
    use crate::core::parser::*;
    use crate::core::stream::*;
    use crate::core::logger::*;
    use super::char;

    // Should parse when character matches
    #[test]
    fn ok() {
        let mut st = CharStream::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('H'),
            char('H').parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    // Should return none when character does not match
    #[test]
    fn fail() {
        let mut st = CharStream::new("Hello");
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
    use crate::core::parser::*;
    use crate::core::stream::*;
    use crate::core::logger::*;
    use super::satisfy;

    // Should parse when character satisifies given condition
    #[test]
    fn ok() {
        let mut st = CharStream::new("Hello");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('H'),
            satisfy(|&ch| ch.is_uppercase()).parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    // Should return none when character does not satisfy given condition
    #[test]
    fn fail() {
        let mut st = CharStream::new("hello");
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
    use crate::core::parser::*;
    use crate::core::stream::*;
    use crate::core::logger::*;
    use super::literal;

    // Should parse when literal matches
    #[test]
    fn ok() {
        let mut st = CharStream::new("Hello!");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some("Hello"),
            literal("Hello").parse(&mut st, &mut log)
        );
        assert_eq!("!", st.as_stream());
        assert_eq!(0, log.len());
    }

    // Should return none when literal does not match
    #[test]
    fn fail() {
        let mut st = CharStream::new("Hell");
        let mut log = ParseLogger::default();
        assert_eq!(
            None,
            literal("Hello").parse(&mut st, &mut log)
        );
        assert_eq!("Hell", st.as_stream());
        assert_eq!(1, log.len());
    }
}
