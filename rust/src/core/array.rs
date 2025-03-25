pub use crate::core::container::*;
pub use crate::core::count::*;
pub use crate::core::signed::*;
pub use crate::core::traits::{GetCount, SetCount, TryClone};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Clear {
    #[default]
    KeepingCapacity,
    DroppingCapacity,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum OrderedRemove {
    #[default]
    Last,
    // TODO
    //First,
    //Index(Index),
}

pub enum OrderedInsert<'a, T> {
    AtEnd(T),
    SliceAtEnd(&'a mut [T]),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Sort {
    #[default]
    Default,
}

/// We require `Default` for insertions mostly because I'm lazy and want to be able
/// to `moot` elements out of slices.  (See, e.g., `OrderedInsert::SliceAtEnd`)
pub trait Array<T: Default + TryClone> {
    fn insert(&mut self, insert: OrderedInsert<T>) -> Containered;
}
