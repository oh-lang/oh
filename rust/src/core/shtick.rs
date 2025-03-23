use crate::core::maybe_local_array::*;

pub type ShtickOptimized8 = MaybeLocalArrayOptimized8<15, u8>;
pub type ShtickOptimized16 = MaybeLocalArrayOptimized16<14, u8>;
pub type ShtickOptimized32 = MaybeLocalArrayOptimized32<12, u8>;
pub type ShtickOptimized64 = MaybeLocalArrayOptimized64<16, u8>;

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
}
