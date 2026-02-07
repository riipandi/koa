//! Symbol table for type checking
//!
//! This module contains the symbol table implementation used by the type checker.

use std::collections::HashMap;

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub type_: crate::ast::Type,
    pub is_const: bool,
}

/// A scope containing symbols
#[derive(Debug, Default)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}
