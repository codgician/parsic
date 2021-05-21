/// # Test: Calculator
///
/// A simple arithmetic expression evaluator that supports
/// `+`, `-`, `*`, `/` and `()`. It accepts input with
/// whitespaces.
///
/// This test contains two implementations: one is implemented
/// with functions that returns a `Parser`, the other is implemented
/// with closures.
///
/// ```plain
/// expr    := term [('+'|'-') expr]
/// term    := factor [('*'|'/') term]
/// factor  := '(' expr ')' | float
/// float   := uint ['.' uint]
/// uint    := digit { digit }
/// digit   := '0' | '1' | ... | '9'
/// ```
use parsic::combinators::*;
use parsic::core::{Parsable, Parser};
use parsic::primitives::*;

/// digit := '0' | '1' | ... | '9'
fn digit<'f>() -> Parser<'f, char, CharStream<'f>> {
    satisfy(|&ch| ch.is_digit(10))
}

/// uint := digit { digit }
fn uint<'f>() -> Parser<'f, String, CharStream<'f>> {
    digit().some().map(|v| v.iter().collect::<String>())
}

/// float := uint ['.' uint]
fn float<'f>() -> Parser<'f, f64, CharStream<'f>> {
    uint()
        .and(char('.').and(lazy(uint)).optional())
        .map_result(|(s, r)| {
            let mut res = s;
            if let Some((dot, frac)) = r {
                res.push(dot);
                res.push_str(&frac[..])
            }
            res.parse::<f64>()
        })
        .trim()
}

/// factor := '(' expr ')' | float
fn factor<'f>() -> Parser<'f, f64, CharStream<'f>> {
    mid(char('(').trim(), lazy(expr), char(')').trim()).or(float())
}

/// term := factor [('*'|'/') term]
fn term<'f>() -> Parser<'f, f64, CharStream<'f>> {
    factor()
        .and(char('*').or(char('/').trim()).and(lazy(term)).optional())
        .map(|(v1, r)| match r {
            Some(('*', v2)) => v1 * v2,
            Some(('/', v2)) => v1 / v2,
            _ => v1,
        })
}

/// expr := term [('+'|'-') expr]
fn expr<'f>() -> Parser<'f, f64, CharStream<'f>> {
    term()
        .and(char('+').or(char('-')).trim().and(lazy(expr)).optional())
        .map(|(v1, r)| match r {
            Some(('+', v2)) => v1 + v2,
            Some(('-', v2)) => v1 - v2,
            _ => v1,
        })
}

/// Another equivlent implementation using closures
fn expr_<'s>() -> impl Parsable<Stream = CharStream<'s>, Result = f64> {
    fix(|expr| {
        // digit := '0' | '1' | ... | '9'
        let digit = satisfy(|&ch| ch.is_digit(10));
        // uint := digit { digit }
        let uint = digit.some().map(|v| v.iter().collect::<String>());
        // float := uint ['.' uint]
        let float = uint
            .clone()
            .and(char('.').and(uint).optional())
            .map_result(|(s, r)| {
                let mut res = s;
                if let Some((dot, frac)) = r {
                    res.push(dot);
                    res.push_str(&frac[..])
                }
                res.parse::<f64>()
            })
            .trim();
        // factor := '(' expr ')' | float
        let factor = mid(char('(').trim(), expr.clone(), char(')').trim()).or(float);
        // term := factor [('*'|'/') term]
        let term = fix(move |term| {
            factor
                .clone()
                .and(char('*').or(char('/').trim()).and(term).optional())
                .map(|(v1, r)| match r {
                    Some(('*', v2)) => v1 * v2,
                    Some(('/', v2)) => v1 / v2,
                    _ => v1,
                })
        });
        // expr := term [('+'|'-') expr]
        term.and(char('+').or(char('-')).trim().and(expr).optional())
            .map(|(v1, r)| match r {
                Some(('+', v2)) => v1 + v2,
                Some(('-', v2)) => v1 - v2,
                _ => v1,
            })
    })
}

/// Helper function for testing
fn test_helper(input: &str, expected: Option<f64>, rem_str: &str, log_size: usize) {
    let mut st1 = CharStream::new(input);
    let mut st2 = st1.clone();

    let (res1, logs1) = expr().exec(&mut st1);
    let (res2, logs2) = expr_().exec(&mut st2);

    // `expr` and `expr_` should be equivalent
    assert_eq!(res1, res2);
    assert_eq!(st1.as_str(), st2.as_str());
    assert_eq!(logs1, logs2);

    // Test if matches expected result
    assert_eq!(expected, res1);
    assert_eq!(rem_str, st1.as_str());
    assert_eq!(log_size, logs1.len());
}

#[test]
fn int_expr() {
    test_helper("2+4*(6+0)/1", Some((2 + 4 * (6 + 0) / 1) as f64), "", 0);
}

#[test]
fn int_expr_with_whitespace() {
    test_helper(
        " 2 + 4 * ( 6 + 0 ) / 1 ",
        Some((2 + 4 * (6 + 0) / 1) as f64),
        "",
        0,
    );
}

#[test]
fn float_expr() {
    test_helper("1.9/(2.6+0.8)+1.7", Some(1.9 / (2.6 + 0.8) + 1.7), "", 0);
}

#[test]
fn float_expr_with_whitespace() {
    test_helper(
        "1.9 / ( 2.6 + 0.8 ) + 1.7 ",
        Some(1.9 / (2.6 + 0.8) + 1.7),
        "",
        0,
    );
}