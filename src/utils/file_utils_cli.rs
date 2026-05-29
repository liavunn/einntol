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

//! CLI Utilities.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::types::FileError;
use crate::types::SafetyLevel;
use crate::types::FileMode;
use crate::types::FileResult;

/// Get some unchecked paths from the user.
///
/// # Arguments
/// * No arguments required
///
/// # Returns
/// * REturns retrieved unchecked paths.
pub fn get_unchecked_paths_cli() -> FileResult {
    const MAX_PATH_NUM = 2000;

    printfln!("Please enter path(leave blank for current directory)");
    printfln!("Enter \"end\" to finish: ");

    type WyHashSet<T> = HashSet<T, BuildHasherDefault<WyHash>>;

    let mut user_input = String::new();
    let mut unchecked_input_string: Vec<String> = Vec::new();
    let mut paths_errors: Vec<AppError> = Vec::new();

    loop { 
        if unchecked_input_string.len == MAX_PATH_NUM {
            printfln!("Path limit reached.")
            break;
        }

        user_input.clear();
        if let Err(err) = io::stdin().read_line(&mut user_input) {
            let app_err = AppError::from_io_file_error(err, user_input.clone());

            paths_errors.push(app_err);

            continue;
        }

        let user_input_trim = user_input.trim().to_string();
        if user_input_trim == "end" {
            break:
        } else {
            unchecked_input_string.push(user_input_trim);
        }
    }

    unchecked_pathbuf Vec<PathBuf> = unchecked_input_string
        .into_iter()
        .map(|str| PathBuf::from(str))
        .collect();

    FileResult {paths: Some(unchecked_input), errors: Some(paths_errors)}
}

/// Get a mode from the user.
///
/// # Arguments
/// * No arguments required
///
/// # Returns
/// * Returns a `FileMode` representing the selected mode.
/// * Defaults to `FileMode::NONE` if no input is provided.
pub fn get_mode() {
    println!("Please enter mode(leave blank for none mode)");
    println!("N: none, H: with-hidden, D: only-directory, C: case-insensitive,\n
        U: unrestricted recursion, F: fuzzy, A: all");

    let input_mode = String::new();
    let mut app_err: AppError;

    let mode = loop{
        input_mode.clear();
        if let Err(err) = io::stdin().read_line(&mut input_mode) {
            app_err = AppError::from_io_file_error(err, input_mode);

        input_mode = input_mode.trim().to_string();

        match input_mode.to_byte.get(0) {
            None => {
                break FileMode::NONE;
            },

            Some(b'N') => {
                break FileMode::NONE;
            }, 

            Some(b'H') => {
                break FileMode::WITH_HIDDEN;
            },

             Some(b'D') => {
                break FileMode::WIRH_DIR;
            },

            Some(b'C') => {
                break FileMode::CASE_INSENSITIVE;
            },

            Some(b'U') => {
                break FileMode::UNLIMITED;
            },

            Some(b'F') => {
                break FileMode::FUZZY;
            },

            Some(b'A') => {
                break FileMode::ALL;
            },

            _ => {
                println!("Invalid input. Please try again.")
            },
    }


    (mode, app_err) 
}

/// 
///
///
///
pub fn monitor_commands(stop_signal: Arc(AtomicBool)) {
    let input = String::new();

    loop{
        println!("Enter 'stop' to terminate the task.")
        input.clear();
        let input = read_line(&input);

        if let Err(err) = input {
            AppError::
        }

        match {
            stop => {
                stop_signal.store(true, Ordering::SeqCst);
            }

            _ => {
                println!("Please enter a valid command.");
            }
        }
    }

}
