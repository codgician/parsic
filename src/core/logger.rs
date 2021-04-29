use crate::core::stream::Pos;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Msg {
    Info(MsgBody),
    Warn(MsgBody),
    Err(MsgBody)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MsgBody {
    pub msg: String,
    pub pos: Option<Pos>
}

impl MsgBody {
    pub fn new(msg: &str, pos: Option<Pos>) -> Self {
        Self { msg: msg.to_string(), pos }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ParseLogger {
    stack: Vec<Msg>
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
