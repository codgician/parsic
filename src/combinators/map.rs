use crate::core::parser::{ Parser, ParseState };

#[derive(Clone, Copy, Debug)]
pub struct Map<P, F> {
    func: F,
    parser: P
}

impl<'a, A, B, F, P> Parser<ParseState<'a>> for Map<P, F> 
    where 
        F: Fn(A) -> B,
        P: Parser<ParseState<'a>, ParsedType = A>
{
    type ParsedType = B;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        self.parser.parse(state).map(&self.func)
    }
}

pub fn map<'a, A, B, F, P>(func: F, parser: P) -> Map<P, F>
    where
        F: Fn(A) -> B,
        P: Parser<ParseState<'a>, ParsedType = A>
{
    Map { func: func, parser: parser }
}

#[cfg(test)]
mod test {
    use crate::core::parser::{ Parser, ParseState };
    use crate::combinators::char::char;
    use crate::combinators::or::or;

    #[test]
    fn ok() {
        let mut st = ParseState::new("Hello");
        let func = |ch: char| -> bool { ch == 'H' };
        let parser = super::map(func, or(char('H'), char('W')));
        assert_eq!(
            Some(true),
            parser.parse(&mut st)
        );
        assert_eq!("ello", st.inp.as_str());
        assert_eq!(0, st.log.len());
    }
}
