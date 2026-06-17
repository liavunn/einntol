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
#![forbid(unsafe_code)]

use std::thread::scope;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::path::Path;

use miette;
use tokio::signal;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::{channel};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::watch;
use sqlx::{
    query,
    SqlitePoolOptions, 
};
use sqlite::SqliteConnectOptions;
use sqlx::Connection;
use clap::Parser;

use einntol::prelude::*;

/// run_cil
#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parser();

    let log_path = Path::new(args.log_path_sql);
    let globals_path = Path::new(args.globals_path_sql);
    let config_path = Path::new(args.config_path_sql);
    let external_config_path = Path::new(args.external_config_path);

    let Some(log_path_dir) = log_path.parent() else {
        eprintln!("Error: log file path error!");
        panic!();
    };
    let Some(globals_path_dir) = globals_path.parent() else {
        eprintln!("Error: globals config file path error");
        panic!();
    };
    let Some(config_path_dir) = config_path.parent() else {
        eprintln!("Error: config file path error");
        panic!();
    };
    let Some(external_config_path_dir) = external_config_path.parent() else {
        eprintln!("Error: external config file path error");
        panic!();
    };

    // Create directory if missing.
    std::fs::create_dir_all(log_path_dir)?;
    std::fs::create_dir_all(globals_path_dir)?;
    std::fs::create_dir_all(config_path_dir)?;
    std::fs::create_dir_all(external_config_path_dir)?;

    let log_path = args.log_path_sql.clone();

    let connect_options = SQLiteConnectOptions::new()
        .filename(args.config_path_sql)
        .create_if_missing(true);

    let pool_sql = SQLitePoolOptions::new()
        .after_connect(move |conn, _| {Box::pin(async move {
            let log_path_clone = log_path;
            let log_sql = format!("ATTACH DATABASE '{}' AS logs;", log_path_clone);
            query(log_sql)
                .execute(conn)
                .await?;
            Ok(())
        })})
        .connect_with(connect_options)
        .await?;

    initialization_tables_sql(&pool_sql);

    let (parser_channel_tx, parser_channel_rx) = channel::<String>(2048);
    let parser_channel = ParserChannelCLI {
        tx: parser_channel_tx,
        rx: parser_channel_rx,
    };

    let (monitor_channel_tx, monitor_channel_rx) = channel::<MonitorCommandCLI>(2048);
    let monitor_channel = MonitorChannelCLI {
        tx: monitor_channel_tx,
        rx: monitor_channel_rx,
    };

    let (state_channel_tx, state_channel_rx) = channel::<StateChange>(2048);
    let state_channel = StateChannel {
        tx: state_channel_tx,
        rx: state_channel_rx,
    };

    let (mut is_task_signal_tx, mut is_task_signal_rx) = watch::channel(false);
    let is_task_signal = IsTaskSignal {
        tx: is_task_signal_tx,
        rx: is_task_signal_rx,
    };

    let (mut einntol_quit_signal_tx, mut einntol_quit_signal_rx) = watch::channel(false);
    let einntol_quit_signal = EinnTolQuitSignal {
        tx: einntol_quit_signal_tx,
        rx: einntol_quit_signal_rx,
    };

    let (mut task_stop_signal_tx, mut task_stop_signal_rx) = watch::channel(false);
    let task_stop_signal = TaskStopSignal {
        tx: task_stop_signal_tx,
        rx: task_stop_signal_rx,
    };

    start_cli(
        pool_sql,
        parser_channel,
        monitor_channel,
        einntol_quit_signal,
        is_task_signal,
        task_stop_signal,
        state_channel,
    ).await;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                einntol_quit_signal.tx.send(true);
            }

            _ = einntol_quit_signal.rx.changed() => {
                if *einntol_quit_signal.rx.borrow() {
                    println!("[EinnTol] Byebye!");
                    break;
                }
            }
        }
    }

    Ok(())
}
