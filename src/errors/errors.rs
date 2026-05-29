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

//! Aggregates all sub-module error types.
//! Provides a unified `AppError` enum as a container for internal and cross-boundary communication.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

/// Specialized error types for file system operations.
pub mod file_errors;
/// converting system errors into application errors.
pub mod errors_conversion;

// /// Represents a standardized error format for frontend or external interfaces.
// #[derive(serde::Serialize)]
// pub struct CommandError {
//      /// The error kind.
//      pub kind: String,
//      /// The error masage.
//      pub message: String,
//  }

/// The top-level error type that aggregates all sub-module errors.
#[derive(Error, Debug)]
pub enum AppError {
    /// Represents a file system error.
    #[error(transparent)]
    File(#[from] file_error::FileOperationError),

    #[error(transparent)]
    Generic(#[from] ),

    /// Represents generic system-level errors with a custom message.
    #[error(System error: {:#?}.)]
    System(String),
}
