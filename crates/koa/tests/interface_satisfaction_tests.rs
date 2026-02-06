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
    assert!(err_msg.contains("does not implement method") || err_msg.contains("print"), 
        "Expected error about missing print method, got: {}", err_msg);
    Ok(())
}
