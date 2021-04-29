use std::rc::Rc;
use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;

pub struct FixP<'a, S, T> {
    fix: Rc<dyn for<'b> Fn(&'b Self)
        -> Box<dyn Parsable<S, T> + 'b> + 'a>,
}

impl<'a, S, T> FixP<'a, S, T>
{
    pub fn new<F>(fix: F) -> FixP<'a, S, T>
        where F: for<'b> Fn(&'b Self)
            -> Box<dyn Parsable<S, T> + 'b> + 'a,
    {
        Self { fix: Rc::new(fix) }
    }
}

impl<'a, S, T> Parsable<S, T> for FixP<'a, S, T> {
    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<T> {
        // Fixed-point Combinator: fix f = f (fix f)
        (self.fix)(self).parse(stream, logger)
    }
}

pub fn fix<'a, S, T, F>(fix: F) -> FixP<'a, S, T>
    where
        F: for<'b> Fn(&'b FixP<'a, S, T>)
            -> Box<dyn Parsable<S, T> + 'b> + 'a,
{
    FixP::new(fix)
}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::core::stream::*;
    use crate::core::logger::ParseLogger;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn recursive_syntax() {
        let mut st = CharStream::new("1110");
        let mut log = ParseLogger::default();
        assert_eq!(
            Some('0'),
            fix(|it| Box::new(
                char('1')
                .and(it)
                .map(|(_, x)| x)
                .or(char('0'))
            )).parse(&mut st, &mut log)
        );
    }
}
