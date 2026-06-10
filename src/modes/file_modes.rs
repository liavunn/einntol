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

#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use std::sync::Arc;

bitflags::bitflags! {
    /// Represents a set of active file behavior flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FileMode: u8 {
        /// No special modes enabled by default.
        const NONE             = 0b0000_0000;

        /// Include hidden files or directories.
        const WITH_HIDDEN      = 0b0000_0001;

        /// Include directories in the results.
        const ONLY_DIR         = 0b0000_0010;

        /// Ignore differences in letter case.
        const CASE_INSENSITIVE = 0b0000_0100;

        /// UNRESTRICTED.
        const UNLIMITED        = 0b0000_1000;

        /// Enable fuzzy matching for file or path searches.
        const FUZZY            = 0b0001_0000;

        /// Enable all of the above flags.
        const ALL              = Self::WITH_HIDDEN.bits() |
                                 Self::ONLY_DIR.bits() |
                                 Self::CASE_INSENSITIVE.bits() |
                                 Self::UNLIMITED.bits() |
                                 Self::FUZZY.bits();
    }
}

/// Represents a specific rule used to filter files and directories during scanning.
pub enum FilterStrategy {
    /// Do not apply any filtering.
    None(Arc<str>),
    /// Filter out all items except directories, passing only directory entries.
    OnlyDir(Arc<str>),
    /// Match file names without considering letter case differences.
    CaseInsensitive(Arc<str>),
    /// Perform a fuzzy or partial match on file names.
    Fuzzy(Arc<str>),
}

impl FilterStrategy {
    pub fn matches(&self, entry: &ignore::DirEntry) -> bool {
        match self {
            // None: Match regular files only; skip hidden files, directories, and paths ignored by .gitignore.
            Self::None(name) => {
                let current_name = entry.file_name().to_string_lossy();

                if !entry.file_type().map_or(false, |filekind| filekind.is_file()) {
                    return false;
                }

                if name.as_ref() != current_name {
                    return false;
                } 

                true
            },

            // Only_dir: Match directories only; exclude regular and hidden files.
            Self::OnlyDir(name) => {
                let current_name = entry.file_name().to_string_lossy();

                if entry.file_type().map_or(false, |filekind| filekind.is_dir()) {
                    return false;
                }

                if name.as_ref() != current_name {
                    return false;
                } 

                true
            },

            // Case_Insensitive: Perform case-insensitive filename comparison.
            Self::CaseInsensitive(name) => {
                let current_name = entry.file_name().to_string_lossy();

                if name.as_ref() != current_name.to_lowercase() {
                    return false;
                }

                if name.as_ref() != current_name {
                    return false;
                } 

                true

            },

            // Fuzzy: Match files by stem name.
            Self::Fuzzy(name) => {
                let current_stem = entry
                 .path()
                .file_stem()
                .map(|str| str.to_string_lossy())
                .unwrap_or_default();

                if name.as_ref() != current_stem {
                    return false;
                }

                true
            },
        }
    }
}
