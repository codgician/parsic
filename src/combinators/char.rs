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

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };

    // Should parse when character matches
    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            Some('H'),
            super::char('H').parse(&mut st)
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
            super::char('h').parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
