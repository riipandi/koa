//! Koa Programming Language Compiler
//!
//! Koa is a general-purpose compiled programming language with:
//! - TypeScript-familiar syntax
//! - Automated memory management (Concurrent GC)
//! - Error sets and error unions
//! - Built-in async/await model

pub mod ast;
pub mod ir;
pub mod lexer;
pub mod llvm_gen;
pub mod parser;
pub mod typeck;

pub use ast::Ast;
pub use ir::{IrLowerer, IrProgram};
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::Parser;
pub use typeck::TypeChecker;

/// Koa compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
