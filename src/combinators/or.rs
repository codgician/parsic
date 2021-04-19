use crate::core::parser::{ Parser, ParseState };

#[derive(Clone, Copy, Debug)]
pub struct Or<PA, PB> {
    pa: PA,
    pb: PB
}

impl<'a, PA, PB> Parser<ParseState<'a>> for Or<PA, PB>
    where
        PA: Parser<ParseState<'a>>,
        PB: Parser<ParseState<'a>, ParsedType = PA::ParsedType>
{
    type ParsedType = PA::ParsedType;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        let st0 = state.clone();
        match self.pa.parse(state) {
            None => {
                *state = st0;
                self.pb.parse(state)
            },
            x => x
        }
    }
}

pub fn or<PA, PB>(pa: PA, pb: PB) -> Or<PA, PB> {
    Or { pa: pa, pb: pb }
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::char::char;

    #[test]
    fn left_ok() {
        let mut st = ParseState::new("Ahhh");
        let parser = super::or(char('A'), char('B'));
        assert_eq!(
            Some('A'),
            parser.parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn right_ok() {
        let mut st = ParseState::new("Ahhh");
        let parser = super::or(char('B'), char('A'));
        assert_eq!(
            Some('A'),
            parser.parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn both_ok() {
        let mut st = ParseState::new("Ahhh");
        let parser = super::or(char('A'), char('A'));
        assert_eq!(
            Some('A'),
            parser.parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn both_fail() {
        let mut st = ParseState::new("Ahhh");
        let parser = super::or(char('B'), char('C'));
        assert_eq!(
            None,
            parser.parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
