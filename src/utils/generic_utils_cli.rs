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

//! Generic CLI Utilities.

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::cargo)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use std::io;

use crate::AppError;

/// Get a name from the user.
///
/// # Arguments
/// * No arguments required.
///
/// # Returns
/// * Returns a String.
/// 
/// # Errors
/// * This function will return an error if the standard input.
pub fn get_name() -> Result<String, AppError> {
    let mut input_name = String::new();

    println!("Please enter name:");

    if let Err(err) = io::stdin().read_line(&mut input_name) {
        AppError::from_io_generic_error(Some(err), None);
    }

    let input_name = input_name.trim();

    Ok(input_name.to_string())
}
