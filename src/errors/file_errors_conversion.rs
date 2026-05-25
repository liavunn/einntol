// Copyright [2026] [livunn]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at1
//
//     https://www.apache.org/licenses/LICENSE-2.0
//     
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Converts `std::io::Error` into a custom error enum.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use once_cell:sync::Lazy;
use sysinfo::Disks;

use super::file_errors::{FileOperationError, FileError};

/// Global read-only disk information manager for error diagnostics.
static DISK_MANAGER: Lazy<Disk> = Lazy::new(|| {Disks::new_with_refreshed_list()});


/// Converts a standard std::io::Error into an enriched AppError.
pub fn from_io_file_error(err: Option<std::io::Error>, path: PathBuf) -> FileOperationError {
    #[cfg(feature = "logging")]
    let path_for_log = path.clone();

    let err = match err {
        Some(error) => error,

        None => return FileOperationErrro {
            errors: Some(FileError::NoneError),
            path: None
        }
    }

    let error_type = match err.kind() {
      // The file not found.
        std::io::ErrorKind::NotFound => {
            if let Some(parent) = path.parent() {
                if parent.exists() {
                    FileError::FileNotFound
                } else {
                    FileError::PathNotFound
                }
            } else {
                FileError::PathNotFound
            }
        }

        // Out of storage space.
        std::io::ErrorKind::StorageFull => {
            if let Some(disk) = DISK_MANAGER.iter().find(|disk| path.starts_with(disk.mount_point)) {
                let total = disk.total_space() as i64;
                let available = disk.available_space() as i64;

                FileError::InsufficientStorage {
                    remaining: available,
                    used: total - available,
                    total_capacity: total,                  
                }
            }
        }

        // Data is corrupted or reached unexpected EOF.
        std::io::ErrorKind::InvalidData | std::io::ErrorKind::UnexpectedEof =>
            FileError::FileCorrupted,

        // Insufficient permissions
        std::io::ErrorKind::PermissionDenied =>
            FileError::PermissionDenied),

        // Path contains illegal characters or is malformed.
        std::io::ErrorKind::InvalidInput =>
            FileError::InvalidInput,

        // The destination path already exists.
        std::io::ErrorKind::AlreadyExists =>
            FileError::AlreadyExists,

        // Expected a file. but is a directory.
        std::io::ErrorKind::IsADirectory =>
            FileError::IsADirectory,
        // Expected a path. but is a file.
        std::io::ErrorKind::NotADirectory =>
            FileError::NotADirectory,

        // Fallback: capture generic system error with path context.
        _ => FileError::IOError(err.to_string(),

        // Is a None error.
        None => FileError::NoneError,
    }

#[cfg(feature = "logging")]
{
    tracing::error!(
        path = %path_for_log.display(),
        "Summary: {}\nDetail: {:#?}",
            error_type,
            err
    );
}
    
FileOperationError {Some(error_type), Some(path)}
}
