use crate::core::allocation::*;

pub use crate::core::array::*;

/// The largest array that this platform can support,
/// in terms of max memory it can hold.
// TODO: on a 32 bit platform, go to NonLocalArrayCount32 instead.
pub type NonLocalArrayMax<T> = NonLocalArrayCount64<T>;

pub type NonLocalArrayCount64<T> = NonLocalArrayCount<i64, T>;
pub type NonLocalArrayCount32<T> = NonLocalArrayCount<i32, T>;
pub type NonLocalArrayCount16<T> = NonLocalArrayCount<i16, T>;
pub type NonLocalArrayCount8<T> = NonLocalArrayCount<i8, T>;

// TODO: it would be nice to create NonLocalArrayOptimizedX structs which
// switch to a max_array type if count is null.  Optimized64 would be an alias to Count64
// because there's no reason to go to max_array with a 64-bit type.
/// Array that can store only up to `Count::<S>::MAX` elements.
/// If there are more than that, e.g., if S = i8 and count == 128,
/// then more insertions will fail.  Changing `S` has marginal
/// impact on this array size, and allows making indices more compact.
/// It is "non-local" because it always stores elements on the heap.
#[repr(C, align(8))]
pub struct NonLocalArrayCount<S: SignedPrimitive, T> {
    pub(crate) allocation: AllocationCount<S, T>,
    pub(crate) count: Count<S>,
}

impl<S: SignedPrimitive, T> Default for NonLocalArrayCount<S, T> {
    fn default() -> Self {
        Self {
            allocation: AllocationCount::<S, T>::default(),
            count: Count::<S>::default(),
        }
    }
}

// TODO: implement Hash
impl<S: SignedPrimitive, T: Default + TryClone> Array<T> for NonLocalArrayCount<S, T> {
    fn len(&self) -> usize {
        self.count().to_usize()
    }

    /// Looking for `fn add(t)` or `fn append(t)`?  use `insert(OrderedInsert::AtEnd(t))`
    // TODO: add test for getting to end of S::MAX
    fn insert(&mut self, insert: OrderedInsert<T>) -> Containered {
        match insert {
            OrderedInsert::AtEnd(t) => self.insert_at_end(t),
        }
    }

    /// Looking for `fn add_all(ts)` or `fn append_all(ts)`?
    /// use `insert_few(OrderedInsertFew::AtEnd(ts), TypeMarker, TypeMarker)`:
    fn insert_few<E, Values: Few<T, Error = E>>(
        &mut self,
        insert: OrderedInsertFew<T, E, Values>,
    ) -> Containered {
        match insert {
            OrderedInsertFew::AtEnd(f, ..) => self.insert_few_at_end(f),
        }
    }
}

impl<S: SignedPrimitive, T: Default + TryClone> NonLocalArrayCount<S, T> {
    pub(crate) fn insert_few_at_end<E, Values: Few<T, Error = E>>(
        &mut self,
        mut values: Values,
    ) -> Containered {
        // TODO: we need tests for this and `insert_at_end` which ensure that we grow capacity
        // correctly.  i.e., we don't always double capacity and we don't always just reallocate
        // to the next amount we need (we double only when we'd run out of space).
        let previous_capacity = self.capacity();
        let previous_count = self.count();
        let new_count =
            previous_count + Count::of(values.size()).map_err(|_| ContainerError::OutOfMemory)?;
        if new_count.is_null() {
            return ContainerError::OutOfMemory.err();
        }
        if new_count > previous_capacity {
            self.grow_to_at_least(new_count)?;
        }
        let mut intermediate_count = previous_count;
        for i in 0..values.size() {
            intermediate_count += S::ONE;
            let value = values.nab(i).map_err(|_| ContainerError::Unknown)?;
            unsafe {
                self.append_with_required_capacity_ready(intermediate_count, value);
            }
        }
        Ok(())
    }
}

impl<S: SignedPrimitive, T> NonLocalArrayCount<S, T> {
    fn insert_at_end(&mut self, t: T) -> Containered {
        let new_count = self.count + S::ONE;
        if new_count.is_null() {
            return ContainerError::OutOfMemory.err();
        }
        if new_count > self.capacity() {
            self.grow_to_at_least(new_count)?;
        }
        unsafe {
            self.append_with_required_capacity_ready(new_count, t);
        }
        return Ok(());
    }

    unsafe fn append_with_required_capacity_ready(&mut self, new_count: Count<S>, value: T) {
        self.allocation
            .write_initializing(new_count.to_highest_offset(), value)
            .expect("should be in bounds");
        self.count = new_count;
    }

