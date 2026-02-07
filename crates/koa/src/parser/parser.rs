//! Parser implementation
//!
//! This module contains the Parser struct and helper functions.

use crate::lexer::{Span, Token, TokenKind};
use miette::Result;

/// Parser for Koa source code
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        if self.position >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.position]
        }
    }

    fn previous(&self) -> &Token {
        if self.position == 0 {
            &self.tokens[0]
        } else {
            &self.tokens[self.position - 1]
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == kind
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        if self.is_at_end() {
            None
        } else {
            Some(self.peek().kind)
        }
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume_token(&mut self, kind: TokenKind) -> Result<&Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(miette::miette!(
                "Expected {:?}, found {:?}",
                kind,
                self.peek().kind
            ))
        }
    }

    fn consume_string(&mut self) -> Result<String> {
        let token = self.consume_token(TokenKind::String)?;
        if let TokenKind::String(s) = &token.kind {
            Ok(s.clone())
        } else {
            Err(miette::miette!("Expected string literal"))
        }
    }

    fn consume_identifier(&mut self) -> Result<String> {
        let token = self.consume_token(TokenKind::Identifier)?;
        if let TokenKind::Identifier(name) = &token.kind {
            Ok(name.clone())
        } else {
            Err(miette::miette!("Expected identifier"))
        }
    }

    fn span_info(&self) -> Span {
        if self.position > 0 && self.position < self.tokens.len() {
            self.tokens[self.position - 1].span
        } else if !self.tokens.is_empty() {
            self.tokens[0].span
        } else {
            Span {
                line: 0,
                column: 0,
                start: 0,
                end: 0,
            }
        }
    }
}
