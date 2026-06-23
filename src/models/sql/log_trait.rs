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

//! Global configuration modes and flag definitions.

#![forbid(warnings)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::all)]

use std::io;

use crate::errors::AppError;
use crate::models::sql::log::Logentry;

pub trait LogSave {
    fn save_fatal_error_log(app_err: AppError) -> Result<(), AppError>;
}

impl LogSave for LogEntry {
    pub fn save_fatal_error_log(app_err: AppError) {
        let tag = LogTag::LifeFatal("LIFE_FATAL".to_string);
        let sesstion_id = query("SELECT value FROM globals WHERE globals_id = 999");

        let log = LogEntry {
            id: None,
            tag: tag.to_string(),
            payload: app_err.to_string(),
            timestamp: get_next_tick(),
            sesstion_id,
            level: 0,
        };

        LogEntry::save(&log, pool);

        panic!("Fatal Error: sevd log");
    }
}
