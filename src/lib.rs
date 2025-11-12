mod config;
pub mod diagnostics;
pub mod enums;
pub mod error;
pub mod patcher;
pub mod phenolinter;
pub use phenolinter::Phenolinter;
pub mod rules;
pub mod traits;
pub use traits::*;

pub mod linter_context;
pub use linter_context::LinterContext;
mod json;

mod new;
#[cfg(test)]
pub(crate) mod test_utils;
