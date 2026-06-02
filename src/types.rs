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

#![deny(warnings)]
#![deny(clippy::pedantic)]
#![deny(clippy::all)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::AppError;

bitflags::bitflags! {
    /// Represents a set of active file behavior flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FileMode: u8 {
        /// No special modes enabled by default.
        const NONE             = 0b0000_0000;

        /// Include hidden files or directories.
        const WITH_HIDDEN      = 0b0000_0001;

        /// Include directories in the results.
        const WITH_DIR         = 0b0000_0010;

        /// Ignore differences in letter case.
        const CASE_INSENSITIVE = 0b0000_0100;

        /// UNRESTRICTED.
        const UNLIMITED        = 0b0000_1000;

        /// 
        const FUZZY            = 0b0001_0000;

        /// Enable all of the above flags.
        const ALL              = Self::WITH_HIDDEN.bits() |
                                 Self::WITH_DIR.bits() |
                                 Self::CASE_INSENSITIVE.bits() |
                                 Self::UNLIMITED.bits() |
                                 Self::FUZZY.bits();
    }
}

/// Represents a message sent through the pipeline, carrying either data or a control signal.
#[derive(Debug, Clone)]
pub enum PipelineMessage {
    /// self::File(Contains the processed file results and any associated non-fatal errors.
    Data(FileResult),

    /// Carries pipeline lifecycle and progress status signals
    Signal(PipelineStatus),
}

/// Defines the operational states of the pipeline lifecycle.
pub enum PipelineStatus {
    /// Initializing the process.
    Starting,
    
    /// Indicates ongoing progress, carrying an optional i64 translation.
    Progress(Option<i64>),

    /// Execution was aborted.
    Aborted,
    
    /// Execution finished successfully.
    Finished,
}

/// Represents the combined results of file discovery and validation.
#[derive(Debug, Clone)]
pub struct FileResult {
    /// List of successfully discovered or validated file paths.
    pub paths: Option<PathBuf>,

    /// Collection of non-fatal errors encountered during the execution.
    pub errors: Option<AppError>,
}

/// Represents the safety assessment of a path, including its risk level and privilege requirement.
pub enum SafetyLevel {
    /// Operations can be performed without restriction.
    Safe {
        needs_su: bool
    },

    /// Carries a degree of risk.
    Warning {
        needs_su: bool
    },

    /// Highly hazardous.
    Danger {
        needs_su: bool
    },
}

impl SafetyLevel {
    /// Returns the hex color code corresponding to the safety level for UI display.
    ///
    /// # Arguments
    /// * `self` - The `SafetyLevel` variant to get the color for.
    ///
    /// # Returns
    /// * Safety level color.
    pub fn get_color(&self) -> &str {
        match self {
            SafetyLevel::Safe {..}    => "#A2F$A2",
            SafetyLevel::Warning {..} => "#FFEA00",
            SafetyLevel::Danger {..}  => "#FF3366",
        }
    }

    /// Evaluates the path's safety based on its root component.
    ///
    /// # Arguments
    /// *  `path` - The filesystem path to be evaluated. Supports any type that 
    ///             implements [`AsRef<std::path::Path>`].
    ///
    /// # Returns
    /// * The determined [`SafetyLevel`].
    pub fn path_safetylevel(path: impl AsRef<std::path::Path>) -> Result<Self, AppError> {
        let path = path.as_ref();

        let absolute_path = path.canonicalize().map_err(|err| AppError::from_io_file_error(Some(err), path.to_path_buf(), None))?;

        let first_component = absolute_path
            .components
            .nth(1)
            .and_then(|com|as_os_str().to_str());

        let level = match first_component {
            Some("boot" | "dev" |"proc" | "sys" | "run" | "lib" | "lib64") => 
                SafetyLevel::Danger {needs_su: true},

            Some("root" | "bin" | "sbin" | "etc" | "usr" | "var") =>
                SafetyLevel::Warning {needs_su: true},
            Some("home") => SafetyLevel::Warning {needs_su: false},

            _ => SafetyLevel::Safe {needs_su: false},

            None => SafetyLevel::Safe {needs_su: false},
        };

        Ok(level)
    }
}
