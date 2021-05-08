use crate::core::{Parsable, Parser};

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
///  use naive_parsec::core::Parsable;
///  use naive_parsec::combinators::bind;
///  use naive_parsec::primitives::{ CharStream, char, satisfy };
///
///  let parser = bind(
///                 satisfy(|_| true),
///                 |ch| if ch.is_uppercase() {
///                     char('+')
///                 } else {
///                     char('-')
///                 }
///              );
///   let (res1, _) = parser.exec(&mut CharStream::new("A+"));
///   assert_eq!(Some('+'), res1);
///   let (res2, _) = parser.exec(&mut CharStream::new("a-"));
///   assert_eq!(Some('-'), res2);
/// ```
pub fn bind<'f, A: 'f, B: 'f, S, P>(
    p: impl Parsable<Stream = S, Result = A> + 'f,
    f: impl Fn(A) -> P + 'f,
) -> Parser<'f, B, S>
where
    P: Parsable<Stream = S, Result = B>,
    S: Clone,
{
    Parser::new(move |stream: &mut S, logger| {
        let st = stream.clone();
        match p.parse(stream, logger) {
            Some(x) => f(x).parse(stream, logger),
            _ => {
                *stream = st;
                None
            }
        }
    })
}

/// Implements `bind` method for `Parsable<S>`.
pub trait BindExt<'f, A: 'f, S>: Parsable<Stream = S, Result = A> {
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
    ///  use naive_parsec::core::Parsable;
    ///  use naive_parsec::combinators::BindExt;
    ///  use naive_parsec::primitives::{ CharStream, char, satisfy };
    ///
    ///  let parser = satisfy(|_| true)
    ///             .bind(|ch| if ch.is_uppercase() {
    ///                 char('+')
    ///             } else {
    ///                 char('-')
    ///             });
    ///
    ///  let (res1, _) = parser.exec(&mut CharStream::new("A+"));
    ///  assert_eq!(Some('+'), res1);
    ///  let (res2, _) = parser.exec(&mut CharStream::new("a-"));
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
    fn bind<B: 'f, P>(self, f: impl Fn(A) -> P + 'f) -> Parser<'f, B, S>
    where
        Self: Sized + 'f,
        P: Parsable<Stream = S, Result = B>,
        S: Clone,
    {
        bind(self, f)
    }
}

impl<'f, A: 'f, S, P> BindExt<'f, A, S> for P where P: Parsable<Stream = S, Result = A> {}

#[cfg(test)]
mod test_bind {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, CharStream};

    #[test]
    fn ok() {
        let parser = satisfy(|_| true).bind(|ch| {
            if ch.is_uppercase() {
                char('+')
            } else {
                char('-')
            }
        });

        let mut st1 = CharStream::new("A+");
        let mut st2 = CharStream::new("a-");

        let (res1, logs1) = parser.exec(&mut st1);
        let (res2, logs2) = parser.exec(&mut st2);

        assert_eq!((Some('+'), Some('-')), (res1, res2));
        assert_eq!(("", ""), (st1.as_str(), st2.as_str()));
        assert_eq!((0, 0), (logs1.len(), logs2.len()));
    }

    #[test]
    fn left_identity() {
        //! Left identity: `pure(x).bind(f) ~ f(x)`
        let f = |b| if b { char('1') } else { char('0') };
        let parser1 = pure::<bool, CharStream>(true).bind(f);
        let parser2 = f(true);

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("1")),
            parser2.exec(&mut CharStream::new("1"))
        );
    }

    #[test]
    fn right_identity() {
        //! Right identity: `p.bind(|x| pure(x)) ~ p`
        let parser1 = char('0').bind(|x| pure(x));
        let parser2 = char('0');

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("1")),
            parser2.exec(&mut CharStream::new("1"))
        );
    }

    #[test]
    fn associativity() {
        //! Right identity: `p.bind(f).bind(g) ~ p.bind(|x| f(x).bind(g))`
        let f = |ch: char| if ch == '0' { char('a') } else { char('b') };
        let g = |ch: char| if ch == 'a' { char('A') } else { char('B') };
        let parser1 = char('0').bind(g.clone()).bind(f.clone());
        let parser2 = char('0').bind(|x| f(x).bind(g));

        assert_eq!(
            parser1.exec(&mut CharStream::new("0")),
            parser2.exec(&mut CharStream::new("0"))
        );
        assert_eq!(
            parser1.exec(&mut CharStream::new("1")),
            parser2.exec(&mut CharStream::new("1"))
        );
    }
}
