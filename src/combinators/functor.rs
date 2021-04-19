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

impl<'a, A, B, F, P> Parser<ParseState<'a>> for Map<F, P> 
    where 
        F: Fn(A) -> B,
        P: Parser<ParseState<'a>, ParsedType = A>
{
    type ParsedType = B;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        self.parser.parse(state).map(&self.func)
    }
}

pub fn map<'a, A, B, F, P>(func: F, parser: P) -> Map<F, P>
    where
        F: Fn(A) -> B,
        P: Parser<ParseState<'a>, ParsedType = A>
{
    Map::new(func, parser)
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::char::char;
    use crate::combinators::and::and;
    use crate::combinators::or::or;

    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        let parser = super::map(
            |ch: char| -> bool { ch == 'H' }, 
            or(char('H'), char('W'))
        );
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
        let parser = super::map(
            |(_, x)| x,
            and(char('-'), char('1'))
        );
        assert_eq!(
            Some('1'),
            parser.parse(&mut st)
        );
        assert_eq!("", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }
}
