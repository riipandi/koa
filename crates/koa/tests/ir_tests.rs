use koa::ir::IrLowerer;
use koa::lexer::Lexer;
use koa::parser::Parser;

fn lower(source: &str) -> miette::Result<koa::ir::IrProgram> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Failed to parse");
    let mut lowerer = IrLowerer::new();
    lowerer.lower(&ast)
}

#[test]
fn test_ir_simple_function() {
    let source = "fn add(a: i32, b: i32): i32 { return a + b; }";
    let ir = lower(source).unwrap();
    assert_eq!(ir.functions.len(), 1);
    let f = &ir.functions[0];
    assert_eq!(f.name, "add");
    assert_eq!(f.params.len(), 2);
    assert!(f.blocks.len() >= 1);
    assert_eq!(f.blocks[0].name, "entry_0");
}

#[test]
fn test_ir_const_declaration() {
    let source = "const PI: f64 = 3.14159;";
    let ir = lower(source).unwrap();
    assert_eq!(ir.globals.len(), 1);
    let g = &ir.globals[0];
    assert_eq!(g.name, "PI");
}
