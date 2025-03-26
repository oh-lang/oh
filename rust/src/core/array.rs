use crate::core::moot::*;

pub use crate::core::container::*;
pub use crate::core::count::*;
pub use crate::core::signed::*;
pub use crate::core::traits::{Few, GetCount, SetCount, TryClone, TypeMarker};

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

pub enum OrderedInsert<T> {
    AtEnd(T),
}

pub enum OrderedInsertFew<T, E, F: Few<T, Error = E>> {
    AtEnd(F, TypeMarker<T>, TypeMarker<E>),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Sort {
    #[default]
    Default,
}

/// We require `Default` for insertions mostly because I'm lazy and want to be able
/// to `moot` elements out of slices.  (See, e.g., `OrderedInsert::FewAtEnd`)
pub trait Array<T: Default + TryClone>: std::ops::Deref<Target = [T]> + std::ops::DerefMut {
    fn len(&self) -> usize;

    fn insert(&mut self, insert: OrderedInsert<T>) -> Containered;
    fn insert_few<E, F: Few<T, Error = E>>(
        &mut self,
        insert: OrderedInsertFew<T, E, F>,
    ) -> Containered;
}

// TODO: because `insert_few` is generic, there's no way to build a vtable here (i.e., for `dyn Array`)
//impl<'a, T: Default> Few<T> for &'a mut dyn Array<T> {
//    type Error = ContainerError;
//
//    fn nab(&mut self, index: usize) -> Result<T, Self::Error> {
//        moot(&mut <Self as std::ops::DerefMut>::deref_mut(self)[index])
//    }
//
//    fn size(&self) -> usize {
//        self.len()
//    }
//}
//
//impl<'a, T: TryClone> Few<T> for &'a dyn Array<T> {
//    type Error = ContainerError;
//
//    fn nab(&mut self, index: usize) -> Result<T, Self::Error> {
//        self[index].try_clone()
//    }
//
//    fn size(&self) -> usize {
//        self.len()
//    }
//}
