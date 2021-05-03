use crate::core::{Parsable, ParseLogger};
use std::marker::PhantomData;

// Bind
#[derive(Clone, Copy, Debug)]
pub struct BindP<F, P, T>(P, F, PhantomData<T>);

impl<F, P, T> BindP<F, P, T> {
    pub fn new(parser: P, func: F) -> Self {
        Self(parser, func, PhantomData)
    }
}

impl<F, P1, P2, S> Parsable<S> for BindP<F, P1, P2>
where
    F: Fn(P1::Result) -> P2,
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    type Result = P2::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        match self.0.parse(state, logger) {
            Some(r1) => self.1(r1).parse(state, logger),
            _ => None,
        }
    }
}

/// ## Combinator: `bind` (function ver.)
/// Monadic bind operator for context sensitive parsing.
/// See the iterator-style variant `bind` in `BindPExt` trait
/// for detailed introductions.
///
/// ### Example
///
/// The code example below parses `expr` with the following grammar:
///
/// ```plain
///  <expr> := <uppercase_letter> '+'
///  <expr> := <lowercase_letter> '-'
/// ```
///
/// ```
///  use naive_parsec::core::*;
///  use naive_parsec::combinators::*;
///  use naive_parsec::primitives::*;
/// 
///  let parser = bind(
///                 satisfy(|_| true),
///                 |ch| if ch.is_uppercase() {
///                     char('+')
///                 } else {
///                     char('-')
///                 }
///              );
///   let (res1, _) = parser.exec(&mut StrState::new("A+"));
///   assert_eq!(Some('+'), res1);
///   let (res2, _) = parser.exec(&mut StrState::new("a-"));
///   assert_eq!(Some('-'), res2);
/// ```
pub fn bind<F, P, S, T>(parser: P, func: F) -> BindP<F, P, T>
where
    F: Fn(P::Result) -> T,
    P: Parsable<S>,
{
    BindP::new(parser, func)
}

// Iterator-style methods for Parsable<S>
pub trait BindPExt<S>: Parsable<S> {
    /// ## Combinator: `bind`
    ///
    /// Monadic bind operator for context sensitive parsing.
    ///
    /// For parser `p: impl Parsable<S, Result = T1>` and
    /// function `f: impl Fn(T1) -> impl Parsable<S, Result = T2>`,
    /// `p.bind(f)` fails if the application of parser `p` to the
    /// input fails, and otherwise applies the function `f` to the
    /// result value of the parser (with type `T1`, denoted as `r1`)
    /// to give another parser `f(r1)` and then applied to the remaining
    /// part of the input to give the final result.
    ///
    /// ### Example
    ///
    /// The code example below parses `expr` with the following grammar:
    ///
    /// ```plain
    ///  <expr> := <uppercase_letter> '+'
    ///  <expr> := <lowercase_letter> '-'
    /// ```
    ///
    /// ```
    ///  use naive_parsec::core::*;
    ///  use naive_parsec::combinators::*;
    ///  use naive_parsec::primitives::*;
    /// 
    ///  let parser = satisfy(|_| true)
    ///             .bind(|ch| if ch.is_uppercase() {
    ///                 char('+')
    ///             } else {
    ///                 char('-')
    ///             });
    ///
    ///  let (res1, _) = parser.exec(&mut StrState::new("A+"));
    ///  assert_eq!(Some('+'), res1);
    ///  let (res2, _) = parser.exec(&mut StrState::new("a-"));
    ///  assert_eq!(Some('-'), res2);
    /// ```
    ///
    /// ### Properties
    ///
    /// - **Left-identity**: `pure(x).bind(f) ~ f(x)`
    /// - **Right-identity**: `pure(x).bind(f) ~ f(x)`
    /// - **Associativity**: `p.bind(f).bind(g) ~ p.bind(|x| f(x).bind(g))`
    ///
    /// Check out `test_bind` module in `bind.rs` for code examples.
    fn bind<F, T>(self, func: F) -> BindP<F, Self, T>
    where
        Self: Sized,
        F: Fn(Self::Result) -> T,
    {
        BindP::new(self, func)
    }
}

impl<S, P: Parsable<S>> BindPExt<S> for P {}

#[cfg(test)]
mod test_bind {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::*;

    #[test]
    fn ok() {
        let parser = satisfy(|_| true).bind(|ch| {
            if ch.is_uppercase() {
                char('+')
            } else {
                char('-')
            }
        });

        let mut st1 = StrState::new("A+");
        let mut st2 = StrState::new("a-");

        let (res1, logs1) = parser.exec(&mut st1);
        let (res2, logs2) = parser.exec(&mut st2);

        assert_eq!((Some('+'), Some('-')), (res1, res2));
        assert_eq!(("", ""), (st1.as_stream(), st2.as_stream()));
        assert_eq!((0, 0), (logs1.len(), logs2.len()));
    }

    #[test]
    fn left_identity() {
        // pure(x).bind(f) ~ f(x)
        let f = |b| if b { char('1') } else { char('0') };
        let parser1 = pure::<StrState, bool>(true).bind(f);
        let parser2 = f(true);

        assert_eq!(
            parser1.exec(&mut StrState::new("0")),
            parser2.exec(&mut StrState::new("0"))
        );
        assert_eq!(
            parser1.exec(&mut StrState::new("1")),
            parser2.exec(&mut StrState::new("1"))
        );
    }

    #[test]
    fn right_identity() {
        // p.bind(|x| pure(x)) ~ p
        let parser1 = char('0').bind(|x| pure(x));
        let parser2 = char('0');

        assert_eq!(
            parser1.exec(&mut StrState::new("0")),
            parser2.exec(&mut StrState::new("0"))
        );
        assert_eq!(
            parser1.exec(&mut StrState::new("1")),
            parser2.exec(&mut StrState::new("1"))
        );
    }

    #[test]
    fn associativity() {
        // p.bind(f).bind(g) ~ p.bind(|x| f(x).bind(g))
        let f = |ch: char| if ch == '0' { char('a') } else { char('b') };
        let g = |ch: char| if ch == 'a' { char('A') } else { char('B') };
        let parser1 = char('0').bind(g.clone()).bind(f.clone());
        let parser2 = char('0').bind(|x| f(x).bind(g));

        assert_eq!(
            parser1.exec(&mut StrState::new("0")),
            parser2.exec(&mut StrState::new("0"))
        );
        assert_eq!(
            parser1.exec(&mut StrState::new("1")),
            parser2.exec(&mut StrState::new("1"))
        );
    }
}
