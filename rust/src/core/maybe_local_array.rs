use crate::core::allocation::*;
use crate::core::container::*;
use crate::core::count::*;
use crate::core::likely::*;
use crate::core::non_local_array::*;
use crate::core::number::*;
use crate::core::signed::*;
use crate::core::traits::*;

use std::mem::ManuallyDrop;
use std::ops::Deref;

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
        if self.special_count <= S::ZERO {
            // optimized_allocation case
            Count::<S>::negating(self.special_count).to_max()
        } else if self.special_count < Self::max_array_count() {
            // unallocated_buffer case
            Count::<S>::negating(-(self.special_count - Self::UNALLOCATED_ZERO_SPECIAL_COUNT))
                .to_max()
        } else {
            cold();
            unsafe {
                // This is needed because the union is packed, but Rust isn't
                // smart enough to know that `self` is aligned and therefore the union is.
                let ptr = std::ptr::addr_of!(self.maybe_allocated.max_array);
                let ptr = unsafe { &*ptr }; // Creating a reference (OK because we're aligned)
                ptr.count()
            }
        }
    }

    /// Some of the elements in the slice might NOT be initialized.  You've been warned.
    /// Things should be initialized up to `count()`.
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
}

impl<S: SignedPrimitive, const N_LOCAL: usize, T> std::ops::Deref
    for MaybeLocalArrayOptimized<S, N_LOCAL, T>
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.fully_allocated_slice()[0..self.count().to_usize()]
    }
}

//impl<S: SignedPrimitive, const N_LOCAL: usize, T> std::ops::DerefMut for MaybeLocalArrayOptimized<S, N_LOCAL, T> {
//    fn deref_mut(&mut self) -> &mut [T] {
//        let count = self.count().into();
//        &mut self.fully_allocated_slice_mut()[0..count]
//    }
//}
