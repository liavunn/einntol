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

use tokio::sync::mpsc;
use tokio::sync::watch;

use crate::monitor::generic_monitor_cli::signal_monitor_cli::{
    monitor_mode,
};
use crate::models::monitor_commands_cli::MonitorCommand;
use crate::models::monitor_signal::{
    StateChange,
    SignalName,
};

/// 
pub async fn run_state_manager(
    monitor: mpsc::Sender<MonitorCommand>,
    einntol_quit_signal_tx: watch::Sender<bool>,
    is_task_signal_tx: watch:: Sender<bool>,
    is_task_signal_rx: watch:: Receiver<bool>,
    task_stop_signal_tx: watch::Sender<bool>,
    state_channel_tx: mpsc::Sender<StateChange>,
    state_channel_rx: &mut mpsc::Receiver<StateChange>,
) {
    monitor_mode(state_channel_tx, is_task_signal_rx.clone());

    while let Some(state) = state_channel_rx.recv().await {
        match state.signal_name {
            SignalName::EinnTolQuitSignal => {
                einntol_quit_signal_tx.send(state.value).unwrap();
            },

            SignalName::TaskStopSignal => {
                task_stop_signal_tx.send(state.value).unwrap();
            },

            SignalName::IsTaskSignal => {
                is_task_signal_tx.send(state.value).unwrap();
            },
        }
    }
}
 
