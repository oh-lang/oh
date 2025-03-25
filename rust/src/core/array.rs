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
