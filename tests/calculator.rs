/// # Test: a simple arithmetic calculator
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
use naive_parsec::core::Parsable;
use naive_parsec::primitives::*;

fn digit() -> impl Parsable<StrState, Result = char> {
    satisfy(|&ch| ch.is_digit(10))
}

fn uint() -> impl Parsable<StrState, Result = String> {
    digit().some()
        .map(|v| v.iter().collect::<String>())
}

fn float() -> impl Parsable<StrState, Result = f64> {
    uint().and(char('.').and(uint()).optional())
        .map_opt(|(s, r)| {
            let mut res = s;
            match r {
                Some((dot, frac)) => {
                    res.push(dot);
                    res.push_str(&frac[..])
                }
                _ => {}
            };
            res.parse::<f64>()
        }).trim()
}

fn factor() -> impl Parsable<StrState, Result = f64> {
    mid(
        char('(').trim(),
        expr(), 
        char(')').trim()
    ).or(float())
}

fn term() -> impl Parsable<StrState, Result = f64> {
    fix(|term_it| {
        Box::new(
            factor()
                .and(char('*').or(char('/').trim()))
                .and(term_it)
                .map(|((v1, op), v2)| if op == '*' { v1 * v2 } else { v1 / v2 })
                .or(factor()),
        )
    })
}

fn expr() -> impl Parsable<StrState, Result = f64> {
    fix(|expr_it| {
        Box::new(
            term()
                .and(char('+').or(char('-')).trim())
                .and(expr_it)
                .map(|((v1, op), v2)| if op == '+' { v1 + v2 } else { v1 - v2 })
                .or(term()),
        )
    })
}

#[test]
fn int_expr() {
    let mut st = StrState::new("2+4*(6+0)/1");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some((2+4*(6+0)/1) as f64), res);
    assert_eq!("", st.as_stream());
    assert_eq!(0, logs.len());
}

#[test]
fn int_expr_with_whitespace() {
    let mut st = StrState::new(" 2 + 4 * ( 6 + 0 ) / 1 ");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some((2+4*(6+0)/1) as f64), res);
    assert_eq!("", st.as_stream());
    assert_eq!(0, logs.len());
}

#[test]
fn float_expr() {
    let mut st = StrState::new("1.9/(2.6+0.8)+1.7");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some(1.9/(2.6+0.8)+1.7), res);
    assert_eq!("", st.as_stream());
    assert_eq!(0, logs.len());
}

#[test]
fn float_expr_with_whitespace() {
    let mut st = StrState::new("1.9 / ( 2.6 + 0.8 ) + 1.7 ");
    let (res, logs) = expr().exec(&mut st);

    assert_eq!(Some(1.9/(2.6+0.8)+1.7), res);
    assert_eq!("", st.as_stream());
    assert_eq!(0, logs.len());
}
