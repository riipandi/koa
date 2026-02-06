use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::typeck::TypeChecker;
use koa::ir::IrLowerer;
use koa::llvm_gen::LLVMCodegen;
use inkwell::context::Context;
use miette::Result;

fn compile_to_llvm(source: &str) -> Result<String> {
    // Lex
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // Type check
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    
    // Lower to IR (with monomorphization)
    let mut lowerer = IrLowerer::new();
    let ir = lowerer.lower(&ast)?;
    
    // Generate LLVM IR
    let context = Context::create();
    let mut codegen = LLVMCodegen::new(&context, "test_module");
    let llvm_ir = codegen.compile(&ir)?;
    
    Ok(llvm_ir)
}

#[test]
fn test_simple_function_llvm() -> Result<()> {
    let source = "
        fn add(x: i32, y: i32): i32 {
            return x;
        }
        
        fn main(): i32 {
            return 0;
        }
    ";
    
    let llvm_ir = compile_to_llvm(source)?;
    
    // Verify basic LLVM IR structure
    assert!(llvm_ir.contains("define"), "Should have function definitions");
    assert!(llvm_ir.contains("add"), "Should have add function");
    assert!(llvm_ir.contains("main"), "Should have main function");
    assert!(llvm_ir.contains("ret"), "Should have return statements");
    
    Ok(())
}

#[test]
fn test_generic_function_parsing() -> Result<()> {
    // Just verify that generic functions can be parsed and type-checked
    let source = "
        fn identity<T>(x: T): T {
            return x;
        }
        
        fn main(): i32 {
            return 0;
        }
    ";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    
    // If we get here, parsing and type checking succeeded
    Ok(())
}

#[test]
fn test_struct_llvm() -> Result<()> {
    let source = "
        struct Point {
            x: i32;
            y: i32;
        }
        
        fn main(): i32 {
            return 0;
        }
    ";
    
    let llvm_ir = compile_to_llvm(source)?;
    
    // Verify LLVM IR was generated
    assert!(llvm_ir.contains("define"), "Should have function definitions");
    assert!(llvm_ir.contains("main"), "Should have main function");
    
    Ok(())
}

