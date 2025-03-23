use crate::core::non_local_array::*;
use crate::core::shtick::*;

/// If you have more than 128 characters per line, it's a moral failing. /s
/// But it won't break because Shtick will internally expand to a `max_array` if necessary.
pub type FileLine = ShtickOptimized8;

/// We probably don't need (or want) more than 2**31 lines in a file.
/// Especially not for an in-memory file.  This also makes everything
/// fit nicely in 16 bytes (8 for the pointer, 4 + 4 for 32 bit count + capacity).
/// We don't optimize for the small file case (e.g., with `MaybeLocalArrayOptimized`)
/// because `FileLine`s are large (16 bytes) and we expect files to usually be
/// more than a few lines.
pub type InMemoryFileLines = NonLocalArrayCount32<FileLine>;

pub struct InMemoryFile {
    pub lines: InMemoryFileLines,
}
