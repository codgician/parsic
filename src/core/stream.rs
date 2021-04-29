// ParseStream trait
pub trait ParseStream<T> {
    fn new(inp: T) -> Self;
    fn as_stream(&self) -> T;
    fn pos(&self) -> Pos;
    fn index(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    fn add(&self, d_row : usize, d_col: usize) -> Self {
        Self::new(self.row + d_row, self.col + d_col)
    }
}

#[derive(Clone, Debug)]
pub struct CharStream<'a> {
    pub(crate) inp: std::str::Chars<'a>,
    pub(crate) pos: Pos,
    pub(crate) len: usize,
    pub(crate) idx: usize
}

impl<'a> ParseStream<&'a str> for CharStream<'a> {
    fn new(inp: &'a str) -> Self {
        Self {
            inp: inp.chars(),
            pos: Pos::new(0, 0),
            len: inp.len(),
            idx: 0,
        }
    }

    fn as_stream(&self) -> &'a str {
        self.inp.as_str()
    }
    fn pos(&self) -> Pos {
        self.pos
    }
    fn index(&self) -> usize {
        self.idx
    }
    fn len(&self) -> usize {
        self.len
    }
}

// Implement Iterator trait for CharStream
impl<'a> Iterator for CharStream<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inp.next()?;

        self.pos = match ch {
            '\n' => Pos::new(self.pos.row + 1, 0),
            _ => self.pos.add(1, 0)
        };
        self.idx += 1;
        Some(ch)
    }
}
