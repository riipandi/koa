//! Type checking implementation
//!
//! This module contains the TypeChecker which validates type correctness.

use super::symbol::{Scope, Symbol};
use crate::ast::*;
use miette::Result;

use std::collections::HashMap;

/// Type checker for Koa
pub struct TypeChecker {
    scopes: Vec<Scope>,
    current_fn_return_type: Option<Type>,
    structs: HashMap<String, StructDecl>,
    functions: HashMap<String, FnDecl>,
    interfaces: HashMap<String, InterfaceDecl>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()], // Global scope
            current_fn_return_type: None,
            structs: HashMap::new(),
            functions: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_symbol(&mut self, name: String, type_: Type, is_const: bool) -> Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.symbols.contains_key(&name) {
                return Err(miette::miette!(
                    "Redefinition of symbol '{}' in this scope",
                    name
                ));
            }
            scope.symbols.insert(
                name.clone(),
                Symbol {
                    name,
                    type_,
                    is_const,
                },
            );
        }
        Ok(())
    }

    fn resolve_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn check(&mut self, ast: &Ast) -> Result<()> {
        for decl in &ast.declarations {
            self.check_declaration(decl)?;
        }
        Ok(())
    }

    fn check_declaration(&mut self, decl: &Declaration) -> Result<()> {
        match decl {
            Declaration::FnDecl(fn_decl) => self.check_fn_decl(fn_decl),
            Declaration::StructDecl(struct_decl) => self.check_struct_decl(struct_decl),
            Declaration::EnumDecl(enum_decl) => self.check_enum_decl(enum_decl),
            Declaration::InterfaceDecl(interface_decl) => self.check_interface_decl(interface_decl),
            Declaration::ConstDecl(const_decl) => self.check_const_decl(const_decl),
            Declaration::ErrorDecl(error_decl) => self.check_error_decl(error_decl),
            Declaration::ImportDecl(import_decl) => self.check_import_decl(import_decl),
            Declaration::ExportDecl(export_decl) => {
                self.check_declaration(&export_decl.declaration)
            }
        }
    }

    fn check_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<()> {
        // Define function in current scope (before entering its own scope)
        self.define_symbol(fn_decl.name.clone(), fn_decl.return_type.clone(), true)?;
        self.functions.insert(fn_decl.name.clone(), fn_decl.clone());

        self.enter_scope();

        // Check type parameters and add them to scope
        for tp in &fn_decl.type_params {
            self.define_symbol(tp.name.clone(), Type::Named(tp.name.clone()), true)?;
        }

        // Check parameter types and define them in local scope
        for param in &fn_decl.params {
            self.check_type(&param.type_annotation)?;
            self.define_symbol(param.name.clone(), param.type_annotation.clone(), false)?;
        }

        // Check return type
        self.check_type(&fn_decl.return_type)?;
        let prev_ret = self.current_fn_return_type.clone();
        self.current_fn_return_type = Some(fn_decl.return_type.clone());

        // Check body
        self.check_block(&fn_decl.body)?;

        self.current_fn_return_type = prev_ret;
        self.leave_scope();

        Ok(())
    }

    fn check_struct_decl(&mut self, struct_decl: &StructDecl) -> Result<()> {
        if self.structs.contains_key(&struct_decl.name) {
            return Err(miette::miette!(
                "Redefinition of struct '{}'",
                struct_decl.name
            ));
        }
        self.structs
            .insert(struct_decl.name.clone(), struct_decl.clone());

        self.enter_scope();
        // Check type parameters and add them to scope
        for tp in &struct_decl.type_params {
            self.define_symbol(tp.name.clone(), Type::Named(tp.name.clone()), true)?;
        }

        // Check field types and methods
        for field in &struct_decl.fields {
            self.check_type(&field.type_)?;
        }

        for method in &struct_decl.methods {
            self.check_fn_decl(method)?;
        }
        self.leave_scope();
        Ok(())
    }

    fn check_interface_decl(&mut self, interface_decl: &InterfaceDecl) -> Result<()> {
        if self.interfaces.contains_key(&interface_decl.name) {
            return Err(miette::miette!(
                "Redefinition of interface '{}'",
                interface_decl.name
            ));
        }
        self.interfaces
            .insert(interface_decl.name.clone(), interface_decl.clone());

        self.enter_scope();
        for tp in &interface_decl.type_params {
            self.define_symbol(tp.name.clone(), Type::Named(tp.name.clone()), true)?;
        }
        for method in &interface_decl.methods {
            for param in &method.params {
                self.check_type(&param.type_annotation)?;
            }
            self.check_type(&method.return_type)?;
        }
        self.leave_scope();
        Ok(())
    }

    fn check_enum_decl(&mut self, _enum_decl: &EnumDecl) -> Result<()> {
        // Check variant types
        Ok(())
    }

    fn check_const_decl(&mut self, const_decl: &ConstDecl) -> Result<()> {
        self.check_type(&const_decl.type_)?;
        let _val_type = self.check_expression(&const_decl.value)?;
        // TODO: Validate _val_type matches const_decl.type_
        self.define_symbol(const_decl.name.clone(), const_decl.type_.clone(), true)?;
        Ok(())
    }

    fn check_error_decl(&mut self, _error_decl: &ErrorDecl) -> Result<()> {
        Ok(())
    }

    fn check_import_decl(&mut self, _import_decl: &ImportDecl) -> Result<()> {
        Ok(())
    }

    fn check_block(&mut self, block: &Block) -> Result<()> {
        for stmt in &block.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    #[allow(clippy::collapsible_if)]
    fn check_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let(let_stmt) => {
                let val_type = if let Some(value) = &let_stmt.value {
                    Some(self.check_expression(value)?)
                } else {
                    None
                };

                let inferred_type = match (&let_stmt.type_annotation, val_type) {
                    (Some(anno), Some(val)) => {
                        if !self.is_assignable(&val, anno) {
                            return Err(miette::miette!(
                                "Type mismatch: cannot assign {:?} to {:?}",
                                val,
                                anno
                            ));
                        }
                        self.check_type(anno)?;
                        anno.clone()
                    }
                    (Some(anno), None) => {
                        self.check_type(anno)?;
                        anno.clone()
                    }
                    (None, Some(val)) => val,
                    (None, None) => {
                        return Err(miette::miette!(
                            "Variable must have a type or an initial value"
                        ));
                    }
                };

                self.define_symbol(let_stmt.name.clone(), inferred_type, false)?;
                Ok(())
            }
            Statement::Const(const_stmt) => {
                let val_type = self.check_expression(&const_stmt.value)?;
                let final_type = if let Some(anno) = &const_stmt.type_annotation {
                    if !self.is_assignable(&val_type, anno) {
                        return Err(miette::miette!(
                            "Type mismatch: cannot assign {:?} to {:?}",
                            val_type,
                            anno
                        ));
                    }
                    self.check_type(anno)?;
                    anno.clone()
                } else {
                    val_type
                };
                self.define_symbol(const_stmt.name.clone(), final_type, true)?;
                Ok(())
            }
            Statement::Expr(expr_stmt) => {
                self.check_expression(&expr_stmt.expr)?;
                Ok(())
            }
            Statement::Return(return_stmt) => {
                let ret_type = if let Some(value) = &return_stmt.value {
                    self.check_expression(value)?
                } else {
                    Type::Void
                };

                if let Some(expected) = &self.current_fn_return_type {
                    if !self.is_assignable(&ret_type, expected) {
                        return Err(miette::miette!(
                            "Type mismatch: return type {:?} does not match expected {:?}",
                            ret_type,
                            expected
                        ));
                    }
                }
                Ok(())
            }
            Statement::Break(_) | Statement::Continue(_) => Ok(()),
            Statement::If(if_stmt) => {
                let cond_type = self.check_expression(&if_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(miette::miette!(
                        "'if' condition must be a boolean, but found {:?}",
                        cond_type
                    ));
                }
                self.check_block(&if_stmt.then_block)?;
                if let Some(else_block) = &if_stmt.else_block {
                    self.check_block(else_block)?;
                }
                Ok(())
            }
            Statement::While(while_stmt) => {
                let cond_type = self.check_expression(&while_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(miette::miette!(
                        "'while' condition must be a boolean, but found {:?}",
                        cond_type
                    ));
                }
                self.check_block(&while_stmt.body)?;
                Ok(())
            }
            Statement::Loop(loop_stmt) => self.check_block(&loop_stmt.body),
            Statement::Match(match_stmt) => {
                self.check_expression(&match_stmt.scrutinee)?;
                for arm in &match_stmt.arms {
                    self.check_pattern(&arm.pattern)?;
                    if let Some(guard) = &arm.guard {
                        self.check_expression(guard)?;
                    }
                    self.check_block(&arm.body)?;
                }
                Ok(())
            }
            Statement::Try(try_stmt) => {
                self.check_block(&try_stmt.body)?;
                self.check_block(&try_stmt.catch_block)?;
                Ok(())
            }
            Statement::Throw(throw_stmt) => {
                self.check_expression(&throw_stmt.expr)?;
                Ok(())
            }
            Statement::Defer(defer_stmt) => self.check_statement(&defer_stmt.statement),
            Statement::ErrDefer(defer_stmt) => self.check_statement(&defer_stmt.statement),
        }
    }

    #[allow(clippy::collapsible_if)]
    fn check_expression(&mut self, expr: &Expression) -> Result<Type> {
        match expr {
            Expression::Binary(binary_expr) => {
                let left = self.check_expression(&binary_expr.left)?;
                let right = self.check_expression(&binary_expr.right)?;

                match binary_expr.op {
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Mod => {
                        if !self.is_numeric(&left) || !self.is_numeric(&right) {
                            return Err(miette::miette!(
                                "Arithmetic operator {:?} requires numeric types, but found {:?} and {:?}",
                                binary_expr.op,
                                left,
                                right
                            ));
                        }
                        if left != right {
                            // TODO: Support implicit numeric promotion?
                            return Err(miette::miette!(
                                "Type mismatch in arithmetic expression: {:?} and {:?}",
                                left,
                                right
                            ));
                        }
                        Ok(left)
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        if !self.is_assignable(&left, &right) && !self.is_assignable(&right, &left)
                        {
                            return Err(miette::miette!(
                                "Comparison operator {:?} requires compatible types, but found {:?} and {:?}",
                                binary_expr.op,
                                left,
                                right
                            ));
                        }
                        Ok(Type::Bool)
                    }
                    BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual => {
                        if !self.is_numeric(&left) || !self.is_numeric(&right) {
                            return Err(miette::miette!(
                                "Comparison operator {:?} requires numeric types, but found {:?} and {:?}",
                                binary_expr.op,
                                left,
                                right
                            ));
                        }
                        Ok(Type::Bool)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left != Type::Bool || right != Type::Bool {
                            return Err(miette::miette!(
                                "Logical operator {:?} requires boolean types, but found {:?} and {:?}",
                                binary_expr.op,
                                left,
                                right
                            ));
                        }
                        Ok(Type::Bool)
                    }
                    _ => Ok(left),
                }
            }
            Expression::Unary(unary_expr) => {
                let operand_type = self.check_expression(&unary_expr.expr)?;
                match unary_expr.op {
                    UnaryOp::Neg => {
                        if !self.is_numeric(&operand_type) {
                            return Err(miette::miette!(
                                "Negation operator '-' requires a numeric type, but found {:?}",
                                operand_type
                            ));
                        }
                        Ok(operand_type)
                    }
                    UnaryOp::Not => {
                        if operand_type != Type::Bool {
                            return Err(miette::miette!(
                                "Logical NOT operator '!' requires a boolean type, but found {:?}",
                                operand_type
                            ));
                        }
                        Ok(Type::Bool)
                    }
                    _ => Ok(operand_type),
                }
            }
            Expression::Literal(lit) => match lit {
                Literal::Int(_) => Ok(Type::I32),
                Literal::Float(_) => Ok(Type::F64),
                Literal::String(_) => Ok(Type::String),
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::Null => Ok(Type::Void),
            },
            Expression::Identifier(name) => {
                if let Some(sym) = self.resolve_symbol(name) {
                    Ok(sym.type_.clone())
                } else if let Some(fn_decl) = self.functions.get(name).cloned() {
                    let params = fn_decl
                        .params
                        .iter()
                        .map(|p| p.type_annotation.clone())
                        .collect();
                    Ok(Type::Function(
                        params,
                        Box::new(fn_decl.return_type.clone()),
                    ))
                } else {
                    Err(miette::miette!("Undefined identifier: {}", name))
                }
            }
            Expression::Call(call_expr) => self.check_call_expr(call_expr),
            Expression::Member(member_expr) => {
                let obj_type = self.check_expression(&member_expr.object)?;
                if let Type::Named(struct_name) = &obj_type {
                    if let Some(s) = self.structs.get(struct_name) {
                        if let Some(field) =
                            s.fields.iter().find(|f| f.name == member_expr.property)
                        {
                            return Ok(field.type_.clone());
                        }
                        // Check methods in struct
                        if let Some(method) =
                            s.methods.iter().find(|m| m.name == member_expr.property)
                        {
                            let params = method
                                .params
                                .iter()
                                .map(|p| p.type_annotation.clone())
                                .collect();
                            return Ok(Type::Function(
                                params,
                                Box::new(method.return_type.clone()),
                            ));
                        }
                    }
                }

                // Check top-level functions for method-like signature: fn foo(self: T, ...)
                if let Some(fn_decl) = self.functions.get(&member_expr.property).cloned() {
                    if !fn_decl.params.is_empty() && fn_decl.params[0].name == "self" {
                        if self.is_assignable(&obj_type, &fn_decl.params[0].type_annotation) {
                            let params = fn_decl
                                .params
                                .iter()
                                .map(|p| p.type_annotation.clone())
                                .collect();
                            return Ok(Type::Function(
                                params,
                                Box::new(fn_decl.return_type.clone()),
                            ));
                        }
                    }
                }

                if let Type::Named(struct_name) = &obj_type {
                    return Err(miette::miette!(
                        "Struct '{}' has no member or method '{}'",
                        struct_name,
                        member_expr.property
                    ));
                }
                Err(miette::miette!(
                    "Type {:?} has no member '{}'",
                    obj_type,
                    member_expr.property
                ))
            }
            Expression::Index(index_expr) => {
                let obj_type = self.check_expression(&index_expr.object)?;
                let idx_type = self.check_expression(&index_expr.index)?;

                if !self.is_integer(&idx_type) {
                    return Err(miette::miette!(
                        "Array index must be an integer, but found {:?}",
                        idx_type
                    ));
                }

                match obj_type {
                    Type::Array(inner) => Ok(*inner),
                    Type::String => Ok(Type::U8), // String indexing returns bytes/chars
                    _ => Err(miette::miette!("Type {:?} cannot be indexed", obj_type)),
                }
            }
            Expression::If(if_expr) => {
                let cond_type = self.check_expression(&if_expr.condition)?;
                if cond_type != Type::Bool {
                    return Err(miette::miette!(
                        "'if' expression condition must be a boolean, but found {:?}",
                        cond_type
                    ));
                }
                let then_type = self.check_expression(&if_expr.then_expr)?;
                if let Some(else_expr) = &if_expr.else_expr {
                    let else_type = self.check_expression(else_expr)?;
                    if then_type != else_type {
                        return Err(miette::miette!(
                            "'if' expression branches must have same type, but found {:?} and {:?}",
                            then_type,
                            else_type
                        ));
                    }
                    Ok(then_type)
                } else {
                    // If no else branch, the expression must result in Void or Optional
                    Ok(Type::Void)
                }
            }
            Expression::Block(block) => {
                self.enter_scope();
                self.check_block(block)?;
                self.leave_scope();
                Ok(Type::Void)
            }
            Expression::ErrorUnion(error_union_expr) => {
                self.check_type(&error_union_expr.value_type)?;
                Ok(Type::ErrorUnion(
                    error_union_expr.error_name.clone(),
                    error_union_expr.value_type.clone(),
                ))
            }
            Expression::Array(array_expr) => {
                let mut first_type = None;
                for element in &array_expr.elements {
                    let t = self.check_expression(element)?;
                    if first_type.is_none() {
                        first_type = Some(t);
                    }
                }
                Ok(Type::Array(Box::new(first_type.unwrap_or(Type::Void))))
            }
            Expression::Tuple(tuple_expr) => {
                let mut types = Vec::new();
                for element in &tuple_expr.elements {
                    types.push(self.check_expression(element)?);
                }
                Ok(Type::Tuple(types))
            }
            Expression::Struct(struct_expr) => {
                if let Some(s) = self.structs.get(&struct_expr.name).cloned() {
                    let mut substitution = HashMap::new();
                    if let Some(args) = &struct_expr.type_args {
                        if args.len() != s.type_params.len() {
                            return Err(miette::miette!(
                                "Struct '{}' expects {} type arguments, but {} were provided",
                                struct_expr.name,
                                s.type_params.len(),
                                args.len()
                            ));
                        }
                        // Check generic constraints
                        self.check_constraints(&s.type_params, args)?;

                        for (i, param) in s.type_params.iter().enumerate() {
                            substitution.insert(param.name.clone(), args[i].clone());
                        }
                    } else if !s.type_params.is_empty() {
                        return Err(miette::miette!(
                            "Struct '{}' requires type arguments for instantiation",
                            struct_expr.name
                        ));
                    }

                    for field_decl in &s.fields {
                        if !struct_expr.fields.iter().any(|f| f.name == field_decl.name) {
                            return Err(miette::miette!(
                                "Missing field '{}' in initializer for struct '{}'",
                                field_decl.name,
                                struct_expr.name
                            ));
                        }
                    }
                    for field in &struct_expr.fields {
                        let decl =
                            s.fields
                                .iter()
                                .find(|f| f.name == field.name)
                                .ok_or_else(|| {
                                    miette::miette!(
                                        "No such field '{}' in struct '{}'",
                                        field.name,
                                        struct_expr.name
                                    )
                                })?;
                        let val_type = self.check_expression(&field.value)?;
                        let expected_type = self.substitute_type(&decl.type_, &substitution);
                        if !self.is_assignable(&val_type, &expected_type) {
                            return Err(miette::miette!(
                                "Type mismatch for field '{}' in struct '{}': expected {:?}, found {:?}",
                                field.name,
                                struct_expr.name,
                                expected_type,
                                val_type
                            ));
                        }
                    }

                    if let Some(args) = &struct_expr.type_args {
                        Ok(Type::Generic(
                            Box::new(Type::Named(struct_expr.name.clone())),
                            args.clone(),
                        ))
                    } else {
                        Ok(Type::Named(struct_expr.name.clone()))
                    }
                } else {
                    Err(miette::miette!("Undefined struct '{}'", struct_expr.name))
                }
            }
            Expression::Await(await_expr) => self.check_expression(&await_expr.expr),
            Expression::Try(try_expr) => self.check_expression(&try_expr.expr),
            Expression::Cast(cast_expr) => {
                self.check_expression(&cast_expr.expr)?;
                self.check_type(&cast_expr.target_type)?;
                Ok(cast_expr.target_type.clone())
            }
        }
    }

    fn check_type(&mut self, _type_: &Type) -> Result<()> {
        match _type_ {
            Type::Named(_name) => {
                // TODO: Check if name is a valid type in scope
                Ok(())
            }
            Type::Generic(base, args) => {
                self.check_type(base)?;
                for arg in args {
                    self.check_type(arg)?;
                }
                Ok(())
            }
            Type::Array(inner) | Type::Pointer(inner) | Type::Optional(inner) => {
                self.check_type(inner)
            }
            Type::Tuple(types) => {
                for t in types {
                    self.check_type(t)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_pattern(&mut self, _pattern: &Pattern) -> Result<()> {
        // Pattern validation would go here
        Ok(())
    }

    #[allow(clippy::collapsible_if)]
    fn check_call_expr(&mut self, call_expr: &CallExpr) -> Result<Type> {
        let callee = &*call_expr.callee;

        // Special case: Method call p.method(args)
        if let Expression::Member(member_expr) = callee {
            let obj_type = self.check_expression(&member_expr.object)?;
            if let Type::Named(struct_name) = &obj_type {
                if let Some(s) = self.structs.get(struct_name).cloned() {
                    if let Some(method) = s.methods.iter().find(|m| m.name == member_expr.property)
                    {
                        let mut substitution = HashMap::new();
                        if let Some(args) = &call_expr.type_args {
                            if args.len() != method.type_params.len() {
                                return Err(miette::miette!(
                                    "Method '{}' expects {} type arguments, but {} were provided",
                                    method.name,
                                    method.type_params.len(),
                                    args.len()
                                ));
                            }
                            // Check constraints
                            self.check_constraints(&method.type_params, args)?;

                            for (i, param) in method.type_params.iter().enumerate() {
                                substitution.insert(param.name.clone(), args[i].clone());
                            }
                        }

                        if method.params.is_empty() {
                            return Err(miette::miette!(
                                "Method {} must have a self parameter",
                                method.name
                            ));
                        }
                        let self_type =
                            self.substitute_type(&method.params[0].type_annotation, &substitution);
                        if !self.is_assignable(&obj_type, &self_type) {
                            return Err(miette::miette!(
                                "Cannot call method {} on type {:?}",
                                method.name,
                                obj_type
                            ));
                        }

                        if method.params.len() - 1 != call_expr.args.len() {
                            return Err(miette::miette!(
                                "Method '{}' expects {} arguments, but {} were provided",
                                method.name,
                                method.params.len() - 1,
                                call_expr.args.len()
                            ));
                        }

                        for (i, arg) in call_expr.args.iter().enumerate() {
                            let arg_type = self.check_expression(arg)?;
                            let expected = self.substitute_type(
                                &method.params[i + 1].type_annotation,
                                &substitution,
                            );
                            if !self.is_assignable(&arg_type, &expected) {
                                return Err(miette::miette!(
                                    "Argument {} to method '{}' has type {:?}, but expected {:?}",
                                    i,
                                    method.name,
                                    arg_type,
                                    expected
                                ));
                            }
                        }
                        return Ok(self.substitute_type(&method.return_type, &substitution));
                    }
                }

                // Check top-level functions for method-like signature: fn foo(self: T, ...)
                if let Some(fn_decl) = self.functions.get(&member_expr.property).cloned() {
                    if !fn_decl.params.is_empty() && fn_decl.params[0].name == "self" {
                        if self.is_assignable(&obj_type, &fn_decl.params[0].type_annotation) {
                            let mut substitution = HashMap::new();
                            if let Some(args) = &call_expr.type_args {
                                if args.len() != fn_decl.type_params.len() {
                                    return Err(miette::miette!(
                                        "Method '{}' expects {} type arguments, but {} were provided",
                                        fn_decl.name,
                                        fn_decl.type_params.len(),
                                        args.len()
                                    ));
                                }
                                // Check constraints
                                self.check_constraints(&fn_decl.type_params, args)?;

                                for (i, param) in fn_decl.type_params.iter().enumerate() {
                                    substitution.insert(param.name.clone(), args[i].clone());
                                }
                            }

                            if fn_decl.params.len() - 1 != call_expr.args.len() {
                                return Err(miette::miette!(
                                    "Method '{}' expects {} arguments, but {} were provided",
                                    fn_decl.name,
                                    fn_decl.params.len() - 1,
                                    call_expr.args.len()
                                ));
                            }

                            for (i, arg) in call_expr.args.iter().enumerate() {
                                let arg_type = self.check_expression(arg)?;
                                let expected = self.substitute_type(
                                    &fn_decl.params[i + 1].type_annotation,
                                    &substitution,
                                );
                                if !self.is_assignable(&arg_type, &expected) {
                                    return Err(miette::miette!(
                                        "Argument {} to method '{}' has type {:?}, but expected {:?}",
                                        i,
                                        fn_decl.name,
                                        arg_type,
                                        expected
                                    ));
                                }
                            }
                            return Ok(self.substitute_type(&fn_decl.return_type, &substitution));
                        }
                    }
                }
            }
        }

        let callee_type = self.check_expression(&call_expr.callee)?;

        if let Type::Function(params, ret) = callee_type {
            if params.len() != call_expr.args.len() {
                return Err(miette::miette!(
                    "Expected {} arguments, but found {}",
                    params.len(),
                    call_expr.args.len()
                ));
            }
            for (i, arg) in call_expr.args.iter().enumerate() {
                let arg_type = self.check_expression(arg)?;
                if !self.is_assignable(&arg_type, &params[i]) {
                    return Err(miette::miette!(
                        "Argument {} type mismatch: expected {:?}, found {:?}",
                        i,
                        params[i],
                        arg_type
                    ));
                }
            }
            return Ok(*ret);
        }

        // Handle case where callee is an identifier for a known function
        if let Expression::Identifier(name) = callee {
            if let Some(fn_decl) = self.functions.get(name).cloned() {
                let mut substitution = HashMap::new();
                if let Some(args) = &call_expr.type_args {
                    if args.len() != fn_decl.type_params.len() {
                        return Err(miette::miette!(
                            "Function '{}' expects {} type arguments, but {} were provided",
                            name,
                            fn_decl.type_params.len(),
                            args.len()
                        ));
                    }
                    // Check constraints
                    self.check_constraints(&fn_decl.type_params, args)?;

                    for (i, param) in fn_decl.type_params.iter().enumerate() {
                        substitution.insert(param.name.clone(), args[i].clone());
                    }
                }

                if fn_decl.params.len() != call_expr.args.len() {
                    return Err(miette::miette!(
                        "Function '{}' expects {} arguments, but {} were provided",
                        name,
                        fn_decl.params.len(),
                        call_expr.args.len()
                    ));
                }
                for (i, arg) in call_expr.args.iter().enumerate() {
                    let arg_type = self.check_expression(arg)?;
                    let expected =
                        self.substitute_type(&fn_decl.params[i].type_annotation, &substitution);
                    if !self.is_assignable(&arg_type, &expected) {
                        return Err(miette::miette!(
                            "Argument {} to function '{}' has type {:?}, but expected {:?}",
                            i,
                            name,
                            arg_type,
                            expected
                        ));
                    }
                }
                let ret_type = self.substitute_type(&fn_decl.return_type, &substitution);
                return Ok(ret_type);
            }
        }

        Err(miette::miette!(
            "Cannot call non-function type {:?}",
            callee_type
        ))
    }

    fn is_assignable(&self, from: &Type, to: &Type) -> bool {
        // Simple structural equality for now
        // TODO: Handle coercions, interface satisfaction, etc.
        if from == to {
            return true;
        }

        match (from, to) {
            // Check if from satisfies to (if to is an interface)
            (Type::Named(_), Type::Named(to_name)) if self.interfaces.contains_key(to_name) => {
                self.satisfies_interface(from, to).is_ok()
            }
            // Null to Optional
            (Type::Void, Type::Optional(_)) => true,
            // Empty array to any array type
            (Type::Array(inner), Type::Array(_)) if **inner == Type::Void => true,
            // Pointer covariance
            (Type::Pointer(inner1), Type::Pointer(inner2)) => self.is_assignable(inner1, inner2),
            // Array covariance (if immutable)
            (Type::Array(inner1), Type::Array(inner2)) => self.is_assignable(inner1, inner2),
            // Optional covariance
            (Type::Optional(inner1), Type::Optional(inner2)) => self.is_assignable(inner1, inner2),
            _ => false,
        }
    }

    fn is_numeric(&self, type_: &Type) -> bool {
        matches!(
            type_,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::Isize
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
                | Type::Usize
                | Type::F32
                | Type::F64
        )
    }

    fn is_integer(&self, type_: &Type) -> bool {
        matches!(
            type_,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::Isize
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
                | Type::Usize
        )
    }

    fn substitute_type(&self, ty: &Type, sub: &HashMap<String, Type>) -> Type {
        match ty {
            Type::Named(name) => {
                if let Some(replacement) = sub.get(name) {
                    replacement.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Generic(base, args) => {
                let new_base = self.substitute_type(base, sub);
                let new_args = args.iter().map(|a| self.substitute_type(a, sub)).collect();
                Type::Generic(Box::new(new_base), new_args)
            }
            Type::Pointer(inner) => Type::Pointer(Box::new(self.substitute_type(inner, sub))),
            Type::Array(inner) => Type::Array(Box::new(self.substitute_type(inner, sub))),
            Type::Optional(inner) => Type::Optional(Box::new(self.substitute_type(inner, sub))),
            Type::ErrorUnion(err, val) => {
                Type::ErrorUnion(err.clone(), Box::new(self.substitute_type(val, sub)))
            }
            Type::Tuple(ts) => {
                Type::Tuple(ts.iter().map(|t| self.substitute_type(t, sub)).collect())
            }
            Type::Function(ps, rs) => {
                let new_ps = ps.iter().map(|p| self.substitute_type(p, sub)).collect();
                Type::Function(new_ps, Box::new(self.substitute_type(rs, sub)))
            }
            _ => ty.clone(),
        }
    }

    fn check_constraints(&self, params: &[TypeParameter], args: &[Type]) -> Result<()> {
        for (i, param) in params.iter().enumerate() {
            let arg = &args[i];
            for constraint in &param.constraints {
                self.satisfies_interface(arg, constraint)?;
            }
        }
        Ok(())
    }

    fn satisfies_interface(&self, type_: &Type, interface: &Type) -> Result<()> {
        match (type_, interface) {
            (Type::Named(type_name), Type::Named(interface_name)) => {
                let iface = self
                    .interfaces
                    .get(interface_name)
                    .cloned()
                    .ok_or_else(|| miette::miette!("Undefined interface '{}'", interface_name))?;

                if let Some(s) = self.structs.get(type_name).cloned() {
                    for iface_method in &iface.methods {
                        let struct_method = s.methods.iter().find(|m| m.name == iface_method.name)
                            .ok_or_else(|| miette::miette!("Type '{}' does not implement method '{}' required by interface '{}'", type_name, iface_method.name, interface_name))?;

                        if struct_method.params.len() != iface_method.params.len() {
                            return Err(miette::miette!(
                                "Method '{}' in type '{}' has {} parameters, but interface '{}' expects {}",
                                iface_method.name,
                                type_name,
                                struct_method.params.len(),
                                interface_name,
                                iface_method.params.len()
                            ));
                        }

                        for (i, (iface_param, struct_param)) in iface_method
                            .params
                            .iter()
                            .zip(struct_method.params.iter())
                            .enumerate()
                        {
                            if iface_param.name == "self" {
                                continue;
                            }

                            let iface_param_type = &iface_param.type_annotation;
                            let struct_param_type = &struct_param.type_annotation;

                            if !self.is_assignable(struct_param_type, iface_param_type) {
                                return Err(miette::miette!(
                                    "Method '{}' in type '{}' has parameter {} with type {:?}, but interface '{}' expects {:?}",
                                    iface_method.name,
                                    type_name,
                                    i,
                                    struct_param_type,
                                    interface_name,
                                    iface_param_type
                                ));
                            }
                        }

                        let iface_return_type = &iface_method.return_type;
                        let struct_return_type = &struct_method.return_type;

                        if !self.is_assignable(struct_return_type, iface_return_type) {
                            return Err(miette::miette!(
                                "Method '{}' in type '{}' returns {:?}, but interface '{}' expects {:?}",
                                iface_method.name,
                                type_name,
                                struct_return_type,
                                interface_name,
                                iface_return_type
                            ));
                        }
                    }
                    Ok(())
                } else {
                    Err(miette::miette!(
                        "Type '{:?}' is not a struct and cannot implement interface '{}'",
                        type_,
                        interface_name
                    ))
                }
            }
            _ => Err(miette::miette!(
                "Interface constraints must be named interfaces, found {:?} : {:?}",
                type_,
                interface
            )),
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
