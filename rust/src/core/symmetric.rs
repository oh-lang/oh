use crate::core::likely::*;
use crate::core::number::*;
use crate::core::signed::*;

use std::convert::From;

pub type Symmetric64 = SymmetricN<i64>;
pub type Symmetric32 = SymmetricN<i32>;
pub type Symmetric16 = SymmetricN<i16>;
pub type Symmetric8 = SymmetricN<i8>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct SymmetricN<T: SignedPrimitive>(T);

impl<T> SymmetricN<T>
where
    T: SignedPrimitive,
{
    pub const MAX: Self = Self(T::MAX);
    // I hate that Rust makes it impossible (or impossibly hard to figure out how) to do this as a const.
    pub fn min() -> Self {
        Self(T::MIN + T::ONE)
    }
    pub const NULL: Self = Self(T::MIN);

    pub fn of(t: T) -> Self {
        Self(t)
    }

    pub fn is_null(self: Self) -> bool {
        unlikely(self == Self::NULL)
    }

    pub fn is_not_null(self: Self) -> bool {
        likely(self != Self::NULL)
    }
}

impl<T> Default for SymmetricN<T>
where
    T: SignedPrimitive,
{
    fn default() -> Self {
        Self(T::ZERO)
    }
}

impl<T> ToPrimitive for SymmetricN<T>
where
    T: SignedPrimitive,
{
    fn to_i64(&self) -> Option<i64> {
        if self.is_not_null() {
            Some(self.0.as_())
        } else {
            None
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if self.0 >= T::ZERO {
            Some(self.0.as_() as u64)
        } else {
            None
        }
    }
}

impl<T: SignedPrimitive> SubAssign<Self> for SymmetricN<T> {
    fn sub_assign(&mut self, other: Self) {
        if unlikely(other.is_null()) {
            self.0 = T::MIN;
        } else {
            *self -= other.0;
        }
    }
}

impl<T: SignedPrimitive> SubAssign<T> for SymmetricN<T> {
    fn sub_assign(&mut self, other: T) {
        if unlikely(self.is_null()) {
        } else if let Some(value) = self.0.checked_sub(&other) {
            self.0 = value;
        } else {
            self.0 = T::MIN;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn symmetric8() {
        let zero = Symmetric8::default();
        assert_eq!(zero, Symmetric8::of(0));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));

        let eno = Symmetric8::of(-1);
        assert_eq!(eno.to_i64(), Some(-1));
        assert_eq!(eno.to_u64(), None);

        let max = Symmetric8::MAX;
        assert_eq!(max, Symmetric8::of(127));
        assert_eq!(max.to_i64(), Some(127));
        assert_eq!(max.to_u64(), Some(127));

        let min = Symmetric8::min();
        assert_eq!(min, Symmetric8::of(-127));
        assert_eq!(min.to_i64(), Some(-127));
        assert_eq!(min.to_u64(), None);
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let null = Symmetric8::NULL;
        assert_eq!(null, Symmetric8::of(-128));
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
    }

    #[test]
    fn symmetric16() {
        let zero = Symmetric16::default();
        assert_eq!(zero, Symmetric16::of(0));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));

        let eno = Symmetric16::of(-1);
        assert_eq!(eno.to_i64(), Some(-1));
        assert_eq!(eno.to_u64(), None);

        let max = Symmetric16::MAX;
        assert_eq!(max, Symmetric16::of(32767));
        assert_eq!(max.to_i64(), Some(32767));
        assert_eq!(max.to_u64(), Some(32767));

        let min = Symmetric16::min();
        assert_eq!(min, Symmetric16::of(-32767));
        assert_eq!(min.to_i64(), Some(-32767));
        assert_eq!(min.to_u64(), None);
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let null = Symmetric16::NULL;
        assert_eq!(null, Symmetric16::of(-32768));
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
    }

    #[test]
    fn symmetric32() {
        let zero = Symmetric32::default();
        assert_eq!(zero, Symmetric32::of(0));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));

        let eno = Symmetric32::of(-1);
        assert_eq!(eno.to_i64(), Some(-1));
        assert_eq!(eno.to_u64(), None);

        let max = Symmetric32::MAX;
        assert_eq!(max, Symmetric32::of(2147483647));
        assert_eq!(max.to_i64(), Some(2147483647));
        assert_eq!(max.to_u64(), Some(2147483647));

        let min = Symmetric32::min();
        assert_eq!(min, Symmetric32::of(-2147483647));
        assert_eq!(min.to_i64(), Some(-2147483647));
        assert_eq!(min.to_u64(), None);
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let null = Symmetric32::NULL;
        assert_eq!(null, Symmetric32::of(-2147483648));
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
    }

    #[test]
    fn symmetric64() {
        let zero = Symmetric64::default();
        assert_eq!(zero, Symmetric64::of(0));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));

        let eno = Symmetric64::of(-1);
        assert_eq!(eno.to_i64(), Some(-1));
        assert_eq!(eno.to_u64(), None);

        let max = Symmetric64::MAX;
        assert_eq!(max, Symmetric64::of(9223372036854775807));
        assert_eq!(max.to_i64(), Some(9223372036854775807));
        assert_eq!(max.to_u64(), Some(9223372036854775807));

        let min = Symmetric64::min();
        assert_eq!(min, Symmetric64::of(-9223372036854775807));
        assert_eq!(min.to_i64(), Some(-9223372036854775807));
        assert_eq!(min.to_u64(), None);
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let null = Symmetric64::NULL;
        assert_eq!(null, Symmetric64::of(-9223372036854775808));
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
    }

    #[test]
    fn sub_assign() {
        let mut sixty_four = Symmetric64::NULL;
        sixty_four -= 123;
        assert_eq!(sixty_four.is_null(), true);

        let mut eight = Symmetric8::default();
        eight -= 123;
        assert_eq!(eight, Symmetric8::of(-123));

        eight -= Symmetric8::NULL;
        assert_eq!(eight, Symmetric8::NULL);

        let mut thirty_two = Symmetric32::min();
        thirty_two -= 1;
        assert_eq!(thirty_two, Symmetric32::NULL);

        let mut sixteen = Symmetric16::MAX;
        sixteen -= -1;
        assert_eq!(sixteen, Symmetric16::NULL);
    }
}
