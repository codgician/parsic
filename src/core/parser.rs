use std::str::Chars;

use crate::core::logger::ParseLogger;

// Parser trait
pub trait Parser<S> {
    type ParsedType;
    fn parse<'a>(&self, state: &mut S) -> Option<Self::ParsedType>;
}

// Parse state
#[derive(Clone, Debug)]
pub struct ParseState<'a> {
    pub inp: Chars<'a>,
    pub pos: Pos,
    pub len: usize,
    pub idx: usize,
    pub log: ParseLogger
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
