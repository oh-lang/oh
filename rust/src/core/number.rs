pub use num_traits::Signed as HasSign;
pub use num_traits::{AsPrimitive, Num, PrimInt, ToPrimitive};

pub use std::cmp::PartialOrd;
pub use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug)]
pub enum NumberError {
    Unrepresentable,
}
