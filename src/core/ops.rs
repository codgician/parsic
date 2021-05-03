use std::marker::PhantomData;
use std::ops::*;
use crate::core::*;
use crate::combinators::*;

// Wrapper for parsers
#[derive(Copy, Clone, Debug)]
pub struct Parser<S, P>(P, PhantomData<fn(&mut S)>);

impl<S, P> Parser<S, P> {
    pub fn new(parser: P) -> Self {
        Self(parser, PhantomData)
    }

    pub fn unwrap(self) -> P {
        self.0
    }
}

impl<S, P: Parsable<S>> Parsable<S> for Parser<S, P> {
    type Result = P::Result;

    fn parse(&self, state: &mut S, logger: &mut ParseLogger) -> Option<Self::Result> {
        self.0.parse(state, logger)
    }
}

// Overload Shl `<<` to `left` combinator
// `p1 << p2` ~ `p1.left(p2)`
impl<S, P1, P2> Shl<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S>
{
    type Output = Parser<S, 
        MapP<fn((P1::Result, P2::Result)) -> P1::Result, AndP<P1, P2>>>;

    fn shl(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.left(rhs))
    }
}

// Overload Shr `>>` to `right` combinator
// `p1 >> p2` ~ `p1.right(p2)`
impl<S, P1, P2> Shr<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S>
{
    type Output = Parser<S,
        MapP<fn((P1::Result, P2::Result)) -> P2::Result, AndP<P1, P2>>>;

    fn shr(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.right(rhs))
    }
}

// Overload operator `/` to `or` combinator
// `p1 / p2` ~ `p1.or(p2)`
impl<S, P1, P2> Div<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S, Result = P1::Result>
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
    F: Fn(P::Result) -> T
{
    type Output = Parser<S, MapP<F, P>>;

    fn bitor(self, rhs: F) -> Self::Output {
        Parser::new(self.0.map(rhs))
    }
}

// Overload operator `&` to `and` combinator
// `p1 & p2` ~ `p1.and(p2)`
impl<S, P1, P2> BitAnd<P2> for Parser<S, P1>
where
    P1: Parsable<S>,
    P2: Parsable<S>
{
    type Output = Parser<S, AndP<P1, P2>>;

    fn bitand(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.and(rhs))
    }
}

// Overload operator `*` to `compose` combinator
// `p1 * p2` ~ `p1.compose(p2)`
impl<F, S, T, P1, P2> Mul<P2> for Parser<S, P1>
where
    F: Fn(P2::Result) -> T,
    P1: Parsable<S, Result = F>,
    P2: Parsable<S>
{
    type Output = Parser<S, ComposeP<P1, P2, T>>;

    fn mul(self, rhs: P2) -> Self::Output {
        Parser::new(self.0.compose(rhs))
    }
}
