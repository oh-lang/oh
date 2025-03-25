/// "Move and reset"s the passed-in value.
pub fn moot<T: Default>(t: &mut T) -> T {
    let mut result = T::default();
    std::mem::swap(&mut result, t);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moot_works() {
        let mut i: i32 = 123;
        assert_eq!(moot(&mut i), 123);
        assert_eq!(i, 0);
    }
}
