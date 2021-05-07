use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Pos(usize, usize); // (row, colum)

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }
    pub fn add(&self, d_row: usize, d_col: usize) -> Self {
        Self(self.0 + d_row, self.1 + d_col)
    }
    pub fn row(&self) -> usize {
        self.0
    }
    pub fn col(&self) -> usize {
        self.1
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Msg {
    Info(MsgBody),
    Warn(MsgBody),
    Error(MsgBody),
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (level, body) = match self {
            Msg::Info(x) => ("INFO", x),
            Msg::Warn(x) => ("WARN", x),
            Msg::Error(x) => ("ERROR", x),
        };

        let pos_text = match body.pos {
            Some(Pos(r, c)) => format!("(at row: {}, col: {})", r, c),
            _ => "".to_string(),
        };

        write!(f, "[{}]: {} {}.", level, body.msg, pos_text)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MsgBody {
    pub msg: String,
    pub pos: Option<Pos>,
}

impl MsgBody {
    pub fn new(msg: &str, pos: Option<Pos>) -> Self {
        Self {
            msg: msg.to_string(),
            pos,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ParseLogger {
    pub stack: Vec<Msg>,
}

impl ParseLogger {
    pub fn add(&mut self, msg: Msg) {
        self.stack.push(msg);
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn with(&mut self, msg: Msg) {
        self.clear();
        self.add(msg);
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

// Implement IntoInterator for ParseLogger
impl IntoIterator for ParseLogger {
    type Item = Msg;
    type IntoIter = std::vec::IntoIter<Msg>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack.into_iter()
    }
}

// Implement IntoInterator for &'a ParseLogger
impl<'a> IntoIterator for &'a ParseLogger {
    type Item = &'a Msg;
    type IntoIter = std::slice::Iter<'a, Msg>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack.iter()
    }
}

// Implement IntoInterator for &'a mut ParseLogger
impl<'a> IntoIterator for &'a mut ParseLogger {
    type Item = &'a mut Msg;
    type IntoIter = std::slice::IterMut<'a, Msg>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack.iter_mut()
    }
}
