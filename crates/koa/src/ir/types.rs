//! IR type definitions
//!
//! This module contains all type definitions for the Intermediate Representation.

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
    Enum { variants: Vec<Vec<IrType>> },
    Function(Vec<IrType>, Box<IrType>),
}
