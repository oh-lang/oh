use crate::core::index::*;
use crate::core::likely::*;
use crate::core::number::*;
use crate::core::offset::*;
use crate::core::signed::*;

use std::fmt::{self, Debug, Formatter};
use std::num::Wrapping;

/// The largest count that this platform can support.
// TODO: on a 32 bit platform, go to Count32 instead.
pub type CountMax = Count64;

pub type Count64 = Count<i64>;
pub type Count32 = Count<i32>;
pub type Count16 = Count<i16>;
pub type Count8 = Count<i8>;

pub type Contains64 = Contains<i64>;
pub type Contains32 = Contains<i32>;
pub type Contains16 = Contains<i16>;
pub type Contains8 = Contains<i8>;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Count<T: SignedPrimitive>(T);

impl<T> Count<T>
where
    T: SignedPrimitive,
{
    pub const MAX: Self = Self(T::MIN);
    pub const MIN: Self = Self(T::ZERO);
    // NOT public because null can be any value > 0 (use `is_null`
    const NULL: Self = Self(T::ONE);

    pub fn negating(t: T) -> Self {
        Self(t)
    }

    pub fn as_negated(self) -> T {
        self.0
    }

    pub fn to_max(self) -> CountMax {
        // TODO: fix for when we support i32 in CountMax on 32bit platforms
        CountMax::negating(self.to_i64().unwrap_or(0))
    }

    pub fn double_or_at_least(self, at_least: T) -> Self {
        if self.0 <= T::MIN / T::TWO {
            // doubling would overflow, just return max.
            Self::MAX
        } else {
            debug_assert!(at_least > T::ZERO);
            // We negate things, so max -> min.
            Self((-at_least).min(T::TWO * self.0))
        }
    }

    pub fn contains(&self, contains: Contains<T>) -> bool {
        if self.is_null() {
            return false;
        }
        match contains {
            Contains::<T>::Offset(offset) => {
                if offset.0 < T::ZERO {
                    cold();
                    false
                } else {
                    self.0 + offset.0 < T::ZERO
                }
            }
            Contains::<T>::Index(index) => {
                if index.0 >= T::ZERO {
                    // if count == 100, self.0 = -100
                    // then an index of 0 would give -100 < 0 (true)
                    // and an index of 99 would give -1 < 0 (true)
                    // but an index of 100 would give 0 < 0 (false)
                    self.0 + index.0 < T::ZERO
                } else {
                    // The LHS here is the actual offset.
                    index.0 - self.0 >= T::ZERO
                }
            }
        }
    }

    /// Essentially `index = count - 1`.
    /// `index < 0` should be considered empty (count == 0 or count == null).
    pub fn to_highest_offset(self) -> Offset<T> {
        Offset::<T>::of(-(self.0 + T::ONE))
    }

    // TODO: these should probably be `from`/`into` methods for
    // more general `Ordinal` vs. `Signed` classes.  `Count` comes from `Ordinal`,
    // `Offset` comes from `Signed`.
    /// Essentially `count = offset + 1`.
    /// `offset < 0` will be considered empty, returning `count = 0`.
    /// Note `offset == 0` corresponds to `count == 1`!
    pub fn from_highest_offset(offset: Offset<T>) -> Self {
        if offset.0 < T::ZERO {
            Self(T::ZERO)
        } else {
            Self(-offset.0 - T::ONE)
        }
    }

    /// WARNING: Converts null into 0.
    pub fn to_usize(self) -> usize {
        self.to_u64().unwrap_or(0) as usize
    }

    pub fn of(value: usize) -> Result<Self, NumberError> {
        if value > T::MAX.to_u64().unwrap() as usize + 1 {
            cold();
            Err(NumberError::Unrepresentable)
        } else {
            let negated_value: i64 = (-(Wrapping(value as i64) - Wrapping(1)) - Wrapping(1)).0;
            Ok(Self(T::try_from(negated_value).unwrap_or(T::ONE)))
        }
    }

    /// Returns true iff Count is not null and Count > 0.
    pub fn is_positive(self) -> bool {
        // Remember that we're negated internally.
        !(self.0 >= T::ZERO)
    }

    pub fn is_null(self: Self) -> bool {
        unlikely(self.0 > T::ZERO)
    }

    pub fn is_not_null(self: Self) -> bool {
        likely(self.0 <= T::ZERO)
    }
}

