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
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use std::thread::scope;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use miette;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::{channel};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::watch;

use einntol::file_tools;
use einntol::PipelineMessage;
use einntol::PipelineStatus;
use einntol::cli::start_cli_logic::start_cli;
use einntol::models::monitor_signal::StateChange;
use einntol::models::monitor_commands_cli::MonitorCommandCLI;
use einntol::models::generic_bundles::{
    StateChannel
};
use einntol::models::bundles_cli::{
    ParserChannelCLI,
    MonitorChannelCLI,
};
use einntol::utils::{
    file_utils_cli,
    generic_utils_cli
};


/// run_cil
#[tokio::main]
async fn main() -> miette::Result<()> {
    let mut reader = BufReader::new(tokio::io::stdin()).lines();

    let (parser_channel_tx, parser_channel_rx) = channel::<String>(2048);
    let parser_channel = ParserChannelCLI {
        tx: parser_channel_tx,
        rx: parser_channel_rx
    };

    let (monitor_channel_tx, monitor_channel_rx) = channel::<MonitorCommandCLI>(2048);
    let monitor_channel = MonitorChannelCLI {
        tx: monitor_channel_tx,
        rx: monitor_channel_rx,
    }

    let (state_channel_tx, state_channel_rx) = channel::<StateChange>(2048);
    let state_channel = StateChannel {
        tx: state_channel_tx,
        rx: state_channel_rx,
    }

    let (mut is_task_signal_tx, mut is_task_signal_rx) = watch::channel(false);
    let (mut einntol_quit_signal_tx, mut einntol_quit_signal_rx) = watch::channel(false);
    let (mut task_stop_signal_tx, mut task_stop_signal_rx) = watch::channel(false);

    start_cli(
        parser_tx,
        parser_rx,
        monitor_tx,
        monitor_rx,
        einntol_quit_signal_tx,
        einntol_quit_signal_rx.clone(),
        is_task_signal_tx,
        is_task_signal_rx,
        task_stop_signal_tx,
        task_stop_signal_rx,
        state_channel_tx,
        state_channel_rx,
        start_tx.clone(),
        start_rx.clone(),
    ).await;

    loop {
        tokio::select! {
            _ = einntol_quit_signal_rx.changed() => {
                if *einntol_quit_signal_rx.borrow() {
                    println!("[EinnTol] Byebye!");
                    break;
                }
            }
        }
    }

    Ok(())
}
