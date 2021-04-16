#![allow(dead_code)]

// Parsec library
pub type Func<A, B> = Box<dyn Fn(A) -> B>;
pub type ParseResult<'a, A> = Vec<(&'a str, A)>;

pub trait Parser<'a, A> {
    fn parse(&self, inp: &'a str) -> ParseResult<'a, A>;
}

// Implement Parser trait for any function 
// that matches the signature of a parser
impl<'a, F, A> Parser<'a, A> for F
    where F: Fn(&'a str) -> ParseResult<A>,
{
    fn parse(&self, inp: &'a str) -> ParseResult<'a, A> {
        self(inp)
    }
}

// Parser builders
// A function that produces parser for a static string for any length
fn match_literal<'a>(expected: &'a str) -> impl Parser<'a, &str> {
    move |inp: &'a str| match inp.strip_prefix(expected) {
        Some(res) => vec![(res, expected)],
        None      => vec![]
    }
}

fn inject<'a, A: Clone>(value: A) -> impl Parser<'a, A> {
    move |inp| vec![(inp, value.clone())]
}

// Parser Combinators
// Map combinator
fn pmap<'a, P, A, B>(func: Func<A, B>, parser: P) -> impl Parser<'a, B>
    where P: Parser<'a, A>
{
    move |inp| parser.parse(inp)
        .into_iter()
        .map(|(inp1, r1)| (inp1, func(r1)))
        .collect::<ParseResult<'a, B>>()
}

// Pair Combinators
fn pair<'a, P1, P2, A, B>(p1: P1, p2: P2) -> impl Parser<'a, B>
    where 
        P1: Parser<'a, Func<A, B>>,
        P2: Parser<'a, A>
{
    move |inp| p1.parse(inp)
        .into_iter()
        .flat_map(
            |(inp1, f)| p2.parse(inp1)
                .into_iter()
                .map(move |(inp2, a)| (inp2, f(a)))
        ).collect::<ParseResult<'a, B>>()
}

#[test]
fn literal_parser() {
    let hw_parser = match_literal("Hello World");
    assert_eq!(
        vec![("", "Hello World")],
        hw_parser.parse("Hello World")
    );
    assert_eq!(
        vec![("!", "Hello World")],
        hw_parser.parse("Hello World!")
    );
    assert_eq!(
        vec![] as ParseResult<&str>,
        hw_parser.parse("Hello Worl")
    );
}

#[test]
fn pmap_combinator() {
    
    fn curried_const<A: Clone, B>(a: A) -> impl Fn(B) -> A {
        move |_| a.clone()
    }

    let hw_parser = match_literal("Hello World");
    let mapped_parser = pmap(
        Box::new(curried_const(true)), 
        hw_parser
    );

    assert_eq!(
        vec![("", true)],
        mapped_parser.parse("Hello World")
    );
    assert_eq!(
        vec![("!", true)],
        mapped_parser.parse("Hello World!")
    );
    assert_eq!(
        vec![] as ParseResult<bool>,
        mapped_parser.parse("Hello Worl")
    );
}

#[test]
fn pair_combinator() {
    
}
