use crate::combinators::*;
use crate::core::{Parsable, Parser};
use std::ops::{BitAnd, BitOr, Mul, Shl, Shr};

/// # Overload Shl `<<` to `left` combinator
///
/// `p1 << p2` ~ `p1.left(p2)`
///
/// ## Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::{Parsable, Parser};
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = char('A') >> char('B');
///
/// let mut st = CharStream::new("ABC");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('B'), res);
/// assert_eq!("C", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
impl<'f, A: 'f, B: 'f, S, P> Shl<P> for Parser<'f, A, S>
where
    P: Parsable<Stream = S, Result = B> + 'f,
    S: Clone + 'f,
    Self: Sized + 'f,
{
    type Output = Parser<'f, A, S>;
    fn shl(self, rhs: P) -> Self::Output {
        left(self, rhs)
    }
}

/// # Overload Shr `>>` to `right` combinator
///
/// `p1 >> p2` ~ `p1.right(p2)`
///
/// ## Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::{Parsable, Parser};
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = char('A') << char('B');
///
/// let mut st = CharStream::new("ABC");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('A'), res);
/// assert_eq!("C", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
impl<'f, A: 'f, B: 'f, S, P> Shr<P> for Parser<'f, A, S>
where
    P: Parsable<Stream = S, Result = B> + 'f,
    S: Clone + 'f,
    Self: Sized + 'f,
{
    type Output = Parser<'f, B, S>;
    fn shr(self, rhs: P) -> Self::Output {
        right(self, rhs)
    }
}

/// # Overload operator `|` to `or` combinator
///
/// `p1 ^ p2` ~ `p1.or(p2)`
///
/// ## Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::{Parsable, Parser};
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = char('A') | char('B');
///
/// let mut st = CharStream::new("ABC");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some('A'), res);
/// assert_eq!("BC", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
impl<'f, A: 'f, S, P> BitOr<P> for Parser<'f, A, S>
where
    P: Parsable<Stream = S, Result = A> + 'f,
    S: Clone + 'f,
    Self: Sized + 'f,
{
    type Output = Parser<'f, A, S>;
    fn bitor(self, rhs: P) -> Self::Output {
        or(self, rhs)
    }
}

/// # Overload operator `&` to `and` combinator
/// 
/// `p1 & p2` ~ `p1.and(p2)`
/// 
/// ## Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::{Parsable, Parser};
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = char('A') & char('B');
///
/// let mut st = CharStream::new("ABC");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(('A', 'B')), res);
/// assert_eq!("C", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
impl<'f, A: 'f, B: 'f, S, P> BitAnd<P> for Parser<'f, A, S>
where
    P: Parsable<Stream = S, Result = B> + 'f,
    S: Clone + 'f,
    Self: Sized + 'f,
{
    type Output = Parser<'f, (A, B), S>;

    fn bitand(self, rhs: P) -> Self::Output {
        and(self, rhs)
    }
}

/// # Overload operator `*` to `compose` combinator
/// 
/// `p1 * p2` ~ `p1.compose(p2)`
/// 
/// ## Example
/// ```
/// use naive_parsec::combinators::*;
/// use naive_parsec::core::{Parsable, Parser};
/// use naive_parsec::primitives::{char, CharStream};
///
/// let parser = pure(|x| x == 'A') * char('A');
///
/// let mut st = CharStream::new("ABC");
/// let (res, logs) = parser.exec(&mut st);
///
/// assert_eq!(Some(true), res);
/// assert_eq!("BC", st.as_str());
/// assert_eq!(0, logs.len());
/// ```
impl<'f, A: 'f, B: 'f, F, S, P> Mul<P> for Parser<'f, F, S>
where
    F: Fn(A) -> B + 'f,
    P: Parsable<Stream = S, Result = A> + 'f,
    S: Clone + 'f,
    Self: Sized + 'f,
{
    type Output = Parser<'f, B, S>;

    fn mul(self, rhs: P) -> Self::Output {
        compose(self, rhs)
    }
}

#[cfg(test)]
mod test_ops {
    use crate::combinators::*;
    use crate::core::Parsable;
    use crate::primitives::{char, satisfy, CharStream};

    #[test]
    fn mutual_recursive_syntax() {
        // expr     := term '+' expr | term
        // term     := factor '*' term | factor
        // factor   := '(' expr ')' | uint
        // uint     := digit { digit }
        // digit    := '0' | '1' | ... | '9'
        let expr = fix(move |expr| {
            let digit = satisfy(|&ch| ch.is_digit(10));
            let uint = digit.some().map(|v| v.iter().collect::<String>().parse::<u64>().unwrap());
            let factor = char('(') >> expr.clone() << char(')') | uint;
            let term = fix(move |term| {
                (factor.clone() << char('*') & term.clone()).map(|(v1, v2)| v1 * v2) | factor.clone()
            });
            (term.clone() << char('+') & expr).map(|(v1, v2)| v1 + v2) | term
        });

        let mut st = CharStream::new("1+2*(3+4)");
        let (res, logs) = expr.exec(&mut st);

        assert_eq!(Some(15), res);
        assert_eq!(0, logs.len());
    }
}