    // TODO: a `removeAll` method that returns an iterator to the elements getting removed.
    // e.g., for converting into a count, etc.

    /// Looking for `fn pop`? use `remove(OrderedRemove::Last)`
    pub fn remove(&mut self, remove: OrderedRemove) -> Option<T> {
        match remove {
            OrderedRemove::Last => self.remove_last(),
        }
    }

    pub(crate) fn remove_last(&mut self) -> Option<T> {
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

    #[inline]
    pub fn capacity(&self) -> Count<S> {
        self.allocation.capacity()
    }

    /// Will reallocate to exactly this capacity.
    /// Will delete items if `new_capacity < self.count()`
    pub fn set_capacity(&mut self, new_capacity: Count<S>) -> Containered {
        if new_capacity == self.capacity() {
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

    fn grow_to_at_least(&mut self, required_capacity: Count<S>) -> Containered {
        let desired_capacity = self
            .capacity()
            .double_or_at_least(required_capacity)
            .map_err(|_| ContainerError::OutOfMemory)?;
        self.set_capacity(desired_capacity)
    }
}

impl<S: SignedPrimitive, T> GetCount<S> for NonLocalArrayCount<S, T> {
    fn count(&self) -> Count<S> {
        self.count
    }
}

impl<S: SignedPrimitive, T: std::default::Default> SetCount<S> for NonLocalArrayCount<S, T> {
    type Error = ContainerError;

    fn set_count(&mut self, new_count: Count<S>) -> Containered {
        if new_count < self.count {
            while self.count > new_count {
                _ = self.remove(OrderedRemove::Last);
            }
        } else if new_count > self.count {
            if new_count > self.capacity() {
                self.set_capacity(new_count)?;
            }
            while self.count < new_count {
                self.insert_at_end(Default::default())
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
    // TODO: this should probably be one_of(ContainerError, <T as TryClone>::Error)
    type Error = ContainerError;

    fn try_clone(&self) -> Result<Self, ContainerError> {
        let mut result = Self::default();
        // Only need to clone up to `count`, not the full `capacity`.
        result.set_capacity(self.count())?;
        for i in 0..self.count.to_usize() {
            let clone = self[i].try_clone().map_err(|_| ContainerError::Unknown)?;
            result
                .insert_at_end(clone)
                .expect("already at necessary capacity");
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::testing::*;

    use std::ops::{Deref, DerefMut};

    #[test]
    fn can_deref_an_empty_array() {
        let array = NonLocalArrayCount8::<u8>::default();
        let slice = array.deref();
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn can_deref_mut_an_empty_array() {
        let mut array = NonLocalArrayCount8::<u8>::default();
        let slice = array.deref_mut();
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn drop_frees_allocation() {
        {
            let mut array = NonLocalArrayCount8::<TestingNoisy>::default();
            array
                .set_capacity(Count::of(7).expect("ok"))
                .expect("small alloc");
            testing_unprint(vec![Vec::from("create(A: 7)")]);
            for i in 1..=7 {
                array
                    .insert(OrderedInsert::AtEnd(TestingNoisy::new(i as i32)))
                    .expect("already allocked");
            }
            assert_eq!(array.count(), Count::of(7).expect("ok"));
            testing_unprint(vec![
                Vec::from("noisy_new(1)"),
                Vec::from("noisy_new(2)"),
                Vec::from("noisy_new(3)"),
                Vec::from("noisy_new(4)"),
                Vec::from("noisy_new(5)"),
                Vec::from("noisy_new(6)"),
                Vec::from("noisy_new(7)"),
            ]);
        }
        testing_unprint(vec![
            Vec::from("noisy_drop(7)"),
            Vec::from("noisy_drop(6)"),
            Vec::from("noisy_drop(5)"),
            Vec::from("noisy_drop(4)"),
            Vec::from("noisy_drop(3)"),
            Vec::from("noisy_drop(2)"),
            Vec::from("noisy_drop(1)"),
            Vec::from("delete(A)"),
        ]);
    }

    #[test]
    fn insert_and_remove() {
        let mut array = NonLocalArrayCount64::<u32>::default();
        array
            .set_capacity(Count::of(6).expect("ok"))
            .expect("small alloc");
        array
            .insert(OrderedInsert::AtEnd(1))
            .expect("already allocked");
        array
            .insert(OrderedInsert::AtEnd(2))
            .expect("already allocked");
        array
            .insert(OrderedInsert::AtEnd(3))
            .expect("already allocked");
        array
            .insert_few(OrderedInsertFew::AtEnd(
                &mut [4, 5, 6][..],
                TypeMarker,
                TypeMarker,
            ))
            .expect("ok");
        assert_eq!(array.count(), Count::of(6).expect("ok"));
        assert_eq!(array.capacity(), Count::of(6).expect("ok"));
        assert_eq!(array.remove(OrderedRemove::Last), Some(6));
        assert_eq!(array.remove(OrderedRemove::Last), Some(5));
        assert_eq!(array.remove(OrderedRemove::Last), Some(4));
        assert_eq!(array.remove(OrderedRemove::Last), Some(3));
        assert_eq!(array.remove(OrderedRemove::Last), Some(2));
        assert_eq!(array.remove(OrderedRemove::Last), Some(1));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        assert_eq!(array.capacity(), Count::of(6).expect("ok"));
    }

    #[test]
    fn set_count_supplies_defaults() {
        let mut array = NonLocalArrayCount16::<u32>::default();
        array
            .set_count(Count::of(5).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.count(), Count::of(5).expect("ok"));
        assert_eq!(array.remove(OrderedRemove::Last), Some(0));
        assert_eq!(array.remove(OrderedRemove::Last), Some(0));
        assert_eq!(array.remove(OrderedRemove::Last), Some(0));
        assert_eq!(array.remove(OrderedRemove::Last), Some(0));
        assert_eq!(array.remove(OrderedRemove::Last), Some(0));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
    }

    #[test]
    fn clear_keep_capacity() {
        let mut array = NonLocalArrayCount8::<TestingNoisy>::default();
        array
            .set_capacity(Count::of(10).expect("ok"))
            .expect("small alloc");
        testing_unprint(vec![Vec::from("create(A: 10)")]);
        array
            .set_count(Count::of(5).expect("ok"))
            .expect("already allocked");
        assert_eq!(array.capacity(), Count::of(10).expect("ok"));
        assert_eq!(array.count(), Count::of(5).expect("ok"));
        testing_unprint(vec![
            Vec::from("noisy_new(256)"),
            Vec::from("noisy_new(256)"),
            Vec::from("noisy_new(256)"),
            Vec::from("noisy_new(256)"),
            Vec::from("noisy_new(256)"),
        ]);

        array.clear(Clear::KeepingCapacity);

        assert_eq!(array.capacity(), Count::of(10).expect("ok"));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        testing_unprint(vec![
            Vec::from("noisy_drop(256)"),
            Vec::from("noisy_drop(256)"),
            Vec::from("noisy_drop(256)"),
            Vec::from("noisy_drop(256)"),
            Vec::from("noisy_drop(256)"),
        ]);
    }

    #[test]
    fn clone_adds_all_values() {
        let mut array = NonLocalArrayCount32::<TestingNoisy>::default();
        array
            .insert(OrderedInsert::AtEnd(TestingNoisy::new(1)))
            .expect("ok");
        array
            .insert(OrderedInsert::AtEnd(TestingNoisy::new(100)))
            .expect("ok");
        array
            .insert(OrderedInsert::AtEnd(TestingNoisy::new(40)))
            .expect("ok");
        array
            .insert(OrderedInsert::AtEnd(TestingNoisy::new(-771)))
            .expect("ok");
        array
            .insert(OrderedInsert::AtEnd(TestingNoisy::new(6)))
            .expect("ok");
        assert_eq!(array.capacity(), Count::of(8).expect("ok"));
        testing_unprint(vec![
            Vec::from(b"noisy_new(1)"),
            Vec::from(b"create(A: 1)"),
            Vec::from(b"noisy_new(100)"),
            Vec::from(b"delete(A)"),
            Vec::from(b"create(B: 2)"),
            Vec::from(b"noisy_new(40)"),
            Vec::from(b"delete(B)"),
            Vec::from(b"create(C: 4)"),
            Vec::from(b"noisy_new(-771)"),
            Vec::from(b"noisy_new(6)"),
            Vec::from(b"delete(C)"),
            Vec::from(b"create(D: 8)"),
        ]);

        let clone = array.try_clone().expect("ok");
        assert_eq!(clone.count(), Count::of(5).expect("ok"));
        assert_eq!(clone.capacity(), Count::of(5).expect("ok"));
        assert_eq!(array, clone);
        assert_eq!(clone[1].value(), 100);
        assert_eq!(clone[2].value(), 40);
        assert_eq!(clone[3].value(), -771);
        testing_unprint(vec![
            Vec::from(b"create(E: 5)"),
            Vec::from(b"noisy_clone(1)"),
            Vec::from(b"noisy_clone(100)"),
            Vec::from(b"noisy_clone(40)"),
            Vec::from(b"noisy_clone(-771)"),
            Vec::from(b"noisy_clone(6)"),
        ]);
    }
}
