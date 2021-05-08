mod lazy;
mod logger;
mod parser;

pub use crate::core::{self, lazy::*, logger::*, parser::*};

// `ret_none`: Helper function
pub(crate) fn return_none<S: Clone, T>(cur: &mut S, bak: &S) -> Option<T> {
    *cur = bak.to_owned();
    None
}
