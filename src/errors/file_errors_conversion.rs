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

#![deny(warnings)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::all)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

use once_cell::sync::Lazy;
use sysinfo::Disks;

use crate::{FileOperationError, FileError};
use crate::AppError;

/// Global read-only disk information manager for error diagnostics.
static DISK_MANAGER: Lazy<Disks> = Lazy::new(|| {Disks::new_with_refreshed_list()});

impl AppError {
    /// Converts a standard std::io::Error into an enriched AppError.
    pub fn from_io_file_error(err: Option<std::io::Error>, err_path: PathBuf) -> FileOperationError {
        #[cfg(feature = "logging")]
        let path_for_log = err_path.clone();

        let err = match err {
            Some(error) => error,

            None => return FileOperationError {
                error_type: Some(FileError::NoneError),
                path: None
            }
        };

        let error_type = match err.kind() {
            // The file not found.
            std::io::ErrorKind::NotFound => {
                if let Some(parent) = err_path.parent() {
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
                if let Some(disk) = DISK_MANAGER.iter().find(|disk| err_path.starts_with(disk.mount_point())) {
                    let total = disk.total_space() as i64;
                    let available = disk.available_space() as i64;

                    FileError::InsufficientStorage {
                        remaining: available,
                        used: total - available,
                        total_capacity: total,                  
                    }
                } else {
                    FileError::Unknown
                }
            }

            // Data is corrupted or reached unexpected EOF.
            std::io::ErrorKind::InvalidData | std::io::ErrorKind::UnexpectedEof =>
                FileError::FileCorrupted,

            // Insufficient permissions.
            std::io::ErrorKind::PermissionDenied =>
                FileError::PermissionDenied,

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
            _ => FileError::IOError(err.to_string()),
        };

        #[cfg(feature = "logging")]
        {
            tracing::error!(
                path = %path_for_log.display(),
                "Summary: {}\nDetail: {:#?}",
                    error_type,
                    err
            );
        }

    FileOperationError {error_type: Some(error_type), err_path: Some(err_path)}
    }
}
