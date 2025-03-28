use crate::core::container::*;
use crate::core::count::*;
use crate::core::likely::*;
use crate::core::number::*;
use crate::core::offset::*;
use crate::core::signed::*;
use crate::core::testing::*;

use std::alloc;

pub type AllocationCount64<T> = AllocationCount<i64, T>;
pub type AllocationCount32<T> = AllocationCount<i32, T>;
pub type AllocationCount16<T> = AllocationCount<i16, T>;
pub type AllocationCount8<T> = AllocationCount<i8, T>;

/// Low-level structure that has a pointer to contiguous memory,
/// with a capacity up to Count::<S>::MAX elements.
/// You need to keep track of which elements are initialized, etc.
/// Because of that, you need to MANUALLY drop this allocation after
/// freeing any initialized elements, by calling `set_capacity(Count::of(0))`
/// WARNING! because this is packed, you may need to wrap it in `Aligned(...)`
/// in order to ensure that `ptr` is on an aligned boundary.
#[repr(C, packed)]
pub struct AllocationCount<S: SignedPrimitive, T> {
    ptr: *mut T,
    capacity: Count<S>,
}

impl<S: SignedPrimitive, T> Default for AllocationCount<S, T> {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            capacity: Count::<S>::default(),
        }
    }
}

// TODO: allow passing in an allocator, defaulting to Global.
// https://doc.rust-lang.org/std/alloc/trait.Allocator.html
// pass up through Array, etc.
impl<S: SignedPrimitive, T> AllocationCount<S, T> {
    pub fn capacity(&self) -> Count<S> {
        self.capacity
    }

    /// Caller MUST ensure that they've already dropped elements that you might delete here
    /// if the new capacity is less than the old.  The old capacity will be updated
    /// iff the capacity change succeeds.
    pub fn set_capacity(&mut self, new_capacity: Count<S>) -> Containered {
        let old_capacity = self.capacity;
        let old_ptr = self.as_ptr_mut() as *mut u8;
        if !new_capacity.is_positive() {
            if old_capacity > Count::<S>::default() {
                unsafe {
                    alloc::dealloc(
                        old_ptr,
                        Self::layout_of(old_capacity).expect("already allocked"),
                    );
                }
                testing_unname_pointer(old_ptr);
                self.ptr = std::ptr::null_mut();
                self.capacity = Count::<S>::default();
            }
            return Ok(());
        } else if new_capacity == old_capacity {
            return Ok(());
        }
        let new_layout = Self::layout_of(new_capacity)?;
        let new_ptr = unsafe {
            if old_capacity.is_positive() {
                alloc::realloc(
                    old_ptr,
                    Self::layout_of(old_capacity).expect("already allocked"),
                    new_layout.size(),
                )
            } else {
                alloc::alloc(new_layout)
            }
        } as *mut T;
        if new_ptr != std::ptr::null_mut() {
            if old_ptr != std::ptr::null_mut() {
                // We unname first in case the allocator re-uses the same location
                // (in which case re-naming it would panic), even though conceptually
                // we create the new pointer and copy over the data to the new pointer
                // before deleting the old pointer.
                testing_unname_pointer(old_ptr);
            }
            testing_name_pointer(TestingPointer::Count(new_ptr, new_capacity));
            self.ptr = new_ptr;
            self.capacity = new_capacity;
            Ok(())
        } else {
            cold();
            ContainerError::OutOfMemory.err()
        }
    }

    /// Writes to an offset that should *not* be initialized until *after* this call.
    /// I.e., it is uninitialized before calling this.
    pub fn write_initializing(&mut self, offset: Offset<S>, value: T) -> Containered {
        let capacity = self.capacity;
        if !capacity.contains(Contains::Offset(offset)) {
            return ContainerError::InvalidAt.err();
        }
        unsafe {
            std::ptr::write(self.as_ptr_mut().add(offset.0.as_() as usize), value);
        }
        Ok(())
    }

    /// Reads at the offset, and from now on, that offset should be considered
    /// uninitialized.
    pub fn read_destructively(&mut self, offset: Offset<S>) -> ContainerResult<T> {
        let capacity = self.capacity;
        if !capacity.contains(Contains::<S>::Offset(offset)) {
            return Err(ContainerError::InvalidAt);
        }
        Ok(unsafe { std::ptr::read(self.as_ptr_mut().add(offset.0.as_() as usize)) })
    }

    pub fn grow(&mut self) -> Containered {
        let desired_capacity = Self::roughly_double_capacity(self.capacity())
            .map_err(|_| ContainerError::OutOfMemory)?;
        self.set_capacity(desired_capacity)
    }

    fn roughly_double_capacity(capacity: Count<S>) -> NumberResult<Count<S>> {
        if capacity.is_positive() {
            // If someone has purposely done `set_capacity(1)`, then we won't
            // do any fancy logic here.
            capacity.double_or_at_least(Count::negating(-S::ONE))
        } else {
            // When starting out, we allocate more for smaller (in size) T.
            // This interpolates from starting_alloc = 32 down to 1 over a large range.
            let t_bytes = std::mem::size_of::<T>() as i64;
            let multiplier = (8 - (t_bytes / 16)).max(1);
            let desired_total_bytes = 24 + t_bytes * multiplier;
            let desired_total_words = desired_total_bytes / 8;
            let starting_alloc = ((desired_total_words * 8) / t_bytes).max(1);
            capacity.double_or_at_least(Count::negating(-S::from(starting_alloc).unwrap()))
        }
    }

