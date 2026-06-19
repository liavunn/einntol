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

//! Command-Line Interface Parameters.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]

use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use std::sync::atomic::Ordering;

use clap::Parser;

use crate::errors::AppError;

#[derive(clap::Parser, Debug)]
#[command(version = "1.0", about = "EinnTol")]
pub struct Args {
    /// Log database path.
    #[arg(long = "log-path", default_value = "log/life_logs.db", help = "Path to the log database")]
    pub log_path_sql: PathBuf,

    /// Global configuration database path.
    #[arg(long = "globals", default_value = "config/globals.db", help = "Global configuration database path")]
    pub globals_path_sql: PathBuf,

    /// Configuration database path.
    #[arg(long = "config-path", default_value = "config/configs.db", help = "Configuration database path")]
    pub config_path_sql: PathBuf,

    /// External user configuration path.
    #[arg(long = "external_config-path", help = "External user configuration path")]
    pub external_config_txt: Option<PathBuf>,

    /// Debug mode.
    #[arg(long, help = "Debug mode")]
    pub debug: Option<bool>,

    /// Use last configuration.
    #[arg(long = "last-config", default_value_t = false, help = "Use last configuration")]
    pub last_config: bool,
}
