use crate::core::parser::Parser;

use std::rc::Rc;

pub struct Fix<'a, S, T> {
    fix: Rc<dyn for<'b> Fn(&'b Self) 
        -> Box<dyn Parser<S, ParsedType = T> + 'b> + 'a>,
}

impl<'a, S, T> Fix<'a, S, T>
{
    pub fn new<F>(fix: F) -> Fix<'a, S, T> 
        where F: for<'b> Fn(&'b Self) 
        -> Box<dyn Parser<S, ParsedType = T> + 'b> + 'a,
    {
        Self { fix: Rc::new(fix) }
    }
}

impl<'a, S, T> Parser<S> for Fix<'a, S, T> {
    type ParsedType = T;

    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
        // Fixed-point Combinator: fix f = f (fix f)
        (self.fix)(self).parse(state)
    }
}

pub fn fix<'a, S, T, F>(fix: F) -> Fix<'a, S, T>
    where
        F: for<'b> Fn(&'b Fix<'a, S, T>) 
            -> Box<dyn Parser<S, ParsedType = T> + 'b> + 'a,
{
    Fix::new(fix)
}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn recursive_syntax() {
        let parser = fix(|it| Box::new(
            char('1')
            .and(it)
            .map(|(_, x)| x)
            .or(char('0'))
        ));
        let res = parser.parse(&mut ParseState::new("1110")).unwrap();
        assert_eq!(res, '0');
    }
}
