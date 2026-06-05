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

#![deny(warnings)]
#![deny(clippy::pedantic)]
#![deny(clippy::all)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use crate::pipeline_outcome::ResultOutcome;

/// Represents a message sent through the pipeline, carrying either data or a control signal.
#[derive(Debug, Clone)]
pub enum PipelineMessage {
    /// `self::File`(Contains the processed file results and any associated non-fatal errors.
    Data(ResultOutcome),

    /// Carries pipeline lifecycle and progress status signals
    Signal(PipelineStatus),
}

/// Defines the operational states of the pipeline lifecycle.
#[derive(Debug, Clone)]
pub enum PipelineStatus {
    /// Initializing the process.
    Starting,
    
    /// Indicates ongoing progress, carrying an optional i64 translation.
    Progress(ProgressData),

    /// Execution was aborted.
    Aborted,
    
    /// Execution finished successfully.
    Finished,
}

/// Data carrier for pipeline progress
#[derive(Debug, Clone)]
pub enum ProgressData {
    /// Number of processed items.
    Size(usize),

    /// Total number of items.
    TotalSize(usize),
}


