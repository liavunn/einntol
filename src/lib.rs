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

#![deny(warnings)]
#![deny(clippy::pedantic)]
#![deny(clippy::all)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

/// Error handling utilities and custom error types.
pub mod errors;
/// Core business logic and system-level functional capabilities.
pub mod capabilities;
/// Utilities
pub mod utils;
/// 
pub mod domain;
/// 
pub mod security;
/// 
pub mod models;
/// 
pub mod modes;

/// File processing capabilities.
pub use crate::capabilities::file_tools;

/// Application-specific error types.
pub use crate::errors::AppError;

/// Application-specific error types.
pub use crate::errors::file_errors::{FileError, FileOperationError};

/// Generic error types for miscellaneous operations.
pub use crate::errors::generic_errors::GenericError;

/// Safety levels used for validating file paths and permissions.
pub use crate::security::file_safe::SafetyLevel;

/// File tool mode.
pub use crate::modes::file_modes::FileMode;

/// Represents the paths resulting from file discovery.
pub use crate::models::file_pipeline_data::FileResultPaths;

/// Represents the collection of errors encountered during file discovery.
pub use crate::models::file_pipeline_data::FileResultErrors;

/// Passing pipeline messages.
pub use crate::models::generic_pipeline::PipelineMessage;

/// Pipeline signal passing.
pub use crate::models::generic_pipeline::PipelineStatus;

/// Data carrier for pipeline progress.
pub use crate::models::generic_pipeline::ProgressData;

/// Provides filesystem utility functions.
pub use crate::utils::file_utils;

/// Provides command-line interface utilities.
pub use crate::utils::file_utils_cli;

/// Provides generic utility functions for common tasks.
pub use crate::utils::generic_utils_cli;
