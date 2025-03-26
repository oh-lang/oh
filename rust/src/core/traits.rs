use crate::core::count::*;
use crate::core::moot::*;
use crate::core::signed::*;

pub use std::convert::Infallible;
pub use std::marker::PhantomData as TypeMarker;

pub trait TryClone {
    type Error;

    fn try_clone(&self) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T: Clone> TryClone for T {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(self.clone())
    }
}

pub trait GetCount<S: SignedPrimitive> {
    fn count(&self) -> Count<S>;
}

pub trait SetCount<S: SignedPrimitive> {
    type Error;

    fn set_count(&mut self, new_count: Count<S>) -> Result<(), Self::Error>;
}

pub trait Few<T> {
    type Error;

    fn nab(&mut self, index: usize) -> Result<T, Self::Error>;

    // Needs to be `size` and not `len` because of weird Rust casting rules.
    // E.g., We can't use `<self as &mut [T]>.len()` inside `len` impls below.
    fn size(&self) -> usize;
}

impl<'a, T: Default> Few<T> for &'a mut [T] {
    type Error = Infallible;

    fn nab(&mut self, index: usize) -> Result<T, Self::Error> {
        Ok(moot(&mut self[index]))
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, T: TryClone> Few<T> for &'a [T] {
    type Error = <T as TryClone>::Error;

    fn nab(&mut self, index: usize) -> Result<T, Self::Error> {
        self[index].try_clone()
    }

    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::testing::*;

    fn do_a_few_things<T: std::fmt::Debug, E: std::fmt::Debug, F: Few<T, Error = E>>(mut few: F) {
        for i in 0..few.size() {
            let value = few.nab(i).expect("ok");
            testing_print_string(format!("{}: {:?}", i, value));
        }
    }

    #[test]
    fn test_few_mut_uses_moot() {
        let mut y: [TestingNoisy; 3] = [
            TestingNoisy::new(6),
            TestingNoisy::new(7),
            TestingNoisy::new(8),
        ];
        testing_unprint(vec![
            Vec::from(b"noisy_new(6)"),
            Vec::from(b"noisy_new(7)"),
            Vec::from(b"noisy_new(8)"),
        ]);

        do_a_few_things(&mut y[..]);

        testing_unprint(vec![
            Vec::from(b"noisy_new(256)"), // for mooting
            Vec::from(b"0: noisy(6)"),
            Vec::from(b"noisy_drop(6)"),
            Vec::from(b"noisy_new(256)"), // for mooting
            Vec::from(b"1: noisy(7)"),
            Vec::from(b"noisy_drop(7)"),
            Vec::from(b"noisy_new(256)"), // for mooting
            Vec::from(b"2: noisy(8)"),
            Vec::from(b"noisy_drop(8)"),
        ]);
        assert_eq!(
            y,
            [
                TestingNoisy::default(),
                TestingNoisy::default(),
                TestingNoisy::default(),
            ],
        );
    }

    #[test]
    fn test_few_const_uses_try_clone() {
        let x: [TestingNoisy; 2] = [TestingNoisy::new(12), TestingNoisy::new(24)];
        testing_unprint(vec![
            Vec::from(b"noisy_new(12)"),
            Vec::from(b"noisy_new(24)"),
        ]);

        do_a_few_things(&x[..]);

        testing_unprint(vec![
            Vec::from(b"noisy_clone(12)"), // from TryClone
            Vec::from(b"0: noisy(12)"),
            Vec::from(b"noisy_drop(12)"),
            Vec::from(b"noisy_clone(24)"), // from TryClone
            Vec::from(b"1: noisy(24)"),
            Vec::from(b"noisy_drop(24)"),
        ]);
        assert_eq!(x, [TestingNoisy::new(12), TestingNoisy::new(24)]);
    }
}
