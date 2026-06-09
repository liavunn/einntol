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

use tokio::spawn;

use tokio::signal;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use tokio::sync::watch;

use crate::PipelineMessage;
use crate::PipelineStatus;

/// 
pub async fn monitor_mode(
    is_task_signal_rx: watch::Receiver<bool>,
    einntol_quit_signal_rx: watch::Receiver<bool>,
    task_stop_signal_rx: watch::Receiver<bool>,
    monitor_task_commands_stop_signal_rx: watch::Receiver<bool>,
    tx: Sender<PipelineMessage>,
) {
    spawn(async move {
        loop {
            tokio::select! {
                _ = is_task_signal_rx.changed() => {
                    if *is_task_signal_rx.borrow() {
                        monitor_task_commands(einntol_quit_signal_rx, task_stop_signal_rx, monitor_task_commands_stop_signal_rx,tx.clone()).await;
                    } else {
                        einntol
                    }
                }    
            }
        }
    });
}

/// Monitor input and send a signal upon detecting a command.
///
/// # Arguments
/// * No arguments required.
///
/// # Returns
/// * Ret88urns nothing.
///
/// # Panics
/// * This function will panic if the pipeline message sending fails.
pub async fn monitor_task_commands(
    einntol_quit_signal_rx: watch::Receiver<bool>,
    task_stop_signal_rx: watch::Receiver<bool>,
    monitor_task_commands_stop_signal_rx: watch::Receiver<bool>,
    tx: Sender<PipelineMessage>
) {
    let mut reader = BufReader::new(tokio::io::stdin()).lines();

    println!("Enter 'stop' to terminate the task.");

    println!("Entre \'bye\' to quit.");

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                einntol_quit_signal_rx.send(true);
                break;
            }

            line = reader.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        match input.trim() {
                            "stop" => {
                                task_stop_signal_rx.send(true);
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

            _stop = monitor_task_commands_stop_signal_rx.changed() => {
                if *monitor_task_commands_stop_signal_rx.borrow() {
                  break;
                }
            }
        }
    }
}

pub async fn monitor_einntol_commands(einntol_quit_signal_rx: watch::Receiver) {
    let mut reader = BufReader::new(tokio::io::stdin()).lines();

    println!("Entre \'bye\' to quit.");

    loop {
        tokio::select! {
            line = reader.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        match input.trim() {
                             "bye" => {
                                 einntol_quit_signal_rx.send(true);
                             }
                        }
                    }
                }
            }
        }
    }
}
