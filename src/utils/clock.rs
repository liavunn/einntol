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

//! File CLI utilities.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]

use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::AppError;
use crate::FileMode;
use crate::models::file_pipeline_data::FileResults;

/// Global logical clock for ordering events within a single session.
pub static GLOBALS_CLOCK: AtomicU64 = AtomicU64::new(0);

/// Generates the next unique logical timestamp.
pub fn get_next_tick() -> u64 {
    GLOBALS_CLOCK.fetch_add(1, Ordering::SeqCst) + 1
}
