pub use num_traits::{AsPrimitive, Num, PrimInt, Signed, ToPrimitive};

pub use std::cmp::PartialOrd;
pub use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub enum NumberError {
    Unrepresentable,
}
