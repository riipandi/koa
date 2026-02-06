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
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Undefined identifier: x")
    );
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
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("expects 2 arguments, but 1 were provided")
    );
}

#[test]
fn test_typeck_function_call_arg_type() {
    let result =
        check("fn add(a: i32, b: i32): i32 { return a + b; } fn test(): void { add(1, \"2\"); }");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Argument 1 to function 'add' has type String, but expected I32")
    );
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
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Struct 'Point' has no member or method 'z'")
    );
}

#[test]
fn test_typeck_return_type_mismatch() {
    let result = check("fn test(): i32 { return \"not an int\"; }");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("return type String does not match expected I32")
    );
}

#[test]
fn test_typeck_duplicate_definition() {
    let result = check("fn test(): void { let x = 1; let x = 2; }");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Redefinition of symbol 'x'")
    );
}

#[test]
fn test_typeck_if_condition_bool() {
    let result = check("fn test(): void { if 10 { return; } }");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("'if' condition must be a boolean")
    );
}

#[test]
fn test_typeck_arithmetic_mismatch() {
    let result = check("fn test(): i32 { return 1 + \"2\"; }");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Arithmetic operator Add requires numeric types")
    );
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
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Array index must be an integer")
    );
}

#[test]
fn test_typeck_unary_mismatch() {
    let result = check("fn test(): void { let x = !10; }");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Logical NOT operator '!' requires a boolean type")
    );
}

#[test]
fn test_typeck_method_call() {
    let source = "
        struct Point { x: i32; y: i32; }
        fn get_x(self: Point): i32 { return self.x; }
        fn test(): i32 {
            let p = Point { x: 10, y: 20 };
            return p.get_x();
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_typeck_method_call_args() {
    let source = "
        struct Point { x: i32; y: i32; }
        fn scale(self: Point, factor: i32): i32 { return self.x * factor; }
        fn test(): i32 {
            let p = Point { x: 10, y: 20 };
            return p.scale(\"wrong\");
        }
    ";
    let result = check(source);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Argument 0 to method 'scale' has type String, but expected I32")
    );
}

#[test]
fn test_typeck_struct_literal_fields() {
    let source = "
        struct Point { x: i32; y: i32; }
        fn test(): void {
            let p = Point { x: 10 }; // Missing y
        }
    ";
    let result = check(source);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Missing field 'y' in initializer for struct 'Point'")
    );
}

#[test]
fn test_type_inference_int_literal() {
    let source = "
        fn test(): i32 {
            let x = 42;
            return x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_float_literal() {
    let source = "
        fn test(): f64 {
            let x = 3.14;
            return x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_string_literal() {
    let source = "
        fn test(): string {
            let x = \"hello\";
            return x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_bool_literal() {
    let source = "
        fn test(): bool {
            let x = true;
            return x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_from_variable() {
    let source = "
        fn test(): i32 {
            let x = 42;
            let y = x;
            return y;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_from_function_call() {
    let source = "
        fn get_value(): i32 { return 100; }
        fn test(): i32 {
            let x = get_value();
            return x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_from_struct_literal() {
    let source = "
        struct Point { x: i32; y: i32; }
        fn test(): Point {
            let p = Point { x: 1, y: 2 };
            return p;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_generic_struct() {
    let source = "
        struct Box<T> { value: T; }
        fn test(): Box<i32> {
            let b = Box<i32> { value: 42 };
            return b;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_arithmetic() {
    let source = "
        fn test(): i32 {
            let x = 10;
            let y = 20;
            let z = x + y;
            return z;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_complex_expression() {
    let source = "
        fn test(): i32 {
            let x = 10;
            let y = x * 2 + 5;
            return y;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_with_reassignment() {
    let source = "
        fn test(): i32 {
            let x = 10;
            let y = x;
            let z = y + 5;
            return z;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_array_literal() {
    let source = "
        fn test(): []i32 {
            let arr = [1, 2, 3];
            return arr;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_null() {
    let source = "
        fn test(): ?i32 {
            let x: ?i32 = null;
            return x;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_void_value() {
    let source = "
        fn dummy(): void { return; }
        fn test(): void {
            let x = dummy();
            return;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}

#[test]
fn test_type_inference_mixed_types() {
    let source = "
        fn test(): i32 {
            let a = 42;
            let b = 3.14;
            let c = \"hello\";
            let d = true;
            return a;
        }
    ";
    let result = check(source);
    assert!(result.is_ok());
}
