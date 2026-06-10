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

use tokio::signal;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch;

use crate::models::generic_pipeline::{
    PipelineMessage,
};

/// 
pub async fn start_cli(
    einntol_quit_signal_tx: watch::Sender<bool>,
    einntol_quit_signal_rx: watch::Receiver<bool>,
    is_task_signal_tx: watch:: Sender<bool>,
    is_task_signal_rx: watch::Receiver<bool>,
    task_stop_signal_tx: watch::Sender<bool>,
    task_stop_signal_rx: watch::Receiver<bool>,
    tx: Sender<PipelineMessage>
) {
    println!("[EinnTol] Hi, einntol initialized.");

    state_manager(einntol);

    loop{
        tokio::select! {
            _ = signal::ctrl_c() => {
                einntol_quit_signal_tx.send(true);
                break;
            }
        }
    }
}
