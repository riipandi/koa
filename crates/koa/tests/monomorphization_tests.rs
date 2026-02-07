use koa::ir::IrLowerer;
use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::typeck::TypeChecker;
use miette::Result;

fn lower_to_ir(source: &str) -> Result<String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;

    let mut lowerer = IrLowerer::new();
    let ir = lowerer.lower(&ast)?;
    Ok(format!("{}", ir))
}

#[test]
fn test_monomorphized_function() -> Result<()> {
    let ir = lower_to_ir(
        "fn identity<T>(x: T): T { return x; } fn main(): void { identity<i32>(42); identity<f64>(3.14); }",
    )?;

    // Check if we have two versions of identity
    assert!(ir.contains("fn identity<I32>(...)"));
    assert!(ir.contains("fn identity<F64>(...)"));
    Ok(())
}

#[test]
fn test_monomorphized_struct() -> Result<()> {
    let ir = lower_to_ir(
        "struct Box<T> { value: T; } fn main(): void { let b1: Box<i32> = Box<i32> { value: 42 }; let b2: Box<f64> = Box<f64> { value: 3.14 }; }",
    )?;

    // Check if we have two versions of Box
    assert!(ir.contains("type Box<I32> ="));
    assert!(ir.contains("type Box<F64> ="));
    Ok(())
}
