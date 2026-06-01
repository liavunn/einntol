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

//! File manipulation tools.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use std::sync::atomic::Ordering;

use crossbeam_channel::Sender;

use crate::AppError;
use crate::FileMode;
use crate::FileResult;
use crate::PipelineMessage;
use crate::PipelineStatus;
use crate::file_utils::find_paths;

/// Gets the path from user input.
///
/// # Arguments
/// * `input_vec_path` - A vector of paths to be validated and processed.
///
/// # Returns
/// * Returns a FileResult containing successfully validated and normalized paths,
/// * along with any non-fatal errors encountered during processing. 
 pub fn scan_and_find(input_vec_paths: Vec<PathBuf>,name: &str, mode: FileMode, stop_signal: Arc<AtomicBool>, tx: Sender<PipelineMessage>) {
    if input_vec_paths.is_empty() {
        tx.send(PipelineMessage::PipelineStatus::Finished).unwrap();
        return;
    }

    const BATCH_SIZE: usize = 64;
    let mut pending_paths = Vec::with_capacity(BATCH_SIZE);
    let mut processed_count: i64 = 0;

    for input_path in input_vec_paths.into_iter() {
        if stop_signal.load(Ordering::SeqCst) == true {
            tx.send(PipelineMessage::Signal(
                PipelineStatus::Aborted
            )).unwrap();
        }

        if pending_paths.len() == BATCH_SIZE {
            processed_count += BATCH_SIZE;
            tx.send(PipelineMessage::Signal(
                PipelineStatus::Progress(Some(processed_count))
            )).unwrap();

            find_paths(pending_paths, name, mode.clone());

            pending_paths = Vec::with_capacity(BATCH_SIZE);
        }

        let path_status = std::fs::metadata(&input_path)
            .map(|_| input_path);

        let Ok(current_path) = path_status else {
            let err = path_status.unwrap_err();
            let app_err = AppError::from_io_file_error(err, input_path);
            tx.send(PipelineMessage::Data(
                FileResult {
                    path: None,
                    error: app_err
                }
            )).unwrap();

            continue;
        };

        pending_paths.push(current_path)
    }

    if !pending_paths.is_empty() {
        processed_count += pending_paths.len();
        tx.send(PipelineMessage::Signal(
            PipelineStatus::Progress(Some(processed_count))
        ));
        find_paths(pending_paths);
    }
}

