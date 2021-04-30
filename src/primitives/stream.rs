use crate::core::logger::Pos;

#[derive(Clone, Debug)]
pub struct StrState<'a> {
    pub(crate) inp: std::str::Chars<'a>,
    pub(crate) pos: Pos,
    pub(crate) len: usize,
    pub(crate) idx: usize
}

impl<'a> StrState<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self {
            inp: inp.chars(),
            pos: Pos::new(0, 0),
            len: inp.len(),
            idx: 0,
        }
    }

    pub fn as_stream(&self) -> &'a str {
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
impl<'a> Iterator for StrState<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inp.next()?;

        self.pos = match ch {
            '\n' => Pos::new(self.pos.row() + 1, 0),
            _ => self.pos.add(1, 0)
        };
        self.idx += 1;
        Some(ch)
    }
}
