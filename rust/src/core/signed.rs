use crate::core::number::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub type Signed64 = Signed<i64>;
pub type Signed32 = Signed<i32>;
pub type Signed16 = Signed<i16>;
pub type Signed8 = Signed<i8>;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Signed<T: SignedPrimitive>(T);

impl<T> Signed<T>
where
    T: SignedPrimitive,
{
    pub const MAX: Self = Self(T::MAX);
    pub const MIN: Self = Self(T::MIN);

    pub fn of(t: T) -> Self {
        Self(t)
    }
}

impl<T> Deref for Signed<T>
where
    T: SignedPrimitive,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Signed<T>
where
    T: SignedPrimitive,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Debug for Signed<T>
where
    T: SignedPrimitive,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Signed{}::", T::BITS)?;
        if let Some(value) = self.to_u64() {
            write!(f, "of({})", value)
        } else {
            write!(f, "NULL")
        }
    }
}

pub trait SignedPrimitive:
    PrimInt
    + TryFrom<i64>
    + ToPrimitive
    + AsPrimitive<i64>
    + HasSign
    + AddAssign
    + Add<Output = Self>
    + SubAssign
    + Sub<Output = Self>
    + PartialOrd
    + Sized
    + Neg
    + Num
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const BITS: u64;
}

impl SignedPrimitive for i64 {
    const MIN: Self = i64::MIN;
    const MAX: Self = i64::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const BITS: u64 = 64;
}

impl SignedPrimitive for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const BITS: u64 = 32;
}

impl SignedPrimitive for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const BITS: u64 = 16;
}

impl SignedPrimitive for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const BITS: u64 = 8;
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
        assert_eq!(Signed8::MIN, Signed8::of(-128));
        assert_eq!(Signed8::MAX, Signed8::of(127));
    }

    #[test]
    fn signed16() {
        assert_eq!(i16::ZERO, 0);
        assert_eq!(i16::ONE, 1);
        assert_eq!(i16::TWO, 2);
        assert_eq!(i16::MIN, -32768);
        assert_eq!(i16::MAX, 32767);
        assert_eq!(Signed16::MIN, Signed16::of(-32768));
        assert_eq!(Signed16::MAX, Signed16::of(32767));
    }

    #[test]
    fn signed32() {
        assert_eq!(i32::ZERO, 0);
        assert_eq!(i32::ONE, 1);
        assert_eq!(i32::TWO, 2);
        assert_eq!(i32::MIN, -2147483648);
        assert_eq!(i32::MAX, 2147483647);
        assert_eq!(Signed32::MIN, Signed32::of(-2147483648));
        assert_eq!(Signed32::MAX, Signed32::of(2147483647));
    }

    #[test]
    fn signed64() {
        assert_eq!(i64::ZERO, 0);
        assert_eq!(i64::ONE, 1);
        assert_eq!(i64::TWO, 2);
        assert_eq!(i64::MIN, -9223372036854775808);
        assert_eq!(i64::MAX, 9223372036854775807);
        assert_eq!(Signed64::MIN, Signed64::of(-9223372036854775808));
        assert_eq!(Signed64::MAX, Signed64::of(9223372036854775807));
    }
}
