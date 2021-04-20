use crate::core::parser::{ Parser, ParseState };
use crate::core::logger::{ Msg, MsgBody };

// Parser builder: Char
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Char {
    ch: char
}

impl Char {
    pub fn new(ch: char) -> Self {
        Self { ch: ch }
    } 
}

impl<'a> Parser<ParseState<'a>> for Char {
    type ParsedType = char;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        match state.inp.next() {
            Some(ch) => {
                if ch == self.ch {
                    Some(ch)
                } else {
                    state.log.with(Msg::Err(
                        MsgBody {
                            pos: state.pos,
                            msg: format!("expecting '{}', but got '{}'.", self.ch, ch)
                        }
                    ));
                    None
                }
            }
            None => {
                state.log.with(Msg::Err(
                    MsgBody {
                        pos: state.pos,
                        msg: format!("unexpected end of input.")
                    }
                ));
                None
            }
        }
    }
}

pub fn char(ch: char) -> Char {
    Char::new(ch)
}

// Satisfy parser
#[derive(Clone, Copy, Debug)]
pub struct Satisfy<F> {
    func: F,
    // err_msg: Option<&'a str>
}

impl<F> Satisfy<F> {
    pub fn new(func: F) -> Satisfy<F> {
        Self { func: func }
    } 
}

impl<'a, F> Parser<ParseState<'a>> for Satisfy<F>
    where F: Fn(&char) -> bool
{
    type ParsedType = char;
    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        match state.inp.next() {
            Some(ch) => {
                if (self.func)(&ch) {
                    Some(ch)
                } else {
                    state.log.with(Msg::Err(
                        MsgBody {
                            pos: state.pos,
                            msg: format!("'{}' does not satisfy required conditions.", ch)
                        }
                    ));
                    None
                }
            }
            None => {
                state.log.with(Msg::Err(
                    MsgBody {
                        pos: state.pos,
                        msg: format!("unexpected end of input.")
                    }
                ));
                None
            }
        }
    }
}

pub fn satisfy< F>(f: F) -> Satisfy<F>
    where F: Fn(&char) -> bool
{
    Satisfy::new(f)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    expected: String
}

impl Literal {
    pub fn new(expected: String) -> Literal {
        Self { expected: expected }
    }
}

impl<'a> Parser<ParseState<'a>> for Literal {
    type ParsedType = &'a str;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        if state.inp.as_str().starts_with(&self.expected[..]) {
            let ret = &state.inp.as_str()[0 .. self.expected.len()];
            state.take(self.expected.len()).for_each(|_| {});
            Some(ret)
        } else {
            state.log.with(Msg::Err(
                MsgBody {
                    pos: state.pos,
                    msg: format!("expecting \"{}\".", self.expected)
                }
            ));
            None
        }
    }
}

pub fn literal(s: &str) -> Literal {
    Literal::new(s.to_owned())
}

#[cfg(test)]
mod test_char {
    use crate::core::parser::{ Parser, ParseState };
    use super::char;

    // Should parse when character matches
    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            Some('H'),
            char('H').parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    // Should return none when character does not match
    #[test]
    fn fail() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            None,
            char('h').parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}

#[cfg(test)]
mod test_satisfy {
    use crate::core::parser::{ Parser, ParseState };
    use super::satisfy;

    // Should parse when character satisifies given condition
    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            Some('H'),
            satisfy(|&ch| ch.is_uppercase()).parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    // Should return none when character does not satisfy given condition
    #[test]
    fn fail() {
        let mut st = ParseState::new("hello");
        assert_eq!(
            None,
            satisfy(|&ch| ch.is_uppercase()).parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}

#[cfg(test)]
mod test_literal {
    use crate::core::parser::{ Parser, ParseState };
    use super::literal;

    // Should parse when literal matches
    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello!");
        assert_eq!(
            Some("Hello"),
            literal("Hello").parse(&mut st)
        );
        assert_eq!("!", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    // Should return none when literal does not match
    #[test]
    fn fail() {
        let mut st = ParseState::new("Hell");
        assert_eq!(
            None,
            literal("Hello").parse(&mut st)
        );
        assert_eq!("Hell", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
