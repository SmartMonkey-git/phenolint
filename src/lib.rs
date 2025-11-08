mod config;
pub mod diagnostics;
pub mod enums;
pub mod error;
pub mod linter_policy;
pub mod patcher;
pub mod phenolinter;
pub mod rules;
pub mod traits;

pub mod linter_context;
pub use linter_context::LinterContext;
mod json;
#[cfg(test)]
pub(crate) mod test_utils;
