use crate::core::aligned::*;
use crate::core::allocation::*;
use crate::core::container::*;
use crate::core::count::*;
use crate::core::likely::*;
use crate::core::non_local_array::*;
use crate::core::number::*;
use crate::core::offset::*;
use crate::core::signed::*;
use crate::core::traits::*;

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

pub type MaybeLocalArrayOptimized64<const N_LOCAL: usize, T> =
    MaybeLocalArrayOptimized<i64, N_LOCAL, T>;
pub type MaybeLocalArrayOptimized32<const N_LOCAL: usize, T> =
    MaybeLocalArrayOptimized<i32, N_LOCAL, T>;
pub type MaybeLocalArrayOptimized16<const N_LOCAL: usize, T> =
    MaybeLocalArrayOptimized<i16, N_LOCAL, T>;
pub type MaybeLocalArrayOptimized8<const N_LOCAL: usize, T> =
    MaybeLocalArrayOptimized<i8, N_LOCAL, T>;

/// Array that can store up to `N_LOCAL` elements locally (before
/// needing to allocate).  After allocating, the local elements
/// are wasted space here, so try to use only if elements are small
/// (measured by `std::mem::size_of::<T>()`) and fit well in the
/// additional space opened up by `S` being smaller.  This array
/// is optimized for holding up to `Count::<S>::MAX` elements, but
/// can hold more (if `S != i64`) by allocating a pointer to a
/// `NonLocalArrayMax`.
#[repr(C, align(8))]
pub struct MaybeLocalArrayOptimized<S: SignedPrimitive, const N_LOCAL: usize, T> {
    maybe_allocated: MaybeAllocated<S, N_LOCAL, T>,
    /// If negative or zero, then use `maybe_allocated.optimized_allocation`
    ///     and use it as `Count::<S>::negated(special_count)`.
    /// If positive:
    ///     if less than `N_LOCAL + 1`, use `maybe_allocated.unallocated_buffer`
    ///     and consider it as `Count::<S>::negated(-(special_count - 1))`
    ///     otherwise: use `maybe_allocated.max_array`.
    special_count: S,
}

