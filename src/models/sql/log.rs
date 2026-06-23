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

use sqlx;
use sqlx::query;

/// Maps directly to the database log schema.
#[derive(Debug, PartialEq,sqlx::FromRow)]
pub struct LogEntry {
    /// Primary key.
    pub id: Option<i64>,

    /// Log category identifier.
    pub tag: String,

    /// The actual data paload.
    pub payload: String,

    /// Logical clock value.
    pub timestamp: u64,

    /// Unique session identifier for windowing.
    pub sesstion_id: i32,

    /// Severity level.
    pub level: i32,
}

pub enum LogTag {
    /// Program Heartbeat Started.
    LifeBirth(String),

    /// Heartbeat running.
    Life(String),

    /// Heartbeat stopped.
    LifeEnd(String),

    /// Module heartbeat started.
    LifeFuncStrat(String),

    /// Module heartbeat running.
    LifeFunc(String),

    ///Module heartbeat terminated.
    LifeFuncEnd(String),

    /// Heartbeat timeout.
    LifeDead(String),

    /// Module heartbeat timeout.
    LifeFuncDead(String),

    /// Fatal error.
    LifeFatal(String),

    /// Non-fatal error.
    LifeError(String),
}

impl LogEntry {
    pub async fn save(&self, pool: &SQLitePool) {
        query("INSERT INTO logs.life_logs (tag, payload, timestamp, seestion_id, level VALUES (?, ?, ?, ?, ?)")
            .bind(self.tag)
            .bind(self.payload)
            .bind(self.timestamp)
            .bind(self.sesstion_id)
            .bind(self.level)
            .execute(pool)
            .await;
    }
}

