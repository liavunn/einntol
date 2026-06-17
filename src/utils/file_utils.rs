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

use std::path::PathBuf;
use std::sync::Arc;
use std::mem;

use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkBuilder, WalkState, DirEntry};
use tokio::sync::mpsc::Sender;
use tokio::sync::watch;

use crate::errors::AppError;
use crate::modes::file_modes::{
    FileMode,
    FilterStrategy,
};
use crate::models::pipeline_outcome::ResultOutcome;
use crate::models::file_pipeline_data::{
    FileResultPaths,
    FileResultErrors,
};
use crate::PipelineMessage;
use crate::PipelineStatus;

struct FileVisitorBuilder {
    tx: Sender<PipelineMessage>,
    task_stop_signal_rx: watch::Receiver<bool>,
    mode: FileMode,
    name: Arc<str>,
    datas_vec: Vec<PathBuf>,
    errors_vec: Vec<AppError>,
    count: i32,
}

impl<'a> ParallelVisitorBuilder<'a> for FileVisitorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 'a> {
        Box::new(FileVisitor {
            tx: self.tx.clone(),
            task_stop_signal_rx: self.task_stop_signal_rx.clone(),
            mode: self.mode,
            name: self.name.clone(),
            datas_vec: mem::take(&mut self.datas_vec),
            errors_vec: mem::take(&mut self.errors_vec),
            count: self.count,
        })
    }
}

struct FileVisitor {
    name: Arc<str>,
    mode: FileMode,
    task_stop_signal_rx: watch::Receiver<bool>,
    tx: Sender<PipelineMessage>,
    datas_vec: Vec<PathBuf>,
    errors_vec: Vec<AppError>,
    count: i32,
}

impl ParallelVisitor for FileVisitor {
    fn visit(&mut self, entry: Result<DirEntry, ignore::Error>) -> WalkState {
        if (self.count & 63) == 0 {
            if *self.task_stop_signal_rx.borrow() {
                return WalkState::Quit;
            }
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

            self.errors_vec.push(app_err.into());

            if self.errors_vec.len() == 64 {
                let batch = mem::take(&mut self.errors_vec);

                self.tx.blocking_send(PipelineMessage::Data(
                    struct LogEntry {
        id: i64,
        tag: String,
        payload: String,
        boot_timastamp: i64,
        sesstion_id: i32,
        error_level: i32,
    }
    ResultOutcome::FileErrors(
                        FileResultErrors {
                            errors: batch,
                        }
                    )
                )).unwrap();
            }


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

        self.datas_vec.push(entry.path().to_path_buf());

        if self.datas_vec.len() == 64 {
            let batch = mem::take(&mut self.datas_vec);

            self.tx.blocking_send(PipelineMessage::Data(
                ResultOutcome::FilePaths(
                    FileResultPaths {
                        paths: batch,
                    }
                )
            )).unwrap();
        }

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
    task_stop_signal_rx: watch::Receiver<bool>,
    tx: &Sender<PipelineMessage>
) {
    let Some(first_determined_path) = determined_paths.first() else {
        return;
    };

    let mut builder = WalkBuilder::new(first_determined_path); 
    let arc_str_name = Arc::from(name);
    let mut datas_vec: Vec<PathBuf> = Vec::new();
    let mut errors_vec: Vec<AppError> = Vec::new();

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

    let count = 0;

    let mut visitor_builder = FileVisitorBuilder {
        task_stop_signal_rx: task_stop_signal_rx.clone(),
        tx: tx.clone(),
        mode,
        name: Arc::clone(&arc_str_name),
        datas_vec: mem::take(&mut datas_vec),
        errors_vec: mem::take(&mut errors_vec),
        count,
    };
 
    tx.blocking_send(PipelineMessage::Signal(
        PipelineStatus::Starting
    )).unwrap();

    parallel_walker.visit(&mut visitor_builder);

    if !visitor_builder.datas_vec.is_empty() {
        tx.blocking_send(PipelineMessage::Data(
            ResultOutcome::FilePaths(
                FileResultPaths{
                    paths: visitor_builder.datas_vec
                }
            )
        )).unwrap();
    }

    if !visitor_builder.errors_vec.is_empty() {
        tx.blocking_send(PipelineMessage::Data(
            ResultOutcome::FileErrors(
                FileResultErrors{
                    errors: visitor_builder.errors_vec
                }
            )
        )).unwrap();
    }

    tx.blocking_send(PipelineMessage::Signal(
        PipelineStatus::Finished
    )).unwrap();
}
