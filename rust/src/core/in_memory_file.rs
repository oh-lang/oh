use crate::core::moot::*;
use crate::core::non_local_array::*;
use crate::core::shtick::*;

use std::ffi::{OsStr, OsString};
use std::io::Read;

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
        Ok(result)
    }

    pub fn is_directory(&self) -> bool {
        let path_len = self.path.len();
        if path_len == 0 {
            // Assume an empty path is the current directory.
            true
        } else {
            let last_char = self.path[path_len - 1];
            last_char == std::path::MAIN_SEPARATOR as u8
        }
    }

    pub fn read(&mut self) -> Filed {
        if self.is_directory() {
            // TODO
            return Err(FileError::Unknown);
        }
        let mut file = self.open_file(FileOpen::Read)?;
        let mut lines = InMemoryFileLines::default();
        let mut buffer = [0u8; 256];
        let mut current_line = FileLine::default();
        loop {
            let bytes_read = file.read(&mut buffer).map_err(|_| FileError::Read)?;
            if bytes_read == 0 {
                break;
            }
            let buffer = &buffer[0..bytes_read];
            let mut handled_up_to = 0;
            for i in 0..bytes_read {
                if buffer[i] == b'\n' {
                    current_line
                        .insert_few(OrderedInsertFew::AtEnd(
                            &buffer[handled_up_to..i],
                            TypeMarker,
                            TypeMarker,
                        ))
                        .map_err(|_| FileError::OutOfMemory)?;
                    lines
                        .insert(OrderedInsert::AtEnd(moot(&mut current_line)))
                        .map_err(|_| FileError::OutOfMemory)?;
                    handled_up_to = i + 1;
                }
            }
            current_line
                .insert_few(OrderedInsertFew::AtEnd(
                    &buffer[handled_up_to..bytes_read],
                    TypeMarker,
                    TypeMarker,
                ))
                .map_err(|_| FileError::OutOfMemory)?;
        }
        lines
            .insert(OrderedInsert::AtEnd(current_line))
            .map_err(|_| FileError::OutOfMemory)?;
        self.lines = lines;
        Ok(())
    }

    fn open_file(&self, file_open: FileOpen) -> FileResult<std::fs::File> {
        match file_open {
            FileOpen::Read => {
                if cfg!(unix) {
                    use std::os::unix::ffi::OsStrExt;
                    let os_str = std::path::Path::new(OsStr::from_bytes(&self.path[..]));
                    std::fs::File::open(os_str)
                } else {
                    // Need to convert to a UTF16-like string for Windows.
                    let os_path: OsString =
                        String::from(String::from_utf8_lossy(&self.path[..])).into();
                    std::fs::File::open(std::path::Path::new(&os_path))
                }
            }
        }
        .map_err(|_| FileError::Open)
    }

    // TODO: add a `fn has_changed()` method
}

enum FileOpen {
    Read,
}

pub enum FileError {
    OutOfMemory,
    Open,
    Read,
    Unknown,
}

pub type FileResult<T> = Result<T, FileError>;

pub type Filed = FileResult<()>;
