use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::ast::{Ast, Declaration, Type};

fn parse(source: &str) -> Ast {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    parser.parse().expect("Failed to parse")
}

#[test]
fn test_parse_const_declaration() {
    let ast = parse("const x: i32 = 10; const y: f64 = 20.0;");
    assert_eq!(ast.declarations.len(), 2);
    if let Declaration::ConstDecl(c) = &ast.declarations[0] {
        assert_eq!(c.name, "x");
    }
}

#[test]
fn test_parse_function() {
    let source = "fn add(a: i32, b: i32): i32 { return a + b; }";
    let ast = parse(source);
    assert_eq!(ast.declarations.len(), 1);
    if let Declaration::FnDecl(f) = &ast.declarations[0] {
        assert_eq!(f.name, "add");
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.body.statements.len(), 1);
    }
}

#[test]
fn test_parse_types() {
    let source = "
        const a: ?i32 = null;
        const b: []string = [];
        const c: *u8 = null;
        const d: MyError!void = null;
    ";
    let ast = parse(source);
    assert_eq!(ast.declarations.len(), 4);
}

#[test]
fn test_parse_struct() {
    let source = "
        struct Point {
            x: f64;
            y: f64;
            fn norm(self): f64 { return 0.0; }
        }
    ";
    let ast = parse(source);
    assert_eq!(ast.declarations.len(), 1);
    if let Declaration::StructDecl(s) = &ast.declarations[0] {
        assert_eq!(s.name, "Point");
        assert_eq!(s.fields.len(), 2);
        assert_eq!(s.methods.len(), 1);
    }
}

#[test]
fn test_parse_complex_expressions() {
    let source = "fn test(): void { let x = 1 + 2 * 3 == 7 && true; }";
    let ast = parse(source);
    assert_eq!(ast.declarations.len(), 1);
    if let Declaration::FnDecl(f) = &ast.declarations[0] {
        assert_eq!(f.body.statements.len(), 1);
    }
}

#[test]
fn test_parse_if_while() {
    let source = r#"
        fn control_flow(): void {
            if true {
                return;
            } else {
                return;
            }
            while false {
                break;
            }
        }
    "#;
    let ast = parse(source);
    assert_eq!(ast.declarations.len(), 1);
}

#[test]
fn test_parse_error_union_type() {
    let source = "fn foo(): !i32 { return 42; }";
    let ast = parse(source);
    if let Declaration::FnDecl(f) = &ast.declarations[0] {
        if let Type::ErrorUnion(err, _) = &f.return_type {
            assert!(err.is_none());
        } else {
            panic!("Expected error union type");
        }
    }
}
