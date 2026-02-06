//! Lexical analysis - tokenization
//!
//! The lexer breaks source code into tokens for parsing.

pub mod token;

pub use token::{Span, Token, TokenKind};

use miette::Result;
use std::iter::Peekable;
use std::str::Chars;

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
            let is_eof = token.kind == TokenKind::EOF;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let start = self.position;
        let line = self.line;
        let column = self.column;

        let kind = match self.peek() {
            None => TokenKind::EOF,
            Some(ch) => match ch {
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
                    if self.peek() == Some('/') {
                        self.bump(); // consume second '/'
                        if self.peek() == Some('/') || self.peek() == Some('!') {
                            // Doc comment
                            return self.lex_doc_comment(start, line, column);
                        } else {
                            self.skip_line_comment();
                            return self.next_token();
                        }
                    } else if self.peek() == Some('*') {
                        self.skip_block_comment();
                        return self.next_token();
                    } else {
                        TokenKind::Slash
                    }
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
                        TokenKind::Illegal
                    }
                }
                '|' => {
                    self.bump();
                    if self.peek() == Some('|') {
                        self.bump();
                        TokenKind::Or
                    } else {
                        TokenKind::Illegal
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
                    if self.peek() == Some('.') {
                        self.bump();
                        if self.peek() == Some('=') {
                            self.bump();
                            TokenKind::DotDotEqual
                        } else {
                            TokenKind::DotDot
                        }
                    } else {
                        TokenKind::Dot
                    }
                }
                ',' => {
                    self.bump();
                    TokenKind::Comma
                }
                ':' => {
                    self.bump();
                    if self.peek() == Some(':') {
                        self.bump();
                        TokenKind::DoubleColon
                    } else {
                        TokenKind::Colon
                    }
                }
                ';' => {
                    self.bump();
                    TokenKind::Semicolon
                }
                '?' => {
                    self.bump();
                    TokenKind::Question
                }
                '@' => {
                    self.bump();
                    TokenKind::At
                }
                _ => {
                    self.bump();
                    TokenKind::Illegal
                }
            },
        };

        let end = self.position;
        let literal = if kind == TokenKind::Ident
            || kind == TokenKind::StringLiteral
            || kind == TokenKind::IntLiteral
            || kind == TokenKind::FloatLiteral
            || kind == TokenKind::SelfValue
            || kind == TokenKind::SelfType
        {
            let raw = self.input[start..end].to_string();
            if kind == TokenKind::StringLiteral {
                if raw.len() >= 2 && raw.starts_with('"') && raw.ends_with('"') {
                    Some(raw[1..raw.len() - 1].to_string())
                } else {
                    Some(raw)
                }
            } else {
                Some(raw)
            }
        } else {
            None
        };

        Ok(Token {
            kind,
            span: Span::new(start, end, line, column),
            literal,
        })
    }

    fn lex_string(&mut self) -> Result<TokenKind> {
        self.bump(); // opening quote
        // TODO: Handle escape sequences
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                return Ok(TokenKind::StringLiteral);
            }
            if ch == '\\' {
                self.bump();
            }
            self.bump();
        }
        // Unclosed string
        Ok(TokenKind::StringLiteral)
    }

    fn lex_number(&mut self) -> Result<TokenKind> {
        let mut is_float = false;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.bump();
            } else if ch == '.' {
                // Peek next to see if it's another dot (range operator)
                let mut next_chars = self.chars.clone();
                next_chars.next();
                if next_chars.peek() == Some(&'.') {
                    // It's a range operator like 1..10
                    break;
                }

                if is_float {
                    break; // Already a float, second dot is something else
                }
                is_float = true;
                self.bump();
            } else {
                break;
            }
        }

        if is_float {
            Ok(TokenKind::FloatLiteral)
        } else {
            Ok(TokenKind::IntLiteral)
        }
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
            "interface" => TokenKind::Interface,
            "match" => TokenKind::Match,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "pub" => TokenKind::Pub,
            "priv" => TokenKind::Priv,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "throw" => TokenKind::Throw,
            "defer" => TokenKind::Defer,
            "error" => TokenKind::Error,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "from" => TokenKind::From,
            "as" => TokenKind::As,
            "self" => TokenKind::SelfValue,
            "Self" => TokenKind::SelfType,

            // Types
            "i8" => TokenKind::I8,
            "i16" => TokenKind::I16,
            "i32" => TokenKind::I32,
            "i64" => TokenKind::I64,
            "i128" => TokenKind::I128,
            "isize" => TokenKind::Isize,
            "u8" => TokenKind::U8,
            "u16" => TokenKind::U16,
            "u32" => TokenKind::U32,
            "u64" => TokenKind::U64,
            "u128" => TokenKind::U128,
            "usize" => TokenKind::Usize,
            "f32" => TokenKind::F32,
            "f64" => TokenKind::F64,
            "bool" => TokenKind::Bool,
            "string" => TokenKind::String,
            "void" => TokenKind::Void,

            _ => TokenKind::Ident,
        };

        Ok(kind)
    }

    fn lex_doc_comment(&mut self, start: usize, line: usize, column: usize) -> Result<Token> {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.bump();
        }
        let end = self.position;
        Ok(Token {
            kind: TokenKind::DocComment,
            span: Span::new(start, end, line, column),
            literal: Some(self.input[start..end].to_string()),
        })
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

    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.bump();
        }
    }

    fn skip_block_comment(&mut self) {
        self.bump(); // consume '*'
        while let Some(ch) = self.peek() {
            if ch == '*' {
                self.bump();
                if self.peek() == Some('/') {
                    self.bump();
                    break;
                }
                continue;
            }
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.bump();
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
