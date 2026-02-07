use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::typeck::TypeChecker;
use miette::Result;

fn check(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    Ok(())
}

#[test]
fn test_generic_function_decl() -> Result<()> {
    check("fn identity<T>(x: T): T { return x; }")
}

#[test]
fn test_generic_struct_decl() -> Result<()> {
    check("struct Box<T> { value: T; }")
}

#[test]
fn test_interface_decl() -> Result<()> {
    check("interface Stringer { fn to_string(self): string; }")
}

#[test]
fn test_generic_interface_decl() -> Result<()> {
    check("interface Wrapper<T> { fn wrap(self, val: T): void; }")
}

#[test]
fn test_generic_constraint_parsing() -> Result<()> {
    // This just tests parsing for now
    check("interface Read {} interface Write {} fn copy<T: Read + Write>(x: T): void {}")
}

#[test]
fn test_generic_call_parsing() -> Result<()> {
    check("fn identity<T>(x: T): T { return x; } fn main(): void { identity<i32>(42); }")
}

#[test]
fn test_generic_struct_instantiation_parsing() -> Result<()> {
    check(
        "struct Box<T> { value: T; } fn main(): void { let b: Box<i32> = Box<i32> { value: 42 }; }",
    )
}
