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

use tokio::sync::mpsc;
use tokio::sync::watch;
use crate::models::monitor_commands_cli::MonitorCommandCLI;

/// Standard Input Line Reader.
pub type StdinLineReader = tokio::io::Lines<tokio::io::BufReader<tokio::io::Stdin>>;

/// 
pub async fn reader_manager(
    monitor_channel_tx: mpsc::Sender<MonitorCommandCLI>,
    parser_channel_tx: mpsc::Sender<String>,
) {
    let reader = StdinLineReader;

    loop {
        tokio::select! {
            line = reader.next_line() => {
                match line.split("||").map(|str| str.trim()).collect::<Vec<&str>>() {
                    ["bye"] => {
                        monitor_channel_tx.send(
                            MonitorCommandCLI::EinnTolQuit
                        ).unwrap();
                        break;
                    },

                    ["stop"] => {
                        monitor_channel_tx.send(
                            MonitorCommandCLI::Stop
                        ).unwrap();
                    },

                    [first, ref rest @ ..] if !first.is_empty() && !rest.is_empty() && rest.iter().all(|str| !str.is_empty()) => {
                        parser_channel_tx.send(
                            line
                        ).unwrap();
                    },

                    _ => {
                        monitor_channel_tx.send(
                            MonitorCommandCLI::Other
                        ).unwrap();
                    },
                }
            }
        }
    }
}
