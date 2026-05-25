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
use crossbeam_channel::bounded;

use crate::types::FileError;
use crate::types::SafetyLevel;
use crate::types::FileMode;
use crate::types::FileResult;
use crate::types::PipelineMessage; 

/// Find files in the input path that match the specified pattern and filename.
///
/// # Arguments
/// * `determined_path` - Target path to search.
/// * `name`            - Target filename to search.
/// * `mode`            - Configuration flags for the search mode.
/// * `stop_signal`     - Atomic flag for cancellation.
/// * `tx`              - Transmit results and status updates to the caller.
///
/// # Returns
/// * Returns a FileResult containing successfully validated and normalized paths,
/// * along with any non-fatal errors encountered during processing. 
pub fn find_paths(
    determined_paths: Vec<PathBuf>,
    name: &str,
    mode: FileMode,
    stop_signal: Arc<AtomicBool>,
    tx: Sender<PipelineMessage>
) {
    let mut builder = WalkBuilder::new(determined_paths); 

    // Add paths to the builder.
    for path in determined_paths.iter_into().skip(1) {
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
        let stop_signal_clone = Arc::clone(&stop_signal);
        let tx_clone = tx.clone();
        let mode_clone = mode.clone();
        let name_string = name.to_string();

        Box::new(move |result_path| {
            if stop_signal.load(Ordering::Relaxed) {
                return WalkState::Quit;
            }

            let Ok(entry) = result_path else {
                let err = result_path.unwrap_err();
                let err_path = err.path().map(|path| path.to_path_buf().unwrap_or_default());
                let app_err = AppError::from_io_file_error(std::io::Error::from(err), err_path);

                tx.send(PipelineMessage::Data(
                    FileResult {
                        paths: None,
                        errors: Some(app_err),
                    })).unwrap();

                return WalkState::Continue;
            };
            
            if filter_ignore_match() == false {
                return WlakStaate::Continue;
            }

            tx_clone.send(PipelineMessage::Data(
                FileResult {
                    paths: Some(entry.path().to_path_buf()),
                    errors: None,
                })).unwrap();
        });
    }
}

/// Validates file entries based on user-defined filtering modes.
/// While reporting error states to the pipeline in the event of missing metadata.
///
/// # Arguments
/// * `entry` - The file entry object provided by the ignore crate.
/// * `name`  - Target filename to search.
/// * `mode`  - Configuration flags for the search mode.
/// * `tx`    - transmit results and status updates to the caller.
///
/// # Returns
/// * Returns `true` If the entry matches the filtering criteria.
/// * Returns `false`  If the entry not matches the filtering criteria or if a non-fatal error occurred during processing.
fn filter_ignore_match(entry: &DirEntry, name: &str, mode: &FileMode, tx: Sender<PipelineMessage>) -> bool {
    let entry_type = entry.file_type();

    let entry_type = match entry_type {
        Some(ent_type) => ent_type,

        None => {
            let app_err = AppError::from_io_file_error(None, entry.path());
            tx.send(PipelineMessage::Data(
                FileResult {
                    paths: None,
                    errors: Some(app_err),
                }
            )).unwrap();
            return false;
        }
    }

    // NONE: Match regular files only; skip hidden files, directories, and paths ignored by .gitignore.
    if mode.contains(FileMode::NONE) && !entry_type.is_file() {
        return false;
    }

    // WITH_DIR: Match directories only; exclude regular and hidden files.
    if mode.contains(FileMode::WITH_DIR) && !entry_type.is_dir() {
        return false;
    }

    // FUZZY: Match files by stem name.
    if mode.contains(FileMode::FUZZY) {
        let current_stem = entry.file_stem().to_string_lossy();
        if name != current_stem {
            return false;
        }
    }

    // CASE_INSENSITIVE: Perform case-insensitive filename comparison.
    let current_name = entry.file_name().to_string_lossy();
    if mode.contains(FileMode::CASE_INSENSITIVE) && name.to_lowercase() != current_name.to_lowercase() {
        return false;
    } else if name != current_name {
        return false;
    }

true
}
