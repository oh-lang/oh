use crate::core::count::*;
use crate::core::signed::*;

pub use std::convert::Infallible;

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
