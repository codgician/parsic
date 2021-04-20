use crate::core::parser::Parser;

#[derive(Clone, Copy, Debug)]
pub struct Or<PA, PB> {
    pa: PA,
    pb: PB
}

impl<PA, PB> Or<PA, PB> {
    pub fn new(pa: PA, pb: PB) -> Or<PA, PB> {
        Self { pa: pa, pb: pb }
    }
}

impl<S: Clone, PA, PB> Parser<S> for Or<PA, PB>
    where
        PA: Parser<S>,
        PB: Parser<S, ParsedType = PA::ParsedType>
{
    type ParsedType = PA::ParsedType;

    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
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
    Or::new(pa, pb)
}

pub trait OrExt<S> : Parser<S> {
    /// Or Combinator
    fn or<PB>(self, pb: PB) -> Or<Self, PB>
        where
            Self: Sized,
            PB: Parser<S, ParsedType = Self::ParsedType>
    {
        Or::new(self, pb)
    }
}

impl<S, P: Parser<S>> OrExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn left_ok() {
        let mut st = ParseState::new("Ahhh");
        assert_eq!(
            Some('A'),
            char('A').or(char('B')).parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn right_ok() {
        let mut st = ParseState::new("Ahhh");
        assert_eq!(
            Some('A'),
            char('B').or(char('A')).parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn both_ok() {
        let mut st = ParseState::new("Ahhh");
        assert_eq!(
            Some('A'),
            char('A').or(char('A')).parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn both_fail() {
        let mut st = ParseState::new("Ahhh");
        assert_eq!(
            None,
            char('B').or(char('C')).parse(&mut st)
        );
        assert_eq!("hhh", st.inp.as_str());
        assert_eq!(1, st.log.len());
    }
}
