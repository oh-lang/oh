use crate::core::aligned::*;
use crate::core::allocation::*;
use crate::core::likely::*;
use crate::core::moot::*;
use crate::core::non_local_array::*;
use crate::core::offset::*;

pub use crate::core::array::*;

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

// TODO: it would be nice to create MaybeLocalArrayCountX structs which
// do not go to the max_array type.  Optimized64 would be an alias to Count64
// because there's no reason to go to max_array with a 64-bit type.
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

impl<S: SignedPrimitive, const N_LOCAL: usize, T> GetCount<i64>
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    fn count(&self) -> CountMax {
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
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T: Default> SetCount<i64>
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    type Error = ContainerError;

    fn set_count(&mut self, new_count: Count<i64>) -> Containered {
        let old_count = self.count();
        if new_count < old_count {
            for _ in 0..(old_count - new_count).to_usize() {
                _ = self.remove(OrderedRemove::Last);
            }
        } else if new_count > old_count {
            if new_count > self.capacity() {
                self.set_capacity(new_count)?;
            }
            for _ in 0..(new_count - old_count).to_usize() {
                self.insert_at_end(Default::default())
                    .expect("already allocated enough above");
            }
        }
        return Ok(());
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

    /// Can end up dropping elements if `new_capacity < self.count()`.
    /// Note that if `new_capacity < N_LOCAL`, elements will be dropped
    /// but capacity will be set to `N_LOCAL` (i.e., using the `unallocated_buffer`).
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
            let was_present = self.remove(OrderedRemove::Last).is_some();
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

    fn grow_to_at_least(&mut self, required_capacity: CountMax) -> Containered {
        let desired_capacity = self
            .capacity()
            .double_or_at_least(required_capacity)
            .map_err(|_| ContainerError::OutOfMemory)?;
        self.set_capacity(desired_capacity)
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

    pub fn remove(&mut self, remove: OrderedRemove) -> Option<T> {
        match remove {
            OrderedRemove::Last => self.remove_last(),
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

    pub(crate) fn insert_at_end(&mut self, value: T) -> Containered {
        let previous_capacity = self.capacity();
        let previous_count = self.count();
        let new_count = previous_count + 1;
        if new_count.is_null() {
            // Probably shouldn't happen with i64 as the backing integer,
            // but might be possible if we allow 32bit architectures.
            return ContainerError::OutOfMemory.err();
        }
        if new_count > previous_capacity {
            self.grow_to_at_least(new_count)?;
        }
        unsafe { self.append_with_required_capacity_ready(new_count, value) };
        Ok(())
    }

    unsafe fn append_with_required_capacity_ready(&mut self, new_count: CountMax, value: T) {
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
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T: Default + TryClone> Array<T>
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    fn len(&self) -> usize {
        self.count().to_usize()
    }

    /// Looking for `fn add(t)` or `fn append(t)`?  use `insert(OrderedInsert::AtEnd(t))`:
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

impl<S: SignedPrimitive, const N_LOCAL: usize, T: Default + TryClone>
    MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
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
            // Probably shouldn't happen with i64 as the backing integer,
            // but might be possible if we allow 32bit architectures.
            return ContainerError::OutOfMemory.err();
        }
        if new_count > previous_capacity {
            self.grow_to_at_least(new_count)?;
        }
        let mut intermediate_count = previous_count;
        for i in 0..values.size() {
            intermediate_count += 1;
            let value = values.nab(i).map_err(|_| ContainerError::Unknown)?;
            unsafe {
                self.append_with_required_capacity_ready(intermediate_count, value);
            }
        }
        Ok(())
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

// TODO: we should have a better ArrayEquality trait.
impl<S: SignedPrimitive, const N_LOCAL: usize, T: std::cmp::PartialEq> PartialEq<Self>
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    fn eq(&self, other: &Self) -> bool {
        let count = self.count();
        if count != other.count() {
            return false;
        }
        for i in 0..count.to_usize() {
            let i = i as usize;
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }
}

// TODO: we should add a non-debug Array formatter which just uses [] and not the type.
impl<S: SignedPrimitive, const N_LOCAL: usize, T: std::fmt::Debug> std::fmt::Debug
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MaybeLocalArrayOptimized{}::<{}, _>::from([",
            S::BITS,
            N_LOCAL
        )?;
        for i in 0..self.count().to_usize() {
            write!(f, "{:?}, ", self[i])?;
        }
        write!(f, "])")
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T: std::cmp::Eq> Eq
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T: TryClone> TryClone
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    // TODO: this should probably be one_of(ContainerError, <T as TryClone>::Error)
    type Error = ContainerError;

    fn try_clone(&self) -> Result<Self, ContainerError> {
        let mut result = Self::default();
        // Only need to clone up to `count`, not the full `capacity`.
        let count = self.count();
        result.set_capacity(count)?;
        for i in 0..count.to_usize() {
            let clone = self[i].try_clone().map_err(|_| ContainerError::Unknown)?;
            result
                .insert_at_end(clone)
                .expect("already at necessary capacity");
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::testing::*;

    #[test]
    fn insert_and_remove_unallocated_buffer() {
        let mut array = MaybeLocalArrayOptimized64::<16, u8>::default();
        assert_eq!(array.capacity(), Count::of(16).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        array
            .set_capacity(Count::of(3).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.capacity(), Count::of(16).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        array
            .insert(OrderedInsert::AtEnd(1))
            .expect("already allocked");
        array
            .insert(OrderedInsert::AtEnd(2))
            .expect("already allocked");
        array
            .insert(OrderedInsert::AtEnd(3))
            .expect("already allocked");
        assert_eq!(array.count(), Count::of(3).expect("ok"));
        array
            .insert_few(OrderedInsertFew::AtEnd(
                &mut [4, 5, 6][..],
                TypeMarker,
                TypeMarker,
            ))
            .expect("ok");
        assert_eq!(array.count(), Count::of(6).expect("ok"));
        assert_eq!(array.remove(OrderedRemove::Last), Some(6));
        assert_eq!(array.remove(OrderedRemove::Last), Some(5));
        assert_eq!(array.remove(OrderedRemove::Last), Some(4));
        assert_eq!(array.remove(OrderedRemove::Last), Some(3));
        assert_eq!(array.remove(OrderedRemove::Last), Some(2));
        assert_eq!(array.remove(OrderedRemove::Last), Some(1));
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        assert_eq!(array.capacity(), Count::of(16).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
    }

    #[test]
    fn insert_and_remove_optimized_allocation() {
        let mut array = MaybeLocalArrayOptimized32::<8, u8>::default();
        assert_eq!(array.capacity(), Count::of(8).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        array
            .set_capacity(Count::of(100).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.capacity(), Count::of(100).expect("ok"));
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        for i in 0..95 {
            array
                .insert(OrderedInsert::AtEnd(i as u8))
                .expect("already allocked");
        }
        array
            .insert_few(OrderedInsertFew::AtEnd(
                &mut [95, 96, 97, 98, 99][..],
                TypeMarker,
                TypeMarker,
            ))
            .expect("ok");
        assert_eq!(array.count(), Count::of(100).expect("ok"));
        assert_eq!(array.capacity(), Count::of(100).expect("ok"));
        for j in 0..100 {
            let i = 99 - j;
            assert_eq!(array.remove(OrderedRemove::Last), Some(i as u8));
            assert_eq!(array.count(), Count::of(i).expect("ok"));
        }
        assert_eq!(array.capacity(), Count::of(100).expect("ok"));
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
    }

    #[test]
    fn insert_and_remove_max_array() {
        let mut array = MaybeLocalArrayOptimized8::<17, u8>::default();
        assert_eq!(array.capacity(), Count::of(17).expect("ok"));
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        array
            .set_capacity(Count::of(200).expect("ok"))
            .expect("small alloc");
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.count(), Count::of(0).expect("ok"));
        for i in 0..199 {
            array
                .insert(OrderedInsert::AtEnd(i as u8))
                .expect("already allocked");
        }
        array
            .insert_few(OrderedInsertFew::AtEnd(
                &mut [199][..],
                TypeMarker,
                TypeMarker,
            ))
            .expect("ok");
        assert_eq!(array.count(), Count::of(200).expect("ok"));
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        for j in 0..200 {
            let i = 199 - j;
            assert_eq!(array.remove(OrderedRemove::Last), Some(i as u8));
            assert_eq!(array.count(), Count::of(i).expect("ok"));
        }
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.memory(), Memory::MaxArray);
    }

    // ================================
    // Starting with unallocated_buffer
    // ================================
    #[test]
    fn set_capacity_truncating_unallocated_buffer() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized64::<13, u8>::default();
        for i in 0..7 {
            array
                .insert(OrderedInsert::AtEnd(19 + i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(13).expect("ok"));
        assert_eq!(array.count(), Count::of(7).expect("ok"));

        array.set_capacity(Count::of(5).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.count(), Count::of(5).expect("ok"));
        assert_eq!(array.capacity(), Count::of(13).expect("ok"));
        for i in 0..5 {
            assert_eq!(array[i], 19 + i as u8);
        }
    }

    #[test]
    fn set_capacity_expanding_unallocated_buffer() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized32::<13, u8>::default();
        for i in 0..8 {
            array
                .insert(OrderedInsert::AtEnd(29 + i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(13).expect("ok"));
        assert_eq!(array.count(), Count::of(8).expect("ok"));

        array.set_capacity(Count::of(13).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(13).expect("ok"));
        assert_eq!(array.count(), Count::of(8).expect("ok"));
        for i in 0..8 {
            assert_eq!(array[i], 29 + i as u8);
        }
    }

    #[test]
    fn set_capacity_expanding_unallocated_buffer_to_optimized_allocation() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized64::<13, u8>::default();
        for i in 0..13 {
            array
                .insert(OrderedInsert::AtEnd(200 + i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(13).expect("ok"));
        assert_eq!(array.count(), Count::of(13).expect("ok"));

        array.set_capacity(Count::of(28).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(28).expect("ok"));
        assert_eq!(array.count(), Count::of(13).expect("ok"));
        for i in 0..13 {
            assert_eq!(array[i], 200 + i as u8);
        }
    }

    #[test]
    fn set_capacity_expanding_unallocated_buffer_to_max_array() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<13, u8>::default();
        for i in 0..12 {
            array
                .insert(OrderedInsert::AtEnd(200 + i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(13).expect("ok"));
        assert_eq!(array.count(), Count::of(12).expect("ok"));

        array.set_capacity(Count::of(200).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.count(), Count::of(12).expect("ok"));
        for i in 0..12 {
            assert_eq!(array[i], 200 + i as u8);
        }
    }

    // ==================================
    // Starting with optimized_allocation
    // ==================================
    #[test]
    fn set_capacity_truncating_optimized_allocation_to_unallocated_buffer() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized16::<10, u8>::default();
        array
            .set_capacity(Count::of(50).expect("ok"))
            .expect("small alloc");
        for i in 0..50 {
            array
                .insert(OrderedInsert::AtEnd(49 - i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(50).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));

        array.set_capacity(Count::of(10).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(10).expect("ok"));
        assert_eq!(array.count(), Count::of(10).expect("ok"));
        for i in 0..10 {
            assert_eq!(array[i], 49 - i as u8);
        }
    }

    #[test]
    fn set_capacity_truncating_optimized_allocation() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized16::<10, u8>::default();
        array
            .set_capacity(Count::of(50).expect("ok"))
            .expect("small alloc");
        for i in 0..50 {
            array
                .insert(OrderedInsert::AtEnd(49 - i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(50).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));

        array.set_capacity(Count::of(11).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(11).expect("ok"));
        assert_eq!(array.count(), Count::of(11).expect("ok"));
        for i in 0..11 {
            assert_eq!(array[i], 49 - i as u8);
        }
    }

    #[test]
    fn set_capacity_expanding_optimized_allocation() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized16::<10, u8>::default();
        array
            .set_capacity(Count::of(50).expect("ok"))
            .expect("small alloc");
        for i in 0..50 {
            array
                .insert(OrderedInsert::AtEnd(49 - i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(50).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));

        array.set_capacity(Count::of(123).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(123).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));
        for i in 0..50 {
            assert_eq!(array[i], 49 - i as u8);
        }
    }

    #[test]
    fn set_capacity_expanding_optimized_allocation_to_max_array() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<10, u8>::default();
        array
            .set_capacity(Count::of(50).expect("ok"))
            .expect("small alloc");
        for i in 0..50 {
            array
                .insert(OrderedInsert::AtEnd(49 - i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(50).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));

        array.set_capacity(Count::of(130).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(130).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));
        for i in 0..50 {
            assert_eq!(array[i], 49 - i as u8);
        }
    }

    // =======================
    // Starting with max_array
    // =======================
    #[test]
    fn set_capacity_truncating_max_array() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<5, u8>::default();
        array
            .set_capacity(Count::of(200).expect("ok"))
            .expect("small alloc");
        for i in 0..200 {
            array
                .insert(OrderedInsert::AtEnd(3 + i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.count(), Count::of(200).expect("ok"));

        array.set_capacity(Count::of(150).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(150).expect("ok"));
        assert_eq!(array.count(), Count::of(150).expect("ok"));
        for i in 0..150 {
            assert_eq!(array[i], 3 + i as u8);
        }
    }

    #[test]
    fn set_capacity_expanding_max_array() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<15, u8>::default();
        array
            .set_capacity(Count::of(150).expect("ok"))
            .expect("small alloc");
        for i in 0..150 {
            array
                .insert(OrderedInsert::AtEnd(8 + i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(150).expect("ok"));
        assert_eq!(array.count(), Count::of(150).expect("ok"));

        array.set_capacity(Count::of(200).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(200).expect("ok"));
        assert_eq!(array.count(), Count::of(150).expect("ok"));
        for i in 0..150 {
            assert_eq!(array[i], 8 + i as u8);
        }
    }

    #[test]
    fn set_capacity_truncating_big_max_array_to_unallocated_buffer() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<5, u8>::default();
        array
            .set_capacity(Count::of(192).expect("ok"))
            .expect("small alloc");
        for i in 0..192 {
            array
                .insert(OrderedInsert::AtEnd(i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(192).expect("ok"));
        assert_eq!(array.count(), Count::of(192).expect("ok"));

        array.set_capacity(Count::of(4).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(5).expect("ok")); // set_capacity is semi-ignored...
        assert_eq!(array.count(), Count::of(4).expect("ok")); // note count is less than capacity
        for i in 0..4 {
            assert_eq!(array[i], i as u8);
        }
    }

    #[test]
    fn set_capacity_truncating_small_max_array_to_unallocated_buffer() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<6, u8>::default();
        array
            .set_capacity(Count::of(192).expect("ok"))
            .expect("small alloc");
        for i in 0..3 {
            array
                .insert(OrderedInsert::AtEnd(100 - i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(192).expect("ok"));
        assert_eq!(array.count(), Count::of(3).expect("ok"));

        array.set_capacity(Count::of(2).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::UnallocatedBuffer);
        assert_eq!(array.capacity(), Count::of(6).expect("ok")); // set_capacity is semi-ignored...
        assert_eq!(array.count(), Count::of(2).expect("ok")); // note count is less than capacity
        for i in 0..2 {
            assert_eq!(array[i], 100 - i as u8);
        }
    }

    #[test]
    fn set_capacity_truncating_big_max_array_to_optimized_allocation() {
        // TODO: use `Noisy` instead of `u8` so that we can verify they get freed.
        let mut array = MaybeLocalArrayOptimized8::<10, u8>::default();
        array
            .set_capacity(Count::of(150).expect("ok"))
            .expect("small alloc");
        for i in 0..50 {
            array
                .insert(OrderedInsert::AtEnd(49 - i as u8))
                .expect("already allocked");
        }
        assert_eq!(array.memory(), Memory::MaxArray);
        assert_eq!(array.capacity(), Count::of(150).expect("ok"));
        assert_eq!(array.count(), Count::of(50).expect("ok"));

        array.set_capacity(Count::of(11).expect("ok")).expect("ok");

        assert_eq!(array.memory(), Memory::OptimizedAllocation);
        assert_eq!(array.capacity(), Count::of(11).expect("ok"));
        assert_eq!(array.count(), Count::of(11).expect("ok"));
        for i in 0..11 {
            assert_eq!(array[i], 49 - i as u8);
        }
    }

    #[test]
    fn clone_adds_all_values() {
        let mut array = MaybeLocalArrayOptimized32::<4, TestingNoisy>::default();
        array.set_capacity(Count::of(5).expect("ok")).expect("ok");
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
        testing_unprint(vec![
            Vec::from(b"create(A: 5)"),
            Vec::from(b"noisy_new(1)"),
            Vec::from(b"noisy_new(100)"),
            Vec::from(b"noisy_new(40)"),
            Vec::from(b"noisy_new(-771)"),
            Vec::from(b"noisy_new(6)"),
        ]);

        let clone = array.try_clone().expect("ok");
        assert_eq!(clone.count(), Count::of(5).expect("ok"));
        assert_eq!(clone.capacity(), Count::of(5).expect("ok"));
        assert_eq!(array, clone);
        assert_eq!(clone[1].value(), 100);
        assert_eq!(clone[2].value(), 40);
        assert_eq!(clone[3].value(), -771);
        testing_unprint(vec![
            Vec::from(b"create(B: 5)"),
            Vec::from(b"noisy_clone(1)"),
            Vec::from(b"noisy_clone(100)"),
            Vec::from(b"noisy_clone(40)"),
            Vec::from(b"noisy_clone(-771)"),
            Vec::from(b"noisy_clone(6)"),
        ]);
    }

    #[test]
    fn set_count_works_for_same_count() {
        let mut array = MaybeLocalArrayOptimized32::<4, TestingNoisy>::default();
        array.set_count(Count::of(4).expect("ok")).expect("ok");
        assert_eq!(array.count(), Count::of(4).expect("ok"));

        testing_unprint(vec![
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
        ]);

        array.set_count(Count::of(4).expect("ok")).expect("ok");
        assert_eq!(array.count(), Count::of(4).expect("ok"));
        testing_unprint(vec![]);

        array
            .insert(OrderedInsert::AtEnd(TestingNoisy::new(77)))
            .expect("ok");
        testing_unprint(vec![
            Vec::from(b"noisy_new(77)"),
            Vec::from(b"create(A: 8)"),
        ]);
        assert_eq!(array.capacity(), Count::of(8).expect("ok"));

        array.set_count(Count::of(5).expect("ok")).expect("ok");
        assert_eq!(array.count(), Count::of(5).expect("ok"));
        assert_eq!(array.capacity(), Count::of(8).expect("ok")); // doesn't change this.
        testing_unprint(vec![]);
    }

    #[test]
    fn set_count_works_to_expand_count() {
        let mut array = MaybeLocalArrayOptimized32::<4, TestingNoisy>::default();
        array.set_count(Count::of(10).expect("ok")).expect("ok");
        assert_eq!(array.count(), Count::of(10).expect("ok"));
        assert_eq!(array.capacity(), Count::of(10).expect("ok"));

        testing_unprint(vec![
            Vec::from(b"create(A: 10)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
            Vec::from(b"noisy_new(256)"),
        ]);
    }

    #[test]
    fn set_count_works_to_truncate_count() {
        let mut array = MaybeLocalArrayOptimized32::<4, TestingNoisy>::default();
        array.set_capacity(Count::of(10).expect("ok")).expect("ok");
        for i in 1..=10 {
            array
                .insert(OrderedInsert::AtEnd(TestingNoisy::new(i as i32)))
                .expect("ok");
        }
        assert_eq!(array.count(), Count::of(10).expect("ok"));
        _ = testing_prints(); // ignore noise before truncation

        array.set_count(Count::of(5).expect("ok")).expect("ok");
        assert_eq!(array.capacity(), Count::of(10).expect("ok")); // doesn't change this.

        testing_unprint(vec![
            Vec::from(b"noisy_drop(10)"),
            Vec::from(b"noisy_drop(9)"),
            Vec::from(b"noisy_drop(8)"),
            Vec::from(b"noisy_drop(7)"),
            Vec::from(b"noisy_drop(6)"),
        ]);
    }
}
