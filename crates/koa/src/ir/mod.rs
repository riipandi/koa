//! Intermediate Representation (IR)
//!
//! The IR is a simplified representation of the AST optimized for code generation.

mod lower;
mod types;

pub use lower::IrLowerer;
pub use types::*;
