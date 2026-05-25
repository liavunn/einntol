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

//! Error types related ro file system operations.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use thiserror;

#[derive(thiserror::Error, Debug, serde::Serialize, miette::Diagnostic)]
#[error("File error at {path:#?} : {Error_type}")]
pub FileOperationError {
    error_type: Option<FileError>,
    path: Option<PathBuf>,
}

/// File-related error types.
#[derive(thiserror::Error, Debug, serde::Serialize, miette::Diagnostic)]
pub enum FileError {
    /// The file at the specified path does not exist.
    #[error("The file not found.")]
    FileNotFound,

    /// The path at the specified path does not exist.
    #[error("The path not found")]
    PathNotFound,

    /// The file at the specified path is corrupted.
    #[error("The file corrupted: {reason}")]
    FileCorrupted {
        reason: String,
    },

    /// Permission denied for the specified path.
    #[error("Permission denied: The file cannot be accessed.")]
    PermissionDenied,

    /// Path contains invalid characters
    #[error("The path contains invalid characters.")]
    InvalidInput,

    /// The specified path already exists.
    #[error("The file already exists.")]
    AlreadyExists,

    /// The provided path is a directory, but a file was expected.
    #[error("The path is a directory, not a file.")]
    IsADirectory,

    /// The provided path is not a directory, but a one was expected.
    #[error("The path is a file, not a directory.")]
    NotADirectory,

    /// Insufficient storage space.
    #[error("Insufficient storage space, {remaining} bytes remaining, ({used}/{total_capacity}) bytes used")]
    InsufficientStorage{
        remaining: i64,
        used: i64,
        total_capacity: i64,
    },

    /// Generic input/output error with a descriptive message.
    #[error("IO Error")]
    IOError(String),

    /// Is a None Error.
    #[error("The error is None")]
    NoneErrro,
}
