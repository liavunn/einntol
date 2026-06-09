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
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tokio::sync::mpsc::Sender;
use tokio::sync::watch;

use crate::monitor::generic_monitor_cli::signal_monitor_cli::monitor_task_commands;
use crate::models::generic_pipeline::{
    PipelineMessage,
};

/// 
pub async fn start_cli(einntol_quit_signal_rx: watch::Receiver, task_stop_signal: Arc<AtomicBool>, tx: Sender<PipelineMessage>) {
    println!("[EinnTol] Hi, einntol initialized.");

    tokio::selesct! {
        _ = signal::ctrl_c() => {
            einntol_quit_signal_rx.send(true);
            break;
        }
    }

   ||| monitor_commands(einntol_quit_signal, task_stop_signal, tx);
    
}
