pub use num_traits::Signed as HasSign;
pub use num_traits::{AsPrimitive, Num, PrimInt, ToPrimitive};

pub use std::cmp::{Ordering, PartialOrd};
pub use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq)]
pub enum NumberError {
    Unrepresentable,
}

pub type NumberResult<T> = Result<T, NumberError>;
