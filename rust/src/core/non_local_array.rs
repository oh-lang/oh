use crate::core::allocation::*;
use crate::core::container::*;
use crate::core::count::*;
use crate::core::signed::*;
use crate::core::traits::*;

/// The largest array that this platform can support,
/// in terms of max memory it can hold.
// TODO: on a 32 bit platform, go to NonLocalArrayCount32 instead.
pub type NonLocalArrayMax<T> = NonLocalArrayCount64<T>;

pub type NonLocalArrayCount64<T> = NonLocalArrayCount<i64, T>;
pub type NonLocalArrayCount32<T> = NonLocalArrayCount<i32, T>;
pub type NonLocalArrayCount16<T> = NonLocalArrayCount<i16, T>;
pub type NonLocalArrayCount8<T> = NonLocalArrayCount<i8, T>;

/// Array that can store only up to `Count::<S>::MAX` elements.
/// If there are more than that, e.g., if S = i8 and count == 128,
/// then more insertions will fail.  Changing `S` has marginal
/// impact on this array size, and allows making indices more compact.
/// It is "non-local" because it always stores elements on the heap.
// TODO: we probably could relax the `align(8)` restriction but we
// could only support offsets that get `ptr` (inside `allocation`)
// aligned on a word boundary, e.g., `ptr, capacity, count` or `count, ptr, capacity`.
#[repr(C, align(8))]
pub struct NonLocalArrayCount<S: SignedPrimitive, T> {
    allocation: AllocationCount<S, T>,
    count: Count<S>,
}

impl<S: SignedPrimitive, T> Default for NonLocalArrayCount<S, T> {
    fn default() -> Self {
        Self {
            allocation: AllocationCount::<S, T>::default(),
            count: Count::<S>::default(),
        }
    }
}

// TODO: implement #[derive(Debug, Hash)]
impl<S: SignedPrimitive, T> NonLocalArrayCount<S, T> {
    // TODO: this should be a Countable Trait
    pub fn count(&self) -> Count<S> {
        return self.count;
    }

    /// Looking for `fn add`?  use `append`:
    pub fn append(&mut self, value: T) -> Containered {
        if self.allocation.capacity() == self.count {
            self.grow()?;
        }
        self.count += S::ONE;
        debug_assert!(self.count.is_not_null());
        self.allocation
            .write_initializing(self.count.to_highest_offset(), value)
            .expect("should be in bounds");
        return Ok(());
    }

    // TODO: a `removeAll` method that returns an iterator to the elements getting removed.
    // e.g., for converting into a count, etc.

    /// Looking for `fn pop`? use `remove(Remove::Last)`
    pub fn remove(&mut self, remove: Remove) -> Option<T> {
        match remove {
            Remove::Last => self.remove_last(),
        }
    }

    fn remove_last(&mut self) -> Option<T> {
        if self.count <= Count::<S>::default() {
            return None;
        }
        let result = self
            .allocation
            .read_destructively(self.count.to_highest_offset())
            .expect("should be in bounds");
        self.count -= S::ONE;
        debug_assert!(self.count.is_not_null());
        Some(result)
    }

    pub fn clear(&mut self, options: Clear) {
        match options {
            Clear::KeepingCapacity => {
                // We could optimize this but we do need Rust to drop each individual
                // element (if necessary), so we can't just dealloc the `ptr` itself.
                while let Some(_) = self.remove_last() {}
            }
            Clear::DroppingCapacity => self
                .set_capacity(Count::<S>::default())
                .expect("clearing should not alloc"),
        }
        assert!(self.count == Count::<S>::default());
    }

    pub fn capacity(&self) -> Count<S> {
        self.allocation.capacity()
    }

    /// Will reallocate to exactly this capacity.
    /// Will delete items if `new_capacity < self.count()`
    pub fn set_capacity(&mut self, new_capacity: Count<S>) -> Containered {
        if new_capacity == self.allocation.capacity() {
            return Ok(());
        }
        while self.count > new_capacity {
            // We could optimize this but we do need Rust to drop each individual
            // element (if necessary), so we can't just dealloc the `ptr` itself.
            if self.remove_last().is_none() {
                // Could happen if new_capacity < 0
                break;
            }
        }
        self.allocation.set_capacity(new_capacity)
    }

    pub fn grow(&mut self) -> Containered {
        self.allocation.grow()
    }
}

