//! Koa Programming Language Compiler
//!
//! Koa is a modern compiled programming language with:
//! - TypeScript-familiar syntax
//! - Concurrent mark-sweep GC
//! - Error sets and error unions
//! - Built-in async/await

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
