use std::fmt;

/// Source location information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

    pub fn combine(&self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line,
            column: self.column,
        }
    }
}

/// A single token in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, literal: Option<String>) -> Self {
        Self { kind, span, literal }
    }
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
    Error,     // 'error' keyword
    SelfValue, // 'self'
    SelfType,  // 'Self'

    // Types
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
    F32, F64,
    Bool,
    String,
    Void,

    // Literals
    Ident,
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    DocComment, // '///' or '//!'

    // Operators
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    Equal,         // =
    EqualEqual,    // ==
    BangEqual,     // !=
    Less,          // <
    LessEqual,     // <=
    Greater,       // >
    GreaterEqual,  // >=
    And,           // &&
    Or,            // ||
    Bang,          // !
    Question,      // ?

    // Symbols
    LParen,        // (
    RParen,        // )
    LBrace,        // {
    RBrace,        // }
    LBracket,      // [
    RBracket,      // ]
    Dot,           // .
    Comma,         // ,
    Colon,         // :
    DoubleColon,   // ::
    Semicolon,     // ;
    Arrow,         // ->
    FatArrow,      // =>
    At,            // @ (for attributes)

    // Range operators
    DotDot,        // ..
    DotDotEqual,   // ..=

    // Error and EOF
    Illegal,
    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::Fn => "fn",
            TokenKind::Let => "let",
            TokenKind::Const => "const",
            TokenKind::Struct => "struct",
            TokenKind::Enum => "enum",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Match => "match",
            TokenKind::Loop => "loop",
            TokenKind::While => "while",
            TokenKind::For => "for",
            TokenKind::Return => "return",
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::Async => "async",
            TokenKind::Await => "await",
            TokenKind::Try => "try",
            TokenKind::Catch => "catch",
            TokenKind::Throw => "throw",
            TokenKind::Defer => "defer",
            TokenKind::Pub => "pub",
            TokenKind::Priv => "priv",
            TokenKind::Import => "import",
            TokenKind::Export => "export",
            TokenKind::From => "from",
            TokenKind::As => "as",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Null => "null",
            TokenKind::Error => "error",
            TokenKind::SelfValue => "self",
            TokenKind::SelfType => "Self",
            TokenKind::Ident => "Identifier",
            TokenKind::StringLiteral => "StringLiteral",
            TokenKind::IntLiteral => "IntLiteral",
            TokenKind::FloatLiteral => "FloatLiteral",
            TokenKind::DocComment => "DocComment",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::BangEqual => "!=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::Bang => "!",
            TokenKind::Question => "?",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Dot => ".",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::DoubleColon => "::",
            TokenKind::Semicolon => ";",
            TokenKind::Arrow => "->",
            TokenKind::FatArrow => "=>",
            TokenKind::At => "@",
            TokenKind::DotDot => "..",
            TokenKind::DotDotEqual => "..=",
            TokenKind::Illegal => "Illegal",
            TokenKind::EOF => "EOF",
            _ => "Unknown",
        };
        write!(f, "{}", s)
    }
}
