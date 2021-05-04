use crate::combinators::*;
use crate::core::Parsable;
use crate::primitives::{char, StrState};

pub fn space() -> impl Parsable<StrState, Result = char> + Copy {
    char(' ').or(char('\n')).or(char('\r')).or(char('\t'))
}

pub fn trim<T>(parser: impl Parsable<StrState, Result = T>) -> impl Parsable<StrState, Result = T> {
    mid(space().many(), parser, space().many())
}
