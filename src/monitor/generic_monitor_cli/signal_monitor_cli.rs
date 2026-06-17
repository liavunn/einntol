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

use tokio::spawn;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::sync::watch;

use crate::models::monitor_commands_cli::MonitorCommandCLI;
use crate::models::state_signal::{
    StateChange,
    SignalName,
};

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
pub async fn monitor_commands(
    moniotr_channel_rx: &mut mpsc::Receiver<MonitorCommandCLI>,
    state_channel_tx: mpsc::Sender<StateChange>,
    mut monitor_task_commands_stop_signal_rx: watch::Receiver<bool>,
) {
    println!("Enter 'stop' to terminate the task.");

    loop {
        tokio::select! {
            line = moniotr_channel_rx.revc() => {
                match line {
                    MonitorCommandCLI::Stop => {
                        state_channel_tx.send(
                            StateChange {
                                signal_name: SignalName::TaskStopSignal,
                                value: true,
                            }
                        ).await.unwrap();
                    }

                    MonitorCommandCLI::EinnTolQuit => {
                        state_channel_tx.send(StateChange {
                            signal_name: SignalName::EinnTolQuitSignal,
                            value: true,
                        }).await.unwrap();
                    }

                    MonitorCommandCLI::Other => println!("Please enter a valid command."),
                }
            }
        }
    }
}
