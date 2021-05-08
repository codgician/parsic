/// # Arithmetic expression evaluator
///
/// Supports `+`, `-`, `*`, `/` and `()`.
///
/// Production rules:
///
/// ```plain
/// expr    := term ('+'|'-') expr | term
/// term    := factor ('*'|'/') term | factor
/// factor  := '(' expr ')' | float
/// float   := uint {'.' uint}
/// uint    := digit { digit }
/// digit   := '0' | '1' | ... | '9'
/// ```
use naive_parsec::combinators::*;
use naive_parsec::core::{lazy, Parsable, Parser};
use naive_parsec::primitives::*;

fn digit<'f>() -> Parser<'f, char, CharStream<'f>> {
    satisfy(|&ch| ch.is_digit(10))
}

fn uint<'f>() -> Parser<'f, String, CharStream<'f>> {
    digit().some().map(|v| v.iter().collect::<String>())
}

fn float<'f>() -> Parser<'f, f64, CharStream<'f>> {
    uint()
        .and(char('.').and(lazy(uint)).optional())
        .map_result(|(s, r)| {
            let mut res = s;
            match r {
                Some((dot, frac)) => {
                    res.push(dot);
                    res.push_str(&frac[..])
                }
                _ => {}
            };
            res.parse::<f64>()
        })
        .trim()
}

fn factor<'f>() -> Parser<'f, f64, CharStream<'f>> {
    mid(char('(').trim(), lazy(expr), char(')').trim()).or(float())
}

fn term<'f>() -> Parser<'f, f64, CharStream<'f>> {
    factor()
        .and(char('*').or(char('/').trim()))
        .and(lazy(term))
        .map(|((v1, op), v2)| if op == '*' { v1 * v2 } else { v1 / v2 })
        .or(lazy(factor))
}

fn expr<'f>() -> Parser<'f, f64, CharStream<'f>> {
    term()
        .and(char('+').or(char('-')).trim())
        .and(lazy(expr))
        .map(|((v1, op), v2)| if op == '+' { v1 + v2 } else { v1 - v2 })
        .or(lazy(term))
}

#[test]
fn int_expr() {
    let mut st = CharStream::new("2+4*(6+0)/1");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some((2 + 4 * (6 + 0) / 1) as f64), res);
    assert_eq!("", st.as_str());
    assert_eq!(0, logs.len());
}

#[test]
fn int_expr_with_whitespace() {
    let mut st = CharStream::new(" 2 + 4 * ( 6 + 0 ) / 1 ");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some((2 + 4 * (6 + 0) / 1) as f64), res);
    assert_eq!("", st.as_str());
    assert_eq!(0, logs.len());
}

#[test]
fn float_expr() {
    let mut st = CharStream::new("1.9/(2.6+0.8)+1.7");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some(1.9 / (2.6 + 0.8) + 1.7), res);
    assert_eq!("", st.as_str());
    assert_eq!(0, logs.len());
}

#[test]
fn float_expr_with_whitespace() {
    let mut st = CharStream::new("1.9 / ( 2.6 + 0.8 ) + 1.7 ");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some(1.9 / (2.6 + 0.8) + 1.7), res);
    assert_eq!("", st.as_str());
    assert_eq!(0, logs.len());
}
