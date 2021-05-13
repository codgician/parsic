mod lazy;
mod logger;
mod ops;
mod parser;

pub use crate::core::{self, lazy::*, logger::*, ops::*, parser::*};

/// Helper function that undo changes to stream
pub(crate) fn return_none<S: Clone, T>(cur: &mut S, bak: &S) -> Option<T> {
    *cur = bak.to_owned();
    None
}
