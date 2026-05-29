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

use super::Generic_errors::GenericError;

/// Global read-only disk information manager for error diagnostics.
static DISK_MANAGER: Lazy<Disk> = Lazy::new(|| {Disks::new_with_refreshed_list()});

impl AppError {
    /// Converts a standard std::io::Error into an enriched AppError.
    pub fn from_io_generic_error(err: Option<std::io::Error>) -> GenericError {
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
            // 
            std::io::ErrorKind::=> {
            }

            // Fallback: capture generic system error with path context.
            _ => FileError::IOError(err.to_string(),
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
}
