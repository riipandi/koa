//! LLVM IR generation
//!
//! Converts the intermediate representation to LLVM IR for compilation.

use crate::ir::*;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;

/// LLVM code generator
pub struct LLVMCodegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    locals: HashMap<String, PointerValue<'ctx>>,
    local_types: HashMap<String, IrType>,
    temps: HashMap<String, BasicValueEnum<'ctx>>,
    temp_types: HashMap<String, IrType>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> LLVMCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            functions: HashMap::new(),
            locals: HashMap::new(),
            local_types: HashMap::new(),
            temps: HashMap::new(),
            temp_types: HashMap::new(),
            current_function: None,
        }
    }

    /// Compile IR program to LLVM IR
    pub fn compile(&mut self, ir_program: &IrProgram) -> Result<String> {
        // Declare external functions
        self.declare_external_functions();

        // Generate global variables
        for global in &ir_program.globals {
            self.codegen_global(global)?;
        }

        // Generate functions
        for function in &ir_program.functions {
            self.codegen_function_decl(function)?;
        }

        // Generate function bodies
        for function in &ir_program.functions {
            self.codegen_function_body(function)?;
        }

        Ok(self.module.print_to_string().to_string())
    }

    /// Declare external C functions (like printf)
    fn declare_external_functions(&mut self) {
        // Declare printf
        let printf_type = self.context.i32_type().fn_type(
            &[BasicMetadataTypeEnum::PointerType(
                self.context.i8_type().ptr_type(AddressSpace::default()),
            )],
            true,
        );

        self.module.add_function("printf", printf_type, None);

        // Declare puts
        let puts_type = self.context.i32_type().fn_type(
            &[BasicMetadataTypeEnum::PointerType(
                self.context.i8_type().ptr_type(AddressSpace::default()),
            )],
            false,
        );

        self.module.add_function("puts", puts_type, None);
    }

    /// Generate global variable
    fn codegen_global(&mut self, global: &IrGlobal) -> Result<()> {
        let llvm_type = self.ir_type_to_llvm(&global.type_);

        let initializer = if let Some(init) = &global.init {
            Some(self.ir_constant_to_llvm(init)?)
        } else {
            None
        };

        let global_value =
            self.module
                .add_global(llvm_type, Some(AddressSpace::default()), &global.name);

        if let Some(init) = initializer {
            global_value.set_initializer(&init);
        }

        if global.is_pub {
            global_value.set_linkage(inkwell::module::Linkage::External);
        }

        Ok(())
    }

    /// Generate function declaration
    fn codegen_function_decl(&mut self, function: &IrFunction) -> Result<()> {
        let param_types: Vec<BasicMetadataTypeEnum> = function
            .params
            .iter()
            .map(|p| self.ir_type_to_llvm(&p.type_).into())
            .collect();

        let return_type = self.ir_type_to_llvm(&function.return_type);
        let fn_type = return_type.fn_type(&param_types, false);

        let fn_value = if let Some(existing) = self.module.get_function(&function.name) {
            existing
        } else {
            self.module.add_function(&function.name, fn_type, None)
        };

        self.functions.insert(function.name.clone(), fn_value);

        Ok(())
    }

    /// Generate function body
    fn codegen_function_body(&mut self, function: &IrFunction) -> Result<()> {
        let fn_value = self
            .functions
            .get(&function.name)
            .ok_or_else(|| miette::miette!("Function not found: {}", function.name))?;

        self.current_function = Some(*fn_value);
        self.locals.clear();
        self.local_types.clear();
        self.temps.clear();
        self.temp_types.clear();

        // Pre-create all basic blocks
        let mut block_map = HashMap::new();
        for block in &function.blocks {
            let llvm_block = self.context.append_basic_block(*fn_value, &block.name);
            block_map.insert(block.name.clone(), llvm_block);
        }

        // Entry logic: parameters alloca
        if let Some(first_block) = function.blocks.first() {
            let llvm_block = block_map.get(&first_block.name).unwrap();
            self.builder.position_at_end(*llvm_block);

            // Allocate parameters and store initial values
            for (i, param) in function.params.iter().enumerate() {
                let param_value = fn_value
                    .get_nth_param(i as u32)
                    .ok_or_else(|| miette::miette!("Parameter {} not found", i))?;

                let alloca = self
                    .builder
                    .build_alloca(self.ir_type_to_llvm(&param.type_), &param.name)
                    .into_diagnostic()?;

                self.builder
                    .build_store(alloca, param_value)
                    .into_diagnostic()?;

                self.locals.insert(param.name.clone(), alloca);
                self.local_types
                    .insert(param.name.clone(), param.type_.clone());
            }
        }

        // Generate instructions for each block
        for block in &function.blocks {
            let llvm_block = block_map.get(&block.name).unwrap();
            self.builder.position_at_end(*llvm_block);
            for instruction in &block.instructions {
                self.codegen_instruction(instruction, &block_map)?;
            }
        }

        self.current_function = None;
        self.locals.clear();
        self.temps.clear();

        Ok(())
    }

    /// Generate instruction
    fn codegen_instruction(
        &mut self,
        instruction: &IrInstruction,
        block_map: &HashMap<String, inkwell::basic_block::BasicBlock<'ctx>>,
    ) -> Result<()> {
        match instruction {
            IrInstruction::Alloca { name, type_ } => {
                let llvm_type = self.ir_type_to_llvm(type_);
                let alloca = self
                    .builder
                    .build_alloca(llvm_type, name)
                    .into_diagnostic()?;
                self.locals.insert(name.clone(), alloca);
                self.local_types.insert(name.clone(), type_.clone());
            }

            IrInstruction::Store { value, dest } => {
                let value_llvm = self.operand_to_llvm_value(value)?;
                let dest_ptr = self.operand_to_llvm_pointer(dest)?;
                self.builder
                    .build_store(dest_ptr, value_llvm)
                    .into_diagnostic()?;
            }

            IrInstruction::Load { src, dest } => {
                let src_ptr = self.operand_to_llvm_pointer(src)?;
                let src_type = self.get_operand_type(src)?;
                let llvm_type = self.ir_type_to_llvm(&src_type);
                let res = self
                    .builder
                    .build_load(llvm_type, src_ptr, dest)
                    .into_diagnostic()?;
                self.temps.insert(dest.clone(), res);
                self.temp_types.insert(dest.clone(), src_type);
            }

            IrInstruction::Binary {
                op,
                left,
                right,
                dest,
            } => {
                let left_val = self.operand_to_llvm_value(left)?;
                let right_val = self.operand_to_llvm_value(right)?;

                let res = match op {
                    IrBinaryOp::Add => self
                        .builder
                        .build_int_add(left_val.into_int_value(), right_val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrBinaryOp::Sub => self
                        .builder
                        .build_int_sub(left_val.into_int_value(), right_val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrBinaryOp::Mul => self
                        .builder
                        .build_int_mul(left_val.into_int_value(), right_val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrBinaryOp::Div => self
                        .builder
                        .build_int_signed_div(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            dest,
                        )
                        .into_diagnostic()?,
                    IrBinaryOp::Mod => self
                        .builder
                        .build_int_signed_rem(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            dest,
                        )
                        .into_diagnostic()?,
                    IrBinaryOp::And => self
                        .builder
                        .build_and(left_val.into_int_value(), right_val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrBinaryOp::Or => self
                        .builder
                        .build_or(left_val.into_int_value(), right_val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrBinaryOp::Xor => self
                        .builder
                        .build_xor(left_val.into_int_value(), right_val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrBinaryOp::Shl => self
                        .builder
                        .build_left_shift(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            dest,
                        )
                        .into_diagnostic()?,
                    IrBinaryOp::Shr => self
                        .builder
                        .build_right_shift(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            false,
                            dest,
                        )
                        .into_diagnostic()?,
                };
                self.temps.insert(dest.clone(), res.into());
            }

            IrInstruction::Unary { op, operand, dest } => {
                let val = self.operand_to_llvm_value(operand)?;
                let res = match op {
                    IrUnaryOp::Neg => self
                        .builder
                        .build_int_neg(val.into_int_value(), dest)
                        .into_diagnostic()?,
                    IrUnaryOp::Not => self
                        .builder
                        .build_not(val.into_int_value(), dest)
                        .into_diagnostic()?,
                };
                self.temps.insert(dest.clone(), res.into());
            }

            IrInstruction::Cmp {
                op,
                left,
                right,
                dest,
            } => {
                let left_val = self.operand_to_llvm_value(left)?;
                let right_val = self.operand_to_llvm_value(right)?;
                let llvm_op = match op {
                    IrCmpOp::Eq => inkwell::IntPredicate::EQ,
                    IrCmpOp::Ne => inkwell::IntPredicate::NE,
                    IrCmpOp::Lt => inkwell::IntPredicate::SLT,
                    IrCmpOp::Le => inkwell::IntPredicate::SLE,
                    IrCmpOp::Gt => inkwell::IntPredicate::SGT,
                    IrCmpOp::Ge => inkwell::IntPredicate::SGE,
                };
                let res = self
                    .builder
                    .build_int_compare(
                        llvm_op,
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        dest,
                    )
                    .into_diagnostic()?;
                self.temps.insert(dest.clone(), res.into());
            }

            IrInstruction::GEP {
                base,
                indices,
                dest,
            } => {
                let base_ptr = self.operand_to_llvm_pointer(base)?;
                // Inkwell's build_gep is a bit different. For structs/arrays:
                let mut llvm_indices = Vec::new();
                for &idx in indices {
                    llvm_indices.push(self.context.i32_type().const_int(idx as u64, false));
                }

                // This is a placeholder, proper GEP needs careful type handling
                let res = unsafe {
                    self.builder
                        .build_gep(self.context.i8_type(), base_ptr, &llvm_indices, dest)
                        .into_diagnostic()?
                };
                self.temps.insert(dest.clone(), res.into());
            }

            IrInstruction::Call { callee, args, dest } => {
                // Map builtin and imported functions to runtime
                let actual_callee = match callee.as_str() {
                    "println" => "puts",
                    "print" => "printf",
                    "std__io__println" => "puts",
                    "std__io__print" => "printf",
                    "io__println" => "puts",
                    "io__print" => "printf",
                    _ => callee.as_str(),
                };

                let fn_value = self
                    .module
                    .get_function(actual_callee)
                    .ok_or_else(|| miette::miette!("Function not found: {}", actual_callee))?;

                let mut llvm_args = Vec::new();
                for arg in args {
                    let arg_value = self.operand_to_llvm_value(arg)?;
                    llvm_args.push(BasicMetadataValueEnum::from(arg_value));
                }

                let call_site = self
                    .builder
                    .build_call(fn_value, &llvm_args, "call")
                    .into_diagnostic()?;
                call_site.set_tail_call(false);

                if let Some(d) = dest
                    && let Some(val) = call_site.try_as_basic_value().left()
                {
                    self.temps.insert(d.clone(), val);
                }
            }

            IrInstruction::Return { value } => {
                if let Some(v) = value {
                    let ret_val = self.operand_to_llvm_value(v)?;
                    self.builder
                        .build_return(Some(&ret_val))
                        .into_diagnostic()?;
                } else {
                    self.builder.build_return(None).into_diagnostic()?;
                }
            }

            IrInstruction::Branch {
                condition,
                true_block,
                false_block,
            } => {
                let cond_val = self.operand_to_llvm_value(condition)?;
                let tb = block_map
                    .get(true_block)
                    .ok_or_else(|| miette::miette!("Block not found: {}", true_block))?;
                let fb = block_map
                    .get(false_block)
                    .ok_or_else(|| miette::miette!("Block not found: {}", false_block))?;
                self.builder
                    .build_conditional_branch(cond_val.into_int_value(), *tb, *fb)
                    .into_diagnostic()?;
            }

            IrInstruction::Jump { target } => {
                let dest = block_map
                    .get(target)
                    .ok_or_else(|| miette::miette!("Block not found: {}", target))?;
                self.builder
                    .build_unconditional_branch(*dest)
                    .into_diagnostic()?;
            }
        }

        Ok(())
    }

    /// Convert IR operand to LLVM value
    fn operand_to_llvm_value(&self, operand: &IrOperand) -> Result<BasicValueEnum<'ctx>> {
        match operand {
            IrOperand::Constant(constant) => Ok(self.ir_constant_to_llvm(constant)?),
            IrOperand::Local(name) => {
                let ptr = self.get_local_pointer(name)?;
                let operand_type = self.get_operand_type(operand)?;
                let llvm_type = self.ir_type_to_llvm(&operand_type);
                let value = self
                    .builder
                    .build_load(llvm_type, ptr, &format!("load_{}", name))
                    .into_diagnostic()?;
                Ok(value)
            }
            IrOperand::Temp(name) => self
                .temps
                .get(name)
                .copied()
                .ok_or_else(|| miette::miette!("Temp variable not found: {}", name)),
            IrOperand::Global(name) => {
                let global = self
                    .module
                    .get_global(name)
                    .ok_or_else(|| miette::miette!("Global not found: {}", name))?;
                Ok(global.as_pointer_value().into())
            }
        }
    }

    /// Convert IR operand to LLVM pointer
    fn operand_to_llvm_pointer(&self, operand: &IrOperand) -> Result<PointerValue<'ctx>> {
        match operand {
            IrOperand::Local(name) | IrOperand::Temp(name) => self.get_local_pointer(name),
            IrOperand::Global(name) => {
                let global = self
                    .module
                    .get_global(name)
                    .ok_or_else(|| miette::miette!("Global not found: {}", name))?;
                Ok(global.as_pointer_value())
            }
            IrOperand::Constant(_) => Err(miette::miette!("Cannot get pointer from constant")),
        }
    }

    /// Get local variable pointer
    fn get_local_pointer(&self, name: &str) -> Result<PointerValue<'ctx>> {
        self.locals
            .get(name)
            .copied()
            .ok_or_else(|| miette::miette!("Local variable not found: {}", name))
    }

    /// Get type of operand
    fn get_operand_type(&self, operand: &IrOperand) -> Result<IrType> {
        match operand {
            IrOperand::Local(name) => self
                .local_types
                .get(name)
                .or_else(|| self.temp_types.get(name))
                .cloned()
                .ok_or_else(|| miette::miette!("Type not found for local: {}", name)),
            IrOperand::Temp(name) => self
                .temp_types
                .get(name)
                .cloned()
                .ok_or_else(|| miette::miette!("Type not found for temp: {}", name)),
            IrOperand::Constant(constant) => {
                let ir_type = match constant {
                    IrConstant::Int(_) => IrType::Int32,
                    IrConstant::Float(_) => IrType::Float64,
                    IrConstant::Bool(_) => IrType::Bool,
                    IrConstant::String(_) => IrType::String,
                    IrConstant::Null => IrType::Int32,
                    IrConstant::Unit => IrType::Void,
                };
                Ok(ir_type)
            }
            IrOperand::Global(_) => Ok(IrType::Int32),
        }
    }

    /// Convert IR constant to LLVM value
    fn ir_constant_to_llvm(&self, constant: &IrConstant) -> Result<BasicValueEnum<'ctx>> {
        match constant {
            IrConstant::Int(n) => Ok(self.context.i32_type().const_int(*n as u64, false).into()),
            IrConstant::Float(f) => Ok(self.context.f64_type().const_float(*f).into()),
            IrConstant::Bool(b) => Ok(self
                .context
                .i8_type()
                .const_int(if *b { 1 } else { 0 }, false)
                .into()),
            IrConstant::String(s) => {
                // For now, just create a simple string constant
                let str_type = self.context.i8_type().array_type(s.len() as u32 + 1);
                let global = self.module.add_global(
                    str_type,
                    None,
                    &format!("str_{}", s.len()), // Use length as unique ID
                );

                let bytes: Vec<u8> = s.bytes().chain(std::iter::once(0)).collect();
                let const_str = self.context.i8_type().const_array(
                    &bytes
                        .iter()
                        .map(|&b| self.context.i8_type().const_int(b as u64, false))
                        .collect::<Vec<_>>(),
                );

                global.set_initializer(&const_str);
                global.set_constant(true);
                global.set_unnamed_addr(true);

                Ok(global.as_pointer_value().into())
            }
            IrConstant::Null => Ok(self.context.i32_type().const_int(0, false).into()),
            IrConstant::Unit => Ok(self.context.i32_type().const_int(0, false).into()),
        }
    }

    /// Convert IR type to LLVM type
    fn ir_type_to_llvm(&self, type_: &IrType) -> BasicTypeEnum<'ctx> {
        match type_ {
            IrType::Void => self.context.i32_type().into(),
            IrType::Bool => self.context.i8_type().into(),
            IrType::Int8 | IrType::Uint8 => self.context.i8_type().into(),
            IrType::Int16 | IrType::Uint16 => self.context.i16_type().into(),
            IrType::Int32 | IrType::Uint32 => self.context.i32_type().into(),
            IrType::Int64 | IrType::Uint64 => self.context.i64_type().into(),
            IrType::Int128 | IrType::Uint128 => self.context.i128_type().into(),
            IrType::Isize | IrType::Usize => self.context.i64_type().into(), // Assuming 64-bit
            IrType::Float32 => self.context.f32_type().into(),
            IrType::Float64 => self.context.f64_type().into(),
            IrType::String => self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            IrType::Pointer(_inner) => self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            IrType::Array(inner, size) => {
                self.ir_type_to_llvm(inner).array_type(*size as u32).into()
            }
            IrType::Struct(fields) => {
                let field_types: Vec<BasicTypeEnum> =
                    fields.iter().map(|t| self.ir_type_to_llvm(t)).collect();

                self.context.struct_type(&field_types, false).into()
            }
            IrType::Enum { variants: _ } => {
                // For now, represent enum as a struct with:
                // - tag (i32)
                // - union of all variant payloads (simplified as i32 for now)
                // TODO: Implement proper tagged union
                let tag_type = self.context.i32_type();
                let payload_type = self.context.i32_type(); // Placeholder
                let field_types = vec![tag_type.into(), payload_type.into()];
                self.context.struct_type(&field_types, false).into()
            }
            IrType::Function(_, _) => {
                // Function types are handled separately in codegen_function_decl
                self.context.i32_type().into()
            }
        }
    }

    /// Write LLVM IR to file
    pub fn write_to_file(&self, path: &str) -> Result<()> {
        use std::io::Write;

        let llvm_ir = self.module.print_to_string().to_string();
        let mut file = std::fs::File::create(path).into_diagnostic()?;
        file.write_all(llvm_ir.as_bytes()).into_diagnostic()?;

        Ok(())
    }

    /// Get module reference
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }
}

/// Compile IR program to LLVM module and return as string
pub fn compile_to_llvm(ir_program: &IrProgram) -> Result<String> {
    let context = Context::create();
    let mut codegen = LLVMCodegen::new(&context, "koa_module");
    codegen.compile(ir_program)
}
