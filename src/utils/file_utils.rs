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

//! General file utilities.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkBuilder, WalkState, DirEntry};
use crossbeam_channel::Sender;

use crate::AppError;
use crate::FileMode;
use crate::FileResult;
use crate::PipelineMessage;
use crate::PipelineStatus;

struct FileVisitorBuilder {
    tx: Sender<PipelineMessage>,
    stop_signal: Arc<AtomicBool>,
    mode: FileMode,
    name: Arc<str>,
}

impl<'a> ParallelVisitorBuilder<'a> for FileVisitorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 'a> {
        Box::new(FileVisitor {
            tx: self.tx.clone(),
            stop_signal: self.stop_signal.clone(),
            mode: self.mode,
            name: self.name.clone(),
        })
    }
}

struct FileVisitor {
    name: Arc<str>,
    mode: FileMode,
    stop_signal: Arc<AtomicBool>,
    tx: Sender<PipelineMessage>
}

impl ParallelVisitor for FileVisitor {
    fn visit(&mut self, entry: Result<DirEntry, ignore::Error>) -> WalkState {
        if self.stop_signal.load(Ordering::SeqCst) {
            return WalkState::Quit;
        }

        let Ok(entry) = entry else {
            let err_reason = entry.unwrap_err().to_string();
            let err_path = PathBuf::new();
            let app_err = AppError::from_io_file_error(
                Some(
                    std::io::Error::other(err_reason.clone())
                ),
                err_path,
                Some(err_reason)
            );

            self.tx.send(PipelineMessage::Data(
                FileResult {
                    paths: None,
                    errors: Some(vec![app_err.into()]),
                })).unwrap();

            return WalkState::Continue;
        };
            
        match filter_ignore_match(&entry, &self.name, self.mode) {
            Ok(is_match) => {
                if is_match {
                    self.tx.send(PipelineMessage::Data(
                    FileResult {
                        paths: Some(vec![entry.path().to_path_buf()]),
                        errors: None,
                    })).unwrap();
                }

                WalkState::Continue
            }

            Err(app_err) => {
                self.tx.send(PipelineMessage::Data(
                FileResult {
                    paths: None,
                    errors: Some(vec![app_err]),
                })).unwrap();
            
                WalkState::Continue
            }
        }
    }
}

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
/// * Returns a `FileResult` containing successfully validated and normalized paths,
/// * along with any non-fatal errors encountered during processing.
///
/// # Panics
/// * This function will panic if the pipeline message sending fails.
pub fn find_paths(
    determined_paths: Vec<PathBuf>,
    name: &str,
    mode: FileMode,
    stop_signal: &Arc<AtomicBool>,
    tx: &Sender<PipelineMessage>
) {
    let Some(first_determined_path) = determined_paths.first() else {
        return;
    };

    let mut builder = WalkBuilder::new(first_determined_path); 
    let arc_str_name = Arc::from(name);

    // Add paths to the builder.
    for path in determined_paths.into_iter().skip(1) {
         builder.add(path);
    }

    // Check if hidden files should be included.
    if mode.contains(FileMode::WITH_HIDDEN) {
        builder.hidden(false);
    }

    // Check if unrestricted recursion is enabled.
    let depth = if mode.contains(FileMode::UNLIMITED) {
        None 
    } else {
        Some(3)
    };

    builder.max_depth(depth);

    let parallel_walker = builder.build_parallel();

    let mut visitor_builder = FileVisitorBuilder {
        stop_signal: stop_signal.clone(),
        tx: tx.clone(),
        mode,
        name: Arc::clone(&arc_str_name),
    };
 
    tx.send(PipelineMessage::Signal(PipelineStatus::Starting)).unwrap();

    parallel_walker.visit(&mut visitor_builder);

    tx.send(PipelineMessage::Signal(PipelineStatus::Finished)).unwrap();
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
fn filter_ignore_match(entry: &DirEntry, name: &str, mode: FileMode) -> Result<bool, AppError> {
    let entry_type = entry.file_type();

    let Some(entry_type) = entry_type else { 
            let app_err = AppError::from_io_file_error(None, entry.path().to_path_buf(), None);
            return Err(app_err.into());
    };

    // NONE: Match regular files only; skip hidden files, directories, and paths ignored by .gitignore.
    if mode.contains(FileMode::NONE) && !entry_type.is_file() {
        return Ok(false);
    }

    // WITH_DIR: Match directories only; exclude regular and hidden files.
    if mode.contains(FileMode::WITH_DIR) && !entry_type.is_dir() {
        return Ok(false);
    }

    // FUZZY: Match files by stem name.
    if mode.contains(FileMode::FUZZY) {
        let current_stem = entry
            .path()
            .file_stem()
            .map(|str| str.to_string_lossy())
            .unwrap_or_default();

        if name != current_stem {
            return Ok(false);
        }
    }

    // CASE_INSENSITIVE: Perform case-insensitive filename comparison.
    let current_name = entry.file_name().to_string_lossy();
    if mode.contains(FileMode::CASE_INSENSITIVE) && name.to_lowercase() != current_name.to_lowercase() && name != current_name{
        return Ok(false);
    }

    Ok(true)
}
