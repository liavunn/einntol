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

//! The core library of the Einntol program, responsible for the core logic.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

/// Error handling utilities and custom error types.
pub mod errors;
/// Core business logic and system-level functional capabilities.
pub mod capabilities;
/// Common data structures and global type definitions.
pub mod types;

use thiserror;
use fastrand;
use wyhash::WyHash;

/// Re-expots the capabilities for convenient external access.
///
/// Can be accessed via the specialized capability modules within the capabilities namespace.
pub use crate::capabilities::{
    file_tool,
//  network_tool,
}
/// Re-expots the error types for convenient external access.
///
/// Can be accessed via `einntol::FileError`.
pub use crate::errors::file_errors::FileError;
/// Re-exports the error types for convenient external access.
///
/// Can be accessed via `einntol::SafetyLevel`.
pub use crate::types::SafetyLevel;
/// Safety levels used for validating file paths and permissions.
///
/// Can be accessed via `einntol::FileMode`.
pub use crate::types::FileMode;
/// Standardized result wrapper for all file-related operations.
///
/// Can be accessed via `einntol::Result`.
pub use crate::types::FileResult;
