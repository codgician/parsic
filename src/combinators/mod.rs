mod alternative;
mod applicative;
mod error;
mod fix;
mod functor;
mod monad;
mod replicative;
mod sequential;

pub use crate::combinators::{
    self, alternative::*, applicative::*, error::*, fix::*, functor::*, monad::*, replicative::*,
    sequential::*,
};
