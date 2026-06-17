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

//! The entry point of the Einntol GUI program, responsible for environment initialization.

#![deny(warnings)]
#![deny(clippy::pedantic)]
#![deny(clippy::all)]

use tokio::sync::mpsc::Receiver;

use crate::models::generic_pipeline::{
    PipelineMessage,
    PipelineStatus,
    ProgressData,
};
use crate::models::file_pipeline_data::{
    FileResultPaths,
    FileResultErrors,
};
use crate::models::pipeline_outcome::ResultOutcome;

/// 
pub fn handle_find_file_path(rx: Receiver<PipelineMessage>) {
    while let Ok(message) = rx.recv() {
        match message {
            PipelineMessage::Data(result) => {
                match result {
                    ResultOutcome::Datas(FileResultPaths {paths}) => {
                        println!("Found: ");
                        println!("{}", paths.first().unwrap().display());
                    }

                    ResultOutcome::Errors(FileResultErrors {errors}) => {
                        println!("Error: ");
                        println!("{}", errors.first().unwrap());
                    }
                }
            }

            PipelineMessage::Signal(status) => {
                match status {
                    PipelineStatus::Starting => 
                        println!("Starting search..."),

                    PipelineStatus::Progress(count) => {
                        match count {
                            ProgressData::Size(num) =>
                                println!("Items found: [{}]", num),

                            ProgressData::TotalSize(num) =>
                                println!("Total: [{}]", num),
                        }
                    }

                    PipelineStatus::Aborted =>
                        println!("Search aborted."),

                    PipelineStatus::Finished =>
                        println!("Search completed."),
                }
            }

            PipelineMessage::FatalError(_fatal_error) => {
                panic!();
            }
        }
    }
}
