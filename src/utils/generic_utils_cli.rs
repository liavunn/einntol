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

//! Generic CLI Utilities.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crossbeam_channel::Sender;

use crate::AppError;
use crate::PipelineMessage;
use crate::PipelineStatus;

/// Monitor input and send a signal upon detecting a command.
///
/// # Arguments
/// * No arguments required.
///
/// # Returns
/// * Returns nothing.
///
/// # Panics
/// * This function will panic if the pipeline message sending fails.
pub fn monitor_commands(stop_signal: &Arc<AtomicBool>, tx: &Sender<PipelineMessage>) {
    let mut input = String::new();

    loop{
        println!("Enter 'stop' to terminate the task.");

        input.clear();
        if let Err(err) = io::stdin().read_line(&mut input) {
            AppError::from_io_generic_error(Some(err), None);
            continue;
        }

        let input_trim = input.trim();

        match input_trim {
            "stop" => {
                stop_signal.store(true, Ordering::SeqCst);
                tx.send(PipelineMessage::Signal(
                    PipelineStatus::Aborted
                )).unwrap();
            }

            _ => {
                println!("Please enter a valid command.");
            }
        }
    }
}

/// Get a name from the user.
///
/// # Arguments
/// * No arguments required.
///
/// # Returns
/// * Returns a String.
/// 
/// # Errors
/// * This function will return an error if the standard input.
pub fn get_name() -> Result<String, AppError> {
    let mut input_name = String::new();

    println!("Please enter name:");

    if let Err(err) = io::stdin().read_line(&mut input_name) {
        AppError::from_io_generic_error(Some(err), None);
    }

    let input_name = input_name.trim();

    Ok(input_name.to_string())
}
