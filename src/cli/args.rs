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

//! File manipulation tools.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

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
    /// 
    #[arg(long = "globals", default_value = "config/globals.db")]
    pub globals_path_sql: PathBuf,

    /// 
    #[arg(long = "config-path", default_value = "config/configs.db")]
    pub config_path_sql: PathBuf,

    /// 
    #[arg(long = "log-path", default_value = "log/life_logs.db")]
    pub log_path_sql: PathBuf,

    /// 
    #[arg(long = "external_config")]
    pub external_config_toml: Option<PathBuf>,

    /// 
    #[arg(long)]
    pub debug: Option<bool>,

    /// 
    #[arg(long = "last-chain", default_value_t = false)]
    pub last_chain: bool,
}
