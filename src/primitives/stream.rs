use crate::core::Pos;

#[derive(Clone, Debug)]
pub struct CharStream<'s> {
    pub(crate) inp: std::str::Chars<'s>,
    pub(crate) pos: Pos,
    pub(crate) len: usize,
    pub(crate) idx: usize,
}

impl<'s> CharStream<'s> {
    pub fn new(inp: &'s str) -> Self {
        Self {
            inp: inp.chars(),
            pos: Pos::new(0, 0),
            len: inp.len(),
            idx: 0,
        }
    }

    pub fn as_str(&self) -> &'s str {
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
}

// Implement Iterator trait for CharStream
impl<'s> Iterator for CharStream<'s> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inp.next()?;

        self.pos = match ch {
            '\n' => Pos::new(self.pos.row() + 1, 0),
            _ => self.pos.add(1, 0),
        };
        self.idx += 1;
        Some(ch)
    }
}
