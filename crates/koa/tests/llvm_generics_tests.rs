use inkwell::context::Context;
use koa::ir::IrLowerer;
use koa::lexer::Lexer;
use koa::llvm_gen::LLVMCodegen;
use koa::parser::Parser;
use koa::typeck::TypeChecker;
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

fn lower_to_ir(source: &str) -> Result<String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    let mut lowerer = IrLowerer::new();
    let ir = lowerer.lower(&ast)?;

    let mut output = String::new();
    for func in &ir.functions {
        output.push_str(&format!("fn {}(...) : {:?}\n", func.name, func.return_type));
        output.push_str("  Params:\n");
        for param in &func.params {
            output.push_str(&format!("    {} : {:?}\n", param.name, param.type_));
        }
        output.push_str("  Blocks:\n");
        for block in &func.blocks {
            output.push_str(&format!("    {}:\n", block.name));
            for instr in &block.instructions {
                output.push_str(&format!("      {:?}\n", instr));
            }
        }
    }
    Ok(output)
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
    assert!(
        llvm_ir.contains("define"),
        "Should have function definitions"
    );
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
    assert!(
        llvm_ir.contains("define"),
        "Should have function definitions"
    );
    assert!(llvm_ir.contains("main"), "Should have main function");

    Ok(())
}

#[test]
fn test_generic_function_llvm() -> Result<()> {
    let source = "
        fn identity<T>(x: T): T {
            return x;
        }
        
        fn main(): i32 {
            let result: i32 = identity<i32>(42);
            return result;
        }
    ";

    let llvm_ir = compile_to_llvm(source)?;

    println!("=== LLVM IR ===");
    println!("{}", llvm_ir);
    println!("=== END LLVM IR ===");

    // Verify specialized function was generated
    assert!(
        llvm_ir.contains("define"),
        "Should have function definitions"
    );
    assert!(llvm_ir.contains("main"), "Should have main function");
    // The specialized function name should be mangled
    assert!(
        llvm_ir.contains("identity"),
        "Should have specialized identity function"
    );

    Ok(())
}

#[test]
fn test_generic_struct_llvm() -> Result<()> {
    let source = "
        struct Box<T> {
            value: T;
        }
        
        fn main(): i32 {
            let b: Box<i32> = Box<i32> { value: 42 };
            return 0;
        }
    ";

    let llvm_ir = compile_to_llvm(source)?;

    // Verify LLVM IR was generated
    assert!(
        llvm_ir.contains("define"),
        "Should have function definitions"
    );
    assert!(llvm_ir.contains("main"), "Should have main function");
    // Struct types should be present
    assert!(llvm_ir.contains("%"), "Should have LLVM struct types");

    Ok(())
}

#[test]
fn test_multiple_generic_instantiations() -> Result<()> {
    let source = "
        fn identity<T>(x: T): T {
            return x;
        }
        
        fn main(): i32 {
            let a: i32 = identity<i32>(42);
            let b: f64 = identity<f64>(3.14);
            return 0;
        }
    ";

    let llvm_ir = compile_to_llvm(source)?;

    println!("=== Multiple Generic LLVM IR ===");
    println!("{}", llvm_ir);
    println!("=== END ===");

    // Should have specialized versions for both i32 and f64
    assert!(
        llvm_ir.contains("identity"),
        "Should have identity function"
    );
    assert!(llvm_ir.contains("main"), "Should have main function");

    Ok(())
}

#[test]
fn test_generic_with_constraints() -> Result<()> {
    let source = "
        interface Printable {
            fn print(self): void;
        }
        
        fn show<T: Printable>(item: T): void {
            return;
        }
        
        struct Book {
            title: string;
            fn print(self): void {
                return;
            }
        }
        
        fn main(): i32 {
            let book: Book = Book { title: \"Test\" };
            show<Book>(book);
            return 0;
        }
    ";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;

    // If we get here without errors, interface satisfaction works
    Ok(())
}

#[test]
fn test_debug_ir_output() -> Result<()> {
    let source = r#"
        fn identity<T>(x: T): T {
            return x;
        }
        
        fn main(): i32 {
            let a: i32 = identity<i32>(42);
            let b: f64 = identity<f64>(3.14);
            return 0;
        }
    "#;

    let ir = lower_to_ir(source)?;

    println!("=== IR Output ===");
    println!("{}", ir);
    println!("=== END IR ===");

    Ok(())
}
