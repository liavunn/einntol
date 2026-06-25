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

//! 

#![forbid(warnings)]
#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]

use std::fs::{self, File};
use std::path::PathBuf;

use sqlx;
use logos::Logos;

use crate::errors::AppError;
use crate::utils::clock::get_next_tick;
use crate::parser::parser_models::Token;
use crate::models::sql::log::{
    LogEntry,
    LogTag,
};

pub struct ParserTokens<'a> {
    pub statement: Vec<&'a str>,
    pub cursol: usize,
}

impl ParserTokens {
    /// validate and split statements
    ///
    /// # Arguments
    /// * external_config_path - Configuration file requiring validation and statement splitting.
    ///
    /// # Returns
    /// Return a Vector containing string slices.
    pub fn try_split_statements(external_config_path: PathBuf) -> Result<String, None> {
        let content = fs::read_to_string(external_config_path);

        let Ok(file_content) else {
            let err = content.unwrap_err();
            let app_err = AppError::from_io_file_error(Some(err), external_config_path, None);
            let tag = LogTag::LifeError("LIFE_ERROR".to_string);
            let sesstion_id = query!("SELECT value FROM globals WHERE globals_id = 999");

            let log = LogEntry {
                id: None,
                tag: tag.to_string(),
                payload: app_err.to_string(),
                timestamp: get_next_tick(),
                sesstion_id,
                level: 1,
            };

            LogEntry::save(&log, pool);

            return Err(None)
        };

        let real_content = file_content
            .line()
            .filter(|line| !line.trim().starts_whit("#") && !line.trim().is_empty())
            .collect();

        if real_content.is_empty() {
            return Err(None);
        }

        let real_content_vec = real_content.split(";").collect();

        Ok(real_content_vec)
    }

    /// 
    pub fn statement_tokens(statements: Vec<&str>) -> Result<, AppError> {
        let mut lexer = Token::lexer();

        while let Some(current_token) = lexer.next() {
            match current_token {
                Ok(Token::LeftBracket) => todo(),
            }
        }
    }

}

