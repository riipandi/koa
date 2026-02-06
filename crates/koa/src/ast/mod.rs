//! Abstract Syntax Tree (AST) definitions
//!
//! The AST represents the structure of Koa source code after parsing.

use crate::lexer::Span;

/// The complete AST
#[derive(Debug, Clone)]
pub struct Ast {
    pub declarations: Vec<Declaration>,
}

/// Top-level declarations
#[derive(Debug, Clone)]
pub enum Declaration {
    FnDecl(FnDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    ConstDecl(ConstDecl),
    ErrorDecl(ErrorDecl),
    ImportDecl(ImportDecl),
    ExportDecl(ExportDecl),
}

/// Function declaration
#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: Block,
    pub span: Span,
    pub is_pub: bool,
    pub is_async: bool,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_annotation: Type,
    pub span: Span,
}

/// Block of statements
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Statements
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStmt),
    Const(ConstStmt),
    Expr(ExprStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    If(IfStmt),
    While(WhileStmt),
    Loop(LoopStmt),
    Match(MatchStmt),
    Try(TryStmt),
    Throw(ThrowStmt),
    Defer(DeferStmt),
    ErrDefer(ErrDeferStmt),
}

/// Let statement
#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub value: Option<Box<Expression>>,
    pub span: Span,
}

/// Const statement
#[derive(Debug, Clone)]
pub struct ConstStmt {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub value: Box<Expression>,
    pub span: Span,
}

/// Expression statement
#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Box<Expression>,
    pub span: Span,
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Box<Expression>>,
    pub span: Span,
}

/// Break statement
#[derive(Debug, Clone)]
pub struct BreakStmt {
    pub span: Span,
}

/// Continue statement
#[derive(Debug, Clone)]
pub struct ContinueStmt {
    pub span: Span,
}

/// If statement
#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Box<Expression>,
    pub then_block: Block,
    pub else_block: Option<Block>,
    pub span: Span,
}

/// While statement
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Box<Expression>,
    pub body: Block,
    pub span: Span,
}

/// Loop statement
#[derive(Debug, Clone)]
pub struct LoopStmt {
    pub body: Block,
    pub span: Span,
}

/// Match statement
#[derive(Debug, Clone)]
pub struct MatchStmt {
    pub scrutinee: Box<Expression>,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expression>>,
    pub body: Block,
    pub span: Span,
}

/// Patterns
#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Struct(String, Vec<FieldPattern>),
    Tuple(Vec<Pattern>),
    Wildcard,
}

/// Field pattern
#[derive(Debug, Clone)]
pub struct FieldPattern {
    pub name: String,
    pub pattern: Pattern,
    pub span: Span,
}

/// Try statement
#[derive(Debug, Clone)]
pub struct TryStmt {
    pub body: Block,
    pub catch_block: Block,
    pub span: Span,
}

/// Throw statement
#[derive(Debug, Clone)]
pub struct ThrowStmt {
    pub expr: Box<Expression>,
    pub span: Span,
}

/// Defer statement
#[derive(Debug, Clone)]
pub struct DeferStmt {
    pub statement: Box<Statement>,
    pub span: Span,
}

/// Errdefer statement
#[derive(Debug, Clone)]
pub struct ErrDeferStmt {
    pub statement: Box<Statement>,
    pub span: Span,
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
    Identifier(String),
    Call(CallExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    If(IfExpr),
    Block(Block),
    ErrorUnion(ErrorUnionExpr),
    Array(ArrayExpr),
    Tuple(TupleExpr),
    Struct(StructExpr),
    Await(AwaitExpr),
    Try(TryExpr),
    Cast(CastExpr),
}

/// Binary expression
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub op: BinaryOp,
    pub right: Box<Expression>,
    pub span: Span,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

/// Unary expression
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expression>,
    pub span: Span,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    Address,
}

/// Literals
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

/// Function call expression
#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expression>,
    pub args: Vec<Box<Expression>>,
    pub span: Span,
}

/// Member access expression
#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub object: Box<Expression>,
    pub property: String,
    pub span: Span,
}

/// Index expression
#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
    pub span: Span,
}

/// If expression
#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Option<Box<Expression>>,
    pub span: Span,
}

/// Error union expression
#[derive(Debug, Clone)]
pub struct ErrorUnionExpr {
    pub error_name: Option<String>,
    pub value_type: Box<Type>,
    pub span: Span,
}

/// Array expression
#[derive(Debug, Clone)]
pub struct ArrayExpr {
    pub elements: Vec<Box<Expression>>,
    pub span: Span,
}

/// Tuple expression
#[derive(Debug, Clone)]
pub struct TupleExpr {
    pub elements: Vec<Box<Expression>>,
    pub span: Span,
}

/// Struct expression
#[derive(Debug, Clone)]
pub struct StructExpr {
    pub name: String,
    pub fields: Vec<StructField>,
    pub span: Span,
}

/// Struct field
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub value: Box<Expression>,
    pub span: Span,
}

/// Await expression
#[derive(Debug, Clone)]
pub struct AwaitExpr {
    pub expr: Box<Expression>,
    pub span: Span,
}

/// Try expression
#[derive(Debug, Clone)]
pub struct TryExpr {
    pub expr: Box<Expression>,
    pub span: Span,
}

/// Cast expression
#[derive(Debug, Clone)]
pub struct CastExpr {
    pub expr: Box<Expression>,
    pub target_type: Type,
    pub span: Span,
}

/// Struct declaration
#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<StructFieldDecl>,
    pub methods: Vec<FnDecl>,
    pub span: Span,
    pub is_pub: bool,
}

/// Struct field declaration
#[derive(Debug, Clone)]
pub struct StructFieldDecl {
    pub name: String,
    pub type_: Type,
    pub span: Span,
    pub is_pub: bool,
}

/// Enum declaration
#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
    pub is_pub: bool,
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
    pub span: Span,
}

/// Const declaration
#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub name: String,
    pub type_: Type,
    pub value: Expression,
    pub span: Span,
    pub is_pub: bool,
}

/// Error set declaration
#[derive(Debug, Clone)]
pub struct ErrorDecl {
    pub name: String,
    pub variants: Vec<String>,
    pub span: Span,
    pub is_pub: bool,
}

/// Import declaration
#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub specifiers: Vec<ImportSpecifier>,
    pub from: String,
    pub span: Span,
}

/// Import specifier
#[derive(Debug, Clone)]
pub enum ImportSpecifier {
    Named(String, Option<String>),
    Star(Option<String>),
}

/// Export declaration
#[derive(Debug, Clone)]
pub struct ExportDecl {
    pub declaration: Box<Declaration>,
    pub span: Span,
}

/// Types
#[derive(Debug, Clone)]
pub enum Type {
    // Primitives
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
    F32, F64,
    Bool,
    String,
    Void,

    // Complex
    Named(String),
    Generic(Box<Type>, Vec<Type>),
    ErrorUnion(Option<String>, Box<Type>),
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Pointer(Box<Type>),
    Optional(Box<Type>),
}
