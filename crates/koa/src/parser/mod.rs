//! Parser - converts tokens into AST
//!
//! The parser is responsible for building the Abstract Syntax Tree from tokens.

use crate::ast::*;
use crate::lexer::{Token, TokenKind};
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

    pub fn parse(&mut self) -> Result<Ast> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            if let Some(decl) = self.parse_declaration()? {
                declarations.push(decl);
            }
        }

        Ok(Ast { declarations })
    }

    fn parse_declaration(&mut self) -> Result<Option<Declaration>> {
        if self.match_token(TokenKind::Pub) {
            self.parse_pub_declaration()
        } else {
            self.parse_non_pub_declaration()
        }
    }

    fn parse_pub_declaration(&mut self) -> Result<Option<Declaration>> {
        let decl = self.parse_non_pub_declaration()?;
        Ok(decl.map(|mut d| {
            // Mark as public
            match &mut d {
                Declaration::FnDecl(f) => f.is_pub = true,
                Declaration::StructDecl(s) => s.is_pub = true,
                Declaration::EnumDecl(e) => e.is_pub = true,
                Declaration::ConstDecl(c) => c.is_pub = true,
                Declaration::ErrorDecl(e) => e.is_pub = true,
                _ => {}
            }
            d
        }))
    }

    fn parse_non_pub_declaration(&mut self) -> Result<Option<Declaration>> {
        match self.peek_kind() {
            Some(TokenKind::Fn) => Ok(Some(self.parse_fn_decl()?)),
            Some(TokenKind::Struct) => Ok(Some(self.parse_struct_decl()?)),
            Some(TokenKind::Enum) => Ok(Some(self.parse_enum_decl()?)),
            Some(TokenKind::Const) => Ok(Some(self.parse_const_decl()?)),
            Some(TokenKind::Error) => Ok(Some(self.parse_error_decl()?)),
            Some(TokenKind::Import) => Ok(Some(self.parse_import_decl()?)),
            Some(TokenKind::Export) => Ok(Some(self.parse_export_decl()?)),
            _ => Ok(None),
        }
    }

    fn parse_fn_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Fn)?.span;
        let name = self.consume_identifier()?;

        // Type parameters
        let type_params = if self.match_token(TokenKind::Less) {
            self.parse_type_params()?
        } else {
            Vec::new()
        };

        self.consume_token(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.consume_token(TokenKind::RParen)?;

        self.consume_token(TokenKind::Colon)?;
        let return_type = self.parse_type()?;

        let body = self.parse_block()?;

        Ok(Declaration::FnDecl(FnDecl {
            name,
            type_params,
            params,
            return_type,
            body,
            span,
            is_pub: false,
            is_async: false,
        }))
    }

    fn parse_struct_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Struct)?.span;
        let name = self.consume_identifier()?;

        let type_params = if self.match_token(TokenKind::Less) {
            self.parse_type_params()?
        } else {
            Vec::new()
        };

        self.consume_token(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            // Check if this is a method or a field
            let next = self.peek_kind();
            if next == Some(TokenKind::Fn) {
                if let Declaration::FnDecl(mut fn_decl) = self.parse_fn_decl()? {
                    fn_decl.is_pub = false;
                    methods.push(fn_decl);
                }
            } else {
                let is_pub = self.match_token(TokenKind::Pub);
                let name = self.consume_identifier()?;
                self.consume_token(TokenKind::Colon)?;
                let type_ = self.parse_type()?;
                self.consume_token(TokenKind::Semicolon)?;
                fields.push(StructFieldDecl {
                    name,
                    type_,
                    span,
                    is_pub,
                });
            }
        }

        self.consume_token(TokenKind::RBrace)?;

        Ok(Declaration::StructDecl(StructDecl {
            name,
            type_params,
            fields,
            methods,
            span,
            is_pub: false,
        }))
    }

    fn parse_enum_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Enum)?.span;
        let name = self.consume_identifier()?;

        let type_params = if self.match_token(TokenKind::Less) {
            self.parse_type_params()?
        } else {
            Vec::new()
        };

        self.consume_token(TokenKind::LBrace)?;
        let mut variants = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let name = self.consume_identifier()?;
            let mut fields = Vec::new();

            if self.match_token(TokenKind::LParen) {
                while !self.check(TokenKind::RParen) && !self.is_at_end() {
                    fields.push(self.parse_type()?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume_token(TokenKind::RParen)?;
            }

            self.consume_token(TokenKind::Comma)?;
            variants.push(EnumVariant { name, fields, span });
        }

        self.consume_token(TokenKind::RBrace)?;

        Ok(Declaration::EnumDecl(EnumDecl {
            name,
            type_params,
            variants,
            span,
            is_pub: false,
        }))
    }

    fn parse_const_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Const)?.span;
        let name = self.consume_identifier()?;

        let type_ = if self.match_token(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume_token(TokenKind::Equal)?;
        let value = self.parse_expression()?;
        self.consume_token(TokenKind::Semicolon)?;

        Ok(Declaration::ConstDecl(ConstDecl {
            name,
            type_: type_.unwrap_or(Type::Named("auto".to_string())),
            value,
            span,
            is_pub: false,
        }))
    }

    fn parse_error_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Error)?.span;
        let name = self.consume_identifier()?;

        self.consume_token(TokenKind::LBrace)?;
        let mut variants = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let name = self.consume_identifier()?;
            self.consume_token(TokenKind::Comma)?;
            variants.push(name);
        }

        self.consume_token(TokenKind::RBrace)?;

        Ok(Declaration::ErrorDecl(ErrorDecl {
            name,
            variants,
            span,
            is_pub: false,
        }))
    }

    fn parse_import_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Import)?.span;

        let mut specifiers = Vec::new();
        self.consume_token(TokenKind::LBrace)?;

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let name = self.consume_identifier()?;
            let alias = if self.match_token(TokenKind::As) {
                Some(self.consume_identifier()?)
            } else {
                None
            };
            specifiers.push(ImportSpecifier::Named(name, alias));
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.consume_token(TokenKind::RBrace)?;
        self.consume_token(TokenKind::From)?;

        let from = self.consume_string()?;
        self.consume_token(TokenKind::Semicolon)?;

        Ok(Declaration::ImportDecl(ImportDecl {
            specifiers,
            from,
            span,
        }))
    }

    fn parse_export_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Export)?.span;
        let declaration = Box::new(
            self.parse_declaration()?
                .ok_or_else(|| miette::miette!("Expected declaration after export"))?,
        );
        Ok(Declaration::ExportDecl(ExportDecl { declaration, span }))
    }

    fn parse_type_params(&mut self) -> Result<Vec<String>> {
        let mut params = Vec::new();

        while !self.check(TokenKind::Greater) && !self.is_at_end() {
            params.push(self.consume_identifier()?);
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.consume_token(TokenKind::Greater)?;
        Ok(params)
    }

    fn parse_params(&mut self) -> Result<Vec<Param>> {
        let mut params = Vec::new();

        while !self.check(TokenKind::RParen) && !self.is_at_end() {
            let name = self.consume_identifier()?;
            self.consume_token(TokenKind::Colon)?;
            let type_annotation = self.parse_type()?;
            params.push(Param {
                name,
                type_annotation,
                span: self.peek().span,
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type> {
        // Simple type parsing for now
        let name = self.consume_identifier()?;
        Ok(Type::Named(name))
    }

    fn parse_block(&mut self) -> Result<Block> {
        let span = self.consume_token(TokenKind::LBrace)?.span;
        let mut statements = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume_token(TokenKind::RBrace)?;
        Ok(Block { statements, span })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek_kind() {
            Some(TokenKind::Let) => self.parse_let_stmt(),
            Some(TokenKind::Const) => self.parse_const_stmt(),
            Some(TokenKind::Return) => self.parse_return_stmt(),
            Some(TokenKind::If) => self.parse_if_stmt(),
            Some(TokenKind::While) => self.parse_while_stmt(),
            Some(TokenKind::Loop) => self.parse_loop_stmt(),
            Some(TokenKind::Match) => self.parse_match_stmt(),
            Some(TokenKind::Break) => self.parse_break_stmt(),
            Some(TokenKind::Continue) => self.parse_continue_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Let)?.span;
        let name = self.consume_identifier()?;

        let type_annotation = if self.match_token(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let value = if self.match_token(TokenKind::Equal) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Let(LetStmt {
            name,
            type_annotation,
            value,
            span,
        }))
    }

    fn parse_const_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Const)?.span;
        let name = self.consume_identifier()?;

        let type_annotation = if self.match_token(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume_token(TokenKind::Equal)?;
        let value = Box::new(self.parse_expression()?);
        self.consume_token(TokenKind::Semicolon)?;

        Ok(Statement::Const(ConstStmt {
            name,
            type_annotation,
            value,
            span,
        }))
    }

    fn parse_return_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Return)?.span;
        let value = if !self.check(TokenKind::Semicolon) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Return(ReturnStmt { value, span }))
    }

    fn parse_break_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Break)?.span;
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Break(BreakStmt { span }))
    }

    fn parse_continue_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Continue)?.span;
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Continue(ContinueStmt { span }))
    }

    fn parse_if_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::If)?.span;
        let condition = Box::new(self.parse_expression()?);
        let then_block = self.parse_block()?;
        let else_block = if self.match_token(TokenKind::Else) {
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Statement::If(IfStmt {
            condition,
            then_block,
            else_block,
            span,
        }))
    }

    fn parse_while_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::While)?.span;
        let condition = Box::new(self.parse_expression()?);
        let body = self.parse_block()?;
        Ok(Statement::While(WhileStmt {
            condition,
            body,
            span,
        }))
    }

    fn parse_loop_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Loop)?.span;
        let body = self.parse_block()?;
        Ok(Statement::Loop(LoopStmt { body, span }))
    }

    fn parse_match_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Match)?.span;
        let scrutinee = Box::new(self.parse_expression()?);

        self.consume_token(TokenKind::LBrace)?;
        let mut arms = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            let guard = if self.match_token(TokenKind::If) {
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };
            self.consume_token(TokenKind::FatArrow)?;
            let body = self.parse_block()?;
            arms.push(MatchArm {
                pattern,
                guard,
                body,
                span,
            });
        }

        self.consume_token(TokenKind::RBrace)?;
        Ok(Statement::Match(MatchStmt {
            scrutinee,
            arms,
            span,
        }))
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        // Simple identifier pattern for now
        let name = self.consume_identifier()?;
        Ok(Pattern::Identifier(name))
    }

    fn parse_expr_stmt(&mut self) -> Result<Statement> {
        let expr = Box::new(self.parse_expression()?);
        let span = expr.span();
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Expr(ExprStmt { expr, span }))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, precedence: u8) -> Result<Expression> {
        let mut left = self.parse_unary_expr()?;

        while let Some(op) = self.get_binary_op() {
            let op_precedence = self.get_precedence(op);
            if op_precedence < precedence {
                break;
            }

            self.advance();
            let right = self.parse_binary_expr(op_precedence + 1)?;
            let span = left.span().combine(right.span());
            left = Expression::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> Result<Expression> {
        if let Some(op) = self.get_unary_op() {
            let span = self.advance().span;
            let expr = self.parse_unary_expr()?;
            return Ok(Expression::Unary(UnaryExpr {
                op,
                expr: Box::new(expr),
                span,
            }));
        }

        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<Expression> {
        match self.peek_kind() {
            Some(TokenKind::Number) => {
                let token = self.advance();
                let value = token
                    .literal
                    .as_ref()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                Ok(Expression::Literal(Literal::Number(value)))
            }
            Some(TokenKind::String) => {
                let token = self.advance();
                let value = token.literal.clone().unwrap_or_default();
                Ok(Expression::Literal(Literal::String(value)))
            }
            Some(TokenKind::True) => {
                self.advance();
                Ok(Expression::Literal(Literal::Bool(true)))
            }
            Some(TokenKind::False) => {
                self.advance();
                Ok(Expression::Literal(Literal::Bool(false)))
            }
            Some(TokenKind::Null) => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            Some(TokenKind::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume_token(TokenKind::RParen)?;
                Ok(expr)
            }
            Some(TokenKind::Ident) => {
                let token = self.advance();
                let name = token.literal.clone().unwrap_or_default();
                Ok(Expression::Identifier(name))
            }
            _ => Err(miette::miette!("Unexpected token in expression")),
        }
    }

    fn get_binary_op(&self) -> Option<BinaryOp> {
        match self.peek_kind() {
            Some(TokenKind::Plus) => Some(BinaryOp::Add),
            Some(TokenKind::Minus) => Some(BinaryOp::Sub),
            Some(TokenKind::Star) => Some(BinaryOp::Mul),
            Some(TokenKind::Slash) => Some(BinaryOp::Div),
            Some(TokenKind::EqualEqual) => Some(BinaryOp::Equal),
            Some(TokenKind::BangEqual) => Some(BinaryOp::NotEqual),
            Some(TokenKind::Less) => Some(BinaryOp::Less),
            Some(TokenKind::LessEqual) => Some(BinaryOp::LessEqual),
            Some(TokenKind::Greater) => Some(BinaryOp::Greater),
            Some(TokenKind::GreaterEqual) => Some(BinaryOp::GreaterEqual),
            Some(TokenKind::And) => Some(BinaryOp::And),
            Some(TokenKind::Or) => Some(BinaryOp::Or),
            _ => None,
        }
    }

    fn get_unary_op(&self) -> Option<UnaryOp> {
        match self.peek_kind() {
            Some(TokenKind::Minus) => Some(UnaryOp::Neg),
            Some(TokenKind::Bang) => Some(UnaryOp::Not),
            _ => None,
        }
    }

    fn get_precedence(&self, op: BinaryOp) -> u8 {
        match op {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Equal | BinaryOp::NotEqual => 3,
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => 4,
            BinaryOp::Add | BinaryOp::Sub => 5,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 6,
            _ => 0,
        }
    }

    fn consume_identifier(&mut self) -> Result<String> {
        let token = self.advance();
        if token.kind == TokenKind::Ident {
            Ok(token.literal.clone().unwrap_or_default())
        } else {
            Err(miette::miette!("Expected identifier"))
        }
    }

    fn consume_string(&mut self) -> Result<String> {
        let token = self.advance();
        if token.kind == TokenKind::String {
            Ok(token.literal.clone().unwrap_or_default())
        } else {
            Err(miette::miette!("Expected string"))
        }
    }

    fn consume_token(&mut self, kind: TokenKind) -> Result<&Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(miette::miette!("Expected {:?}", kind))
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

    fn check(&self, kind: TokenKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.tokens.get(self.position).map(|t| t.kind)
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.position)
            .unwrap_or(&self.tokens.last().unwrap())
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        &self.tokens[self.position - 1]
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() || self.peek_kind() == Some(TokenKind::EOF)
    }
}

/// Span helper trait
trait SpanHelper {
    fn span(&self) -> crate::lexer::Span;
    fn combine(&self, other: crate::lexer::Span) -> crate::lexer::Span;
}

impl SpanHelper for Expression {
    fn span(&self) -> crate::lexer::Span {
        match self {
            Expression::Binary(e) => e.span,
            Expression::Unary(e) => e.span,
            Expression::Literal(_) => crate::lexer::Span::default(),
            Expression::Identifier(_) => crate::lexer::Span::default(),
            Expression::Call(e) => e.span,
            Expression::Member(e) => e.span,
            Expression::Index(e) => e.span,
            Expression::If(e) => e.span,
            Expression::Block(b) => b.span,
            Expression::ErrorUnion(e) => e.span,
            Expression::Array(e) => e.span,
            Expression::Tuple(e) => e.span,
            Expression::Struct(e) => e.span,
            Expression::Await(e) => e.span,
            Expression::Try(e) => e.span,
            Expression::Cast(e) => e.span,
        }
    }

    fn combine(&self, other: crate::lexer::Span) -> crate::lexer::Span {
        let self_span = self.span();
        crate::lexer::Span {
            start: self_span.start.min(other.start),
            end: self_span.end.max(other.end),
            line: self_span.line,
            column: self_span.column,
        }
    }
}
