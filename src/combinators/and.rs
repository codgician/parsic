use crate::core::parser::Parser;

#[derive(Clone, Copy, Debug)]
pub struct And<PA, PB> {
    pa: PA,
    pb: PB
}

impl<PA, PB> And<PA, PB> {
    pub fn new(pa: PA, pb: PB) -> Self {
        Self { pa: pa, pb: pb }
    }
}

impl<S, PA:, PB> Parser<S> for And<PA, PB>
    where
        PA: Parser<S>, 
        PB: Parser<S>
{
    type ParsedType = (PA::ParsedType, PB::ParsedType);

    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
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

/// And Combinator
pub fn and<PA, PB>(pa: PA, pb: PB) -> And<PA, PB> {
    And::new(pa, pb)
}

pub trait AndExt<S> : Parser<S> {
    /// And Combinator
    fn and<PB>(self, pb: PB) -> And<Self, PB>
        where
            Self: Sized,
            PB: Parser<S, ParsedType = Self::ParsedType>
    {
        And::new(self, pb)
    }
}

impl<S, P: Parser<S>> AndExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::*;

    #[test]
    fn ok() {
        let mut st = ParseState::new("ABC");        
        assert_eq!(
            Some(('A', 'B')),
            char('A').and(char('B')).parse(&mut st)
        );
        assert_eq!("C", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn left_fail() {
        let mut st = ParseState::new("BBC");
        assert_eq!(
            None,
            char('A').and(char('B')).parse(&mut st)
        );
        assert_eq!("BC", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }

    #[test]
    fn right_fail() {
        let mut st = ParseState::new("ACC");
        assert_eq!(
            None,
            char('A').and(char('B')).parse(&mut st)
        );
        assert_eq!("C", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }

    #[test]
    fn both_fail() {
        let mut st = ParseState::new("CCC");
        assert_eq!(
            None,
            char('A').and(char('B')).parse(&mut st)
        );
        assert_eq!("CC", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
