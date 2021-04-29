use crate::core::parser::Parsable;
use crate::core::logger::ParseLogger;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct Map<F, P, T1>(F, P, PhantomData<T1>);

impl<S, T1, T2, F, P> Parsable<S, T2> for Map<F, P, T1> 
    where F: Fn(T1) -> T2, P: Parsable<S, T1>
{
    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<T2> 
    {
        self.1.parse(state, logger).map(&self.0)
    }
}

pub fn map<S, T1, T2, F, P>(func: F, parser: P) -> Map<F, P, T1>
    where F: Fn(T1) -> T2, P: Parsable<S, T1>
{
    Map(func, parser, PhantomData)
}

pub trait FunctorExt<S, T1> : Parsable<S, T1> {
     /// Map Combinator
     fn map<T2, F>(self, func: F) -> Map<F, Self, T1>
        where Self: Sized, F: Fn(T1) -> T2,
    {
        Map(func, self, PhantomData)
    }
}

impl<S, T, P: Parsable<S, T>> FunctorExt<S, T> for P {}

#[cfg(test)]
mod test {
    use crate::core::parser::*;
    use crate::core::stream::*;
    use crate::core::logger::ParseLogger;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let mut st = CharStream::new("Hello");
        let mut log = ParseLogger::default();
        let parser = char('H')
                    .or(char('W'))
                    .map(|ch: char| ch == 'H');
        assert_eq!(
            Some(true),
            parser.parse(&mut st, &mut log)
        );
        assert_eq!("ello", st.as_stream());
        assert_eq!(0, log.len());
    }

    #[test]
    fn select_ok() {
        let mut st = CharStream::new("-1");
        let mut log = ParseLogger::default();
        let parser = char('-')
                    .and(char('1'))
                    .map(|(_, x)| x);
        assert_eq!(
            Some('1'),
            parser.parse(&mut st, &mut log)
        );
        assert_eq!("", st.as_stream());
        assert_eq!(0, log.len());
    }
}
