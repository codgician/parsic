use crate::core::parser::{ Parser, ParseState };
use crate::core::logger::{ Msg, MsgBody };

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    expected: String
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
    Literal { expected: s.to_owned() }
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };

    // Should parse when literal matches
    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello!");
        assert_eq!(
            Some("Hello"),
            super::literal("Hello").parse(&mut st)
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
            super::literal("Hello").parse(&mut st)
        );
        assert_eq!("Hell", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
