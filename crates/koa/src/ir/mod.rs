//! Intermediate Representation (IR)
//!
//! The IR is a simplified representation of the AST optimized for code generation.

use crate::ast::*;
use miette::Result;
use std::collections::HashMap;

/// Intermediate representation of a Koa program
#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
    pub globals: Vec<IrGlobal>,
    pub types: HashMap<String, IrType>,
}

/// Function in IR
#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<IrParam>,
    pub return_type: IrType,
    pub body: IrBlock,
    pub is_pub: bool,
}

/// Parameter in IR
#[derive(Debug, Clone)]
pub struct IrParam {
    pub name: String,
    pub type_: IrType,
}

/// Block of IR instructions
#[derive(Debug, Clone)]
pub struct IrBlock {
    pub instructions: Vec<IrInstruction>,
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
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Pointer(Box<IrType>),
    Array(Box<IrType>, u64),
    Struct(Vec<IrType>),
    Function(Vec<IrType>, Box<IrType>),
}

/// Lower AST to IR
pub struct IrLowerer {
    // State for lowering
}

impl IrLowerer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lower(&mut self, ast: &Ast) -> Result<IrProgram> {
        let mut functions = Vec::new();
        let mut globals = Vec::new();
        let mut types = HashMap::new();

        for decl in &ast.declarations {
            match decl {
                Declaration::FnDecl(fn_decl) => {
                    let ir_fn = self.lower_fn_decl(fn_decl)?;
                    functions.push(ir_fn);
                }
                Declaration::ConstDecl(const_decl) => {
                    let global = self.lower_const_decl(const_decl)?;
                    globals.push(global);
                }
                _ => {
                    // Skip other declarations for now
                }
            }
        }

        Ok(IrProgram {
            functions,
            globals,
            types,
        })
    }

    fn lower_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<IrFunction> {
        let mut params = Vec::new();
        for param in &fn_decl.params {
            params.push(IrParam {
                name: param.name.clone(),
                type_: self.lower_type(&param.type_annotation)?,
            });
        }

        let return_type = self.lower_type(&fn_decl.return_type)?;
        let body = self.lower_block(&fn_decl.body)?;

        Ok(IrFunction {
            name: fn_decl.name.clone(),
            params,
            return_type,
            body,
            is_pub: fn_decl.is_pub,
        })
    }

    fn lower_block(&mut self, block: &Block) -> Result<IrBlock> {
        let mut instructions = Vec::new();

        for stmt in &block.statements {
            self.lower_statement(stmt, &mut instructions)?;
        }

        Ok(IrBlock { instructions })
    }

    fn lower_statement(
        &mut self,
        stmt: &Statement,
        instructions: &mut Vec<IrInstruction>,
    ) -> Result<(), miette::Report> {
        match stmt {
            Statement::Let(let_stmt) => {
                // Lower let statement
                if let Some(value) = &let_stmt.value {
                    let operand = self.lower_expression(value, instructions)?;
                    instructions.push(IrInstruction::Alloca {
                        name: let_stmt.name.clone(),
                        type_: IrType::Int32, // Simplified
                    });
                    instructions.push(IrInstruction::Store {
                        value: operand,
                        dest: IrOperand::Local(let_stmt.name.clone()),
                    });
                }
            }
            Statement::Const(const_stmt) => {
                // Lower const statement
                let operand = self.lower_expression(&const_stmt.value, instructions)?;
                instructions.push(IrInstruction::Alloca {
                    name: const_stmt.name.clone(),
                    type_: IrType::Int32, // Simplified
                });
                instructions.push(IrInstruction::Store {
                    value: operand,
                    dest: IrOperand::Local(const_stmt.name.clone()),
                });
            }
            Statement::Return(return_stmt) => {
                let value = if let Some(expr) = &return_stmt.value {
                    Some(self.lower_expression(expr, instructions)?)
                } else {
                    None
                };
                instructions.push(IrInstruction::Return { value });
            }
            Statement::Expr(expr_stmt) => {
                self.lower_expression(&expr_stmt.expr, instructions)?;
            }
            _ => {
                // Other statements not implemented yet
            }
        }
        Ok(())
    }

    fn lower_expression(
        &mut self,
        expr: &Expression,
        instructions: &mut Vec<IrInstruction>,
    ) -> Result<IrOperand> {
        match expr {
            Expression::Literal(literal) => {
                let constant = self.lower_literal(literal)?;
                Ok(IrOperand::Constant(constant))
            }
            Expression::Identifier(name) => Ok(IrOperand::Local(name.clone())),
            Expression::Binary(binary_expr) => {
                let left = self.lower_expression(&binary_expr.left, instructions)?;
                let right = self.lower_expression(&binary_expr.right, instructions)?;
                let dest = format!("{}_tmp", instructions.len());
                let op = self.lower_binary_op(binary_expr.op);
                instructions.push(IrInstruction::Binary {
                    op,
                    left,
                    right,
                    dest: dest.clone(),
                });
                Ok(IrOperand::Temp(dest))
            }
            Expression::Call(call_expr) => {
                let mut args = Vec::new();
                for arg in &call_expr.args {
                    args.push(self.lower_expression(arg, instructions)?);
                }
                let callee = match &*call_expr.callee {
                    Expression::Identifier(name) => name.clone(),
                    _ => return Err(miette::miette!("Expected function name")),
                };
                let dest = Some(format!("{}_ret", instructions.len()));
                instructions.push(IrInstruction::Call {
                    callee,
                    args,
                    dest: dest.clone(),
                });
                Ok(IrOperand::Temp(dest.unwrap()))
            }
            _ => {
                // Other expressions not implemented yet
                Ok(IrOperand::Constant(IrConstant::Unit))
            }
        }
    }

    fn lower_literal(&mut self, literal: &Literal) -> Result<IrConstant> {
        match literal {
            Literal::Number(n) => {
                // Check if it's an integer or float
                if n.fract() == 0.0 && n.abs() < (i64::MAX as f64) {
                    Ok(IrConstant::Int(*n as i64))
                } else {
                    Ok(IrConstant::Float(*n))
                }
            }
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
            _ => IrBinaryOp::Add, // Default
        }
    }

    fn lower_type(&mut self, _type_: &Type) -> Result<IrType> {
        // Simplified type lowering
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
