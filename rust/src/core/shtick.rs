use crate::core::container::*;
use crate::core::count::*;
use crate::core::maybe_local_array::*;
use crate::core::signed::*;

pub type ShtickOptimized8 = ShtickOptimized<i8, 15>;
pub type ShtickOptimized16 = ShtickOptimized<i16, 14>;
pub type ShtickOptimized32 = ShtickOptimized<i32, 12>;
pub type ShtickOptimized64 = ShtickOptimized<i64, 16>;

pub type ShtickOptimized<S, const N_LOCAL: usize> = MaybeLocalArrayOptimized<S, N_LOCAL, u8>;

impl<S: SignedPrimitive, const N_LOCAL: usize> TryFrom<&str> for ShtickOptimized<S, N_LOCAL> {
    type Error = ContainerError;

    fn try_from(value: &str) -> ContainerResult<Self> {
        let mut result = ShtickOptimized::default();
        let count = Count::of(value.len()).map_err(|_| ContainerError::OutOfMemory)?;
        result.set_capacity(count)?;
        // In case `result.capacity() > count`, e.g., for the `unallocated_buffer`,
        // we need to limit the slice to the correct size.
        result.fully_allocated_slice_mut()[0..value.len()].copy_from_slice(value.as_bytes());
        result.only_set_count(count);
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::testing::*;

    #[test]
    fn shtick_optimized_x_is_correct_size() {
        assert_eq!(std::mem::size_of::<ShtickOptimized8>(), 16);
        assert_eq!(std::mem::size_of::<ShtickOptimized16>(), 16);
        assert_eq!(std::mem::size_of::<ShtickOptimized32>(), 16);
        assert_eq!(std::mem::size_of::<ShtickOptimized64>(), 24);
    }

    #[test]
    fn shtick_optimized_8_works_with_array_deref() {
        let mut shtick = ShtickOptimized8::default();
        shtick.insert(OrderedInsert::AtEnd(b'h')).expect("ok");
        shtick.insert(OrderedInsert::AtEnd(b'i')).expect("ok");
        assert_eq!(shtick.count(), Count::of(2).expect("ok"));
        assert_eq!(shtick.capacity(), Count::of(15).expect("ok"));
        assert_eq!(&shtick[..], b"hi");
        shtick[0] = b'o';
        shtick[1] = b'h';
        assert_eq!(&shtick[..], b"oh");
    }

    #[test]
    fn shtick_optimized_8_can_initialize_from_string() {
        {
            let mut shtick = ShtickOptimized8::try_from("hello world").expect("ok");
            assert_eq!(&shtick[..], b"hello world");

            shtick =
                ShtickOptimized8::try_from("let's make something that is more bytes").expect("ok");
            assert_eq!(&shtick[..], b"let's make something that is more bytes");
            testing_unprint(vec![Vec::from(b"create(A: 39)")]);
        }
        testing_unprint(vec![Vec::from(b"delete(A)")]);
    }
}
