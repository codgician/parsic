use crate::core::Pos;

#[derive(Clone, Debug)]
pub struct StrState {
    pub(crate) inp: std::str::Chars<'static>,
    pub(crate) pos: Pos,
    pub(crate) len: usize,
    pub(crate) idx: usize,
}

impl StrState {
    pub fn new(inp: &'static str) -> Self {
        Self {
            inp: inp.chars(),
            pos: Pos::new(0, 0),
            len: inp.len(),
            idx: 0,
        }
    }

    pub fn as_stream(&self) -> &'static str {
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
}

// Implement Iterator trait for StrState
impl Iterator for StrState {
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
