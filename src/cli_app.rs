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

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::thead::scope;

use anyhow;
use fastrand;
use wyhash;
use crossbeam_channel::unbounded;

use crate::types::FileError;
use crate::errors::file_errors_conversion::from_io_file_error;
use crate::types::SafetyLevel;
use crate::types::FileMode;
use crate::types::FileResult;

/// Main
fn main() -> miette::Result<()> {
    let (tx, rx) = unbounded<PipelineMesage>();df

    println!("Hi, einntol initialized.");

    println!("Entre \'bye\' to quit.");

    scope (|s| {
        s.spawn {|| {
            let unchecked_paths = get_unchecked_paths_cli();

            match unchecked_paths.errors {
                Some(errors) if errors.is_empty() => 
                    for unchecked_path in errors.iter {
                        println!("");
                    },

                _ => println!("");,
            }
                    
            scan_and_find()
        }}
    })

    loop {
    // 
    
    }
}
