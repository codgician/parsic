use crate::core::parser::{ Parser, ParseState };
use crate::core::logger::{ Msg, MsgBody };

// Parser builder: Char
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Char {
    ch: char
}

impl<'a> Parser<ParseState<'a>> for Char {
    type Target = char;
    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
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
                        msg: format!("failed to consume '{}'.", self.ch)
                    }
                ));
                None
            }
        }
    }
}

pub fn char(ch: char) -> Char {
    Char { ch }
}

#[cfg(test)]
mod test_char {
    use crate::core::parser::{ Parser, ParseState };

    #[test]
    // Should parse when character satisifies
    fn test_char_ok() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            Some('H'),
            super::char('H').parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    // Should return none when character does not satisfy
    fn test_char_fail() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            None,
            super::char('h').parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
