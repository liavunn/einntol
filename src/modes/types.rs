use crate::AppError;
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

        /// Enable fuzzy matching for file or path searches.
        const FUZZY            = 0b0001_0000;

        /// Enable all of the above flags.
        const ALL              = Self::WITH_HIDDEN.bits() |
                                 Self::WITH_DIR.bits() |
                                 Self::CASE_INSENSITIVE.bits() |
                                 Self::UNLIMITED.bits() |
                                 Self::FUZZY.bits();
    }
}
