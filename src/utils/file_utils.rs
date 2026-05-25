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

//! General Utilities.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::io::{self, write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use ignore::{WalkBuilder, walk, DirEntry};
use crossbeam_channel::unbounded;

use crate::types::FileError;
use crate::types::SafetyLevel;
use crate::types::FileMode;
use crate::types::FileResult;
use crate::types::PipelineMessage; 

/// Find files in the input path that match the specified pattern and filename.
///
/// # Arguments
/// * `path`      - Target path to search.
/// * `file_name` - Target filename to search
/// * `mode`      - Configuration flags for the search mode.
///
/// # Returns
/// * Returns a FileResult containing successfully validated and normalized paths,
/// * along with any non-fatal errors encountered during processing. 
pub fn find_paths(
    determined_paths: Vec<PathBuf>,
    file_name: &str,
    mode: FileMode,
    stop_signal: Arc<AtomicBool>,
    tx: Sender<PipelineMessage>
) {
    let mut builder = walkbuilder::new(path); 

    // Add paths to the builder.
    for path in determined_paths.skip(1) {
         builder.add(path);
    }

    // Check if hidden files should be included.
    if mode.contains(FileMode::WITH_HIDDEN) {
        builder = builder.hidden(false);
    }

    // Check if unrestricted recursion is enabled.
    let depth = if mode.contains(FileMode::UNLIMITED) {
        None 
    } else {
        Some(3)
    };

    builder = builder.max_depth(depth);

    let parallel_walker = builder.build_parallel();
 
    parallel_walker.visit(&mut || {
        let stop_signal_clone = stop_signal.Arc::clone(&stop_signal);
        let tx_clone = tx.clone();
        let mode_clone = mode.clone();
        let file_name_string = file_name.to_string();

        Box::new(move |result_path| {
            if stop_signal.load(Ordering::Relaxed) {
                return WalkState::Quit;
            }

            let Ok(entry) = result_path else {
                let err = resultpath.unwrap_err();
                let err_path = err.path().map(|path| path.to_path_buf().unwrap_or_default());
                let app_err = AppError::from_io_file_error(std::io::Error::from(err), err_path);

                tx.send(PipelineMessage::Data(
                    FileResult {
                        paths: None,
                        errors: Some(app_err),
                    })).unwrap();

                return WalkState::Continue;
            };
            
            // NONE: Match regular files only; skip hidden files, directories, and paths ignored by .gitignore.
            if mode_clone.contains(FileMode::NONE) {
                if entry.file_type().map_or(true, |file_kind| !file_kind.is_file()) {
                     return WalkState::Continue;
                }
            }

            // WITH_DIR: Match directories only; exclude regular and hidden files.
            if mode_clone.contains(FileMode::WITH_DIR) {
                if !entry.file_type.map_or(true, |file_kind| file_kind.is_dir()) {
                    return WalkState::Continue;
                } 
            }

            // FUZZY: Match files by stem name.
            if mode_clone.contains(FileMode::FUZZY) {
                let current_stem = entry.file_stem().to_string_lossy();
                if file_name != current_stem {
                    return WalkState::Continue;
                }
            }

            // CASE_INSENSITIVE: Perform case-insensitive filename comparison.
            let current_name = entry.file_name.to_string_lossy();
            if mode_clone.contains(FileMode::CASE_INSENSITIVE) {
                if file_name.to_lowercase() != current_name.to_lowercase() {
                    return WalkState::Continue;
                }
            } else {
                if file_name != current_name {
                    return WalkState::Continue;
                }
            }

            tx_clone.send(PipelineMessage::Data(
                FileResult {
                    paths: Some(entry.path().to_path_buf()),
                    errors: None,
                })).unwrap();
        });
    }
}
