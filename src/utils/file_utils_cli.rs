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
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::io;
use std::path::PathBuf;

use wyhash::WyHash;

use crate::AppError;
use crate::FileMode;
use crate::FileResult;

/// Get some unchecked paths from the user.
///
/// # Arguments
/// * No arguments required.
///
/// # Returns
/// * REturns retrieved unchecked paths.
pub fn get_unchecked_paths_cli() -> FileResult {
    const MAX_PATH_NUM: usize = 2000;

    println!("Please enter path(leave blank for current directory)");
    println!("Enter \"end\" to finish: ");

    type WyHashSet<T> = HashSet<T, BuildHasherDefault<WyHash>>;

    let mut user_input = String::new();
    let mut unchecked_input_string: Vec<String> = Vec::new();
    let mut paths_errors: Vec<AppError> = Vec::new();

    loop { 
        if unchecked_input_string.len() == MAX_PATH_NUM {
            println!("Path limit reached.");
            break;
        }

        user_input.clear();
        if let Err(err) = io::stdin().read_line(&mut user_input) {
            let app_err = AppError::from_io_generic_error(Some(err), None);

            paths_errors.push(app_err.into());

            continue;
        }

        let user_input_trim = user_input.trim().to_string();
        if user_input_trim == "end" {
            break;
        }
        unchecked_input_string.push(user_input_trim);
    }

    let unchecked_pathbuf: Vec<PathBuf> = unchecked_input_string
        .into_iter()
        .map(|str| PathBuf::from(str))
        .collect();

    FileResult {paths: Some(unchecked_pathbuf), errors: Some(paths_errors)}
}

/// Get a mode from the user.
///
/// # Arguments
/// * No arguments required.
///
/// # Returns
/// * Returns a `FileMode` representing the selected mode.
/// * Defaults to `FileMode::NONE` if no input is provided.
pub fn get_mode() -> (FileMode, Option<Vec<AppError>>) {
    println!("Please enter mode(leave blank for none mode)");
    println!("N: none, H: with-hidden, D: only-directory, C: case-insensitive,\n
        U: unrestricted recursion, F: fuzzy, A: all");

    let mut input_mode = String::new();
    let mut app_err: Option<Vec<AppError>> = None;

    let mode = loop{
        input_mode.clear();
        if let Err(err) = io::stdin().read_line(&mut input_mode) {
            app_err = Some(vec![AppError::from_io_generic_error(Some(err), None).into()]);
        }

        input_mode = input_mode.trim().to_string();

        match input_mode.as_bytes().get(0) {
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
               break FileMode::WITH_DIR;
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
    };

    match app_err {
        Some(err) => (mode, Some(err)),

        None => (mode, None),
    }
}
