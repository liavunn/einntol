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
pub fn monitor_commands(stop_signal: Arc<AtomicBool>, tx: Sender<PipelineMessage>) {
    let input = String::new();

    loop{
        println!("Enter 'stop' to terminate the task.");

        input.clear();
        if let Err(err) = io::stdin().read_line(&mut input) {
            AppError::from_io_generic_errors(Some(err));
            continue;
        }

        let input_trim = input.trim();

        match input_trim {
            stop => {
                stop_signal.store(true, Ordering::SeqCst);
                tx.send(PipelineMessage::Signal(PipelineStatus::Aborted)).unwrap();
            }

            _ => {
                println!("Please enter a valid command.");
            }
        }
    }
}
