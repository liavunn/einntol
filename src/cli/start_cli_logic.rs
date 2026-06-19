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

//! EinnTol CLI logic.

#![deny(warnings)]
#![deny(clippy::pedantic)]
#![deny(clippy::all)]

use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::watch;
use sqlx::SQLitePool;

use crate::models::generic_pipeline::PipelineMessage;
use crate::models::monitor_signal::StateChange;
use crate::monitor::generic_monitor_cli::state_manager::run_state_manager;
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
pub async fn start_cli(
    pool_sql: SqLitePool,
    parser_channel: ParserChannelCLI,
    monitor_channel: MonitorCommandCLI,
    einntol_quit_signal: EinnTolQuitSignal,
    is_task_signal: IsTaskSignal,
    task_stop_signal: TaskStopSignal,
    state_channel: StateChannel,
) {
    println!("[EinnTol] Hi, einntol initialized.");

    tokio::spawn (async move {
        reader_manager(
            monitor_channel.tx,
            parser_channel.tx,
        );
    });

    tokio::spawn (async move {
        run_state_manager(
            monitor_channel.rx,
            is_task_signal,
            state_channel,
            einntol_quit_signal.tx,
            task_stop_signal.tx,

        );
    });
}