pub enum Contains<T: SignedPrimitive> {
    Offset(Offset<T>),
    Index(Index<T>),
}

impl<T> Debug for Count<T>
where
    T: SignedPrimitive,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Count{}::", T::BITS)?;
        if let Some(value) = self.to_u64() {
            write!(f, "of({}).expect(\"ok\")", value)
        } else {
            write!(f, "NULL")
        }
    }
}

impl<T> Default for Count<T>
where
    T: SignedPrimitive,
{
    fn default() -> Self {
        Self(T::ZERO)
    }
}

impl<T> ToPrimitive for Count<T>
where
    T: SignedPrimitive,
{
    fn to_i64(&self) -> Option<i64> {
        if self.is_not_null() && self.0.as_() != i64::MIN {
            Some(-(self.0 + T::ONE).as_() + 1)
        } else {
            None
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if self.0 <= T::ZERO {
            Some((Wrapping((-(Wrapping(self.0.as_()) + Wrapping(1))).0 as u64) + Wrapping(1)).0)
        } else {
            None
        }
    }
}

impl<T: SignedPrimitive> PartialOrd for Count<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.partial_cmp(&other.0).unwrap().reverse())
    }
}

impl<T: SignedPrimitive> Add<Self> for Count<T> {
    type Output = Count<T>;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl<T: SignedPrimitive> AddAssign<Self> for Count<T> {
    fn add_assign(&mut self, other: Self) {
        if other.is_null() {
            cold();
            *self = Self::NULL;
        } else {
            *self -= other.0; // subtract because in AddAssign<T> we subtract
        }
    }
}

impl<T: SignedPrimitive> Add<T> for Count<T> {
    type Output = Count<T>;

    fn add(mut self, other: T) -> Self::Output {
        self += other;
        self
    }
}

impl<T: SignedPrimitive> AddAssign<T> for Count<T> {
    fn add_assign(&mut self, other: T) {
        if self.is_null() {
            cold();
        } else if let Some(value) = self.0.checked_sub(&other) {
            if value <= T::ZERO {
                self.0 = value;
            } else {
                cold();
                *self = Self::NULL;
            }
        } else {
            cold();
            *self = Self::NULL;
        }
    }
}

impl<T: SignedPrimitive> Sub<Self> for Count<T> {
    type Output = Count<T>;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl<T: SignedPrimitive> SubAssign<Self> for Count<T> {
    fn sub_assign(&mut self, other: Self) {
        if other.is_null() {
            cold();
            *self = Self::NULL;
        } else {
            *self += other.0; // add because in SubAssign<T> we add
        }
    }
}

impl<T: SignedPrimitive> Sub<T> for Count<T> {
    type Output = Count<T>;

