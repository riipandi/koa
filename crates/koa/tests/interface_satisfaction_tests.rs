use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::typeck::TypeChecker;
use miette::Result;

fn check_type(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    Ok(())
}

#[test]
fn test_interface_satisfaction_success() -> Result<()> {
    let source = "
        interface Printable {
            fn print(self): void;
        }

        struct Book {
            title: string;
            fn print(self): void { return; }
        }

        fn show<T: Printable>(x: T): void {
            return;
        }

        fn main(): void {
            return;
        }
    ";
    check_type(source)
}

#[test]
fn test_interface_satisfaction_failure() -> Result<()> {
    let source = "
        interface Printable {
            fn print(self): void;
        }

        struct Secret {
            code: i32;
        }

        fn show<T: Printable>(x: T): void {
            return;
        }

        fn main(): void {
            let s: Secret = Secret { code: 1234 };
            show<Secret>(s);
            return;
        }
    ";
    let result = check_type(source);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("does not implement method") || err_msg.contains("print"),
        "Expected error about missing print method, got: {}",
        err_msg
    );
    Ok(())
}

#[test]
fn test_interface_parameter_type_mismatch() -> Result<()> {
    let source = "
        interface Processor {
            fn process(self, value: i32): void;
        }

        struct DataHandler {
            fn process(self, value: f64): void { return; }
        }

        fn handle<T: Processor>(p: T): void { return; }

        fn main(): void {
            let h = DataHandler { };
            handle<DataHandler>(h);
            return;
        }
    ";
    let result = check_type(source);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("parameter") || err_msg.contains("expects"),
        "Expected error about parameter type mismatch, got: {}",
        err_msg
    );
    Ok(())
}

#[test]
fn test_interface_return_type_mismatch() -> Result<()> {
    let source = "
        interface Calculator {
            fn compute(self): i32;
        }

        struct BadCalculator {
            fn compute(self): f64 { return 0.0; }
        }

        fn calculate<T: Calculator>(c: T): void { return; }

        fn main(): void {
            let c = BadCalculator { };
            calculate<BadCalculator>(c);
            return;
        }
    ";
    let result = check_type(source);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("returns") || err_msg.contains("expects"),
        "Expected error about return type mismatch, got: {}",
        err_msg
    );
    Ok(())
}

#[test]
fn test_interface_with_multiple_parameters() -> Result<()> {
    let source = "
        interface Transformer {
            fn transform(self, input: i32, scale: f64): i32;
        }

        struct MyTransformer {
            fn transform(self, input: i32, scale: f64): i32 { return 0; }
        }

        fn process<T: Transformer>(t: T, x: i32, y: f64): void {
            return;
        }

        fn main(): void {
            let t = MyTransformer { };
            process<MyTransformer>(t, 42, 3.14);
            return;
        }
    ";
    check_type(source)
}

#[test]
fn test_interface_parameter_count_mismatch() -> Result<()> {
    let source = "
        interface Writer {
            fn write(self, data: i32, count: i32): void;
        }

        struct BadWriter {
            fn write(self, data: i32): void { return; }
        }

        fn write_data<T: Writer>(w: T): void { return; }

        fn main(): void {
            let w = BadWriter { };
            write_data<BadWriter>(w);
            return;
        }
    ";
    let result = check_type(source);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("parameters") || err_msg.contains("expects"),
        "Expected error about parameter count mismatch, got: {}",
        err_msg
    );
    Ok(())
}

#[test]
fn test_generic_interface_satisfaction() -> Result<()> {
    let source = "
        interface Comparable<T> {
            fn compare(self, other: T): i32;
        }

        struct Number {
            value: i32;
            fn compare(self, other: Number): i32 { return 0; }
        }

        fn max<T: Comparable<T>>(a: T, b: T): T {
            return a;
        }

        fn main(): void {
            let n1 = Number { value: 10 };
            let n2 = Number { value: 20 };
            return;
        }
    ";
    check_type(source)
}
