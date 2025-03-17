use num_traits::{AsPrimitive, Num, PrimInt, Signed, ToPrimitive};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub type Signed64 = i64;
pub type Signed32 = i32;
pub type Signed16 = i16;
pub type Signed8 = i8;

pub trait SignedPrimitive:
    PrimInt
    + ToPrimitive
    + AsPrimitive<i64>
    + Signed
    + AddAssign
    + Add<Output = Self>
    + SubAssign
    + Sub<Output = Self>
    + std::cmp::PartialOrd
    + Sized
    + std::ops::Neg
    + Num
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
}

impl SignedPrimitive for i64 {
    const MIN: Self = i64::MIN;
    const MAX: Self = i64::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

impl SignedPrimitive for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

impl SignedPrimitive for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

impl SignedPrimitive for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signed8() {
        assert_eq!(i8::ZERO, 0);
        assert_eq!(i8::ONE, 1);
        assert_eq!(i8::TWO, 2);
        assert_eq!(i8::MIN, -128);
        assert_eq!(i8::MAX, 127);
    }

    #[test]
    fn signed16() {
        assert_eq!(i16::ZERO, 0);
        assert_eq!(i16::ONE, 1);
        assert_eq!(i16::TWO, 2);
        assert_eq!(i16::MIN, -32768);
        assert_eq!(i16::MAX, 32767);
    }

    #[test]
    fn signed32() {
        assert_eq!(i32::ZERO, 0);
        assert_eq!(i32::ONE, 1);
        assert_eq!(i32::TWO, 2);
        assert_eq!(i32::MIN, -2147483648);
        assert_eq!(i32::MAX, 2147483647);
    }

    #[test]
    fn signed64() {
        assert_eq!(i64::ZERO, 0);
        assert_eq!(i64::ONE, 1);
        assert_eq!(i64::TWO, 2);
        assert_eq!(i64::MIN, -9223372036854775808);
        assert_eq!(i64::MAX, 9223372036854775807);
    }
}
