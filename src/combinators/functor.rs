use crate::core::{ Parsable, ParseLogger };

#[derive(Clone, Copy, Debug)]
pub struct MapP<F, P>(F, P);

impl<F, P, S, T> Parsable<S> for MapP<F, P> 
    where 
        F: Fn(P::Result) -> T, 
        P: Parsable<S>
{
    type Result = T;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) 
        -> Option<Self::Result> 
    {
        self.1.parse(state, logger).map(&self.0)
    }
}

pub fn map<F, P, S, T>(func: F, parser: P) -> MapP<F, P>
    where 
        F: Fn(P::Result) -> T, 
        P: Parsable<S>
{
    MapP(func, parser)
}

pub trait FunctorExt<S> : Parsable<S> {
    /// MapP Combinator
    fn map<T, F>(self, func: F) -> MapP<F, Self>
        where 
            Self: Sized, 
            F: Fn(Self::Result) -> T,
    {
        MapP(func, self)
    }
}

impl<S, P: Parsable<S>> FunctorExt<S> for P {}

#[cfg(test)]
mod test {
    use crate::core::*;
    use crate::combinators::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let mut st = StrState::new("Hello");
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
        let mut st = StrState::new("-1");
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
