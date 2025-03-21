#[derive(Eq, PartialEq, Copy, Clone, Default, Debug, Hash)]
pub enum ContainerError {
    #[default]
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
