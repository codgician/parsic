pub mod functor;
pub mod applicative;
pub mod monadic;
pub mod fix;
pub mod sequential;
pub mod or;
pub mod log;

pub use crate::combinators::{
    self,
    sequential::*,
    applicative::*,
    fix::*,
    functor::*,
    monadic::*,
    or::*,
    log::*,
};
