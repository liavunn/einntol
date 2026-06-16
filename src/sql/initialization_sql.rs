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

use sqlx::SqlitePool;
use sqlx::{
    query,
    query_as,
};

/// 
pub async fn initialization_tables_sql(pool: &SQLitePool) { 
    query(
        "CREATE TABLE IF NOT EXISTS logs.life_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tag TEXT NOT NULL,
            payload TEXT ,
            boot_timestamp INTEGER
        );"
    )
        .execute(pool)
        .await;

    query(
        "CREATE TABLE IF NOT EXISTS globals (
            globals_id INTEGER PRIMARY KEY,
            value TEXT NOT NULL
        );"
    )
        .execute(pool)
        .await;

    query(
        "CREATE TABLE IF NOT EXISTS setting (
            capability_id INTEGER PRIMARY KEY NOT NULL,
            value TAXT NOT NULL
        );"
    )
        .execute(pool)
        .await;

    let (test_tag, test_payload) = (String::new(), String::new());

    query("INSERT OR IGNORE INTO logs.life_logs(id, tag, payload, boot_timestamp) VALUES(?, ?, ?, ?)")
        .bind(0)
        .bind(test_tag)
        .bind(test_payload)
        .bind(None)
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
        query_as("SELECT payload FROM logs.life_logs WHERE id = 0;")
        .fetch_optional(pool)
        .await,
        query_as("SELECT value FROM globals WHERE globals_id = 0;")
        .fetch_optional(pool)
        .await,
        query_as("SELECT value FROM setting WHERE capability_id = 0;")
        .fetch_optional(pool)
        .await,
    );

    match tests {
        (Ok(Some(test_log)), Ok(Some(test_globals)), Ok(Some(test_setting))) => {
            assert_eq!(test_log.0, "test_payload");
            assert_eq!(test_globals.0, "test");
            assert_eq!(test_setting.0, "test");
        },

        _ => {
            eprintln!("Error: Database initialization failed");
            panic!();
        }
    }
}