impl<S: SignedPrimitive, T: std::default::Default> NonLocalArrayCount<S, T> {
    // TODO: this should be a Countable Trait for default-constructable T
    pub fn set_count(&mut self, new_count: Count<S>) -> Containered {
        if new_count < self.count {
            while self.count > new_count {
                _ = self.remove(Remove::Last);
            }
        } else if new_count > self.count {
            if new_count > self.allocation.capacity() {
                self.set_capacity(new_count)?;
            }
            while self.count < new_count {
                self.append(Default::default())
                    .expect("already allocated enough above");
            }
        }
        return Ok(());
    }
}

impl<S: SignedPrimitive, T> std::ops::Deref for NonLocalArrayCount<S, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.allocation[0..self.count.to_usize()]
    }
}

impl<S: SignedPrimitive, T> std::ops::DerefMut for NonLocalArrayCount<S, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.allocation[0..self.count.to_usize()]
    }
}

impl<S: SignedPrimitive, T: std::cmp::PartialEq> PartialEq<Self> for NonLocalArrayCount<S, T> {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            return false;
        }
        for i in 0..self.count.to_usize() {
            let i = i as usize;
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }
}

impl<S: SignedPrimitive, T: std::cmp::Eq> Eq for NonLocalArrayCount<S, T> {}

impl<S: SignedPrimitive, T: TryClone> TryClone for NonLocalArrayCount<S, T> {
    type Error = ContainerError;

    fn try_clone(&self) -> Result<Self, ContainerError> {
        let mut result = Self::default();
        // Only need to clone up to `count`, not the full `capacity`.
        result.set_capacity(self.count())?;
        for i in 0..self.count.to_usize() {
            let clone = self[i].try_clone().map_err(|_| ContainerError::Unknown)?;
            result.append(clone).expect("already at necessary capacity");
        }
        Ok(result)
    }
}

impl<S: SignedPrimitive, T: std::fmt::Debug> std::fmt::Debug for NonLocalArrayCount<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NonLocalArrayCount{}::<_>::from([", S::BITS)?;
        for i in 0..self.count.to_usize() {
            write!(f, "{:?}, ", self[i])?;
        }
        write!(f, "])")
    }
}

impl<S: SignedPrimitive, T> Drop for NonLocalArrayCount<S, T> {
    fn drop(&mut self) {
        self.clear(Clear::DroppingCapacity);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Clear {
    #[default]
    KeepingCapacity,
    DroppingCapacity,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Remove {
    #[default]
    Last,
    // TODO
    //First,
    //Index(Index),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Sort {
    #[default]
    Default,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn append_and_remove() {
        let mut array = NonLocalArrayCount64::<u32>::default();
        array
            .set_capacity(Count::of(3).expect("ok"))
            .expect("small alloc");
        array.append(1).expect("already allocked");
        array.append(2).expect("already allocked");
        array.append(3).expect("already allocked");
        assert_eq!(array.count(), Count::of(3).expect("ok"));
        assert_eq!(array.remove(Remove::Last), Some(3));
        assert_eq!(array.remove(Remove::Last), Some(2));
        assert_eq!(array.remove(Remove::Last), Some(1));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        assert_eq!(array.capacity(), Count::of(3).expect("ok"));
    }

    #[test]
    fn set_count_supplies_defaults() {
        let mut array = NonLocalArrayCount16::<u32>::default();
        array
            .set_count(Count::of(5).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.count(), Count::of(5).expect("ok"));
        assert_eq!(array.remove(Remove::Last), Some(0));
        assert_eq!(array.remove(Remove::Last), Some(0));
        assert_eq!(array.remove(Remove::Last), Some(0));
        assert_eq!(array.remove(Remove::Last), Some(0));
        assert_eq!(array.remove(Remove::Last), Some(0));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
    }

    #[test]
    fn clear_keep_capacity() {
        // TODO: switch to noisy
        let mut array = NonLocalArrayCount8::<u8>::default();
        array
            .set_capacity(Count::of(10).expect("ok"))
            .expect("small alloc");
        // TODO
        assert_eq!(array.capacity(), Count::of(10).expect("ok"));
    }

    #[test]
    fn clone_adds_all_values() {
        let mut array = NonLocalArrayCount32::<i32>::default();
        array.append(1).expect("ok");
        array.append(100).expect("ok");
        array.append(40).expect("ok");
        array.append(-771).expect("ok");
        array.append(6).expect("ok");
        assert_eq!(array.capacity(), Count::of(14).expect("ok"));

        let clone = array.try_clone().expect("ok");
        assert_eq!(clone.count(), Count::of(5).expect("ok"));
        assert_eq!(clone.capacity(), Count::of(5).expect("ok"));
        assert_eq!(array, clone);
        assert_eq!(clone[1], 100);
        assert_eq!(clone[2], 40);
        assert_eq!(clone[3], -771);
    }
}
