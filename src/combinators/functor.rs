use crate::core::parser::{ Parser, ParseState };

#[derive(Clone, Copy, Debug)]
pub struct Map<F, P> {
    func: F,
    parser: P
}

impl<F, P> Map<F, P> {
    pub fn new(func: F, parser: P) -> Map<F, P> {
        Self { func: func, parser: parser }
    }
}

impl<'a, B, F, P> Parser<ParseState<'a>> for Map<F, P> 
    where 
        F: Fn(P::ParsedType) -> B,
        P: Parser<ParseState<'a>>
{
    type ParsedType = B;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        self.parser.parse(state).map(&self.func)
    }
}

pub fn map<'a, B, F, P>(func: F, parser: P) -> Map<F, P>
    where
        F: Fn(P::ParsedType) -> B,
        P: Parser<ParseState<'a>>
{
    Map::new(func, parser)
}

pub trait FunctorExt<S> : Parser<S> {
     /// Map Combinator
     fn map<B, F>(self, func: F) -> Map<F, Self>
        where
            Self: Sized,
            F: Fn(Self::ParsedType) -> B,
    {
        Map::new(func, self)
    }
}

impl<S, P: Parser<S>> FunctorExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        let parser = char('H')
                    .or(char('W'))
                    .map(|ch: char| ch == 'H');
        assert_eq!(
            Some(true),
            parser.parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }

    #[test]
    fn select_ok() {
        let mut st = ParseState::new("-1");
        let parser = char('-')
                    .and(char('1'))
                    .map(|(_, x)| x);
        assert_eq!(
            Some('1'),
            parser.parse(&mut st)
        );
        assert_eq!("", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }
}
