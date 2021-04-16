use crate::core::parser::{ Parser, ParseState };
use crate::core::logger::{ Msg, MsgBody };

// Satisfy parser
#[derive(Clone, Copy, Debug)]
pub struct Satisfy<F> {
    func: F,
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

pub fn satisfy<F>(f: F) -> Satisfy<F>
    where F: Fn(&char) -> bool
{
    Satisfy { func: f }
}

#[cfg(test)]
mod test_satisfy {
    use crate::core::parser::{ Parser, ParseState };

    #[test]
    // Should parse when character satisifies
    fn ok() {
        let mut st = ParseState::new("Hello");
        assert_eq!(
            Some('H'),
            super::satisfy(|&ch| ch.is_uppercase()).parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    // Should return none when character does not satisfy
    fn fail() {
        let mut st = ParseState::new("hello");
        assert_eq!(
            None,
            super::satisfy(|&ch| ch.is_uppercase()).parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
