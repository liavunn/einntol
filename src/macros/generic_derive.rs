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

/// 
#[macro_export]
macro_rules! impl_struct_debug_to_display {
    ($(($type_name:ty, $($field:ident),+)),* $(,)?) => {
        $(
            impl std::fmt::Display for $type_name {
                fn fmt(&self, forma: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(forma, "{}", [
                        $(format!("{:?}", self.$field)),+
                    ].join(", "))
                }
            }
        )*
    };
}
