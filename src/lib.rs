type ParseResult<'a, T> = Vec<(&'a str, T)>;

pub trait Parser<'a, R> {
    fn parse(&self, inp: &'a str) -> ParseResult<'a, R>;
}

// Implement Parser trait for any function 
// that matches the signature of a parser
impl<'a, F, R> Parser<'a, R> for F
    where F: Fn(&'a str) -> ParseResult<R>,
{
    fn parse(&self, inp: &'a str) -> ParseResult<'a, R> {
        self(inp)
    }
}

// Parser builders
// A function that produces parser for a static string for any length
fn match_literal<'a>(expected: &'a str) -> impl Parser<'a, ()>
{
    move |inp: &'a str| match inp.strip_prefix(expected) {
        Some(res) => vec![(res, ())],
        None      => vec![]
    }
}

// Pair Combinators
fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
    where 
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
        R1: Clone, R2: Clone
{
    move |inp| p1.parse(inp)
        .into_iter()
        .flat_map(
            |(inp1, r1)| p2.parse(inp1)
                .into_iter()
                .map(move |(inp2, r2)| (inp2, (r1.clone(), r2.clone())))
        ).collect::<ParseResult<'a, (R1, R2)>>()
}

// Map combinator
fn map<'a, P, F, A, B>(parser: P, func: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B
{
    move |inp| parser.parse(inp)
        .into_iter()
        .map(|(inp1, r1)| (inp1, func(r1)))
        .collect::<ParseResult<'a, B>>()
}

#[test]
fn literal_parser() {
    let hw_parser = match_literal("Hello World");
    assert_eq!(
        vec![("", ())],
        hw_parser.parse("Hello World")
    );
    assert_eq!(
        vec![("!", ())],
        hw_parser.parse("Hello World!")
    );
    assert_eq!(
        vec![] as ParseResult<()>,
        hw_parser.parse("Hello Worl")
    );
}

#[test]
fn pair_combinator() {
    let hw_parser = pair(match_literal("Hello "), match_literal("World"));
    assert_eq!(
        vec![("", ((), ()))],
        hw_parser.parse("Hello World")
    )
}
