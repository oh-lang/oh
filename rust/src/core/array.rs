#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Clear {
    #[default]
    KeepingCapacity,
    DroppingCapacity,
}

// TODO: this should probably be `OrderedRemove`
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub enum Remove {
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
