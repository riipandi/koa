//! Parser - converts tokens into AST
//!
//! The parser is responsible for building the Abstract Syntax Tree from tokens.

use crate::ast::*;
use crate::lexer::{Token, TokenKind, Span};
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
            } else {
                // If no declaration matched but not at end, we might need to skip
                self.advance();
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
            match &mut d {
                Declaration::FnDecl(f) => f.is_pub = true,
                Declaration::StructDecl(s) => s.is_pub = true,
                Declaration::EnumDecl(e) => e.is_pub = true,
                Declaration::InterfaceDecl(i) => i.is_pub = true,
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
            Some(TokenKind::Interface) => Ok(Some(self.parse_interface_decl()?)),
            Some(TokenKind::Const) => Ok(Some(self.parse_const_decl()?)),
            Some(TokenKind::Error) => Ok(Some(self.parse_error_decl()?)),
            Some(TokenKind::Import) => Ok(Some(self.parse_import_decl()?)),
            Some(TokenKind::Export) => Ok(Some(self.parse_export_decl()?)),
            _ => Ok(None),
        }
    }

    fn parse_fn_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Fn)?.span;
        let is_async = self.match_token(TokenKind::Async);
        let name = self.consume_identifier()?;

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
            is_async,
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
            if self.check(TokenKind::Fn) {
                if let Declaration::FnDecl(fn_decl) = self.parse_fn_decl()? {
                    methods.push(fn_decl);
                }
            } else {
                let f_is_pub = self.match_token(TokenKind::Pub);
                let f_name = self.consume_identifier()?;
                self.consume_token(TokenKind::Colon)?;
                let f_type = self.parse_type()?;
                self.consume_token(TokenKind::Semicolon)?;
                fields.push(StructFieldDecl {
                    name: f_name,
                    type_: f_type,
                    span,
                    is_pub: f_is_pub,
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
            let v_name = self.consume_identifier()?;
            let mut v_fields = Vec::new();

            if self.match_token(TokenKind::LParen) {
                while !self.check(TokenKind::RParen) && !self.is_at_end() {
                    v_fields.push(self.parse_type()?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume_token(TokenKind::RParen)?;
            }

            self.match_token(TokenKind::Comma);
            variants.push(EnumVariant { name: v_name, fields: v_fields, span });
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

        self.consume_token(TokenKind::Colon)?;
        let type_ = self.parse_type()?;

        self.consume_token(TokenKind::Equal)?;
        let value = self.parse_expression()?;
        self.consume_token(TokenKind::Semicolon)?;

        Ok(Declaration::ConstDecl(ConstDecl {
            name,
            type_,
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
            variants.push(self.consume_identifier()?);
            self.match_token(TokenKind::Comma);
        }

        self.consume_token(TokenKind::RBrace)?;

        Ok(Declaration::ErrorDecl(ErrorDecl {
            name,
            variants,
            span,
            is_pub: false,
        }))
    }

    fn parse_interface_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Interface)?.span;
        let name = self.consume_identifier()?;

        let type_params = if self.match_token(TokenKind::Less) {
            self.parse_type_params()?
        } else {
            Vec::new()
        };

        self.consume_token(TokenKind::LBrace)?;
        let mut methods = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            methods.push(self.parse_interface_method()?);
        }

        self.consume_token(TokenKind::RBrace)?;

        Ok(Declaration::InterfaceDecl(InterfaceDecl {
            name,
            type_params,
            methods,
            span,
            is_pub: false,
        }))
    }

    fn parse_interface_method(&mut self) -> Result<InterfaceMethod> {
        let start_span = self.consume_token(TokenKind::Fn)?.span;
        let name = self.consume_identifier()?;

        self.consume_token(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.consume_token(TokenKind::RParen)?;

        self.consume_token(TokenKind::Colon)?;
        let return_type = self.parse_type()?;
        self.consume_token(TokenKind::Semicolon)?;

        let span = start_span.combine(self.previous().span);

        Ok(InterfaceMethod {
            name,
            params,
            return_type,
            span,
        })
    }

    fn parse_import_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Import)?.span;
        let mut specifiers = Vec::new();

        if self.match_token(TokenKind::Star) {
            let alias = if self.match_token(TokenKind::As) {
                Some(self.consume_identifier()?)
            } else {
                None
            };
            specifiers.push(ImportSpecifier::Star(alias));
        } else {
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
        }

        self.consume_token(TokenKind::From)?;
        let from = self.consume_string()?;
        self.consume_token(TokenKind::Semicolon)?;

        Ok(Declaration::ImportDecl(ImportDecl { specifiers, from, span }))
    }

    fn parse_export_decl(&mut self) -> Result<Declaration> {
        let span = self.consume_token(TokenKind::Export)?.span;
        let decl = self.parse_declaration()?
            .ok_or_else(|| miette::miette!("Expected declaration after export"))?;
        Ok(Declaration::ExportDecl(ExportDecl { declaration: Box::new(decl), span }))
    }

    fn parse_type_params(&mut self) -> Result<Vec<TypeParameter>> {
        let mut params = Vec::new();
        while !self.check(TokenKind::Greater) && !self.is_at_end() {
            let start_span = self.peek().span;
            let name = self.consume_identifier()?;
            let mut constraints = Vec::new();
            if self.match_token(TokenKind::Colon) {
                loop {
                    constraints.push(self.parse_type()?);
                    if !self.match_token(TokenKind::Plus) {
                        break;
                    }
                }
            }
            let span = start_span.combine(self.previous().span);
            params.push(TypeParameter { name, constraints, span });
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
            let (p_name, p_type) = if self.match_token(TokenKind::SelfValue) {
                let name = "self".to_string();
                let type_anno = if self.match_token(TokenKind::Colon) {
                    self.parse_type()?
                } else {
                    Type::Named("Self".to_string())
                };
                (name, type_anno)
            } else {
                let name = self.consume_identifier()?;
                self.consume_token(TokenKind::Colon)?;
                let type_anno = self.parse_type()?;
                (name, type_anno)
            };

            params.push(Param {
                name: p_name,
                type_annotation: p_type,
                span: self.previous().span, // Use the last token's span for now
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type> {
        if self.match_token(TokenKind::Question) {
            return Ok(Type::Optional(Box::new(self.parse_type()?)));
        }
        if self.match_token(TokenKind::Star) {
            return Ok(Type::Pointer(Box::new(self.parse_type()?)));
        }
        if self.match_token(TokenKind::LBracket) {
            self.consume_token(TokenKind::RBracket)?;
            return Ok(Type::Array(Box::new(self.parse_type()?)));
        }
        if self.match_token(TokenKind::Bang) {
            return Ok(Type::ErrorUnion(None, Box::new(self.parse_type()?)));
        }

        let mut base = if self.match_token(TokenKind::LParen) {
            let t = self.parse_type()?;
            self.consume_token(TokenKind::RParen)?;
            t
        } else {
            match self.peek_kind() {
                Some(TokenKind::Void) => { self.advance(); Type::Void }
                Some(TokenKind::I8) => { self.advance(); Type::I8 }
                Some(TokenKind::I16) => { self.advance(); Type::I16 }
                Some(TokenKind::I32) => { self.advance(); Type::I32 }
                Some(TokenKind::I64) => { self.advance(); Type::I64 }
                Some(TokenKind::I128) => { self.advance(); Type::I128 }
                Some(TokenKind::Isize) => { self.advance(); Type::Isize }
                Some(TokenKind::U8) => { self.advance(); Type::U8 }
                Some(TokenKind::U16) => { self.advance(); Type::U16 }
                Some(TokenKind::U32) => { self.advance(); Type::U32 }
                Some(TokenKind::U64) => { self.advance(); Type::U64 }
                Some(TokenKind::U128) => { self.advance(); Type::U128 }
                Some(TokenKind::Usize) => { self.advance(); Type::Usize }
                Some(TokenKind::F32) => { self.advance(); Type::F32 }
                Some(TokenKind::F64) => { self.advance(); Type::F64 }
                Some(TokenKind::Bool) => { self.advance(); Type::Bool }
                Some(TokenKind::String) => { self.advance(); Type::String }
                _ => Type::Named(self.consume_identifier()?),
            }
        };

        // Handle Generics: List<i32>
        if self.match_token(TokenKind::Less) {
            let mut args = Vec::new();
            while !self.check(TokenKind::Greater) && !self.is_at_end() {
                args.push(self.parse_type()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume_token(TokenKind::Greater)?;
            base = Type::Generic(Box::new(base), args);
        }

        // Handle Error Union: ErrorSet!i32
        if self.match_token(TokenKind::Bang) {
            let err_name = match base {
                Type::Named(n) => Some(n),
                _ => None,
            };
            let value_type = self.parse_type()?;
            base = Type::ErrorUnion(err_name, Box::new(value_type));
        }

        Ok(base)
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
            Some(TokenKind::Defer) => self.parse_defer_stmt(),
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
        Ok(Statement::Let(LetStmt { name, type_annotation, value, span }))
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
        Ok(Statement::Const(ConstStmt { name, type_annotation, value, span }))
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

    fn parse_if_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::If)?.span;
        let condition = Box::new(self.parse_expression()?);
        let block = self.parse_block()?;
        let else_block = if self.match_token(TokenKind::Else) {
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Statement::If(IfStmt { condition, then_block: block, else_block, span }))
    }

    fn parse_while_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::While)?.span;
        let condition = Box::new(self.parse_expression()?);
        let body = self.parse_block()?;
        Ok(Statement::While(WhileStmt { condition, body, span }))
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
            arms.push(MatchArm { pattern, guard, body, span });
        }
        self.consume_token(TokenKind::RBrace)?;
        Ok(Statement::Match(MatchStmt { scrutinee, arms, span }))
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

    fn parse_pattern(&mut self) -> Result<Pattern> {
        if self.match_token(TokenKind::Ident) {
            let name = self.previous().literal.clone().unwrap_or_default();
            if name == "_" {
                return Ok(Pattern::Wildcard);
            }
            return Ok(Pattern::Identifier(name));
        }
        Err(miette::miette!("Expected pattern"))
    }

    fn parse_defer_stmt(&mut self) -> Result<Statement> {
        let span = self.consume_token(TokenKind::Defer)?.span;
        let statement = self.parse_statement()?;
        Ok(Statement::Defer(DeferStmt { statement: Box::new(statement), span }))
    }

    fn parse_expr_stmt(&mut self) -> Result<Statement> {
        let expr = Box::new(self.parse_expression()?);
        let span = expr.span_info();
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Expr(ExprStmt { expr, span }))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, precedence: u8) -> Result<Expression> {
        let mut left = self.parse_unary_expr()?;
        while let Some(op) = self.get_binary_op() {
            let op_preced = self.get_precedence(op);
            if op_preced < precedence {
                break;
            }
            self.advance();
            let right = self.parse_binary_expr(op_preced + 1)?;
            let span = left.span_info().combine(right.span_info());
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
            return Ok(Expression::Unary(UnaryExpr { op, expr: Box::new(expr), span }));
        }
        self.parse_postfix_expr()
    }

    fn parse_postfix_expr(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary_expr()?;
        loop {
            match self.peek_kind() {
                Some(TokenKind::Less) => {
                    // Possible generic arguments: callee<Args>(...)
                    // To distinguish from comparison, we look ahead or try to parse.
                    // Simplified: if we can parse type arguments followed by LParen, it's a call.
                    let checkpoint = self.position;
                    self.advance(); // consume '<'
                    let mut type_args = Vec::new();
                    let mut is_generic = true;
                    while !self.check(TokenKind::Greater) && !self.is_at_end() {
                        if let Ok(ty) = self.parse_type() {
                            type_args.push(ty);
                            if !self.match_token(TokenKind::Comma) { break; }
                        } else {
                            is_generic = false;
                            break;
                        }
                    }
                    if is_generic && self.match_token(TokenKind::Greater) && self.check(TokenKind::LParen) {
                        // It's a generic call
                        self.consume_token(TokenKind::LParen)?;
                        let mut args = Vec::new();
                        while !self.check(TokenKind::RParen) && !self.is_at_end() {
                            args.push(Box::new(self.parse_expression()?));
                            if !self.match_token(TokenKind::Comma) { break; }
                        }
                        let r_paren_span = self.consume_token(TokenKind::RParen)?.span;
                        let span = expr.span_info().combine(r_paren_span);
                        expr = Expression::Call(CallExpr { callee: Box::new(expr), type_args: Some(type_args), args, span });
                    } else {
                        // Not a generic call, backtrack
                        self.position = checkpoint;
                        break;
                    }
                }
                Some(TokenKind::LParen) => {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(TokenKind::RParen) && !self.is_at_end() {
                        args.push(Box::new(self.parse_expression()?));
                        if !self.match_token(TokenKind::Comma) { break; }
                    }
                    let r_paren_span = self.consume_token(TokenKind::RParen)?.span;
                    let span = expr.span_info().combine(r_paren_span);
                    expr = Expression::Call(CallExpr { callee: Box::new(expr), type_args: None, args, span });
                }
                Some(TokenKind::Dot) => {
                    self.advance();
                    let (property, prop_span) = {
                        let t = self.consume_token(TokenKind::Ident)?;
                        (t.literal.clone().unwrap_or_default(), t.span)
                    };
                    let span = expr.span_info().combine(prop_span);
                    expr = Expression::Member(MemberExpr { object: Box::new(expr), property, span });
                }
                Some(TokenKind::LBracket) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    let r_bracket_span = self.consume_token(TokenKind::RBracket)?.span;
                    let span = expr.span_info().combine(r_bracket_span);
                    expr = Expression::Index(IndexExpr { object: Box::new(expr), index: Box::new(index), span });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary_expr(&mut self) -> Result<Expression> {
        match self.peek_kind() {
            Some(TokenKind::IntLiteral) => {
                let t = self.advance();
                let val = t.literal.as_ref().and_then(|s| s.parse().ok()).unwrap_or(0);
                Ok(Expression::Literal(Literal::Int(val)))
            }
            Some(TokenKind::FloatLiteral) => {
                let t = self.advance();
                let val = t.literal.as_ref().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                Ok(Expression::Literal(Literal::Float(val)))
            }
            Some(TokenKind::StringLiteral) => {
                let t = self.advance();
                Ok(Expression::Literal(Literal::String(t.literal.clone().unwrap_or_default())))
            }
            Some(TokenKind::True) => { self.advance(); Ok(Expression::Literal(Literal::Bool(true))) }
            Some(TokenKind::False) => { self.advance(); Ok(Expression::Literal(Literal::Bool(false))) }
            Some(TokenKind::Null) => { self.advance(); Ok(Expression::Literal(Literal::Null)) }
            Some(TokenKind::Void) => { self.advance(); Ok(Expression::Literal(Literal::Null)) } // Treat void literal as Null for now
            Some(TokenKind::LBracket) => {
                let span = self.advance().span;
                let mut elements = Vec::new();
                while !self.check(TokenKind::RBracket) && !self.is_at_end() {
                    elements.push(Box::new(self.parse_expression()?));
                    if !self.match_token(TokenKind::Comma) { break; }
                }
                self.consume_token(TokenKind::RBracket)?;
                Ok(Expression::Array(ArrayExpr { elements, span }))
            }
            Some(TokenKind::Ident) | Some(TokenKind::SelfValue) | Some(TokenKind::SelfType) => {
                let name_token = self.advance();
                let name = name_token.literal.clone().unwrap_or_default();
                let name_span = name_token.span;
                
                // Try to parse generic struct instantiation: Name<T> { fields }
                let checkpoint = self.position;
                if self.match_token(TokenKind::Less) {
                    let mut type_args = Vec::new();
                    let mut is_generic_struct = true;
                    while !self.check(TokenKind::Greater) && !self.is_at_end() {
                        if let Ok(ty) = self.parse_type() {
                            type_args.push(ty);
                            if !self.match_token(TokenKind::Comma) { break; }
                        } else {
                            is_generic_struct = false;
                            break;
                        }
                    }
                    
                    if is_generic_struct && self.match_token(TokenKind::Greater) && self.check(TokenKind::LBrace) {
                        self.consume_token(TokenKind::LBrace)?;
                        let mut fields = Vec::new();
                        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
                            let field_name = self.consume_identifier()?;
                            self.consume_token(TokenKind::Colon)?;
                            let value = self.parse_expression()?;
                            fields.push(StructField { name: field_name, value: Box::new(value), span: name_span });
                            if !self.match_token(TokenKind::Comma) { break; }
                        }
                        let r_brace_span = self.consume_token(TokenKind::RBrace)?.span;
                        return Ok(Expression::Struct(StructExpr { 
                            name, 
                            type_args: Some(type_args), 
                            fields, 
                            span: name_span.combine(r_brace_span) 
                        }));
                    } else {
                        // Backtrack: might be a generic function call Name<T>(...) 
                        // or just Name followed by a Less operator.
                        self.position = checkpoint;
                    }
                }

                // Normal struct instantiation: Name { fields }
                if self.check(TokenKind::LBrace) {
                    self.advance();
                    let mut fields = Vec::new();
                    while !self.check(TokenKind::RBrace) && !self.is_at_end() {
                        let field_name = self.consume_identifier()?;
                        self.consume_token(TokenKind::Colon)?;
                        let value = self.parse_expression()?;
                        fields.push(StructField { name: field_name, value: Box::new(value), span: name_span });
                        if !self.match_token(TokenKind::Comma) { break; }
                    }
                    let r_brace_span = self.consume_token(TokenKind::RBrace)?.span;
                    Ok(Expression::Struct(StructExpr { name, type_args: None, fields, span: name_span.combine(r_brace_span) }))
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            Some(TokenKind::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume_token(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err(miette::miette!("Expected expression")),
        }
    }

    fn get_binary_op(&self) -> Option<BinaryOp> {
        match self.peek_kind() {
            Some(TokenKind::Plus) => Some(BinaryOp::Add),
            Some(TokenKind::Minus) => Some(BinaryOp::Sub),
            Some(TokenKind::Star) => Some(BinaryOp::Mul),
            Some(TokenKind::Slash) => Some(BinaryOp::Div),
            Some(TokenKind::Percent) => Some(BinaryOp::Mod),
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
        let t = self.advance();
        if t.kind == TokenKind::Ident || t.kind == TokenKind::SelfValue || t.kind == TokenKind::SelfType {
            Ok(t.literal.clone().unwrap_or_default())
        } else {
            Err(miette::miette!("Expected identifier, but found {:?}", t.kind))
        }
    }

    fn consume_string(&mut self) -> Result<String> {
        let t = self.advance();
        if t.kind == TokenKind::StringLiteral {
            Ok(t.literal.clone().unwrap_or_default())
        } else {
            Err(miette::miette!("Expected string literal"))
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
        self.tokens.get(self.position).unwrap_or_else(|| self.tokens.last().unwrap())
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.position - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek_kind() == Some(TokenKind::EOF)
    }
}

pub trait SpanExt {
    fn span_info(&self) -> Span;
}

impl SpanExt for Expression {
    fn span_info(&self) -> Span {
        match self {
            Expression::Binary(e) => e.span,
            Expression::Unary(e) => e.span,
            Expression::Literal(_) => Span::default(),
            Expression::Identifier(_) => Span::default(),
            Expression::Call(e) => e.span,
            Expression::Member(e) => e.span,
            Expression::Index(e) => e.span,
            Expression::If(e) => e.span,
            Expression::Block(e) => e.span,
            Expression::ErrorUnion(e) => e.span,
            Expression::Array(e) => e.span,
            Expression::Tuple(e) => e.span,
            Expression::Struct(e) => e.span,
            Expression::Await(e) => e.span,
            Expression::Try(e) => e.span,
            Expression::Cast(e) => e.span,
        }
    }
}

