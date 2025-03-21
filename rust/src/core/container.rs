#[derive(Eq, PartialEq, Copy, Clone, Default, Debug, Hash)]
pub enum ContainerError {
    /// Prefer something more descriptive.
    #[default]
    Unknown,
    /// Program ran out of memory.
    OutOfMemory,
    /// Invalid index/key (at) to the container.
    InvalidAt,
}

impl ContainerError {
    pub fn err(self) -> Containered {
        return Err(self);
    }
}

pub type Containered = ContainerResult<()>;
pub type ContainerResult<T> = Result<T, ContainerError>;
