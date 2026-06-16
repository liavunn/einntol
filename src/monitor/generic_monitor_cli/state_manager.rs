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
    monitor_commands,
};
use crate::models::state_signal::{
    StateChange,
    SignalName,
};
use crate::models::monitor_commands_cli::MonitorCommandCLI;
use crate::models::generic_bundles::{
    StateChannel,
    IsTaskSignal,
    EinnTolQuitSignal,
    TaskStopSignal,
};
use crate::models::bundles_cli::{
    ParserChannelCLI,
    MonitorChannelCLI,
};

/// 
pub async fn run_state_manager(
    monitor_channel_rx: watch::Receiver<MonitorCommandCLI>,
    is_task_signal: IsTaskSignal,
    state_channel: StateChannel,
    einntol_quit_signal_tx: watch::Sender<bool>,
    task_stop_signal_tx: watch::Sender<bool>,
) {
    monitor_commands(
        &mut *monitor_channel_rx,
        state_channel.tx,
        is_task_signal.rx.clone()
        );

    while let Some(state) = state_channel.rx.recv().await {
        match state.signal_name {
            SignalName::EinnTolQuitSignal => {
                einntol_quit_signal_tx.send(state.value).unwrap();
            },

            SignalName::TaskStopSignal => {
                task_stop_signal_tx.send(state.value).unwrap();
            },

            SignalName::IsTaskSignal => {
                is_task_signal.tx.send(state.value).unwrap();
            },
        }
    }
}
 