    fn sub(mut self, other: T) -> Self::Output {
        self -= other;
        self
    }
}

impl<T: SignedPrimitive> SubAssign<T> for Count<T> {
    fn sub_assign(&mut self, other: T) {
        if self.is_null() {
            cold();
        } else if let Some(value) = self.0.checked_add(&other) {
            if value <= T::ZERO {
                self.0 = value;
            } else {
                cold();
                *self = Self::NULL;
            }
        } else {
            cold();
            *self = Self::NULL;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn count8() {
        let zero = Count8::default();
        assert_eq!(zero, Count8::of(0).expect("ok"));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));
        assert_eq!(zero.to_usize(), 0);
        assert_eq!(zero.to_highest_offset(), Offset8::of(-1));
        assert_eq!(zero.is_positive(), false);

        let one = Count8::of(1).expect("ok");
        assert_eq!(one.to_i64(), Some(1));
        assert_eq!(one.to_u64(), Some(1));
        assert_eq!(one.to_usize(), 1);
        assert_eq!(one.to_highest_offset(), Offset8::of(0));
        assert_eq!(one.is_positive(), true);

        let max = Count8::MAX;
        assert_eq!(max, Count8::of(128).expect("ok"));
        assert_eq!(max.to_i64(), Some(128));
        assert_eq!(max.to_u64(), Some(128));
        assert_eq!(max.to_usize(), 128);
        assert_eq!(max.to_highest_offset(), Offset8::of(127));
        assert_eq!(max.is_positive(), true);

        let min = Count8::MIN;
        assert_eq!(min, Count8::of(0).expect("ok"));
        assert_eq!(min.to_i64(), Some(0));
        assert_eq!(min.to_u64(), Some(0));
        assert_eq!(min.to_usize(), 0);
        assert_eq!(min.to_highest_offset(), Offset8::of(-1));
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let mut null = Count8::NULL;
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.to_usize(), 0);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset8::of(-2));
        assert_eq!(null.is_positive(), false);
        null.0 = 15; // doesn't matter which positive value we use, still null.
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.to_usize(), 0);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset8::of(-16));
        assert_eq!(null.is_positive(), false);
    }

    #[test]
    fn count16() {
        let zero = Count16::default();
        assert_eq!(zero, Count16::of(0).expect("ok"));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));
        assert_eq!(zero.to_highest_offset(), Offset16::of(-1));
        assert_eq!(zero.is_positive(), false);

        let one = Count16::of(1).expect("ok");
        assert_eq!(one.to_i64(), Some(1));
        assert_eq!(one.to_u64(), Some(1));
        assert_eq!(one.to_highest_offset(), Offset16::of(0));
        assert_eq!(one.is_positive(), true);

        let max = Count16::MAX;
        assert_eq!(max, Count16::of(32768).expect("ok"));
        assert_eq!(max.to_i64(), Some(32768));
        assert_eq!(max.to_u64(), Some(32768));
        assert_eq!(max.to_highest_offset(), Offset16::of(32767));
        assert_eq!(max.is_positive(), true);

        let min = Count16::MIN;
        assert_eq!(min, Count16::of(0).expect("ok"));
        assert_eq!(min.to_i64(), Some(0));
        assert_eq!(min.to_u64(), Some(0));
        assert_eq!(min.to_highest_offset(), Offset16::of(-1));
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let mut null = Count16::NULL;
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset16::of(-2));
        assert_eq!(null.is_positive(), false);
        null.0 = 15; // doesn't matter which positive value we use, still null.
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset16::of(-16));
        assert_eq!(null.is_positive(), false);
    }

    #[test]
    fn count32() {
        let zero = Count32::default();
        assert_eq!(zero, Count32::of(0).expect("ok"));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));
        assert_eq!(zero.to_highest_offset(), Offset32::of(-1));

        let one = Count32::of(1).expect("ok");
        assert_eq!(one.to_i64(), Some(1));
        assert_eq!(one.to_u64(), Some(1));
        assert_eq!(one.to_highest_offset(), Offset32::of(0));

        let max = Count32::MAX;
        assert_eq!(max, Count32::of(2147483648).expect("ok"));
        assert_eq!(max.to_i64(), Some(2147483648));
        assert_eq!(max.to_u64(), Some(2147483648));
        assert_eq!(max.to_highest_offset(), Offset32::of(2147483647));

        let min = Count32::MIN;
        assert_eq!(min, Count32::of(0).expect("ok"));
        assert_eq!(min.to_i64(), Some(0));
        assert_eq!(min.to_u64(), Some(0));
        assert_eq!(min.to_highest_offset(), Offset32::of(-1));
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let mut null = Count32::NULL;
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset32::of(-2));
        null.0 = 15; // doesn't matter which positive value we use, still null.
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset32::of(-16));
    }

    #[test]
    fn count64() {
        let zero = Count64::default();
        assert_eq!(zero, Count64::of(0).expect("ok"));
        assert_eq!(zero.to_i64(), Some(0));
        assert_eq!(zero.to_u64(), Some(0));
        assert_eq!(zero.to_highest_offset(), Offset64::of(-1));

        let one = Count64::of(1).expect("ok");
        assert_eq!(one.to_i64(), Some(1));
        assert_eq!(one.to_u64(), Some(1));
        assert_eq!(one.to_highest_offset(), Offset64::of(0));

        let max = Count64::MAX;
        assert_eq!(max, Count64::of(9223372036854775808).expect("ok"));
        assert_eq!(max.to_i64(), None); // unrepresentable
        assert_eq!(max.to_u64(), Some(9223372036854775808));
        assert_eq!(max.to_highest_offset(), Offset64::of(9223372036854775807));

        let min = Count64::MIN;
        assert_eq!(min, Count64::of(0).expect("ok"));
        assert_eq!(min.to_i64(), Some(0));
        assert_eq!(min.to_u64(), Some(0));
        assert_eq!(min.to_highest_offset(), Offset64::of(-1));
        assert_eq!(min.is_null(), false);
        assert_eq!(min.is_not_null(), true);

        let mut null = Count64::NULL;
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset64::of(-2));
        null.0 = 15; // doesn't matter which positive value we use, still null.
        assert_eq!(null.to_i64(), None);
        assert_eq!(null.to_u64(), None);
        assert_eq!(null.is_null(), true);
        assert_eq!(null.is_not_null(), false);
        assert_eq!(null.to_highest_offset(), Offset64::of(-16));
    }

    #[test]
    fn add() {
        assert_eq!(Count64::of(51).expect("ok") + Count64::NULL, Count64::NULL);
        assert_eq!(Count64::NULL + Count64::of(52).expect("ok"), Count64::NULL);

        assert_eq!(
            Count64::of(51).expect("ok") + 23,
            Count64::of(74).expect("ok")
        );
        assert_eq!(
            Count64::MAX + (-1),
            Count64::of(9223372036854775807).expect("ok")
        );
        assert_eq!(Count64::MAX + Count64::of(23).expect("ok"), Count64::NULL);
        assert_eq!(Count64::MIN + (-23), Count64::NULL);
        assert_eq!(
            Count64::MIN + Count64::of(1).expect("ok"),
            Count64::of(1).expect("ok")
        );

        assert_eq!(
            Count32::of(61).expect("ok") + 7,
            Count32::of(68).expect("ok")
        );
        assert_eq!(Count32::MAX + (-1), Count32::of(2147483647).expect("ok"));
        assert_eq!(Count32::MAX + Count32::of(1).expect("ok"), Count32::NULL);
        assert_eq!(Count32::MIN + (-1), Count32::NULL);
        assert_eq!(
            Count32::MIN + Count32::of(1).expect("ok"),
            Count32::of(1).expect("ok")
        );

        assert_eq!(
            Count16::of(77).expect("ok") + -4,
            Count16::of(73).expect("ok")
        );
        assert_eq!(Count16::MAX + (-1), Count16::of(32767).expect("ok"));
        assert_eq!(Count16::MAX + Count16::of(1).expect("ok"), Count16::NULL);
        assert_eq!(Count16::MIN + (-1), Count16::NULL);
        assert_eq!(
            Count16::MIN + Count16::of(1).expect("ok"),
            Count16::of(1).expect("ok")
        );

        assert_eq!(Count8::of(73).expect("ok") + 4, Count8::of(77).expect("ok"));
        assert_eq!(Count8::MAX + (-1), Count8::of(127).expect("ok"));
        assert_eq!(Count8::MAX + Count8::of(1).expect("ok"), Count8::NULL);
        assert_eq!(Count8::MIN + (-1), Count8::NULL);
        assert_eq!(
            Count8::MIN + Count8::of(1).expect("ok"),
            Count8::of(1).expect("ok")
        );
    }

    #[test]
    fn add_assign() {
        let mut sixty_four = Count64::NULL;
        sixty_four += 123;
        assert_eq!(sixty_four.is_null(), true);

        let mut eight = Count8::default();
        eight += 123;
        assert_eq!(eight, Count8::of(123).expect("ok"));
        eight += Count8::of(5).expect("ok");
        assert_eq!(eight, Count8::of(128).expect("ok"));
        eight += Count8::NULL;
        assert_eq!(eight, Count8::NULL);

        let mut thirty_two = Count32::MAX;
        thirty_two += Count32::of(1).expect("ok");
        assert_eq!(thirty_two, Count32::NULL);

        let mut sixteen = Count16::MIN;
        sixteen += -1;
        assert_eq!(sixteen, Count16::NULL);
    }

    #[test]
    fn sub() {
        assert_eq!(Count64::of(51).expect("ok") - Count64::NULL, Count64::NULL);
        assert_eq!(Count64::NULL - Count64::of(52).expect("ok"), Count64::NULL);

        assert_eq!(
            Count64::of(51).expect("ok") - 23,
            Count64::of(28).expect("ok")
        );
        assert_eq!(
            Count64::MAX - 1,
            Count64::of(9223372036854775807).expect("ok")
        );
        assert_eq!(Count64::MAX - (-1), Count64::NULL);
        assert_eq!(Count64::MIN - 7, Count64::NULL);
        assert_eq!(Count64::MIN - (-1), Count64::of(1).expect("ok"));

        assert_eq!(
            Count32::of(61).expect("ok") - 7,
            Count32::of(54).expect("ok")
        );
        assert_eq!(Count32::MAX - 1, Count32::of(2147483647).expect("ok"));
        assert_eq!(Count32::MAX - (-1), Count32::NULL);
        assert_eq!(Count32::MIN - 1, Count32::NULL);
        assert_eq!(Count32::MIN - (-1), Count32::of(1).expect("ok"));

        assert_eq!(
            Count16::of(53).expect("ok") - 4,
            Count16::of(49).expect("ok")
        );
        assert_eq!(Count16::MAX - 1, Count16::of(32767).expect("ok"));
        assert_eq!(Count16::MAX - (-1), Count16::NULL);
        assert_eq!(Count16::MIN - 1, Count16::NULL);
        assert_eq!(Count16::MIN - (-1), Count16::of(1).expect("ok"));

        assert_eq!(Count8::of(5).expect("ok") - 4, Count8::of(1).expect("ok"));
        assert_eq!(Count8::MAX - 1, Count8::of(127).expect("ok"));
        assert_eq!(Count8::MAX - (-1), Count8::NULL);
        assert_eq!(Count8::MIN - 17, Count8::NULL);
        assert_eq!(Count8::MIN - (-1), Count8::of(1).expect("ok"));
    }

    #[test]
    fn sub_assign() {
        let mut sixty_four = Count64::NULL;
        sixty_four -= 123;
        assert_eq!(sixty_four.is_null(), true);

        let mut eight = Count8::MAX;
        eight -= 123;
        assert_eq!(eight, Count8::of(5).expect("ok"));
        eight -= Count8::of(3).expect("ok");
        assert_eq!(eight, Count8::of(2).expect("ok"));
        eight -= Count8::NULL;
        assert_eq!(eight, Count8::NULL);

        let mut thirty_two = Count32::MIN;
        thirty_two -= 1;
        assert_eq!(thirty_two, Count32::NULL);

        let mut sixteen = Count16::MAX;
        sixteen -= -1;
        assert_eq!(sixteen, Count16::NULL);
    }

    #[test]
    fn double_or_at_least() {
        assert_eq!(
            Count8::negating(-10).double_or_at_least(5),
            Count8::negating(-20)
        );
        assert_eq!(
            Count16::negating(-100).double_or_at_least(5),
            Count16::negating(-200)
        );
        assert_eq!(
            Count32::negating(-1000).double_or_at_least(5),
            Count32::negating(-2000)
        );
        assert_eq!(
            Count64::negating(-10000).double_or_at_least(5),
            Count64::negating(-20000)
        );

        assert_eq!(
            Count8::negating(-1).double_or_at_least(5),
            Count8::negating(-5)
        );
        assert_eq!(
            Count16::negating(0).double_or_at_least(5),
            Count16::negating(-5)
        );
        assert_eq!(
            Count32::negating(-2).double_or_at_least(5),
            Count32::negating(-5)
        );
        assert_eq!(
            Count64::negating(-5).double_or_at_least(11),
            Count64::negating(-11)
        );

        // Near the max.
        assert_eq!(
            Count8::negating(-128).double_or_at_least(5),
            Count8::negating(-128)
        );
        assert_eq!(
            Count8::negating(-127).double_or_at_least(5),
            Count8::negating(-128)
        );
        assert_eq!(
            Count8::negating(-65).double_or_at_least(5),
            Count8::negating(-128)
        );

        assert_eq!(
            Count16::negating(-32768).double_or_at_least(5),
            Count16::negating(-32768)
        );
        assert_eq!(
            Count16::negating(-32767).double_or_at_least(5),
            Count16::negating(-32768)
        );
        assert_eq!(
            Count16::negating(-16385).double_or_at_least(5),
            Count16::negating(-32768)
        );
    }

    #[test]
    fn contains_offset() {
        assert_eq!(
            Count8::negating(-10).contains(Contains8::Offset(Offset8::of(-1))),
            false
        );
        assert_eq!(
            Count8::negating(-10).contains(Contains8::Offset(Offset8::of(0))),
            true
        );
        assert_eq!(
            Count8::negating(-10).contains(Contains8::Offset(Offset8::of(3))),
            true
        );
        assert_eq!(
            Count8::negating(-10).contains(Contains8::Offset(Offset8::of(9))),
            true
        );
        assert_eq!(
            Count8::negating(-10).contains(Contains8::Offset(Offset8::of(10))),
            false
        );
        assert_eq!(
            Count8::negating(-10).contains(Contains8::Offset(Offset8::of(100))),
            false
        );

        assert_eq!(
            Count16::negating(-10).contains(Contains16::Offset(Offset16::of(-1))),
            false
        );
        assert_eq!(
            Count16::negating(-10).contains(Contains16::Offset(Offset16::of(0))),
            true
        );
        assert_eq!(
            Count16::negating(-10).contains(Contains16::Offset(Offset16::of(3))),
            true
        );
        assert_eq!(
            Count16::negating(-10).contains(Contains16::Offset(Offset16::of(9))),
            true
        );
        assert_eq!(
            Count16::negating(-10).contains(Contains16::Offset(Offset16::of(10))),
            false
        );
        assert_eq!(
            Count16::negating(-10).contains(Contains16::Offset(Offset16::of(100))),
            false
        );

        assert_eq!(
            Count32::negating(0).contains(Contains32::Offset(Offset32::of(-1))),
            false
        );
        assert_eq!(
            Count32::negating(0).contains(Contains32::Offset(Offset32::of(0))),
            false
        );

        // count is null:
        assert_eq!(
            Count64::negating(1).contains(Contains64::Offset(Offset64::of(0))),
            false
        );
    }

    #[test]
    fn contains_index() {
        assert_eq!(
            Count8::negating(0).contains(Contains8::Index(Index8::of(-1))),
            false
        );
        assert_eq!(
            Count8::negating(0).contains(Contains8::Index(Index8::of(0))),
            false
        );

        // count is null:
        assert_eq!(
            Count16::negating(1).contains(Contains16::Index(Index16::of(0))),
            false
        );

        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(-11))), // wrap around, too far
            false
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(-10))), // wrap around
            true
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(-1))), // wrap around
            true
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(0))),
            true
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(3))),
            true
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(9))),
            true
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(10))),
            false
        );
        assert_eq!(
            Count32::negating(-10).contains(Contains32::Index(Index32::of(100))),
            false
        );

        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(-11))), // wrap around, too far
            false
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(-10))), // wrap around
            true
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(-1))), // wrap around
            true
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(0))),
            true
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(3))),
            true
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(9))),
            true
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(10))),
            false
        );
        assert_eq!(
            Count64::negating(-10).contains(Contains64::Index(Index64::of(100))),
            false
        );
    }

    #[test]
    fn ordering() {
        assert_eq!(
            Count64::of(30).expect("ok") < Count64::of(31).expect("ok"),
            true
        );
        assert_eq!(
            Count32::of(30).expect("ok") < Count32::of(30).expect("ok"),
            false
        );
        assert_eq!(
            Count16::of(30).expect("ok") <= Count16::of(30).expect("ok"),
            true
        );
        assert_eq!(
            Count8::of(31).expect("ok") <= Count8::of(30).expect("ok"),
            false
        );
    }
}
