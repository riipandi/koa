//! IR lowering - AST to IR transformation
//!
//! This module contains the IrLowerer which transforms AST into IR.

use super::types::*;
use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use miette::Result;
use std::collections::HashMap;
use std::path::Path as StdPath;

pub struct IrLowerer {
    blocks: Vec<IrNamedBlock>,
    block_count: usize,
    temp_count: usize,
    struct_map: HashMap<String, StructDecl>,
    fn_map: HashMap<String, FnDecl>,
    enum_map: HashMap<String, EnumDecl>,
    specialized_functions: Vec<IrFunction>,
    specialized_function_names: Vec<String>,
    specialized_structs: HashMap<String, IrType>,
    specialized_enums: HashMap<String, IrType>,
    type_substitution: HashMap<String, Type>,
    loop_stack: Vec<(String, String)>,
    import_modules: HashMap<String, Vec<String>>,
    import_aliases: HashMap<String, String>,
    builtin_functions: Vec<String>,
}
impl IrLowerer {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            block_count: 0,
            temp_count: 0,
            struct_map: HashMap::new(),
            fn_map: HashMap::new(),
            enum_map: HashMap::new(),
            specialized_functions: Vec::new(),
            specialized_function_names: Vec::new(),
            specialized_structs: HashMap::new(),
            specialized_enums: HashMap::new(),
            type_substitution: HashMap::new(),
            loop_stack: Vec::new(),
            import_modules: HashMap::new(),
            import_aliases: HashMap::new(),
            builtin_functions: vec!["println".to_string(), "print".to_string()],
        }
    }

    pub fn lower(&mut self, ast: &Ast) -> Result<IrProgram> {
        let mut functions = Vec::new();
        let mut globals = Vec::new();
        let mut types = HashMap::new();

        // First pass: collect struct, enum, and function definitions
        for decl in &ast.declarations {
            match decl {
                Declaration::StructDecl(s) => {
                    self.struct_map.insert(s.name.clone(), s.clone());
                }
                Declaration::EnumDecl(e) => {
                    self.enum_map.insert(e.name.clone(), e.clone());
                }
                // Skip builtin functions
                Declaration::FnDecl(f) if !self.builtin_functions.contains(&f.name) => {
                    self.fn_map.insert(f.name.clone(), f.clone());
                }
                _ => {}
            }
        }

        for decl in &ast.declarations {
            match decl {
                Declaration::FnDecl(fn_decl) => {
                    // Skip builtin functions (they're in prelude)
                    if fn_decl.name == "println" || fn_decl.name == "print" {
                        continue;
                    }
                    // Only lower non-generic functions in the top level
                    if fn_decl.type_params.is_empty() {
                        let ir_fn = self.lower_fn_decl(fn_decl)?;
                        functions.push(ir_fn);
                    }
                }
                Declaration::ConstDecl(const_decl) => {
                    let global = self.lower_const_decl(const_decl)?;
                    globals.push(global);
                }
                Declaration::StructDecl(struct_decl) => {
                    // Only lower non-generic structs in the top level
                    if struct_decl.type_params.is_empty() {
                        let ir_type = self.lower_struct_decl(struct_decl)?;
                        types.insert(struct_decl.name.clone(), ir_type);
                    }
                }
                Declaration::EnumDecl(enum_decl) => {
                    // Only lower non-generic enums in the top level
                    if enum_decl.type_params.is_empty() {
                        let ir_type = self.lower_enum_decl(enum_decl)?;
                        types.insert(enum_decl.name.clone(), ir_type);
                    }
                }
                Declaration::ImportDecl(import_decl) => {
                    self.process_import(import_decl)?;
                }
                _ => {}
            }
        }

        // Add specialized functions
        functions.append(&mut self.specialized_functions);
        // Add specialized structs
        for (name, ty) in self.specialized_structs.drain() {
            types.insert(name, ty);
        }
        // Add specialized enums
        for (name, ty) in self.specialized_enums.drain() {
            types.insert(name, ty);
        }

        Ok(IrProgram {
            functions,
            globals,
            types,
        })
    }

    fn process_import(&mut self, import_decl: &ImportDecl) -> Result<()> {
        match &import_decl.kind {
            ImportKind::Module { alias } => {
                let module_name = alias.clone().unwrap_or_else(|| {
                    import_decl
                        .from
                        .split('/')
                        .next_back()
                        .unwrap_or(&import_decl.from)
                        .to_string()
                });

                let file_path = self.resolve_import_path(&import_decl.from)?;
                let source = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read import '{}': {}", file_path, e))?;

                let mut lexer = Lexer::new(&source);
                let tokens = lexer.tokenize()?;

                let mut parser = Parser::new(tokens);
                let ast = parser.parse()?;

                let mut imported_funcs = Vec::new();
                for decl in &ast.declarations {
                    if let Declaration::FnDecl(fn_decl) = decl
                        && fn_decl.is_pub
                    {
                        self.fn_map.insert(fn_decl.name.clone(), fn_decl.clone());
                        imported_funcs.push(fn_decl.name.clone());
                    }
                }

                self.import_modules
                    .insert(module_name.clone(), imported_funcs);
                self.import_aliases
                    .insert(module_name.clone(), import_decl.from.clone());
            }
            ImportKind::Specific { name, alias } => {
                let import_path = if import_decl.from.contains('/') {
                    let parts: Vec<&str> = import_decl.from.rsplitn(2, '/').collect();
                    parts[1].to_string()
                } else {
                    import_decl.from.clone()
                };

                let file_path = self.resolve_import_path(&import_path)?;
                let source = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read import '{}': {}", file_path, e))?;

                let mut lexer = Lexer::new(&source);
                let tokens = lexer.tokenize()?;

                let mut parser = Parser::new(tokens);
                let ast = parser.parse()?;

                for decl in &ast.declarations {
                    if let Declaration::FnDecl(fn_decl) = decl
                        && fn_decl.name == *name
                        && fn_decl.is_pub
                    {
                        let module_prefix = import_path.replace('/', "__");
                        let mangled_name = format!("{}__{}", module_prefix, name);
                        self.fn_map.insert(mangled_name.clone(), fn_decl.clone());

                        let import_name = alias.clone().unwrap_or_else(|| name.clone());
                        self.import_aliases.insert(import_name, mangled_name);
                    }
                }
            }
        }
        Ok(())
    }

    fn resolve_import_path(&self, import_path: &str) -> Result<String> {
        let cleaned_path = import_path.trim_start_matches('/');
        let path_obj = StdPath::new(cleaned_path);

        let file_name = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(cleaned_path);

        let dir_name = path_obj
            .parent()
            .and_then(|p| p.to_str())
            .and_then(|s| if s.is_empty() { None } else { Some(s) });

        let dir_path =
            dir_name.map(|d| d.replace('/', std::path::MAIN_SEPARATOR.to_string().as_str()));

        if let Some(ref dir) = dir_path {
            let std_file_path = StdPath::new("library/std")
                .join(dir.replace(std::path::MAIN_SEPARATOR, "/"))
                .join(format!("{}.koa", file_name));

            if std_file_path.exists() {
                return Ok(std_file_path.to_string_lossy().to_string());
            }

            let std_mod_path = StdPath::new("library/std")
                .join(dir.replace(std::path::MAIN_SEPARATOR, "/"))
                .join(file_name)
                .join("mod.koa");

            if std_mod_path.exists() {
                return Ok(std_mod_path.to_string_lossy().to_string());
            }
        } else {
            let std_file_path = StdPath::new("library/std").join(format!("{}.koa", file_name));

            if std_file_path.exists() {
                return Ok(std_file_path.to_string_lossy().to_string());
            }

            let std_mod_path = StdPath::new("library/std").join(file_name).join("mod.koa");

            if std_mod_path.exists() {
                return Ok(std_mod_path.to_string_lossy().to_string());
            }
        }

        if let Some(ref dir) = dir_path {
            let src_file_path = StdPath::new("src")
                .join(dir.replace(std::path::MAIN_SEPARATOR, "/"))
                .join(format!("{}.koa", file_name));

            if src_file_path.exists() {
                return Ok(src_file_path.to_string_lossy().to_string());
            }

            let src_mod_path = StdPath::new("src")
                .join(dir.replace(std::path::MAIN_SEPARATOR, "/"))
                .join(file_name)
                .join("mod.koa");

            if src_mod_path.exists() {
                return Ok(src_mod_path.to_string_lossy().to_string());
            }
        } else {
            let src_file_path = StdPath::new("src").join(format!("{}.koa", file_name));

            if src_file_path.exists() {
                return Ok(src_file_path.to_string_lossy().to_string());
            }

            let src_mod_path = StdPath::new("src").join(file_name).join("mod.koa");

            if src_mod_path.exists() {
                return Ok(src_mod_path.to_string_lossy().to_string());
            }
        }

        Err(miette::miette!("Import not found: {}", import_path))
    }

    fn new_temp(&mut self) -> String {
        let name = format!("t{}", self.temp_count);
        self.temp_count += 1;
        name
    }

    fn add_instr(&mut self, instr: IrInstruction) {
        if let Some(block) = self.blocks.last_mut() {
            block.instructions.push(instr);
        }
    }

    fn add_block(&mut self, name: String) {
        self.blocks.push(IrNamedBlock {
            name,
            instructions: Vec::new(),
        });
    }

    fn lower_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<IrFunction> {
        self.blocks.clear();
        self.block_count = 0;
        self.temp_count = 0;
        self.loop_stack.clear();

        let mut params = Vec::new();
        for param in &fn_decl.params {
            params.push(IrParam {
                name: param.name.clone(),
                type_: self.lower_type(&param.type_annotation)?,
            });
        }

        let return_type = self.lower_type(&fn_decl.return_type)?;

        // Entry block
        let entry_label = format!("entry_{}", self.block_count);
        self.block_count += 1;
        self.add_block(entry_label);
        self.lower_block(&fn_decl.body)?;

        // Ensure last block has a return if it's void and not terminated
        if return_type == IrType::Void {
            let has_terminator = if let Some(b) = self.blocks.last() {
                if let Some(i) = b.instructions.last() {
                    matches!(
                        i,
                        IrInstruction::Return { .. }
                            | IrInstruction::Jump { .. }
                            | IrInstruction::Branch { .. }
                    )
                } else {
                    false
                }
            } else {
                false
            };
            if !has_terminator {
                self.add_instr(IrInstruction::Return { value: None });
            }
        }

        Ok(IrFunction {
            name: fn_decl.name.clone(),
            params,
            return_type,
            blocks: self.blocks.clone(),
            is_pub: fn_decl.is_pub,
        })
    }

    fn lower_block(&mut self, block: &Block) -> Result<()> {
        for stmt in &block.statements {
            self.lower_statement(stmt)?;
        }
        Ok(())
    }

    fn lower_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let(let_stmt) => {
                let ty = if let Some(anno) = &let_stmt.type_annotation {
                    self.lower_type(anno)?
                } else {
                    IrType::Int32
                };

                self.add_instr(IrInstruction::Alloca {
                    name: let_stmt.name.clone(),
                    type_: ty,
                });

                if let Some(value) = &let_stmt.value {
                    let val_op = self.lower_expression(value)?;
                    self.add_instr(IrInstruction::Store {
                        value: val_op,
                        dest: IrOperand::Local(let_stmt.name.clone()),
                    });
                }
            }
            Statement::Const(const_stmt) => {
                let ty = if let Some(anno) = &const_stmt.type_annotation {
                    self.lower_type(anno)?
                } else {
                    IrType::Int32
                };

                self.add_instr(IrInstruction::Alloca {
                    name: const_stmt.name.clone(),
                    type_: ty,
                });

                let val_op = self.lower_expression(&const_stmt.value)?;
                self.add_instr(IrInstruction::Store {
                    value: val_op,
                    dest: IrOperand::Local(const_stmt.name.clone()),
                });
            }
            Statement::Return(return_stmt) => {
                let value = if let Some(expr) = &return_stmt.value {
                    Some(self.lower_expression(expr)?)
                } else {
                    None
                };
                self.add_instr(IrInstruction::Return { value });
            }
            Statement::Expr(expr_stmt) => {
                self.lower_expression(&expr_stmt.expr)?;
            }
            Statement::If(if_stmt) => {
                let cond = self.lower_expression(&if_stmt.condition)?;

                let then_label = format!("if_then_{}", self.block_count);
                let else_label = format!("if_else_{}", self.block_count);
                let merge_label = format!("if_merge_{}", self.block_count);
                self.block_count += 1;

                self.add_instr(IrInstruction::Branch {
                    condition: cond,
                    true_block: then_label.clone(),
                    false_block: if if_stmt.else_block.is_some() {
                        else_label.clone()
                    } else {
                        merge_label.clone()
                    },
                });

                // Then
                self.add_block(then_label);
                self.lower_block(&if_stmt.then_block)?;
                self.add_instr(IrInstruction::Jump {
                    target: merge_label.clone(),
                });

                // Else
                if let Some(else_block) = &if_stmt.else_block {
                    self.add_block(else_label);
                    self.lower_block(else_block)?;
                    self.add_instr(IrInstruction::Jump {
                        target: merge_label.clone(),
                    });
                }

                // Merge
                self.add_block(merge_label);
            }
            Statement::While(while_stmt) => {
                let cond_label = format!("while_cond_{}", self.block_count);
                let body_label = format!("while_body_{}", self.block_count);
                let end_label = format!("while_end_{}", self.block_count);
                self.block_count += 1;

                self.add_instr(IrInstruction::Jump {
                    target: cond_label.clone(),
                });

                // Condition
                self.add_block(cond_label.clone());
                let cond = self.lower_expression(&while_stmt.condition)?;
                self.add_instr(IrInstruction::Branch {
                    condition: cond,
                    true_block: body_label.clone(),
                    false_block: end_label.clone(),
                });

                // Body
                self.loop_stack
                    .push((cond_label.clone(), end_label.clone()));
                self.add_block(body_label);
                self.lower_block(&while_stmt.body)?;
                self.add_instr(IrInstruction::Jump { target: cond_label });
                self.loop_stack.pop();

                // End
                self.add_block(end_label);
            }
            Statement::Loop(loop_stmt) => {
                let body_label = format!("loop_body_{}", self.block_count);
                let end_label = format!("loop_end_{}", self.block_count);
                self.block_count += 1;

                self.add_instr(IrInstruction::Jump {
                    target: body_label.clone(),
                });

                // Body
                self.loop_stack
                    .push((body_label.clone(), end_label.clone()));
                self.add_block(body_label.clone());
                self.lower_block(&loop_stmt.body)?;
                self.add_instr(IrInstruction::Jump { target: body_label });
                self.loop_stack.pop();

                // End
                self.add_block(end_label);
            }
            Statement::Break(_) => {
                if let Some((_, break_label)) = self.loop_stack.last() {
                    let label = break_label.clone();
                    self.add_instr(IrInstruction::Jump { target: label });
                }
            }
            Statement::Continue(_) => {
                if let Some((continue_label, _)) = self.loop_stack.last() {
                    let label = continue_label.clone();
                    self.add_instr(IrInstruction::Jump { target: label });
                }
            }
            _ => {
                // Other statements not implemented yet
            }
        }
        Ok(())
    }

    fn lower_expression(&mut self, expr: &Expression) -> Result<IrOperand> {
        match expr {
            Expression::Literal(literal) => {
                let constant = self.lower_literal(literal)?;
                Ok(IrOperand::Constant(constant))
            }
            Expression::Identifier(name) => Ok(IrOperand::Local(name.clone())),
            Expression::Binary(binary_expr) => {
                let left = self.lower_expression(&binary_expr.left)?;
                let right = self.lower_expression(&binary_expr.right)?;
                let dest = self.new_temp();

                if let Some(op) = self.get_cmp_op(binary_expr.op) {
                    self.add_instr(IrInstruction::Cmp {
                        op,
                        left,
                        right,
                        dest: dest.clone(),
                    });
                } else {
                    let op = self.lower_binary_op(binary_expr.op);
                    self.add_instr(IrInstruction::Binary {
                        op,
                        left,
                        right,
                        dest: dest.clone(),
                    });
                }
                Ok(IrOperand::Temp(dest))
            }
            Expression::Unary(unary_expr) => {
                let operand = self.lower_expression(&unary_expr.expr)?;
                let dest = self.new_temp();
                let op = match unary_expr.op {
                    UnaryOp::Neg => IrUnaryOp::Neg,
                    UnaryOp::Not => IrUnaryOp::Not,
                    _ => IrUnaryOp::Not,
                };
                self.add_instr(IrInstruction::Unary {
                    op,
                    operand,
                    dest: dest.clone(),
                });
                Ok(IrOperand::Temp(dest))
            }
            Expression::Call(call_expr) => {
                let mut args = Vec::new();
                for arg in &call_expr.args {
                    args.push(self.lower_expression(arg)?);
                }

                let callee = match &*call_expr.callee {
                    Expression::Identifier(name) => {
                        // Check if it's a builtin function
                        if self.builtin_functions.contains(name) {
                            name.clone()
                        } else if let Some(mangled) = self.import_aliases.get(name) {
                            mangled.clone()
                        } else {
                            self.specialize_fn(name, call_expr.type_args.as_ref())?
                        }
                    }
                    Expression::Member(m) => match &*m.object {
                        Expression::Identifier(module_name) => {
                            format!("{}__{}", module_name, m.property)
                        }
                        _ => m.property.clone(),
                    },
                    _ => return Err(miette::miette!("Expected function name")),
                };

                let dest = self.new_temp();
                self.add_instr(IrInstruction::Call {
                    callee,
                    args,
                    dest: Some(dest.clone()),
                });
                Ok(IrOperand::Temp(dest))
            }
            Expression::Member(member_expr) => {
                let base = self.lower_expression(&member_expr.object)?;
                let dest = self.new_temp();

                // Simplified field access
                self.add_instr(IrInstruction::GEP {
                    base,
                    indices: vec![0], // Placeholder
                    dest: dest.clone(),
                });

                let load_dest = self.new_temp();
                self.add_instr(IrInstruction::Load {
                    src: IrOperand::Temp(dest),
                    dest: load_dest.clone(),
                });
                Ok(IrOperand::Temp(load_dest))
            }
            Expression::Struct(struct_expr) => {
                // Trigger struct specialization and get the type
                let struct_type =
                    self.specialize_struct(&struct_expr.name, struct_expr.type_args.as_ref())?;

                // For now, just evaluate field expressions for side effects
                // and return a placeholder
                for field in &struct_expr.fields {
                    self.lower_expression(&field.value)?;
                }

                // Allocate local variable for struct (simplified)
                let local_name = self.new_temp();
                self.add_instr(IrInstruction::Alloca {
                    name: local_name.clone(),
                    type_: struct_type,
                });

                Ok(IrOperand::Local(local_name))
            }
            _ => Ok(IrOperand::Constant(IrConstant::Unit)),
        }
    }

    fn lower_literal(&mut self, literal: &Literal) -> Result<IrConstant> {
        match literal {
            Literal::Int(n) => Ok(IrConstant::Int(*n)),
            Literal::Float(n) => Ok(IrConstant::Float(*n)),
            Literal::Bool(b) => Ok(IrConstant::Bool(*b)),
            Literal::String(s) => Ok(IrConstant::String(s.clone())),
            Literal::Null => Ok(IrConstant::Null),
        }
    }

    fn lower_binary_op(&mut self, op: BinaryOp) -> IrBinaryOp {
        match op {
            BinaryOp::Add => IrBinaryOp::Add,
            BinaryOp::Sub => IrBinaryOp::Sub,
            BinaryOp::Mul => IrBinaryOp::Mul,
            BinaryOp::Div => IrBinaryOp::Div,
            BinaryOp::Mod => IrBinaryOp::Mod,
            BinaryOp::And => IrBinaryOp::And,
            BinaryOp::Or => IrBinaryOp::Or,
            _ => IrBinaryOp::Add,
        }
    }

    fn get_cmp_op(&self, op: BinaryOp) -> Option<IrCmpOp> {
        match op {
            BinaryOp::Equal => Some(IrCmpOp::Eq),
            BinaryOp::NotEqual => Some(IrCmpOp::Ne),
            BinaryOp::Less => Some(IrCmpOp::Lt),
            BinaryOp::LessEqual => Some(IrCmpOp::Le),
            BinaryOp::Greater => Some(IrCmpOp::Gt),
            BinaryOp::GreaterEqual => Some(IrCmpOp::Ge),
            _ => None,
        }
    }

    fn lower_type(&mut self, type_: &Type) -> Result<IrType> {
        let type_ = self.substitute_type(type_);
        match type_ {
            Type::I8 => Ok(IrType::Int8),
            Type::I16 => Ok(IrType::Int16),
            Type::I32 => Ok(IrType::Int32),
            Type::I64 => Ok(IrType::Int64),
            Type::I128 => Ok(IrType::Int128),
            Type::Isize => Ok(IrType::Isize),
            Type::U8 => Ok(IrType::Uint8),
            Type::U16 => Ok(IrType::Uint16),
            Type::U32 => Ok(IrType::Uint32),
            Type::U64 => Ok(IrType::Uint64),
            Type::U128 => Ok(IrType::Uint128),
            Type::Usize => Ok(IrType::Usize),
            Type::F32 => Ok(IrType::Float32),
            Type::F64 => Ok(IrType::Float64),
            Type::Bool => Ok(IrType::Bool),
            Type::String => Ok(IrType::String),
            Type::Void => Ok(IrType::Void),
            Type::Pointer(inner) => Ok(IrType::Pointer(Box::new(self.lower_type(&inner)?))),
            Type::Array(inner) => Ok(IrType::Array(Box::new(self.lower_type(&inner)?), 0)), // 0 size for now
            Type::Optional(inner) => Ok(IrType::Pointer(Box::new(self.lower_type(&inner)?))), // Simplified
            Type::Named(name) => {
                // Return placeholder or look up in specialized structs/enums
                if let Some(spec) = self.specialized_structs.get(&name) {
                    Ok(spec.clone())
                } else if let Some(spec) = self.specialized_enums.get(&name) {
                    Ok(spec.clone())
                } else if self.struct_map.contains_key(&name) {
                    // Try to lower non-generic struct
                    self.specialize_struct(&name, None)
                } else if self.enum_map.contains_key(&name) {
                    // Try to lower non-generic enum
                    self.specialize_enum(&name, None)
                } else {
                    Ok(IrType::Int32)
                }
            }
            Type::Generic(base, args) => {
                if let Type::Named(name) = *base {
                    // Check if it's a struct or enum
                    if self.struct_map.contains_key(&name) {
                        self.specialize_struct(&name, Some(&args))
                    } else if self.enum_map.contains_key(&name) {
                        self.specialize_enum(&name, Some(&args))
                    } else {
                        Ok(IrType::Int32)
                    }
                } else {
                    Ok(IrType::Int32)
                }
            }
            _ => Ok(IrType::Int32), // Simplified
        }
    }

    fn lower_struct_decl(&mut self, struct_decl: &StructDecl) -> Result<IrType> {
        let mut fields = Vec::new();
        for field in &struct_decl.fields {
            fields.push(self.lower_type(&field.type_)?);
        }
        Ok(IrType::Struct(fields))
    }

    fn lower_enum_decl(&mut self, enum_decl: &EnumDecl) -> Result<IrType> {
        let mut variants = Vec::new();
        for variant in &enum_decl.variants {
            let mut variant_fields = Vec::new();
            for field in &variant.fields {
                variant_fields.push(self.lower_type(field)?);
            }
            variants.push(variant_fields);
        }
        Ok(IrType::Enum { variants })
    }

    fn specialize_fn_name(&self, name: &str, type_args: &[Type]) -> String {
        if type_args.is_empty() {
            return name.to_string();
        }
        let args_str = type_args
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<_>>()
            .join("_")
            .replace(" ", "")
            .replace("(", "_")
            .replace(")", "_")
            .replace("[", "_")
            .replace("]", "_")
            .replace(",", "_")
            .replace(":", "_")
            .replace("!", "_")
            .replace("\"", "");
        format!("{}<{}>", name, args_str)
    }

    fn substitute_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Named(name) => {
                if let Some(replacement) = self.type_substitution.get(name) {
                    replacement.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Generic(base, args) => {
                let new_base = self.substitute_type(base);
                let new_args = args.iter().map(|a| self.substitute_type(a)).collect();
                Type::Generic(Box::new(new_base), new_args)
            }
            Type::Pointer(inner) => Type::Pointer(Box::new(self.substitute_type(inner))),
            Type::Array(inner) => Type::Array(Box::new(self.substitute_type(inner))),
            Type::Optional(inner) => Type::Optional(Box::new(self.substitute_type(inner))),
            Type::ErrorUnion(err, val) => {
                Type::ErrorUnion(err.clone(), Box::new(self.substitute_type(val)))
            }
            Type::Tuple(ts) => Type::Tuple(ts.iter().map(|t| self.substitute_type(t)).collect()),
            Type::Function(ps, rs) => {
                let new_ps = ps.iter().map(|p| self.substitute_type(p)).collect();
                Type::Function(new_ps, Box::new(self.substitute_type(rs)))
            }
            _ => ty.clone(),
        }
    }

    fn specialize_fn(&mut self, name: &str, type_args: Option<&Vec<Type>>) -> Result<String> {
        let actual_type_args = type_args.map(|v| v.as_slice()).unwrap_or(&[]);
        let spec_name = self.specialize_fn_name(name, actual_type_args);

        if self.specialized_function_names.contains(&spec_name) {
            return Ok(spec_name);
        }

        if let Some(f) = self.fn_map.get(name).cloned() {
            if f.type_params.len() != actual_type_args.len() {
                return Err(miette::miette!(
                    "Function '{}' expects {} type arguments, but {} were provided",
                    name,
                    f.type_params.len(),
                    actual_type_args.len()
                ));
            }

            // Mark as specialized to avoid recursion
            self.specialized_function_names.push(spec_name.clone());

            // Save old state
            let old_sub = self.type_substitution.clone();
            let old_blocks = self.blocks.clone();
            let old_block_count = self.block_count;
            let old_temp_count = self.temp_count;

            // Set up new substitution
            for (i, param) in f.type_params.iter().enumerate() {
                self.type_substitution
                    .insert(param.name.clone(), actual_type_args[i].clone());
            }

            // Lower specialized function
            let mut spec_f = self.lower_fn_decl(&f)?;
            spec_f.name = spec_name.clone();
            self.specialized_functions.push(spec_f);

            // Restore state
            self.type_substitution = old_sub;
            self.blocks = old_blocks;
            self.block_count = old_block_count;
            self.temp_count = old_temp_count;

            return Ok(spec_name);
        }

        Ok(name.to_string())
    }

    fn specialize_struct(&mut self, name: &str, type_args: Option<&Vec<Type>>) -> Result<IrType> {
        if let Some(args) = type_args {
            let spec_name = self.specialize_fn_name(name, args);
            if let Some(hit) = self.specialized_structs.get(&spec_name) {
                return Ok(hit.clone());
            }

            if let Some(s) = self.struct_map.get(name).cloned() {
                // Save old substitution
                let old_sub = self.type_substitution.clone();
                // Set up new substitution for fields
                for (i, param) in s.type_params.iter().enumerate() {
                    self.type_substitution
                        .insert(param.name.clone(), args[i].clone());
                }

                let mut fields = Vec::new();
                for field in &s.fields {
                    let substituted = self.substitute_type(&field.type_);
                    fields.push(self.lower_type(&substituted)?);
                }

                let ir_type = IrType::Struct(fields);
                self.specialized_structs.insert(spec_name, ir_type.clone());
                self.type_substitution = old_sub;
                return Ok(ir_type);
            }
        }

        if let Some(s) = self.struct_map.get(name).cloned() {
            if !s.type_params.is_empty() {
                return Err(miette::miette!(
                    "Generic struct '{}' requires type arguments",
                    name
                ));
            }
            return self.lower_struct_decl(&s);
        }

        // Final fallback (placeholder logic from before)
        Ok(IrType::Int32)
    }

    fn specialize_enum(&mut self, name: &str, type_args: Option<&Vec<Type>>) -> Result<IrType> {
        if let Some(args) = type_args {
            let spec_name = self.specialize_fn_name(name, args);
            if let Some(hit) = self.specialized_enums.get(&spec_name) {
                return Ok(hit.clone());
            }

            if let Some(e) = self.enum_map.get(name).cloned() {
                // Save old substitution
                let old_sub = self.type_substitution.clone();
                // Set up new substitution for variant fields
                for (i, param) in e.type_params.iter().enumerate() {
                    self.type_substitution
                        .insert(param.name.clone(), args[i].clone());
                }

                let mut variants = Vec::new();
                for variant in &e.variants {
                    let mut variant_fields = Vec::new();
                    for field in &variant.fields {
                        let substituted = self.substitute_type(field);
                        variant_fields.push(self.lower_type(&substituted)?);
                    }
                    variants.push(variant_fields);
                }

                let ir_type = IrType::Enum { variants };
                self.specialized_enums.insert(spec_name, ir_type.clone());
                self.type_substitution = old_sub;
                return Ok(ir_type);
            }
        }

        if let Some(e) = self.enum_map.get(name).cloned() {
            if !e.type_params.is_empty() {
                return Err(miette::miette!(
                    "Generic enum '{}' requires type arguments",
                    name
                ));
            }
            return self.lower_enum_decl(&e);
        }

        // Final fallback
        Ok(IrType::Int32)
    }

    fn lower_const_decl(&mut self, const_decl: &ConstDecl) -> Result<IrGlobal> {
        let type_ = self.lower_type(&const_decl.type_)?;
        let init = match &const_decl.value {
            Expression::Literal(literal) => Some(self.lower_literal(literal)?),
            _ => None,
        };
        Ok(IrGlobal {
            name: const_decl.name.clone(),
            type_,
            init,
            is_pub: const_decl.is_pub,
        })
    }
}

impl Default for IrLowerer {
    fn default() -> Self {
        Self::new()
    }
}
