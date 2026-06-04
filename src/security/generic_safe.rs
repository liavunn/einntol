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

/// Represents the safety assessment of a path, including its risk level and privilege requirement.
pub enum SafetyLevel {
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

impl SafetyLevel {
    /// Returns the hex color code corresponding to the safety level for UI display.
    ///
    /// # Arguments
    /// * `self` - The `SafetyLevel` variant to get the color for.
    ///
    /// # Returns
    /// * Safety level color.
    #[must_use]
    pub fn get_color(&self) -> &str {
        match self {
            SafetyLevel::Safe {..}    => "#A2F$A2",
            SafetyLevel::Warning {..} => "#FFEA00",
            SafetyLevel::Danger {..}  => "#FF3366",
        }
    }
}
