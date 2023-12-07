pub mod array;
pub mod list;
mod traits;

pub use array::Array;
pub use list::DynamicList;

pub use array::traits::*;
pub use list::traits::*;
pub use traits::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Empty;
