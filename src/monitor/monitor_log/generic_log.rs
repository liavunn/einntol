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

use tokio::spawn;

use sqlx::query;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch;

use crate::utils::clock::get_next_clock;
use crate::models::sql::{
    LogEntry,
    GlobalsEntry,
    Config,
};

/// 
pub async fn moniotr_fatal_error_log(err: io::Error) {
    let fatal_error = LogEntry {
        id: None,
        tag: "FATAL_ERROR".to_string(),
        payload: err,
        timestamp: get_next_tick(),
        sesstion_id:
    }
    
    panic!();
}
