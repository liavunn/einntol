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
use tokio::sync::mpsc::Sender;

use crate::errors::AppError;
use crate::modes::file_modes::{
    FileMode,
    FilterStrategy,
};
use crate::models::pipeline_outcome::ResultOutcome;
use crate::models::file_pipeline_data::{
    FileResultPaths,
    FileResultErrors
};
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
                ResultOutcome::Errors(
                    FileResultErrors {
                        errors: vec![app_err.into()],
                    })
                )).unwrap();

            return WalkState::Continue;
        };

        let mut mode_strategy = Vec::new();

        for mode_item in self.mode.iter() { 
            mode_strategy.push(match mode_item {
                FileMode::NONE => FilterStrategy::None(self.name.clone()),

                FileMode::ONLY_DIR => FilterStrategy::OnlyDir(self.name.clone()),

                FileMode::CASE_INSENSITIVE => FilterStrategy::CaseInsensitive(self.name.clone()),

                FileMode::FUZZY => FilterStrategy::Fuzzy(self.name.clone()),

                _ => continue,
            });
        }

        for strategy in mode_strategy {
            let strategy_bool = FilterStrategy::matches(&strategy, &entry);

            if !strategy_bool {
                return WalkState::Continue;
            } 
        }

            self.tx.send(PipelineMessage::Data(
                ResultOutcome::Datas(
                    FileResultPaths {
                        paths: vec![entry.path().to_path_buf()],
                    }
                )
            )).unwrap();

        WalkState::Continue
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

