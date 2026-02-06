use koa::llvm_gen::compile_to_llvm;
use koa::ir::{IrProgram, IrFunction, IrType, IrBlock, IrInstruction, IrOperand, IrConstant};

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
