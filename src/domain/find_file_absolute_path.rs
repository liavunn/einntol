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

use std::thread::scope;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use miette;
use crossbeam_channel::{bounded, Sender, Receiver};

use einntol::utils::{
    file_utils_cli,
    generic_utils_cli
};
use einntol::file_tools;
use einntol::PipelineMessage;
use einntol::PipelineStatus;

/// Main
fn find_file_path() {
    scope (|s| {
        s.spawn (|| {
            let counter = Arc::new(AtomicBool::new(0));
            let unchecked_paths = file_utils_cli::get_unchecked_paths_cli();
            let (file_mode, app_err) = file_utils_cli::get_file_mode();
            let name = generic_utils_cli::get_name();

            s.spawn (|| {
                generic_utils_cli::monitor_commands(&counter, &tx);
            }); 

            match unchecked_paths.errors {
                Some(errors) if !errors.is_empty() => 
                    for unchecked_path in errors.iter() {
                        println!("Bad path: {}", unchecked_path);
                    },

                _ => println!("All paths are valid."),
            }

            match unchecked_paths.paths {
                Some(paths) => file_tools::scan_and_find(paths, name, file_mode, counter, tx),

                _ => println!("No valid paths available."),
            }
        });

        // 
        while let Ok(message) = rx.recv() {
            match message {
                PipelineMessage::Data(result) => {
                    if let Some(message_path) = result.paths {
                        println!("Found: ");
                        println!("{}", message_path);
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
                            println!("Search aborted."),

                        PipelineMessage::Finished =>
                            println!("Search completed."),
                    }
                }
            }
        }
    });
}
