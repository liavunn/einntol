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

//! Global configuration modes and flag definitions.

#![forbid(warnings)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::all)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use std::path::PathBuf;

use crate::errors::AppError;
use crate::impl_struct_debug_to_display;

/// Represents the comprehensive results gathered during the file walking process.
#[derive(Debug)]
pub struct FileResults {
    /// List of successfully discovered or validated file paths.
    pub paths: Vec<PathBuf>,

    /// Collection of non-fatal errors encountered during the execution.
    pub errors: Vec<AppError>,
}

/// Represents the resulting paths from file discovery.
#[derive(Debug)]
pub struct FileResultPaths {
    /// List of successfully discovered or validated file paths.
    pub paths: Vec<PathBuf>,
}

/// Represents errors encountered during file discovery.
#[derive(Debug)]
pub struct FileResultErrors {
    /// Collection of non-fatal errors encountered during the execution.
    pub errors: Vec<AppError>,
}

impl_struct_debug_to_display!(
    (FileResults, paths, errors),
    (FileResultPaths, paths),
    (FileResultErrors, errors),
    );
