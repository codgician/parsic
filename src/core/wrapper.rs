use crate::combinators::*;
use crate::core::{Parsable, ParseLogger};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Div, Mul, Shl, Shr};

/// ## `Parser` struct: a wrapper for `Parsable`
///
/// Parsers are wrapped to overload arithmetic operators, which
/// might provide a more functional flavour to write code using
/// this library. Since currently Rust does not allow us to create
/// new operators, so only a small subset of combinators have their
/// according operator as listed below:
///
/// - `left` combinator: `p1 << p2 ~ p1.left(p2)`
/// - `right` combinator: `p1 >> p2 ~ p1.right(p2)`
/// - `or` combinator: `p1 / p2 ~ p1.or(p2)`
/// - `map` combinator: `p1 | f ~ p1.map(f)`
/// - `and` combinator: `p1 & p2 ~ p1.and(p2)`
/// - `compose` combinator: `p1 * p2 ~ p1.compose(p2)`
#[derive(Copy, Clone, Debug)]
pub struct Parser<S, P: Parsable<S>>(P, PhantomData<S>);

impl<S, P: Parsable<S>> Parser<S, P> {
    pub fn new(parser: P) -> Self {
        Self(parser, PhantomData)
    }

    pub fn unwrap(self) -> P {
        self.0
    }

    pub fn inspect(&self) -> &P {
        &self.0
    }
}

impl<S, P: Parsable<S>> Parsable<S> for Parser<S, P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        self.0.parse(state, logger)
    }
}

// Wrapper function for parsing
pub fn parse<S, P: Parsable<S>>(parser: &P, state: &mut S) -> (Option<P::Result>, ParseLogger) {
    let mut logger = ParseLogger::default();
    (parser.parse(state, &mut logger), logger)
}

// Wrapper for lazy parsing
#[derive(Copy, Clone, Debug)]
pub struct Lazy<F>(F);

impl<F> Lazy<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<S, P, F> Parsable<S> for Lazy<F>
where
    P: Parsable<S>,
    F: Fn() -> P,
{
    type Result = P::Result;

    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        (self.0)().parse(stream, logger)
    }
}

/// Parse function wrapper
#[derive(Copy, Clone, Debug)]
pub struct ParseFn<F>(pub F);

impl<S, T, F> Parsable<S> for ParseFn<F>
where
    F: for<'a> Fn(&'a mut S, &mut ParseLogger) -> Option<T>,
{
    type Result = T;

    fn parse(&self, stream: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        (self.0)(stream, logger)
    }
}

// Overload Shl `<<` to `left` combinator
// `p1 << p2` ~ `p1.left(p2)`
impl<S: Clone, P1, P2> Shl<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    type Output = Parser<S, MapP<fn((P1::Result, P2::Result)) -> P1::Result, AndP<P1, P2>>>;

    fn shl(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.left(rhs))
    }
}

// Overload Shr `>>` to `right` combinator
// `p1 >> p2` ~ `p1.right(p2)`
impl<S: Clone, P1, P2> Shr<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    type Output = Parser<S, MapP<fn((P1::Result, P2::Result)) -> P2::Result, AndP<P1, P2>>>;

    fn shr(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.right(rhs))
    }
}

// Overload operator `/` to `or` combinator
// `p1 / p2` ~ `p1.or(p2)`
impl<S: Clone, P1, P2> Div<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S, Result = P1::Result>,
{
    type Output = Parser<S, OrP<P1, P2>>;

    fn div(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.or(rhs))
    }
}

// Overload operator `|` to `map` combinator
// `p1 | f` ~ `p1.map(f)`
impl<F, S, T, P> BitOr<F> for Parser<S, P>
where
    P: Parsable<S>,
    F: Fn(P::Result) -> T,
{
    type Output = Parser<S, MapP<F, P>>;

    fn bitor(self, rhs: F) -> Self::Output {
        Parser::new(self.0.map(rhs))
    }
}

// Overload operator `&` to `and` combinator
// `p1 & p2` ~ `p1.and(p2)`
impl<S: Clone, P1, P2> BitAnd<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S>,
{
    type Output = Parser<S, AndP<P1, P2>>;

    fn bitand(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.and(rhs))
    }
}

// Overload operator `*` to `compose` combinator
// `p1 * p2` ~ `p1.compose(p2)`
impl<F, S: Clone, T, P1, P2> Mul<P2> for Parser<S, P1>
where
    F: Fn(P2::Result) -> T,
    P1: Parsable<S, Result = F>,
    P2: Parsable<S>,
{
    type Output = Parser<S, ComposeP<P1, P2, T>>;

    fn mul(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.compose(rhs))
    }
}
