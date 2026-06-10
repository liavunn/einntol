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

use crossbeam_channel::Sender;
use wyhash::WyHash;

use crate::errors::AppError;
use crate::modes::file_modes::FileMode;
use crate::models::pipeline_outcome::ResultOutcome;
use crate::models::file_pipeline_data::{
    FileResultErrors,
};
use crate::models::generic_pipeline::{
    PipelineMessage,
    PipelineStatus,
    ProgressData,
};
use crate::file_utils::find_paths;


