//! Intermediate Representation (IR)
//!
//! The IR is a simplified representation of the AST optimized for code generation.

use crate::ast::*;
use miette::Result;
use std::collections::HashMap;
use std::fmt;

/// Intermediate representation of a Koa program
#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
    pub globals: Vec<IrGlobal>,
    pub types: HashMap<String, IrType>,
}

impl fmt::Display for IrProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for global in &self.globals {
            writeln!(f, "global {} : {:?}", global.name, global.type_)?;
        }
        for (name, ty) in &self.types {
            writeln!(f, "type {} = {:?}", name, ty)?;
        }
        for func in &self.functions {
            writeln!(f, "fn {}(...) : {:?} {{", func.name, func.return_type)?;
            for block in &func.blocks {
                writeln!(f, "  {}:", block.name)?;
                for instr in &block.instructions {
                    writeln!(f, "    {:?}", instr)?;
                }
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}

/// Function in IR
#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<IrParam>,
    pub return_type: IrType,
    pub blocks: Vec<IrNamedBlock>,
    pub is_pub: bool,
}

/// Named block of IR instructions
#[derive(Debug, Clone)]
pub struct IrNamedBlock {
    pub name: String,
    pub instructions: Vec<IrInstruction>,
}

/// Parameter in IR
#[derive(Debug, Clone)]
pub struct IrParam {
    pub name: String,
    pub type_: IrType,
}

/// IR instructions
#[derive(Debug, Clone)]
pub enum IrInstruction {
    /// Allocate a local variable
    Alloca { name: String, type_: IrType },
    /// Store a value
    Store { value: IrOperand, dest: IrOperand },
    /// Load a value
    Load { src: IrOperand, dest: String },
    /// Binary operation
    Binary {
        op: IrBinaryOp,
        left: IrOperand,
        right: IrOperand,
        dest: String,
    },
    /// Unary operation
    Unary {
        op: IrUnaryOp,
        operand: IrOperand,
        dest: String,
    },
    /// Function call
    Call {
        callee: String,
        args: Vec<IrOperand>,
        dest: Option<String>,
    },
    /// Return from function
    Return { value: Option<IrOperand> },
    /// Conditional branch
    Branch {
        condition: IrOperand,
        true_block: String,
        false_block: String,
    },
    /// Unconditional jump
    Jump { target: String },
    /// Comparison
    Cmp {
        op: IrCmpOp,
        left: IrOperand,
        right: IrOperand,
        dest: String,
    },
    /// Get element pointer (for arrays/structs)
    GEP {
        base: IrOperand,
        indices: Vec<u32>,
        dest: String,
    },
}

/// Binary operations in IR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

/// Unary operations in IR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrUnaryOp {
    Neg,
    Not,
}

/// Comparison operations in IR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrCmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Operands in IR
#[derive(Debug, Clone)]
pub enum IrOperand {
    Local(String),
    Global(String),
    Constant(IrConstant),
    Temp(String),
}

/// Constants in IR
#[derive(Debug, Clone)]
pub enum IrConstant {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
    Unit,
}

/// Global variable
#[derive(Debug, Clone)]
pub struct IrGlobal {
    pub name: String,
    pub type_: IrType,
    pub init: Option<IrConstant>,
    pub is_pub: bool,
}

/// Types in IR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrType {
    Void,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Isize,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Usize,
    Float32,
    Float64,
    String,
    Pointer(Box<IrType>),
    Array(Box<IrType>, u64),
    Struct(Vec<IrType>),
    Function(Vec<IrType>, Box<IrType>),
}

/// Lower AST to IR
pub struct IrLowerer {
    blocks: Vec<IrNamedBlock>,
    block_count: usize,
    temp_count: usize,
    struct_map: HashMap<String, StructDecl>,
    fn_map: HashMap<String, FnDecl>,
    specialized_functions: Vec<IrFunction>,
    specialized_function_names: Vec<String>,
    specialized_structs: HashMap<String, IrType>,
    type_substitution: HashMap<String, Type>,
    loop_stack: Vec<(String, String)>, // (continue_label, break_label)
}

