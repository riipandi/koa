//! LLVM IR generation
//!
//! Converts the intermediate representation to LLVM IR for compilation.

use crate::ir::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;

/// LLVM code generator
pub struct LLVMCodegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    locals: HashMap<String, PointerValue<'ctx>>,
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
                self.context.ptr_type(AddressSpace::default()),
            )],
            true,
        );

        self.module.add_function("printf", printf_type, None);

        // Declare puts
        let puts_type = self.context.i32_type().fn_type(
            &[BasicMetadataTypeEnum::PointerType(
                self.context.ptr_type(AddressSpace::default()),
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

        let entry_block = self.context.append_basic_block(*fn_value, "entry");
        self.builder.position_at_end(entry_block);

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
        }

        // Generate instructions
        for instruction in &function.body.instructions {
            self.codegen_instruction(instruction)?;
        }

        self.current_function = None;
        self.locals.clear();

        Ok(())
    }

    /// Generate instruction
    fn codegen_instruction(&mut self, instruction: &IrInstruction) -> Result<()> {
        match instruction {
            IrInstruction::Alloca { name, type_ } => {
                let llvm_type = self.ir_type_to_llvm(type_);
                let alloca = self
                    .builder
                    .build_alloca(llvm_type, name)
                    .into_diagnostic()?;
                self.locals.insert(name.clone(), alloca);
            }

            IrInstruction::Store { value, dest } => {
                let value_llvm = self.operand_to_llvm_value(value)?;
                let dest_ptr = self.operand_to_llvm_pointer(dest)?;
                self.builder
                    .build_store(dest_ptr, value_llvm)
                    .into_diagnostic()?;
            }

            IrInstruction::Load { src, dest: _ } => {
                let src_ptr = self.operand_to_llvm_pointer(src)?;
                self.builder
                    .build_load(self.context.i32_type(), src_ptr, "load_tmp")
                    .into_diagnostic()?;
            }

            IrInstruction::Binary {
                op,
                left,
                right,
                dest,
            } => {
                let left_val = self.operand_to_llvm_value(left)?;
                let right_val = self.operand_to_llvm_value(right)?;

                match op {
                    IrBinaryOp::Add => {
                        self.builder
                            .build_int_add(
                                left_val.into_int_value(),
                                right_val.into_int_value(),
                                dest,
                            )
                            .into_diagnostic()?;
                    }
                    IrBinaryOp::Sub => {
                        self.builder
                            .build_int_sub(
                                left_val.into_int_value(),
                                right_val.into_int_value(),
                                dest,
                            )
                            .into_diagnostic()?;
                    }
                    IrBinaryOp::Mul => {
                        self.builder
                            .build_int_mul(
                                left_val.into_int_value(),
                                right_val.into_int_value(),
                                dest,
                            )
                            .into_diagnostic()?;
                    }
                    IrBinaryOp::Div => {
                        self.builder
                            .build_int_signed_div(
                                left_val.into_int_value(),
                                right_val.into_int_value(),
                                dest,
                            )
                            .into_diagnostic()?;
                    }
                    _ => {
                        return Err(miette::miette!(
                            "Binary operation not implemented: {:?}",
                            op
                        ));
                    }
                };
            }

            IrInstruction::Call {
                callee,
                args,
                dest: _,
            } => {
                let fn_value = self
                    .module
                    .get_function(callee)
                    .ok_or_else(|| miette::miette!("Function not found: {}", callee))?;

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

            _ => {
                return Err(miette::miette!(
                    "Instruction not implemented: {:?}",
                    instruction
                ));
            }
        }

        Ok(())
    }

    /// Convert IR operand to LLVM value
    fn operand_to_llvm_value(&self, operand: &IrOperand) -> Result<BasicValueEnum<'ctx>> {
        match operand {
            IrOperand::Constant(constant) => Ok(self.ir_constant_to_llvm(constant)?),
            IrOperand::Local(name) | IrOperand::Temp(name) => {
                let ptr = self.get_local_pointer(name)?;
                let value = self
                    .builder
                    .build_load(self.context.i32_type(), ptr, &format!("load_{}", name))
                    .into_diagnostic()?;
                Ok(value)
            }
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
            IrType::Void => {
                // Void is not a BasicTypeEnum, use i32 as placeholder
                // This will be handled specially in contexts where void is needed
                self.context.i32_type().into()
            }
            IrType::Bool => self.context.i8_type().into(),
            IrType::Int8 => self.context.i8_type().into(),
            IrType::Int16 => self.context.i16_type().into(),
            IrType::Int32 => self.context.i32_type().into(),
            IrType::Int64 => self.context.i64_type().into(),
            IrType::Uint8 => self.context.i8_type().into(),
            IrType::Uint16 => self.context.i16_type().into(),
            IrType::Uint32 => self.context.i32_type().into(),
            IrType::Uint64 => self.context.i64_type().into(),
            IrType::Float32 => self.context.f32_type().into(),
            IrType::Float64 => self.context.f64_type().into(),
            IrType::Pointer(inner) => self
                .ir_type_to_llvm(inner)
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
            IrType::Function(_, _) => {
                // Function types are handled separately in codegen_function_decl
                // Return i32 as fallback
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_function_compilation() {
        let ir_program = IrProgram {
            functions: vec![IrFunction {
                name: "main".to_string(),
                params: vec![],
                return_type: IrType::Int32,
                body: IrBlock {
                    instructions: vec![IrInstruction::Return {
                        value: Some(IrOperand::Constant(IrConstant::Int(0))),
                    }],
                },
                is_pub: true,
            }],
            globals: vec![],
            types: std::collections::HashMap::new(),
        };

        let result = compile_to_llvm(&ir_program);
        assert!(result.is_ok());

        let llvm_ir = result.unwrap();
        assert!(llvm_ir.contains("define i32 @main()"));
        assert!(llvm_ir.contains("ret i32 0"));
    }

    #[test]
    fn test_printf_external_function() {
        let context = Context::create();
        let mut codegen = LLVMCodegen::new(&context, "test_module");

        codegen.declare_external_functions();
        let printf = codegen.module().get_function("printf");

        assert!(printf.is_some());
    }
}
