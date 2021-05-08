use crate::core::{Parsable, Parser};

/// ## Combinator: `many` (function ver.)
pub fn many<'f, A: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Vec<A>, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (mut st, mut lg) = (stream.clone(), logger.clone());
        let mut res = vec![];
        while let Some(x) = p.parse(stream, logger) {
            res.push(x);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        *logger = lg;
        Some(res)
    })
}

/// ## Combinator: `some` (function ver.)
pub fn some<'f, A: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Vec<A>, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (mut st, mut lg) = (stream.clone(), logger.clone());
        let mut res = vec![];
        while let Some(x) = p.parse(stream, logger) {
            res.push(x);
            st = stream.clone();
            lg = logger.clone();
        }

        *stream = st;
        match res {
            v if v.is_empty() => None,
            _ => {
                *logger = lg;
                Some(res)
            }
        }
    })
}

/// ## Combinator: `optional` (function ver.)
pub fn optional<'f, A: 'f, S: Clone>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
) -> Parser<'f, Option<A>, S> {
    Parser::new(move |stream: &mut S, logger| {
        let (st, lg) = (stream.clone(), logger.clone());
        match p.parse(stream, logger) {
            None => {
                *stream = st;
                *logger = lg;
                Some(None)
            }
            x => Some(x),
        }
    })
}

/// Implements following method for `Parsable<S>`:
/// - `many`
/// - `some`
/// - `optional`
pub trait ReplicativeExt<'f, A: 'f, S>:
    Parsable<Stream = S, Result = A>
{
    /// ## Combinator: `many`
    fn many(self) -> Parser<'f, Vec<A>, S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        many(self)
    }

    /// ## Combinator: `some`
    fn some(self) -> Parser<'f, Vec<A>, S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        some(self)
    }

    /// ## Combinator: `optional`
    fn optional(self) -> Parser<'f, Option<A>, S>
    where
        Self: Sized + 'f,
        S: Clone,
    {
        optional(self)
    }
}

impl<'f, A: 'f, S, P: Parsable<Stream = S, Result = A>> ReplicativeExt<'f, A, S>
    for P
{
}

#[cfg(test)]
mod test_many {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, CharStream};

    #[test]
    fn ok_nonempty() {
        let parser = char('y').many();

        let mut st = CharStream::new("yyyyying");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec!['y', 'y', 'y', 'y', 'y']), res);
        assert_eq!("ing", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_empty() {
        let parser = char('y').many();

        let mut st = CharStream::new("ing");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec![]), res);
        assert_eq!("ing", st.as_str());
        assert_eq!(0, logs.len());
    }
}

#[cfg(test)]
mod test_some {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::{char, CharStream};

    #[test]
    fn ok() {
        let parser = char('y').some();

        let mut st = CharStream::new("yyyyycpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(vec!['y', 'y', 'y', 'y', 'y']), res);
        assert_eq!("cpnb", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn fail() {
        let parser = char('y').some();

        let mut st = CharStream::new("cpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("cpnb", st.as_str());
        assert_eq!(1, logs.len());
    }
}

#[cfg(test)]
mod test_optional {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::{char, CharStream};

    #[test]
    fn ok_one() {
        let parser = char('y').optional();

        let mut st = CharStream::new("yyyyycpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(Some('y')), res);
        assert_eq!(0, logs.len());
    }

    #[test]
    fn ok_zero() {
        let parser = char('y').optional();

        let mut st = CharStream::new("cpnb");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(None), res);
        assert_eq!(0, logs.len());
    }
}
