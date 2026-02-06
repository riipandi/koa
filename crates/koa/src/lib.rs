//! Koa Programming Language Compiler
//!
//! Koa is a modern compiled programming language with:
//! - TypeScript-familiar syntax
//! - Concurrent mark-sweep GC
//! - Error sets and error unions
//! - Built-in async/await

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod typeck;
pub mod ir;
pub mod llvm_gen;

pub use ast::Ast;
pub use lexer::{Token, TokenKind, Lexer};
pub use parser::Parser;
pub use typeck::TypeChecker;

/// Koa compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
