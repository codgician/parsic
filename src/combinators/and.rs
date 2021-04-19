use crate::core::parser::{ Parser, ParseState };

#[derive(Clone, Copy, Debug)]
pub struct And<PA, PB> {
    pa: PA,
    pb: PB
}

impl<'a, PA:, PB> Parser<ParseState<'a>> for And<PA, PB>
    where
        PA: Parser<ParseState<'a>>, 
        PB: Parser<ParseState<'a>>
{
    type ParsedType = (PA::ParsedType, PB::ParsedType);

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        match self.pa.parse(state) {
            None => None,
            Some(r1) => {
                match self.pb.parse(state) {
                    None => None,
                    Some(r2) => Some((r1, r2))
                }
            }
        }
    }
}

pub fn and<PA, PB>(pa: PA, pb: PB) -> And<PA, PB> {
    And { pa: pa, pb: pb }
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::char::char;

    #[test]
    fn ok() {
        let mut st = ParseState::new("ABC");
        let parser = super::and(char('A'), char('B'));
        assert_eq!(
            Some(('A', 'B')),
            parser.parse(&mut st)
        );
        assert_eq!("C", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn left_fail() {
        let mut st = ParseState::new("BBC");
        let parser = super::and(char('A'), char('B'));
        assert_eq!(
            None,
            parser.parse(&mut st)
        );
        assert_eq!("BC", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }

    #[test]
    fn right_fail() {
        let mut st = ParseState::new("ACC");
        let parser = super::and(char('A'), char('B'));
        assert_eq!(
            None,
            parser.parse(&mut st)
        );
        assert_eq!("C", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }

    #[test]
    fn both_fail() {
        let mut st = ParseState::new("CCC");
        let parser = super::and(char('A'), char('B'));
        assert_eq!(
            None,
            parser.parse(&mut st)
        );
        assert_eq!("CC", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
