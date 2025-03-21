use crate::core::count::*;
use crate::core::likely::*;
use crate::core::number::*;
use crate::core::offset::*;
use crate::core::signed::*;

use std::alloc;
use std::ptr::{self, NonNull};

#[derive(Eq, PartialEq, Copy, Clone, Default, Debug, Hash)]
pub enum AllocationError {
    #[default]
    OutOfMemory,
    InvalidOffset,
}

impl AllocationError {
    pub fn err(self) -> Allocated {
        return Err(self);
    }
}

pub type Allocated = AllocationResult<()>;
pub type AllocationResult<T> = Result<T, AllocationError>;

pub type AllocationCount64<T> = AllocationCount<i64, T>;
pub type AllocationCount32<T> = AllocationCount<i32, T>;
pub type AllocationCount16<T> = AllocationCount<i16, T>;
pub type AllocationCount8<T> = AllocationCount<i8, T>;

/// Low-level structure that has a pointer to contiguous memory,
/// with a capacity up to Count::<S>::MAX elements.
/// You need to keep track of which elements are initialized, etc.
/// Because of that, you need to MANUALLY drop this allocation after
/// freeing any initialized elements, by calling `mut_capacity(Count::of(0))`
/// WARNING! because this is packed, you may need to wrap it in `Aligned(...)`
/// in order to ensure that `ptr` is on an aligned boundary.
#[repr(C, packed)]
pub struct AllocationCount<S: SignedPrimitive, T> {
    ptr: NonNull<T>,
    capacity: Count<S>,
}

impl<S: SignedPrimitive, T> AllocationCount<S, T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: Count::<S>::default(),
        }
    }

    pub fn capacity(&self) -> Count<S> {
        self.capacity
    }

    /// Caller MUST ensure that they've already dropped elements that you might delete here
    /// if the new capacity is less than the old.  The old capacity will be updated
    /// iff the capacity change succeeds.
    pub fn mut_capacity(&mut self, new_capacity: Count<S>) -> Allocated {
        let old_capacity = self.capacity;
        if !new_capacity.is_positive() {
            if old_capacity > Count::<S>::default() {
                unsafe {
                    alloc::dealloc(
                        self.as_ptr_mut() as *mut u8,
                        Self::layout_of(old_capacity).expect("already allocked"),
                    );
                }
                self.ptr = NonNull::dangling();
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
                    self.as_ptr_mut() as *mut u8,
                    Self::layout_of(old_capacity).expect("already allocked"),
                    new_layout.size(),
                )
            } else {
                alloc::alloc(new_layout)
            }
        } as *mut T;
        match NonNull::new(new_ptr) {
            Some(new_ptr) => {
                self.ptr = new_ptr;
                self.capacity = new_capacity;
                Ok(())
            }
            None => {
                cold();
                AllocationError::OutOfMemory.err()
            }
        }
    }

    /// Writes to an offset that should *not* be initialized until *after* this call.
    /// I.e., it is uninitialized before calling this.
    pub fn write_initializing(&mut self, offset: Offset<S>, value: T) -> Allocated {
        let capacity = self.capacity;
        if !capacity.contains(Contains::<S>::Offset(offset)) {
            return AllocationError::InvalidOffset.err();
        }
        unsafe {
            ptr::write(self.as_ptr_mut().add(offset.0.as_() as usize), value);
        }
        Ok(())
    }

    /// Reads at the offset, and from now on, that offset should be considered
    /// uninitialized.
    pub fn read_deinitializing(&mut self, offset: Offset<S>) -> AllocationResult<T> {
        let capacity = self.capacity;
        if !capacity.contains(Contains::<S>::Offset(offset)) {
            return Err(AllocationError::InvalidOffset);
        }
        Ok(unsafe { ptr::read(self.as_ptr_mut().add(offset.0.as_() as usize)) })
    }

    pub fn grow(&mut self) -> Allocated {
        let capacity = self.capacity;
        let desired_capacity = Self::roughly_double_capacity(capacity);
        if desired_capacity <= capacity {
            return AllocationError::OutOfMemory.err();
        }
        self.mut_capacity(desired_capacity)
    }

    fn roughly_double_capacity(capacity: Count<S>) -> Count<S> {
        // When starting out, we allocate more for smaller (in size) T.
        // This interpolates from starting_alloc = 32 down to 1 over a large range.
        let t_bytes = std::mem::size_of::<T>() as i64;
        let multiplier = (8 - (t_bytes / 16)).max(1);
        let desired_total_bytes = 24 + t_bytes * multiplier;
        let desired_total_words = desired_total_bytes / 8;
        let starting_alloc = ((desired_total_words * 8) / t_bytes).max(1);
        capacity.double_or_at_least(S::from(starting_alloc).unwrap())
    }

    fn layout_of(capacity: Count<S>) -> AllocationResult<alloc::Layout> {
        if let Some(capacity) = capacity.to_u64() {
            alloc::Layout::array::<T>(capacity as usize).or(Err(AllocationError::OutOfMemory))
        } else {
            cold();
            Err(AllocationError::InvalidOffset)
        }
    }

    fn as_non_null_ptr(&self) -> &NonNull<T> {
        let ptr = std::ptr::addr_of!(self.ptr);
        assert_eq!((ptr as usize) % 8, 0);
        unsafe { &*ptr }
    }

    fn as_non_null_ptr_mut(&mut self) -> &mut NonNull<T> {
        let ptr = std::ptr::addr_of_mut!(self.ptr);
        assert_eq!((std::ptr::addr_of!(*self) as usize) % 8, 0);
        assert_eq!((ptr as usize) % 8, 0);
        unsafe { &mut *ptr }
    }

    fn as_ptr(&self) -> *const T {
        self.as_non_null_ptr().as_ptr()
    }

    fn as_ptr_mut(&mut self) -> *mut T {
        self.as_non_null_ptr_mut().as_ptr()
    }
}

