use crate::core::non_local_array::*;
use crate::core::shtick::*;

/// If you have more than 128 characters per line, it's a moral failing. /s
/// But it won't break because Shtick will internally expand to a `max_array` if necessary.
pub type FileLine = ShtickOptimized8;

pub type FilePath = ShtickOptimized8;

/// We probably don't need (or want) more than 2**31 lines in a file.
/// Especially not for an in-memory file.  This also makes everything
/// fit nicely in 16 bytes (8 for the pointer, 4 + 4 for 32 bit count + capacity).
/// We don't optimize for the small file case (e.g., with `MaybeLocalArrayOptimized`)
/// because `FileLine`s are large (16 bytes) and we expect files to usually be
/// more than a few lines.
pub type InMemoryFileLines = NonLocalArrayCount32<FileLine>;

pub struct InMemoryFile {
    /// Include a trailing OS separator (e.g., `/` on Unix-like systems)
    /// in order to indicate that this is a directory and not a file.
    pub path: FilePath,
    pub lines: InMemoryFileLines,
}

impl InMemoryFile {
    pub fn open(path: FilePath) -> FileResult<Self> {
        let mut result = Self {
            path,
            lines: Default::default(),
        };
        result.read()?;
        result
    }

    pub fn is_directory(&self) -> bool {
        let path_count = self.path.count();
        if path_count == Count::default() {
            // Assume an empty path is the current directory.
            true
        } else {
            let last_char = self.path[path_count - 1];
            last_char == std::path::MAIN_SEPARATOR as u8
        }
    }

    pub fn read(&mut self) -> Filed {
        let path = std::path::Path::new(&self.path[..]);
        if self.is_directory() {
            // TODO
            return Err(FileError::Unknown);
        }
        let mut file = std::fs::File::open(path).map_err(|_| FileError::Open)?;
        let mut lines = InMemoryFileLines::default();
        let mut file_offset = 0;
        let mut buffer = [0u8; 256];
        let mut buffer_offset = 0;
        let mut current_line = FileLine::default();
        loop {
            let bytes_read = file.read_at(&mut buffer, file_offset)?;
            if bytes_read == 0 {
                break;
            }
            let buffer = buffer[0..bytes_read];
            for slice in buffer.split(|b| b == b'\n') {
                current_line.append(slice)
            }
            buffer[0..buffer.len() - 1]
        }
        lines.append(current_line);
        self.lines = lines;
    }

    // TODO: add a `fn has_changed()` method
}

pub enum FileError {
    Unknown,
    Open,
}

pub type FileResult<T> = Result<T, FileError>;

pub type Filed = FileResult<()>;
