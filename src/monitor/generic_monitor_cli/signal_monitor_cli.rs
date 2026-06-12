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
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use tokio::spawn;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::sync::watch;

use crate::models::monitor_commands_cli::MonitorCommand;
use crate::models::monitor_signal::{
    StateChange,
    SignalName,
};

/// 
pub async fn monitor_mode(
    monitor_rx: mpsc::Sender<MonitorCommand>,
    state_channel_tx: mpsc::Sender<StateChange>,
    mut is_task_signal_rx: watch::Receiver<bool>,
) {
    let (monitor_einntol_commands_stop_signal_tx, monitor_einntol_commands_stop_signal_rx) = watch::channel(false);
    let (monitor_task_commands_stop_signal_tx,  monitor_task_commands_stop_signal_rx) = watch::channel(false);
    spawn(async move {
        loop {
            tokio::select! {
                _ = is_task_signal_rx.changed() => {
                    if *is_task_signal_rx.borrow() {
                        monitor_task_commands_stop_signal_tx.send(false).unwrap();
                        monitor_task_commands(state_channel_tx.clone(), monitor_task_commands_stop_signal_rx.clone()).await;
                        monitor_einntol_commands_stop_signal_tx.send(true).unwrap();
                    } else {
                        monitor_einntol_commands_stop_signal_tx.send(false).unwrap();
                        monitor_task_commands(state_channel_tx.clone(), monitor_einntol_commands_stop_signal_rx.clone()).await;
                        monitor_task_commands_stop_signal_tx.send(true).unwrap();
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
    state_channel_tx: mpsc::Sender<StateChange>,
    mut monitor_task_commands_stop_signal_rx: watch::Receiver<bool>,
) {
    let mut reader = BufReader::new(tokio::io::stdin()).lines();

    println!("Enter 'stop' to terminate the task.");

    println!("Entre \'bye\' to quit.");

    loop {
        tokio::select! {
            line = reader.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        match input.trim() {
                            "stop" => {
                                state_channel_tx.send(
                                    StateChange {
                                        signal_name: SignalName::TaskStopSignal,
                                        value: true,
                                    }
                                ).await.unwrap();
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

/// 
pub async fn monitor_einntol_commands(
    state_channel_tx: mpsc::Sender<StateChange>,
    mut monitor_einntol_commands_stop_signal_rx: watch::Receiver<bool>,
) {
    let mut reader = BufReader::new(tokio::io::stdin()).lines();

    println!("Entre \'bye\' to quit.");

    loop {
        tokio::select! {
            line = reader.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        match input.trim() {
                             "bye" => {
                                 state_channel_tx.send(StateChange {
                                     signal_name: SignalName::EinnTolQuitSignal,
                                     value: true,
                                 }).await.unwrap();
                             }

                             _ => {}
                        }
                    }

                    Ok(None) => break,

                    Err(_) => break,
                }
            }

            _stop = monitor_einntol_commands_stop_signal_rx.changed() => {
                if *monitor_einntol_commands_stop_signal_rx.borrow() {
                    break;
                }
            }
        }
    }
}
