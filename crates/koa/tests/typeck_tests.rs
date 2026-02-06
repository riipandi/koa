use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::typeck::TypeChecker;

fn check(source: &str) -> miette::Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Failed to parse");
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)
}

#[test]
fn test_typeck_undefined_identifier() {
    let result = check("fn test(): i32 { return x; }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Undefined identifier: x"));
}

#[test]
fn test_typeck_type_mismatch_assignment() {
    let result = check("fn test(): void { let x: i32 = \"hello\"; }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Type mismatch"));
}

#[test]
fn test_typeck_function_call_arg_count() {
    let result = check("fn add(a: i32, b: i32): i32 { return a + b; } fn test(): void { add(1); }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expects 2 arguments, but 1 were provided"));
}

#[test]
fn test_typeck_function_call_arg_type() {
    let result = check("fn add(a: i32, b: i32): i32 { return a + b; } fn test(): void { add(1, \"2\"); }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Argument 1 to function 'add' has type String, but expected I32"));
}

#[test]
fn test_typeck_struct_member_access() {
    let source = "
        struct Point { x: i32; y: i32; }
        fn test(): i32 {
            let p = Point { x: 1, y: 2 };
            return p.x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_typeck_struct_undefined_member() {
    let source = "
        struct Point { x: i32; y: i32; }
        fn test(): i32 {
            let p = Point { x: 1, y: 2 };
            return p.z;
        }
    ";
    let result = check(source);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Struct 'Point' has no member 'z'"));
}

#[test]
fn test_typeck_return_type_mismatch() {
    let result = check("fn test(): i32 { return \"not an int\"; }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("return type String does not match expected I32"));
}

#[test]
fn test_typeck_duplicate_definition() {
    let result = check("fn test(): void { let x = 1; let x = 2; }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Redefinition of symbol 'x'"));
}

#[test]
fn test_typeck_if_condition_bool() {
    let result = check("fn test(): void { if 10 { return; } }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("'if' condition must be a boolean"));
}

#[test]
fn test_typeck_arithmetic_mismatch() {
    let result = check("fn test(): i32 { return 1 + \"2\"; }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Arithmetic operator Add requires numeric types"));
}

#[test]
fn test_typeck_index_type_mismatch() {
    let source = "
        fn test(): void {
            let x: []i32 = [];
            let y = x[\"index\"];
        }
    ";
    let result = check(source);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Array index must be an integer"));
}

#[test]
fn test_typeck_unary_mismatch() {
    let result = check("fn test(): void { let x = !10; }");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Logical NOT operator '!' requires a boolean type"));
}