#[repr(C, packed)]
union MaybeAllocated<S: SignedPrimitive, const N_LOCAL: usize, T> {
    unused: usize,
    unallocated_buffer: [ManuallyDrop<T>; N_LOCAL],
    optimized_allocation: ManuallyDrop<AllocationCount<S, T>>,
    /// This is a bit inefficient because it's a pointer
    /// to a nonlocal array (another pointer).
    max_array: ManuallyDrop<Box<NonLocalArrayMax<T>>>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Memory {
    UnallocatedBuffer,
    OptimizedAllocation,
    MaxArray,
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T> Default
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    fn default() -> Self {
        Self {
            maybe_allocated: MaybeAllocated { unused: 0 },
            special_count: Self::UNALLOCATED_ZERO_SPECIAL_COUNT,
        }
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T> Drop for MaybeLocalArrayOptimized<S, N_LOCAL, T> {
    fn drop(&mut self) {
        match self.memory() {
            Memory::UnallocatedBuffer => {} // Nothing to do, stack allocated
            Memory::OptimizedAllocation => {
                let ptr =
                    unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.set_capacity(Count::<S>::default())
                    .expect("should be able to drop");
            }
            Memory::MaxArray => {
                let ptr = unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.set_capacity(CountMax::default())
                    .expect("should be able to drop");
            }
        }
        // Not really needed for a drop but useful for a move-reset.
        self.special_count = Self::UNALLOCATED_ZERO_SPECIAL_COUNT;
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T> MaybeLocalArrayOptimized<S, N_LOCAL, T> {
    const UNALLOCATED_ZERO_SPECIAL_COUNT: S = S::ONE;

    fn max_array_count() -> S {
        S::from(N_LOCAL + 2)
            .expect("need N_LOCAL+2 as a special value to distinguish the max_array case")
    }

    #[inline]
    fn memory(&self) -> Memory {
        if self.special_count <= S::ZERO {
            Memory::OptimizedAllocation
        } else if self.special_count < Self::max_array_count() {
            Memory::UnallocatedBuffer
        } else {
            cold();
            Memory::MaxArray
        }
    }

    pub fn count(&self) -> CountMax {
        match self.memory() {
            Memory::UnallocatedBuffer => {
                Count::<S>::negating(-(self.special_count - Self::UNALLOCATED_ZERO_SPECIAL_COUNT))
                    .to_max()
            }
            Memory::OptimizedAllocation => Count::<S>::negating(self.special_count).to_max(),
            Memory::MaxArray => {
                // This is needed because the union is packed, but Rust isn't
                // smart enough to know that `self` is aligned and therefore the union is.
                let ptr = unsafe { std::ptr::addr_of!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &*ptr }; // Creating a reference (OK because we're aligned)
                ptr.count()
            }
        }
    }

    pub(crate) fn only_set_count(&mut self, new_count: CountMax) {
        debug_assert!(new_count <= self.capacity());
        match self.memory() {
            Memory::UnallocatedBuffer => {
                self.set_count_unallocated_buffer(new_count);
            }
            Memory::OptimizedAllocation => self.set_count_optimized_allocation(new_count),
            Memory::MaxArray => {
                self.set_count_max_array(new_count);
            }
        }
    }

    #[inline]
    fn set_count_unallocated_buffer(&mut self, new_count: CountMax) {
        self.special_count =
            Self::UNALLOCATED_ZERO_SPECIAL_COUNT + S::from(new_count.to_usize()).expect("ok");
    }

    #[inline]
    fn set_count_optimized_allocation(&mut self, new_count: CountMax) {
        self.special_count = S::from(new_count.as_negated()).expect("ok");
    }

    #[inline]
    fn set_count_max_array(&mut self, new_count: CountMax) {
        self.special_count = Self::max_array_count();

        let ptr = unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
        let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
        ptr.count = new_count;
    }

    pub fn capacity(&self) -> CountMax {
        match self.memory() {
            Memory::UnallocatedBuffer => CountMax::of(N_LOCAL).expect("ok"),
            Memory::OptimizedAllocation => {
                let ptr = unsafe { std::ptr::addr_of!(self.maybe_allocated.optimized_allocation) };
                let ptr = unsafe { &*ptr }; // Creating a reference (OK because we're aligned)
                ptr.capacity().to_max()
            }
            Memory::MaxArray => {
                let ptr = unsafe { std::ptr::addr_of!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &*ptr }; // Creating a reference (OK because we're aligned)
                ptr.capacity()
            }
        }
    }

    /// Can end up dropping elements if `new_capacity < self.count()`
    pub fn set_capacity(&mut self, new_capacity: CountMax) -> Containered {
        let new_capacity = if new_capacity.is_null() {
            CountMax::default()
        } else {
            new_capacity
        };
        let current_memory = self.memory();
        let required_memory = Self::required_memory(new_capacity);
        // First check if we need to remove any elements.
        let mut count = self.count();
        debug_assert!(count.is_not_null());
        while new_capacity < count {
            let was_present = self.remove(Remove::Last).is_some();
            debug_assert!(was_present);
            count -= 1;
        }
        // TODO: maybe switch to a `match (current_memory, required_memory) { (Unallocated, Optimized) => {...} ...}`
        // which should make the transition more readable and less nested.
        if current_memory == required_memory {
            match current_memory {
                Memory::UnallocatedBuffer => Ok(()), // no-op, already OK
                Memory::OptimizedAllocation => {
                    let ptr = unsafe {
                        std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation)
                    };
                    let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                    ptr.set_capacity(Count::<S>::of(new_capacity.to_usize()).expect("OK"))
                }
                Memory::MaxArray => {
                    let ptr = unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                    let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                    ptr.set_capacity(new_capacity)
                }
            }
        } else {
            match required_memory {
                Memory::UnallocatedBuffer => {
                    match current_memory {
                        Memory::UnallocatedBuffer => {
                            panic!("already taken care of");
                        }
                        Memory::OptimizedAllocation => {
                            // Dropping from OptimizedAllocation to UnallocatedBuffer:
                            let from_allocation = unsafe {
                                std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation)
                            }
                                as *mut AllocationCount<S, T>;
                            self.copy_bytes_locally_and_release_allocation(from_allocation, count);
                        }
                        Memory::MaxArray => {
                            // Dropping from MaxArray to UnallocatedBuffer:
                            let max_array =
                                unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                            let max_array = unsafe { &mut *max_array }; // Creating a reference (OK because we're aligned)
                            // Need to grab the `Box` so it will get freed at the end of this block.
                            let mut max_array = unsafe { ManuallyDrop::take(max_array) };
                            // Because we're going into the allocation and messing around with things directly,
                            // make sure we don't free the copied `T`s:
                            max_array.count = Count::default();
                            self.copy_bytes_locally_and_release_allocation(
                                &mut max_array.allocation,
                                count,
                            );
                        }
                    }
                    self.set_count_unallocated_buffer(count);
                    Ok(())
                }
                Memory::OptimizedAllocation => {
                    // Need to make a local allocation first to copy values into,
                    // then move the allocation into place; otherwise we'd obliterate
                    // values we need from max_array or unallocated_buffer.
                    let mut new_allocation = Aligned(AllocationCount::<S, T>::default());
                    // TODO: we need a `Count::try_into` for another S2.
                    new_allocation.set_capacity(Count::negating(
                        S::from(new_capacity.as_negated()).expect("ok"),
                    ))?;
                    match current_memory {
                        Memory::UnallocatedBuffer => {
                            Self::copy_bytes(
                                unsafe {
                                    std::ptr::addr_of!(self.maybe_allocated.unallocated_buffer[0])
                                        as *const T // ManuallyDrop is a thin wrapper around T
                                },
                                std::ptr::addr_of_mut!(new_allocation[0]),
                                count,
                            );
                        }
                        Memory::OptimizedAllocation => {
                            panic!("already taken care of");
                        }
                        Memory::MaxArray => {
                            // Dropping from MaxArray to OptimizedArray:
                            let max_array =
                                unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                            let max_array = unsafe { &mut *max_array }; // Creating a reference (OK because we're aligned)
                            // Need to grab the `Box` so it will get freed at the end of this block.
                            let mut max_array = unsafe { ManuallyDrop::take(max_array) };
                            // Because we're going into the max_array and messing around with things directly,
                            // make sure we don't free the copied `T`s:
                            max_array.count = Count::default();
                            Self::copy_bytes_and_release_allocation(
                                &mut max_array.allocation,
                                std::ptr::addr_of_mut!(new_allocation[0]),
                                count,
                            );
                        }
                    }
                    self.maybe_allocated.optimized_allocation =
                        ManuallyDrop::new(new_allocation.unalign());
                    self.set_count_optimized_allocation(count);
                    Ok(())
                }
                Memory::MaxArray => {
                    // Need to make a max array first to copy values into,
                    // then move the allocation into place; otherwise we'd obliterate
                    // values we need from max_array or unallocated_buffer.
                    let mut new_array = Box::new(NonLocalArrayMax::<T>::default());
                    new_array.set_capacity(new_capacity)?;
                    match current_memory {
                        Memory::UnallocatedBuffer => {
                            Self::copy_bytes(
                                unsafe {
                                    std::ptr::addr_of!(self.maybe_allocated.unallocated_buffer[0])
                                        as *const T // ManuallyDrop is a thin wrapper around T
                                },
                                std::ptr::addr_of_mut!(new_array.allocation[0]),
                                count,
                            );
                        }
                        Memory::OptimizedAllocation => {
                            // Dropping from OptimizedAllocation to UnallocatedBuffer:
                            let from_allocation = unsafe {
                                std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation)
                            }
                                as *mut AllocationCount<S, T>;
                            Self::copy_bytes_and_release_allocation(
                                from_allocation,
                                std::ptr::addr_of_mut!(new_array.allocation[0]),
                                count,
                            );
                        }
                        Memory::MaxArray => {
                            panic!("already taken care of");
                        }
                    }
                    self.maybe_allocated.max_array = ManuallyDrop::new(new_array);
                    self.set_count_max_array(count);
                    Ok(())
                }
            }
        }
    }

    fn copy_bytes_locally_and_release_allocation<S2: SignedPrimitive>(
        &mut self,
        allocation: *mut AllocationCount<S2, T>,
        count: CountMax,
    ) {
        let to = unsafe {
            std::ptr::addr_of_mut!(self.maybe_allocated.unallocated_buffer[0]) as *mut T // Assume ManuallyDrop is a thin wrapper around T
        };
        Self::copy_bytes_and_release_allocation(allocation, to, count);
    }

    fn copy_bytes_and_release_allocation<S2: SignedPrimitive>(
        allocation: *mut AllocationCount<S2, T>,
        to: *mut T,
        count: CountMax,
    ) {
        // In order to copy the bytes, we need to make a local copy of the allocation.
        // This is because the current allocation's location shares bytes with the `to` buffer.
        let allocation = unsafe { &mut *allocation }; // Creating a reference (OK because we're aligned)
        Self::copy_bytes(std::ptr::addr_of!(allocation[0]), to, count);
        // Manually drop the old allocation:
        allocation.set_capacity(Count::default()).expect("ok");
    }

    #[inline]
    fn copy_bytes(from: *const T, to: *mut T, count: CountMax) {
        let count = count.to_usize();
        unsafe {
            std::ptr::copy_nonoverlapping(from, to, count);
        }
    }

    fn required_memory(for_count: CountMax) -> Memory {
        let for_count = for_count.to_usize();
        if for_count <= N_LOCAL {
            Memory::UnallocatedBuffer
        } else if Count::<S>::of(for_count).is_ok() {
            Memory::OptimizedAllocation
        } else {
            cold();
            Memory::MaxArray
        }
    }

    /// Looking for `fn add`?  use `append`:
    pub fn append(&mut self, value: T) -> Containered {
        let previous_capacity = self.capacity();
        let previous_count = self.count();
        let new_count = previous_count + 1;
        if new_count.is_null() {
            // Probably shouldn't happen with i64 as the backing integer,
            // but might be possible if we allow 32bit architectures.
            return ContainerError::OutOfMemory.err();
        }
        if new_count > previous_capacity {
            self.grow_from(previous_capacity)?;
        }
        let offset = new_count.to_highest_offset();
        match self.memory() {
            Memory::UnallocatedBuffer => {
                let ptr = unsafe {
                    std::ptr::addr_of_mut!(
                        self.maybe_allocated.unallocated_buffer[offset.to_inner() as usize]
                    )
                };
                // Destructively write here, don't drop existing value, it wasn't initialized properly anyway.
                unsafe { std::ptr::write(ptr, ManuallyDrop::new(value)) };
            }
            Memory::OptimizedAllocation => {
                let ptr =
                    unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.write_initializing(Offset::of(S::from(offset.to_inner()).expect("OK")), value)
                    .expect("should have the correct capacity");
            }
            Memory::MaxArray => {
                let ptr = unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.allocation
                    .write_initializing(offset, value)
                    .expect("should have the correct capacity");
            }
        }
        self.only_set_count(new_count);
        return Ok(());
    }

    fn grow_from(&mut self, current_capacity: CountMax) -> Containered {
        let desired_capacity = current_capacity.double_or_at_least((N_LOCAL * 2).min(1) as i64);
        if desired_capacity <= current_capacity {
            return ContainerError::OutOfMemory.err();
        }
        self.set_capacity(desired_capacity)
    }

    fn grow(&mut self) -> Containered {
        self.grow_from(self.capacity())
    }

    /// Some of the elements in the slice might NOT be initialized.  You've been warned.
    /// Things should be initialized up to `count()` (see `deref()` implementation).
    pub(crate) fn fully_allocated_slice(&self) -> &[T] {
        match self.memory() {
            Memory::UnallocatedBuffer => {
                let ptr = unsafe { std::ptr::addr_of!(self.maybe_allocated.unallocated_buffer[0]) };
                // We'll assume ManuallyDrop<T> is a very thin wrapper around T.
                unsafe { std::slice::from_raw_parts(ptr as *const T, N_LOCAL) }
            }
            Memory::OptimizedAllocation => {
                let ptr = unsafe { std::ptr::addr_of!(self.maybe_allocated.optimized_allocation) };
                let ptr = unsafe { &*ptr }; // Creating a reference (OK because we're aligned)
                ptr.deref()
            }
            Memory::MaxArray => {
                let ptr = unsafe { std::ptr::addr_of!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &*ptr }; // Creating a reference (OK because we're aligned)
                ptr.allocation.deref()
            }
        }
    }

    /// Some of the elements in the slice might NOT be initialized.  You've been warned.
    /// Things should be initialized up to `count()` (see `deref_mut()` implementation).
    pub(crate) fn fully_allocated_slice_mut(&mut self) -> &mut [T] {
        match self.memory() {
            Memory::UnallocatedBuffer => {
                let ptr =
                    unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.unallocated_buffer[0]) };
                // We'll assume ManuallyDrop<T> is a very thin wrapper around T.
                unsafe { std::slice::from_raw_parts_mut(ptr as *mut T, N_LOCAL) }
            }
            Memory::OptimizedAllocation => {
                let ptr =
                    unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.deref_mut()
            }
            Memory::MaxArray => {
                let ptr = unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.allocation.deref_mut()
            }
        }
    }

