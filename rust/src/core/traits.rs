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
