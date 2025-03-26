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
