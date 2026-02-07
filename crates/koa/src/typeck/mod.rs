//! Type checker - validates type correctness
//!
//! The type checker ensures that all expressions and statements have correct types.

mod checker;
mod symbol;

pub use checker::TypeChecker;
pub use symbol::{Scope, Symbol};
