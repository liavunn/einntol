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

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,
    
    #[token(";")]
    Semicolon,

    #[regex(r"[@\w+]", |leser| lexer.slice().Types::into(), priority = 3)]
    Type(Types),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lexer| lexer.slice().to_string())]
    Ident(String),

    #[token("end")]
    End,
}

pub enum Types {
    /// A text stream.
    String,

    /// A integral value, limits loop iterations or parameters.
    Integer,

    /// Boolean value, parameter.
    Bool,

    /// Immutable constant.
    Const,
}

impl Types {
    pub fn into(input_type: &str) {
        match input_type {
            "String" => Types::String,
            "Integer" => Types::Integer,
            "Bool" => Types::Bool,
            "Const" => Types::Const,
        };
    }
}
