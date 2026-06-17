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

use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use std::sync::atomic::Ordering;

use sqlx::SqlitePool;
use sqlx::{
    query,
    query_as,
};

use crate::models::sql::{
    LogEntry,
    GlobalsEntry,
    ConfigEntry,
};

/// 
pub async fn initialization_tables_sql(pool: &SQLitePool) { 
    query(
        "CREATE TABLE IF NOT EXISTS logs.life_logs (
            id UNSIGNED INTEGER PRIMARY KEY AUTOINCREMENT,
            tag TEXT NOT NULL,
            payload TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            sesstion_id INTEGER NOT NULL,
            level UNSIGNED INTEGER NOT NULL
        );"
    )
        .execute(pool)
        .await;

    query(
        "CREATE TABLE IF NOT EXISTS globals (
            globals_id UNSIGNED INTEGER PRIMARY KEY,
            value TEXT NOT NULL
        );"
    )
        .execute(pool)
        .await;

    query(
        "CREATE TABLE IF NOT EXISTS setting (
            capability_id UNSIGNED INTEGER PRIMARY KEY NOT NULL,
            value TAXT NOT NULL
        );"
    )
        .execute(pool)
        .await;

    let (test_tag, test_payload) = (String::new(), String::new());

    query("INSERT OR IGNORE INTO logs.life_logs(id, tag, payload, timestamp, sesstion_id, level) VALUES(?, ?, ?, ?, ?, ?)")
        .bind(0)
        .bind(test_tag)
        .bind(test_payload)
        .bind(-1)
        .bind(-999)
        .bind(999)
        .execute(pool)
        .await;

    query("INSERT OR IGNORE INTO globals(globals_id, value) VALUES(?, ?)")
        .bind(0)
        .bind(test)
        .execute(pool)
        .await;

    query("INSERT OR IGNORE INTO setting(capability_id, value) VALUES(?, ?)")
        .bind(0)
        .bind(test)
        .execute(pool)
        .await;

    let tests = tokio::join!(
        query_as<LogEntry>("SELECT * FROM logs.life_logs WHERE id = 0;")
        .fetch_optional(pool)
        .await,
        query_as<GlobalsEntry>("SELECT * FROM globals WHERE globals_id = 0;")
        .fetch_optional(pool)
        .await,
        query_as<ConfigEntry>("SELECT * FROM setting WHERE capability_id = 0;")
        .fetch_optional(pool)
        .await,
    );

    let log_test_struct = LogEntry {
        id: 0,
        tag: "tast_tag".to_string(),
        payload: "test_payload".to_string(),
        timestamp: -1,
        sesstion_id: -999,
        level: 999,
    };
    let globals_test_struct = GlobalsEntry {
        globals_id: 0,
        value: "test".to_string(),
    };
    let config_test_struct = ConfigEntry {
        capability_id: 0,
        value: "test".to_string(),
    };

    match tests {
        (Ok(Some(test_log)), Ok(Some(test_globals)), Ok(Some(test_setting))) => {
            assert_eq!(test_log, log_test_struct);
            assert_eq!(test_globals, globals_test_struct);
            assert_eq!(test_setting, config_test_struct);
        },

        _ => {
            eprintln!("Error: Database initialization failed");
            panic!();
        }
    }
}