    fn layout_of(capacity: Count<S>) -> ContainerResult<alloc::Layout> {
        if let Some(capacity) = capacity.to_u64() {
            alloc::Layout::array::<T>(capacity as usize).or(Err(ContainerError::OutOfMemory))
        } else {
            cold();
            Err(ContainerError::InvalidAt)
        }
    }

    fn as_ptr(&self) -> *const T {
        self.ptr
    }

    fn as_ptr_mut(&mut self) -> *mut T {
        self.ptr
    }
}

impl<S: SignedPrimitive, T> std::ops::Deref for AllocationCount<S, T> {
    type Target = [T];
    /// Caller is responsible for only accessing initialized values.
    fn deref(&self) -> &[T] {
        let capacity = self.capacity.to_usize();
        if capacity > 0 {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), capacity) }
        } else {
            cold();
            &[]
        }
    }
}

impl<S: SignedPrimitive, T> std::ops::DerefMut for AllocationCount<S, T> {
    /// Caller is responsible for only accessing initialized values.
    fn deref_mut(&mut self) -> &mut [T] {
        let capacity = self.capacity.to_usize();
        if capacity > 0 {
            unsafe { std::slice::from_raw_parts_mut(self.as_ptr_mut(), capacity) }
        } else {
            cold();
            &mut []
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::aligned::*;

    use std::ops::{Deref, DerefMut};

    #[test]
    fn can_deref_an_empty_allocation() {
        let allocation = AllocationCount64::<u64>::default();
        let slice = allocation.deref();
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn can_deref_mut_an_empty_allocation() {
        let mut allocation = AllocationCount32::<u8>::default();
        let slice = allocation.deref_mut();
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn handles_pointers_correctly() {
        let mut allocation = AllocationCount8::<u8>::default();
        testing_unprint(vec![]); // no allocs in a default allocation.

        allocation
            .set_capacity(Count::of(100).expect("ok"))
            .expect("ok");
        testing_unprint(vec![Vec::from(b"create(A: 100)")]);

        allocation
            .set_capacity(Count::of(50).expect("ok"))
            .expect("ok");
        testing_unprint(vec![Vec::from(b"delete(A)"), Vec::from(b"create(B: 50)")]);

        allocation.set_capacity(Count::default()).expect("ok");
        testing_unprint(vec![Vec::from(b"delete(B)")]);
    }

    #[test]
    fn allocation_internal_offsets() {
        let allocation = AllocationCount8::<u8>::default();
        let allocation_ptr = std::ptr::addr_of!(allocation);
        let ptr_ptr = std::ptr::addr_of!(allocation.ptr);
        assert_eq!(ptr_ptr as usize, allocation_ptr as usize);
    }

    #[test]
    fn allocation_deref() {
        // We can be a bit more nonchalant here because u8s don't need to be initialized.
        let mut allocation = Aligned(AllocationCount16::<u8>::default());
        allocation
            .set_capacity(Count16::of(13).expect("ok"))
            .expect("small alloc");
        allocation
            .deref_mut()
            .copy_from_slice("hello, world!".as_bytes());
        assert_eq!(allocation.deref().deref(), "hello, world!".as_bytes());
    }

    #[test]
    fn roughly_double_capacity() {
        assert_eq!(
            AllocationCount16::<u8>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-32))
        );
        assert_eq!(
            AllocationCount8::<u16>::roughly_double_capacity(Count8::default()),
            Ok(Count8::negating(-20))
        );
        assert_eq!(
            AllocationCount16::<[u8; 3]>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-16))
        );
        assert_eq!(
            AllocationCount64::<u32>::roughly_double_capacity(Count64::default()),
            Ok(Count64::negating(-14))
        );
        assert_eq!(
            AllocationCount16::<[u8; 5]>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-12))
        );
        assert_eq!(
            AllocationCount16::<[u8; 6]>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-12))
        );
        assert_eq!(
            AllocationCount16::<[u8; 7]>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-11))
        );
        assert_eq!(
            AllocationCount32::<i64>::roughly_double_capacity(Count32::default()),
            Ok(Count32::negating(-11))
        );
        assert_eq!(
            AllocationCount8::<i128>::roughly_double_capacity(Count8::default()),
            Ok(Count8::negating(-8))
        );
        assert_eq!(
            AllocationCount16::<[u8; 32]>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-6))
        );
        assert_eq!(
            AllocationCount32::<[u8; 64]>::roughly_double_capacity(Count32::default()),
            Ok(Count32::negating(-4))
        );
        assert_eq!(
            AllocationCount64::<[u8; 96]>::roughly_double_capacity(Count64::default()),
            Ok(Count64::negating(-2))
        );
        assert_eq!(
            AllocationCount8::<[u8; 128]>::roughly_double_capacity(Count8::default()),
            Ok(Count8::negating(-1))
        );
        assert_eq!(
            AllocationCount16::<[u8; 2048]>::roughly_double_capacity(Count16::default()),
            Ok(Count16::negating(-1))
        );
        assert_eq!(
            AllocationCount32::<[u8; 123456789]>::roughly_double_capacity(Count32::default()),
            Ok(Count32::negating(-1))
        );
        assert_eq!(
            AllocationCount32::<[u8; 5]>::roughly_double_capacity(Count32::MAX),
            Err(NumberError::Unrepresentable)
        );
    }
}
