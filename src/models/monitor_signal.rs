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

/// This struct is used to pass commands to the StateManager via channels.
pub struct StateChange {
    /// Signal identifier name.
    pub signal_name: SignalName,

    /// Target state of the signal.
    pub value: bool,
}

/// Represents the name of a signal used to control the state of the system.
pub enum SignalName {
    /// Triggers the global shutdown process of the system.
    EinnTolQuitSignal,

    /// Stops the specific task currently running.
    TaskStopSignal,

    /// Controls the overall startup or shutdown of the task monitoring system.
    IsTaskSignal,
}
