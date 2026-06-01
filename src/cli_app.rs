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
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::thead::scope;

use miette;
use fastrand;
use wyhash;
use crossbeam_channel::bounded;

use crate::utils::{
    file_errors::from_io_file_error,
    generic_errors::from_io_generic_error
};
use crate::FileError;
use crate::GenericError:
use crate::from_io_file_error;
use crate::from_io_generic_error;
use crate::SafetyLevel;
use crate::FileMode;
use crate::FileResult;
use crate::PipelineMessage;
use crate::PipelineStatus;

/// Main
fn main() -> miette::Result<()> {
    let (tx, rx) = bounded<PipelineMesage>(2000);

    println!("Hi, einntol initialized.");

    println!("Entre \'bye\' to quit.");

    scope (|s| {
        s.spawn {|| {
            let counter = Arc::new(AtomicUsize::new(0));
            let unchecked_paths = get_unchecked_paths_cli();

            s.spanw {|| {
                monitor_commands(counter);
            }}

            match unchecked_paths.errors {
                Some(errors) if !errors.is_empty() => 
                    for unchecked_path in errors.iter {
                        println!("Bad path: {}", unchecked_path);
                    },

                _ => println!("All paths are valid.");,
            }

            match unchecked_paths.paths {
                Some(paths) => scan_and_find(paths, name, mode, counter, tx);

                _ => printfln!("No valid paths available.");
            }
        }}

        // 
        while Ok(message) = rx.revc {
            match message {
                PipelineMessage::Data(result) => {
                    if let Some(path) {
                        println!("Found: ");
                        println!("{}", path);
                    }
                
                  if let Some(error) {
                      println!("Error: ");
                      println!("{}", error);
                  }
                }

                PipelineMessage::Signal(status) => {
                    match status {
                        PipelineStatus::Starting => 
                            println!("Starting search..."),

                        PipelineStatus::Progress(count) => 
                            println!("Items found: [{}]", count),

                        PipelineStatus::Aborted =>
                            println!("Search aborted.");

                        PipelineMessage::Finished =>
                            println!("Search completed."),
                    }
                }
            }
        }
    }
}