impl<S: SignedPrimitive, T> Default for AllocationCount<S, T> {
    fn default() -> Self {
        return Self::new();
    }
}

impl<S: SignedPrimitive, T> std::ops::Deref for AllocationCount<S, T> {
    type Target = [T];
    /// Caller is responsible for only accessing initialized values.
    fn deref(&self) -> &[T] {
        let capacity = self.capacity;
        if let Some(capacity) = capacity.to_u64() {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), capacity as usize) }
        } else {
            cold();
            &[]
        }
    }
}

impl<S: SignedPrimitive, T> std::ops::DerefMut for AllocationCount<S, T> {
    /// Caller is responsible for only accessing initialized values.
    fn deref_mut(&mut self) -> &mut [T] {
        let capacity = self.capacity;
        if let Some(capacity) = capacity.to_u64() {
            unsafe { std::slice::from_raw_parts_mut(self.as_ptr_mut(), capacity as usize) }
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
    fn allocation_internal_offsets() {
        let allocation = AllocationCount8::<u8>::new();
        let allocation_ptr = std::ptr::addr_of!(allocation);
        let ptr_ptr = std::ptr::addr_of!(allocation.ptr);
        assert_eq!(ptr_ptr as usize, allocation_ptr as usize);
    }

    #[test]
    fn allocation_deref() {
        // We can be a bit more nonchalant here because u8s don't need to be initialized.
        let mut allocation = Aligned(AllocationCount16::<u8>::new());
        allocation
            .mut_capacity(Count16::of(13).expect("ok"))
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
            Count16::negating(-32)
        );
        assert_eq!(
            AllocationCount8::<u16>::roughly_double_capacity(Count8::default()),
            Count8::negating(-20)
        );
        assert_eq!(
            AllocationCount16::<[u8; 3]>::roughly_double_capacity(Count16::default()),
            Count16::negating(-16)
        );
        assert_eq!(
            AllocationCount64::<u32>::roughly_double_capacity(Count64::default()),
            Count64::negating(-14)
        );
        assert_eq!(
            AllocationCount16::<[u8; 5]>::roughly_double_capacity(Count16::default()),
            Count16::negating(-12)
        );
        assert_eq!(
            AllocationCount16::<[u8; 6]>::roughly_double_capacity(Count16::default()),
            Count16::negating(-12)
        );
        assert_eq!(
            AllocationCount16::<[u8; 7]>::roughly_double_capacity(Count16::default()),
            Count16::negating(-11)
        );
        assert_eq!(
            AllocationCount32::<i64>::roughly_double_capacity(Count32::default()),
            Count32::negating(-11)
        );
        assert_eq!(
            AllocationCount8::<i128>::roughly_double_capacity(Count8::default()),
            Count8::negating(-8)
        );
        assert_eq!(
            AllocationCount16::<[u8; 32]>::roughly_double_capacity(Count16::default()),
            Count16::negating(-6)
        );
        assert_eq!(
            AllocationCount32::<[u8; 64]>::roughly_double_capacity(Count32::default()),
            Count32::negating(-4)
        );
        assert_eq!(
            AllocationCount64::<[u8; 96]>::roughly_double_capacity(Count64::default()),
            Count64::negating(-2)
        );
        assert_eq!(
            AllocationCount8::<[u8; 128]>::roughly_double_capacity(Count8::default()),
            Count8::negating(-1)
        );
        assert_eq!(
            AllocationCount16::<[u8; 2048]>::roughly_double_capacity(Count16::default()),
            Count16::negating(-1)
        );
        assert_eq!(
            AllocationCount32::<[u8; 123456789]>::roughly_double_capacity(Count32::default()),
            Count32::negating(-1)
        );
    }
}
