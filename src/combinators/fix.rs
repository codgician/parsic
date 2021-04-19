use crate::core::parser::{ Parser, ParseState };

use std::rc::Rc;

pub struct Fix<'a, T> {
    fix: Rc<dyn Fn(&Self) 
        -> Box<dyn Parser<ParseState<'a>, ParsedType = T>> + 'a>,
}

impl<'a, T> Fix<'a, T>
{
    pub fn new<F>(fix: F) -> Fix<'a, T> 
        where F: Fn(&Fix<'a, T>) 
        -> Box<dyn Parser<ParseState<'a>, ParsedType = T>> + 'a,
    {
        Self { fix: Rc::new(fix) }
    }
}

impl<'a, T> Parser<ParseState<'a>> for Fix<'a, T> {
    type ParsedType = T;

    fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::ParsedType> {
        (self.fix)(self).parse(state)
    }
}

pub fn fix<'a, T, F>(fix: F) -> Fix<'a, T>
    where
        F: Fn(&Fix<'a, T>) 
            -> Box<dyn Parser<ParseState<'a>, ParsedType = T>> + 'a,
{
    Fix::new(fix)
}

/*
#[cfg(test)]
mod test {
    use crate::core::parser::Parser;
    use crate::combinators::char::char;

    // (Fix f).parse() = (f (Fix f)).parse()
    #[test]
    fn property_holds() {        
    }
}
*/