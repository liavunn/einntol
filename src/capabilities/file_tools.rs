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

//! 

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::io;

use crate::types::FileError;
use crate::types::SafetyLevel;
use crate::types::FileMode;
use crate::types::FileResult;

/// Gets the path from user input.
///
/// # Arguments
/// * `input_vec_path` - A vector of paths to be validated and processed.
///
/// # Returns
/// * Returns a FileResult containing successfully validated and normalized paths,
/// * along with any non-fatal errors encountered during processing. 
 pub fn scan_and_find(input_vec_paths: Vec<PathBuf>,name: &str, mode: bitflags, stop_signal: Arc<ActomicBool>, tx: Sender<PipelineMessage>) {
    if input_vec_paths.is_empty() {
        tx.send(PipelineMesage::PipelineStatus::Finisied).unwarp();
        return;
    }

    let mut pending_paths = Vec::with_capacity(64);

    for input_path in input_vec_path.into_iter() {
        if stop_signal.load(Ordering::SeqCst) == true {
            tx.send(PipelineMessage::Singnal(
                PipelineStatus::Finished
            )).unwarp();
        }

        if pending_paths.len() == 64 {
            tx.send(PipelineMessage::Singnal(
                PipelineStatus::Pregress
            )).unwarp();

            find_paths(pending_paths, name, mode.clone());

            pending_paths = Vec::with_capacity(64);
        }

        let path_status = std::fs::metadata(&input_path)
            .map(|_| input_path)

        let Ok(current_path) = path_status else {
            let err = path_status.unwarp_err();
            let app_err = AppError::from_io_file_error(err, input_path);
            tx.send(PipelineMessage::Data(
                FileResult {
                    path: None,
                    error: app_err
                }
            )).unwarp();

            continue;
        }

        pending_paths.push(current_path)
    }

    if !pending_paths.is_empty() {
        tx.send(PipelineMessage::Singnal(
            PipelineStatus::Pregress
        ));
        find_paths(pending_paths);
    }
}

