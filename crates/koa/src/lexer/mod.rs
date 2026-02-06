//! Lexical analysis - tokenization
//!
//! The lexer breaks source code into tokens for parsing.

use miette::{IntoDiagnostic, Result};
use std::iter::Peekable;
use std::str::Chars;

/// A single token in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub literal: Option<String>,
}

/// The kind of token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,
    Const,
    Struct,
    Enum,
    If,
    Else,
    Match,
    Loop,
    While,
    For,
    Return,
    Break,
    Continue,
    Async,
    Await,
    Try,
    Catch,
    Throw,
    Defer,
    Pub,
    Priv,
    Import,
    Export,
    From,
    As,
    True,
    False,
    Null,

    // Literals
    Ident,
    String,
    Number,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Bang,

    // Symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,

    // Error token
    Error,

    // Unknown
    Unknown,
    EOF,
}

/// Source location information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }

    pub fn combine(&self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line,
            column: self.column,
        }
    }
}

/// Lexer for tokenizing Koa source code
pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<Chars<'input>>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            match token {
                None => break,
                Some(t) if t.kind == TokenKind::EOF => break,
                Some(t) => tokens.push(t),
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();

        let start = self.position;
        let line = self.line;
        let column = self.column;

        match self.peek() {
            None => Ok(Some(Token {
                kind: TokenKind::EOF,
                span: Span::new(start, start, line, column),
                literal: None,
            })),
            Some(ch) => {
                let kind = match ch {
                    '"' => self.lex_string()?,
                    '0'..='9' => self.lex_number()?,
                    'a'..='z' | 'A'..='Z' | '_' => self.lex_ident()?,
                    '+' => {
                        self.bump();
                        TokenKind::Plus
                    }
                    '-' => {
                        self.bump();
                        if self.peek() == Some('>') {
                            self.bump();
                            TokenKind::Arrow
                        } else {
                            TokenKind::Minus
                        }
                    }
                    '*' => {
                        self.bump();
                        TokenKind::Star
                    }
                    '/' => {
                        self.bump();
                        TokenKind::Slash
                    }
                    '%' => {
                        self.bump();
                        TokenKind::Percent
                    }
                    '=' => {
                        self.bump();
                        if self.peek() == Some('=') {
                            self.bump();
                            TokenKind::EqualEqual
                        } else if self.peek() == Some('>') {
                            self.bump();
                            TokenKind::FatArrow
                        } else {
                            TokenKind::Equal
                        }
                    }
                    '!' => {
                        self.bump();
                        if self.peek() == Some('=') {
                            self.bump();
                            TokenKind::BangEqual
                        } else {
                            TokenKind::Bang
                        }
                    }
                    '<' => {
                        self.bump();
                        if self.peek() == Some('=') {
                            self.bump();
                            TokenKind::LessEqual
                        } else {
                            TokenKind::Less
                        }
                    }
                    '>' => {
                        self.bump();
                        if self.peek() == Some('=') {
                            self.bump();
                            TokenKind::GreaterEqual
                        } else {
                            TokenKind::Greater
                        }
                    }
                    '&' => {
                        self.bump();
                        if self.peek() == Some('&') {
                            self.bump();
                            TokenKind::And
                        } else {
                            TokenKind::Unknown
                        }
                    }
                    '|' => {
                        self.bump();
                        if self.peek() == Some('|') {
                            self.bump();
                            TokenKind::Or
                        } else {
                            TokenKind::Unknown
                        }
                    }
                    '(' => {
                        self.bump();
                        TokenKind::LParen
                    }
                    ')' => {
                        self.bump();
                        TokenKind::RParen
                    }
                    '{' => {
                        self.bump();
                        TokenKind::LBrace
                    }
                    '}' => {
                        self.bump();
                        TokenKind::RBrace
                    }
                    '[' => {
                        self.bump();
                        TokenKind::LBracket
                    }
                    ']' => {
                        self.bump();
                        TokenKind::RBracket
                    }
                    '.' => {
                        self.bump();
                        TokenKind::Dot
                    }
                    ',' => {
                        self.bump();
                        TokenKind::Comma
                    }
                    ':' => {
                        self.bump();
                        TokenKind::Colon
                    }
                    ';' => {
                        self.bump();
                        TokenKind::Semicolon
                    }
                    _ => {
                        self.bump();
                        TokenKind::Unknown
                    }
                };

                let end = self.position;
                let literal = if kind == TokenKind::Ident
                    || kind == TokenKind::String
                    || kind == TokenKind::Number
                {
                    Some(self.input[start..end].to_string())
                } else {
                    None
                };

                Ok(Some(Token {
                    kind,
                    span: Span::new(start, end, line, column),
                    literal,
                }))
            }
        }
    }

    fn lex_string(&mut self) -> Result<TokenKind> {
        self.bump(); // opening quote
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                return Ok(TokenKind::String);
            }
            if ch == '\\' {
                self.bump();
            }
            self.bump();
        }
        Ok(TokenKind::String)
    }

    fn lex_number(&mut self) -> Result<TokenKind> {
        let start = self.position;

        // Consume all digits and decimal point
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                self.bump();
            } else {
                break;
            }
        }

        Ok(TokenKind::Number)
    }

    fn lex_ident(&mut self) -> Result<TokenKind> {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.bump();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        let text = &self.input[start..self.position];
        let kind = match text {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "loop" => TokenKind::Loop,
            "match" => TokenKind::Match,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "pub" => TokenKind::Pub,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "try" => TokenKind::Try,
            "throw" => TokenKind::Throw,
            "defer" => TokenKind::Defer,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "as" => TokenKind::As,
            _ => TokenKind::Ident,
        };

        Ok(kind)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                self.bump();
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if ch.is_some() {
            self.position += 1;
            self.column += 1;
        }
        ch
    }
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("fn let const struct enum");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[1].kind, TokenKind::Let);
        assert_eq!(tokens[2].kind, TokenKind::Const);
        assert_eq!(tokens[3].kind, TokenKind::Struct);
        assert_eq!(tokens[4].kind, TokenKind::Enum);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / == != < > && ||");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
        assert_eq!(tokens[4].kind, TokenKind::EqualEqual);
        assert_eq!(tokens[5].kind, TokenKind::BangEqual);
        assert_eq!(tokens[6].kind, TokenKind::Less);
        assert_eq!(tokens[7].kind, TokenKind::Greater);
        assert_eq!(tokens[8].kind, TokenKind::And);
        assert_eq!(tokens[9].kind, TokenKind::Or);
    }

    #[test]
    fn test_simple_function() {
        let source = r#"
            fn main(): i32 {
                return 0;
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Fn);
    }
}
