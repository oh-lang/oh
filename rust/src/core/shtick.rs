use crate::core::count::*;
use crate::core::maybe_local_array::*;
use crate::core::signed::*;

pub type ShtickOptimized8 = ShtickOptimized<i8, 15>;
pub type ShtickOptimized16 = ShtickOptimized<i16, 14>;
pub type ShtickOptimized32 = ShtickOptimized<i32, 12>;
pub type ShtickOptimized64 = ShtickOptimized<i64, 16>;

pub struct ShtickOptimized<S: SignedPrimitive, const N_LOCAL: usize> {
    array: MaybeLocalArrayOptimized<S, N_LOCAL, u8>,
}

impl<S: SignedPrimitive, const N_LOCAL: usize> Default for ShtickOptimized<S, N_LOCAL> {
    fn default() -> Self {
        Self {
            array: Default::default(),
        }
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize> std::ops::Deref for ShtickOptimized<S, N_LOCAL> {
    type Target = MaybeLocalArrayOptimized<S, N_LOCAL, u8>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<S: SignedPrimitive, const N_LOCAL: usize> std::ops::DerefMut for ShtickOptimized<S, N_LOCAL> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        shtick.append(b'h').expect("ok");
        shtick.append(b'i').expect("ok");
        assert_eq!(shtick.count(), Count::of(2).expect("ok"));
        assert_eq!(shtick.capacity(), Count::of(15).expect("ok"));
        assert_eq!(&shtick[..], b"hi");
    }
}
