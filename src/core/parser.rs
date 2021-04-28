use std::rc::Rc;
use crate::core::logger::ParseLogger;

// Parser trait
pub trait Parser<S> {
    type ParsedType;
    fn parse(&self, state: &mut S) -> Option<Self::ParsedType>;
}

// Implement Parser trait for &Parser<_>
impl<S, P: Parser<S>> Parser<S> for &P {
    type ParsedType = P::ParsedType;
    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
        (**self).parse(state)
    }
}

// Implement Parser trait for &mut Parser<_>
impl<S, P: Parser<S>> Parser<S> for &mut P {
    type ParsedType = P::ParsedType;
    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
        (**self).parse(state)
    }
}

// Implement Parser trait for Box<Parser<_>>
impl<S, P: Parser<S>> Parser<S> for Box<P> {
    type ParsedType = P::ParsedType;
    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
        (**self).parse(state)
    }
}

// Implement Parser trait for Rc<Parser<_>>
impl<S, P: Parser<S>> Parser<S> for Rc<P> {
    type ParsedType = P::ParsedType;
    fn parse(&self, state: &mut S) -> Option<Self::ParsedType> {
        (**self).parse(state)
    }
}

// Parse state
#[derive(Clone, Debug)]
pub struct ParseState<'a> {
    pub(crate) inp: std::str::Chars<'a>,
    pub(crate) pos: Pos,
    pub(crate) len: usize,
    pub(crate) idx: usize,
    pub(crate) log: ParseLogger
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize
}

impl<'a> ParseState<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self {
            inp: inp.chars(),
            pos: Pos { row: 0, col: 0 },
            len: inp.len(),
            idx: 0,
            log: ParseLogger::default()
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.inp.as_str()
    }
    pub fn pos(&self) -> Pos {
        self.pos
    }
    pub fn index(&self) -> usize {
        self.idx
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn logs(&self) -> &ParseLogger {
        &self.log
    }
}

impl<'a> Iterator for ParseState<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inp.next()?;
        self.pos = match ch {
            '\n' => Pos {
                row: self.pos.row + 1,
                col: 0,
            },
            _ => Pos {
                col: self.pos.col + 1,
                ..self.pos
            },
        };
        self.idx += 1;
        Some(ch)
    }
}