impl IrLowerer {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            block_count: 0,
            temp_count: 0,
            struct_map: HashMap::new(),
            fn_map: HashMap::new(),
            specialized_functions: Vec::new(),
            specialized_function_names: Vec::new(),
            specialized_structs: HashMap::new(),
            type_substitution: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    pub fn lower(&mut self, ast: &Ast) -> Result<IrProgram> {
        let mut functions = Vec::new();
        let mut globals = Vec::new();
        let mut types = HashMap::new();

        // First pass: collect struct and function definitions
        for decl in &ast.declarations {
            match decl {
                Declaration::StructDecl(s) => {
                    self.struct_map.insert(s.name.clone(), s.clone());
                }
                Declaration::FnDecl(f) => {
                    self.fn_map.insert(f.name.clone(), f.clone());
                }
                _ => {}
            }
        }

        for decl in &ast.declarations {
            match decl {
                Declaration::FnDecl(fn_decl) => {
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
                _ => {}
            }
        }

        // Add specialized functions
        functions.extend(self.specialized_functions.drain(..));
        // Add specialized structs
        for (name, ty) in self.specialized_structs.drain() {
            types.insert(name, ty);
        }

        Ok(IrProgram {
            functions,
            globals,
            types,
        })
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
            let has_terminator = self.blocks.last().map_or(false, |b| {
                b.instructions.last().map_or(false, |i| matches!(i, IrInstruction::Return { .. } | IrInstruction::Jump { .. } | IrInstruction::Branch { .. }))
            });
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
                    false_block: if if_stmt.else_block.is_some() { else_label.clone() } else { merge_label.clone() },
                });

                // Then
                self.add_block(then_label);
                self.lower_block(&if_stmt.then_block)?;
                self.add_instr(IrInstruction::Jump { target: merge_label.clone() });

                // Else
                if let Some(else_block) = &if_stmt.else_block {
                    self.add_block(else_label);
                    self.lower_block(else_block)?;
                    self.add_instr(IrInstruction::Jump { target: merge_label.clone() });
                }

                // Merge
                self.add_block(merge_label);
            }
            Statement::While(while_stmt) => {
                let cond_label = format!("while_cond_{}", self.block_count);
                let body_label = format!("while_body_{}", self.block_count);
                let end_label = format!("while_end_{}", self.block_count);
                self.block_count += 1;

                self.add_instr(IrInstruction::Jump { target: cond_label.clone() });

                // Condition
                self.add_block(cond_label.clone());
                let cond = self.lower_expression(&while_stmt.condition)?;
                self.add_instr(IrInstruction::Branch {
                    condition: cond,
                    true_block: body_label.clone(),
                    false_block: end_label.clone(),
                });

                // Body
                self.loop_stack.push((cond_label.clone(), end_label.clone()));
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

                self.add_instr(IrInstruction::Jump { target: body_label.clone() });

                // Body
                self.loop_stack.push((body_label.clone(), end_label.clone()));
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
                        // Trigger specialization!
                        self.specialize_fn(name, call_expr.type_args.as_ref())?
                    }
                    Expression::Member(m) => m.property.clone(),
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
                // Trigger struct specialization
                self.specialize_struct(&struct_expr.name, struct_expr.type_args.as_ref())?;

                let mut field_values = Vec::new();
                for field in &struct_expr.fields {
                    field_values.push(self.lower_expression(&field.value)?);
                }

                let dest = self.new_temp();
                Ok(IrOperand::Temp(dest))
            }
            _ => {
                Ok(IrOperand::Constant(IrConstant::Unit))
            }
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
                // Return placeholder or look up in specialized_structs
                if let Some(spec) = self.specialized_structs.get(&name) {
                    Ok(spec.clone())
                } else if self.struct_map.contains_key(&name) {
                    // Try to lower non-generic struct
                    self.specialize_struct(&name, None)
                } else {
                    Ok(IrType::Int32)
                }
            }
            Type::Generic(base, args) => {
                if let Type::Named(name) = *base {
                    self.specialize_struct(&name, Some(&args))
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

    fn specialize_fn_name(&self, name: &str, type_args: &[Type]) -> String {
        if type_args.is_empty() {
            return name.to_string();
        }
        let args_str = type_args.iter()
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
            Type::ErrorUnion(err, val) => Type::ErrorUnion(err.clone(), Box::new(self.substitute_type(val))),
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
                 return Err(miette::miette!("Function '{}' expects {} type arguments, but {} were provided", name, f.type_params.len(), actual_type_args.len()));
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
                self.type_substitution.insert(param.name.clone(), actual_type_args[i].clone());
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
                    self.type_substitution.insert(param.name.clone(), args[i].clone());
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
                return Err(miette::miette!("Generic struct '{}' requires type arguments", name));
            }
            return self.lower_struct_decl(&s);
        }
        
        // Final fallback (placeholder logic from before)
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
