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
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]

use std::path::PathBuf;

use crate::errors::AppError;
use crate::impl_struct_debug_to_display;

/// This struct is used to pass commands to the StateManager via channels.
pub struct StateChange {
    /// Signal identifier name.
    pub signal_name: SignalName,

    /// Target state of the signal.
    pub value: bool,
}

pub enum SignalName {
    /// Triggers the global shutdown process of the system.
    einntol_quit_signal,

    /// Stops the specific task currently running.
    task_stop_signal,

    /// Stops the command sender module and pauses monitoring of task commands.
    monitor_task_commands_stop_signal,

    /// Stops the execution of the EINNTOL command monitor.
    monitor_einntol_commands_stop_signal,

    /// Controls the overall startup or shutdown of the task monitoring system.
    is_task_signal,
}
