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

//! The entry point of the Einntol GUI program, responsible for environment initialization.

#![deny(warnings)]
#![deny(clippy::pedantic)]
#![deny(clippy::all)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::thread::scope;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use miette;
use crossbeam_channel::{bounded, Sender, Receiver};

use einntol::utils::{
    file_utils_cli,
    generic_utils_cli
};
use einntol::file_tools;
use einntol::PipelineMessage;
use einntol::PipelineStatus;

/// Main
fn run_cli() -> miette::Result<()> {
    todo;
    OK(())
}
