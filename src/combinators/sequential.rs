use crate::combinators::MapExt;
use crate::core::{Parsable, Parser};

/// ## Combinator: `and` (function ver.)
pub fn and<'f, A: 'f, B: 'f, S: Clone>(
    p1: impl Parsable<Stream = S, Result = A> + 'f,
    p2: impl Parsable<Stream = S, Result = B> + 'f,
) -> Parser<'f, (A, B), S> {
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        match p1.parse(stream, logger) {
            Some(x) => match p2.parse(stream, logger) {
                None => {
                    *stream = st;
                    None
                }
                Some(y) => Some((x, y)),
            },
            None => {
                *stream = st;
                None
            }
        }
    })
}

/// ## Combinator: `left` (function ver.)
pub fn left<'f, A: 'f, B: 'f, S: Clone + 'f>(
    p1: impl Parsable<Stream = S, Result = A> + 'f,
    p2: impl Parsable<Stream = S, Result = B> + 'f,
) -> Parser<'f, A, S> {
    p1.and(p2).map(|(l, _)| l)
}

/// ## Combinator: `right` (function ver.)
pub fn right<'f, A: 'f, B: 'f, S: Clone + 'f>(
    p1: impl Parsable<Stream = S, Result = A> + 'f,
    p2: impl Parsable<Stream = S, Result = B> + 'f,
) -> Parser<'f, B, S> {
    p1.and(p2).map(|(_, r)| r)
}

/// ## Combinator: `mid` (function ver.)
pub fn mid<'f, A: 'f, B: 'f, C: 'f, S: Clone + 'f>(
    p1: impl Parsable<Stream = S, Result = A> + 'f,
    p2: impl Parsable<Stream = S, Result = B> + 'f,
    p3: impl Parsable<Stream = S, Result = C> + 'f,
) -> Parser<'f, B, S> {
    p1.and(p2).and(p3).map(|((_, m), _)| m)
}

/// Implements following methods for `Parsable<S>`:
/// - `and`
/// - `left`
/// - `right`
/// - `mid`
pub trait SequentialExt<'f, A: 'f, S>:
    Parsable<Stream = S, Result = A>
{
    /// ## Combinator: `and`
    fn and<B: 'f>(
        self,
        p: impl Parsable<Stream = S, Result = B> + 'f,
    ) -> Parser<'f, (A, B), S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        and(self, p)
    }

    /// ## Combinator: `left`
    fn left<B: 'f>(
        self,
        p: impl Parsable<Stream = S, Result = B> + 'f,
    ) -> Parser<'f, A, S>
    where
        Self: Sized + 'f,
        S: Clone + 'f,
    {
        left(self, p)
    }

    /// ## Combinator: `right`
    fn right<B: 'f>(
        self,
        p: impl Parsable<Stream = S, Result = B> + 'f,
    ) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        S: Clone + 'f,
    {
        right(self, p)
    }

    /// ## Combinator: `mid`
    fn mid<B: 'f, C: 'f>(
        self,
        p1: impl Parsable<Stream = S, Result = B> + 'f,
        p2: impl Parsable<Stream = S, Result = C> + 'f,
    ) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        S: Clone + 'f,
    {
        mid(self, p1, p2)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> SequentialExt<'f, A, S>
    for P
{
}

#[cfg(test)]
mod test_and {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, CharStream};

    #[test]
    fn same_type_ok() {
        let parser = char('A').and(char('B'));

        let mut st = CharStream::new("ABC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(('A', 'B')), res);
        assert_eq!("C", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn different_type_ok() {
        let parser = satisfy(|&ch| ch.is_digit(10))
            .map_option(|ch| ch.to_digit(10))
            .and(char('A'));

        let mut st = CharStream::new("1A+");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some((1, 'A')), res);
        assert_eq!("+", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn left_fail() {
        let parser = char('A').and(char('B'));

        let mut st = CharStream::new("BBC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("BBC", st.as_str());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn right_fail() {
        let parser = char('A').and(char('B'));

        let mut st = CharStream::new("ACC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("ACC", st.as_str());
        assert_eq!(1, logs.len());
    }

    #[test]
    fn both_fail() {
        let parser = char('A').and(char('B'));

        let mut st = CharStream::new("CCC");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("CCC", st.as_str());
        assert_eq!(1, logs.len());
    }
}
