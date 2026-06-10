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
use tokio::sync::watch;

use einntol::utils::{
    file_utils_cli,
    generic_utils_cli
};
use einntol::file_tools;
use einntol::PipelineMessage;
use einntol::PipelineStatus;
use einntol::cli::start_cli::start_cli;

/// run_cil
#[tokio::main]
async fn run_cli() -> miette::Result<()> {
    let (start_tx, start_rx) = chennel::<PipelineMessage>(2048);
    let (mut is_task_signal_tx, mut is_task_signal_rx) = watch::channel(false);
    let (mut einntol_quit_signal_tx, mut einntol_quit_signal_rx) = watch::channel(false);
    let (mut task_stop_signal_tx, mut task_stop_signal_rx) = watch::channel(false);

    start_cli(
        einntol_quit_signal_tx,
        einntol_quit_signal_rx.clone(),
        is_task_signal_tx,
        is_task_signal_rx,
        task_stop_signal_tx,
        task_stop_signal_rx,
        tx.clone()
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
