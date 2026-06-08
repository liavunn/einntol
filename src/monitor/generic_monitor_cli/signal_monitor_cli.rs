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

#![forbid(clippy::pedantic)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tokio::signal;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;

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
pub async fn monitor_commands(einntol_quit_signal: Arc<AtomicBool>, task_stop_signal: Arc<AtomicBool>, tx: Sender<PipelineMessage>) {
    println!("Enter 'stop' to terminate the task.");

    println!("Entre \'bye\' to quit.");

    let mut reader = BufReader::new(tokio::io::stdin()).lines();

  
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Byebye!");
                einntol_quit_signal.store(true, Ordering::SeqCst);
                break;
            }

            line = reader.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        match input.trim() {
                            "bye" => {
                                 einntol_quit_signal.store(true, Ordering::SeqCst);
                                 println!("Byebye!");
                            }

                            "stop" => {
                                task_stop_signal.store(true, Ordering::SeqCst);
                                tx.send(PipelineMessage::Signal(
                                PipelineStatus::Aborted
                            )).await.unwrap();
                            }

                            _ => println!("Please enter a valid command."),
                        }
                    },

                    Ok(None) => break,

                    Err(_) => break,
                }
            }
        }
    }
}
