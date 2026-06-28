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

    #[regex(r"[@\w+]", |leser| Types::into(lexer.slice()), priority = 4)]
    Type(Types),

    #[regex(r"[@\w+]", |leser| CalculateTypes::into(lexer.slice()), priority = 3)]
    CalculateType(CalculateTypes),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lexer| lexer.slice().to_string())]
    Ident(String),

    #[token("end")]
    End,
}

/// Enumeration type.
pub enum Types {
    /// A text stream.
    String,

    /// A integral value, limits loop iterations or parameters.
    Integer,

    /// Boolean value, parameter.
    Bool,

    /// Immutable constant.
    Const,

    /// Type error.
    Error(String),
}

impl Types {
    pub fn into(input_type: &str) -> Self {
        match input_type {
            "@String" => Types::String,

            "@Integer" => Types::Integer,

            "@Bool" => Types::Bool,

            "@Const" => Types::Const,

            _ => Types::Error(input_type.to_string()),
        }
    }
}

/// Enumeration calculate type.
pub enum CalculateTypes {
    /// Add two numbers.
    Add,

    /// Subtracts two numbers.
    Sub,

    /// Multiplies two numbers.
    Mul,

    /// Divides two numbers.
    Div,
}

impl CalculateTypes {
    pub fn into(input_type: &str) -> Self {
        match input_type {
            "@+" => CalculateTypes::Add,

            "@-" => CalculateTypes::Sub,

            "@*" => CalculateTypes::Mul,

            "@/" => CalculateTypes::Div,
        }
    }
}