    pub fn remove(&mut self, remove: Remove) -> Option<T> {
        match remove {
            Remove::Last => self.remove_last(),
        }
    }

    pub(crate) fn remove_last(&mut self) -> Option<T> {
        let count = self.count();
        if count == CountMax::default() {
            return None;
        }
        let offset = count.to_highest_offset();
        // This function doesn't update any buffers (or change `self.memory()`),
        // so we're still ok to grab the element at `offset` later in this method.
        self.only_set_count(count - CountMax::negating(-1));
        let result = match self.memory() {
            Memory::UnallocatedBuffer => {
                let ptr = unsafe {
                    std::ptr::addr_of_mut!(
                        self.maybe_allocated.unallocated_buffer[offset.0 as usize]
                    )
                };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                unsafe { std::mem::ManuallyDrop::take(ptr) }
            }
            Memory::OptimizedAllocation => {
                let ptr =
                    unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.optimized_allocation) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.read_destructively(Offset::<S>::of(S::from(offset.to_inner()).expect("OK")))
                    .expect("OK")
            }
            Memory::MaxArray => {
                let ptr = unsafe { std::ptr::addr_of_mut!(self.maybe_allocated.max_array) };
                let ptr = unsafe { &mut *ptr }; // Creating a reference (OK because we're aligned)
                ptr.allocation.read_destructively(offset).expect("OK")
            }
        };
        Some(result)
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T> std::ops::Deref
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.fully_allocated_slice()[0..self.count().to_usize()]
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T> std::ops::DerefMut
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    fn deref_mut(&mut self) -> &mut [T] {
        let count = self.count().to_usize();
        &mut self.fully_allocated_slice_mut()[0..count]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn append_and_remove_unallocated_buffer() {
        let mut array = MaybeLocalArrayOptimized64::<16, u8>::default();
        assert_eq!(array.capacity(), Count::of(16).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        array
            .set_capacity(Count::of(3).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.capacity(), Count::of(16).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        array.append(1).expect("already allocked");
        array.append(2).expect("already allocked");
        array.append(3).expect("already allocked");
        assert_eq!(array.count(), Count::of(3).expect("ok"));
        assert_eq!(array.remove(Remove::Last), Some(3));
        assert_eq!(array.remove(Remove::Last), Some(2));
        assert_eq!(array.remove(Remove::Last), Some(1));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        assert_eq!(array.capacity(), Count::of(16).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
    }

    #[test]
    fn append_and_remove_optimized_allocation() {
        let mut array = MaybeLocalArrayOptimized32::<8, u8>::default();
        assert_eq!(array.capacity(), Count::of(8).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        array
            .set_capacity(Count::of(100).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.capacity(), Count::of(100).expect("ok"));
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        for i in 0..100 {
            array.append(i as u8).expect("already allocked");
        }
        assert_eq!(array.count(), Count::of(100).expect("ok"));
        assert_eq!(array.capacity(), Count::of(100).expect("ok"));
        for j in 0..100 {
            let i = 99 - j;
            assert_eq!(array.remove(Remove::Last), Some(i as u8));
            assert_eq!(array.count(), Count::of(i).expect("ok"));
        }
        assert_eq!(array.capacity(), Count::of(100).expect("ok"));
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
    }

    #[test]
    fn append_and_remove_max_array() {
        let mut array = MaybeLocalArrayOptimized8::<17, u8>::default();
        assert_eq!(array.capacity(), Count::of(17).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        array
            .set_capacity(Count::of(200).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        for i in 0..200 {
            array.append(i as u8).expect("already allocked");
        }
        assert_eq!(array.count(), Count::of(200).expect("ok"));
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        for j in 0..200 {
            let i = 199 - j;
            assert_eq!(array.remove(Remove::Last), Some(i as u8));
            assert_eq!(array.count(), Count::of(i).expect("ok"));
        }
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.memory(), Memory::MaxArray);
    }
}
