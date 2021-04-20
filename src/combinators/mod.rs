pub mod functor;
pub mod applicative;
pub mod monadic;
pub mod fix;
pub mod and;
pub mod or;

pub use crate::combinators::{
    self,
    and::*,
    applicative::*,
    fix::*,
    functor::*,
    monadic::*,
    or::*,
};
