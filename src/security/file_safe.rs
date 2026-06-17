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

use crate::errors::AppError;
use crate::security::generic_safe::ColorDisplay;
use crate::impl_get_color_display;

/// Represents the safety assessment of a path, including its risk level and privilege requirement.
pub enum FileSafetyLevel {
    /// Operations can be performed without restriction.
    Safe {
        /// Indicates if superuser privileges are required.
        needs_su: bool
    },

    /// Carries a degree of risk.
    Warning {
        /// Indicates if superuser privileges are required.
        needs_su: bool
    },

    /// Highly hazardous.
    Danger {
        /// Indicates if superuser privileges are required.
        needs_su: bool
    },
}

impl FileSafetyLevel {
    /// Evaluates the path's safety based on its root component.
    ///
    /// # Arguments
    /// * `path` - The filesystem path to be evaluated. Supports any type that 
    ///   implements [`AsRef<std::path::Path>`].
    ///
    /// # Returns
    /// * The determined [`SafetyLevel`].
    ///
    /// # Errors
    /// * Returns an `AppError` if the path component cannot be converted
    ///   to a string or if the path is invalid.
    pub fn path_safetylevel(path: impl AsRef<std::path::Path>) -> Result<Self, AppError> {
        let path = path.as_ref();

        let absolute_path = path.canonicalize().map_err(|err| AppError::from_io_file_error(Some(err), path.to_path_buf(), None))?;

        let first_component = absolute_path
            .components()
            .nth(1)
            .and_then(|com| com.as_os_str().to_str());

        let level = match first_component {
            Some("boot" | "dev" |"proc" | "sys" | "run" | "lib" | "lib64") => 
                FileSafetyLevel::Danger {needs_su: true},

            Some("root" | "bin" | "sbin" | "etc" | "usr" | "var") =>
                FileSafetyLevel::Warning {needs_su: true},
            Some("home") => FileSafetyLevel::Warning {needs_su: false},

            _ => FileSafetyLevel::Safe {needs_su: false},
        };

        Ok(level)
    }
}

impl_get_color_display!(FileSafetyLevel);
