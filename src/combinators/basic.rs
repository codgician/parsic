use crate::core::Parser;

/// ### Combinator: `empty`
/// A parser that consumes no item and always fails.
pub fn empty<'f, A: 'f, S: 'f>() -> Parser<'f, A, S> {
    Parser::new(|_, _| None)
}

/// ### Combinator: `pure`
/// Injects a value into an identity parser.
pub fn pure<'f, A: Clone + 'f, S: 'f>(x: A) -> Parser<'f, A, S> {
    Parser::new(move |_, _| Some(x.clone()))
}

#[cfg(test)]
mod test_empty {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::CharStream;

    #[test]
    fn should_always_fail() {
        let parser = empty::<char, CharStream>();

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(None, res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(0, logs.len());
    }
}

#[cfg(test)]
mod test_pure {
    use crate::combinators::*;
    use crate::core::*;
    use crate::primitives::CharStream;

    #[test]
    fn injects_value() {
        let parser = pure(true);

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(Some(true), res);
        assert_eq!("Hello", st.as_str());
        assert_eq!(0, logs.len());
    }

    #[test]
    fn injects_function() {
        let parser = pure(|_| true);

        let mut st = CharStream::new("Hello");
        let (res, logs) = parser.exec(&mut st);

        assert_eq!(true, res.unwrap()(1));
        assert_eq!("Hello", st.as_str());
        assert_eq!(0, logs.len());
    }
}
