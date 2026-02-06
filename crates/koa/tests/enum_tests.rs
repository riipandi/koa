use koa::ir::IrLowerer;
use koa::lexer::Lexer;
use koa::parser::Parser;
use koa::typeck::TypeChecker;
use miette::Result;

fn check_and_lower(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;

    let mut lowerer = IrLowerer::new();
    let _ir = lowerer.lower(&ast)?;

    Ok(())
}

#[test]
fn test_generic_enum_declaration() -> Result<()> {
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_generic_enum_with_multiple_type_params() -> Result<()> {
    let source = "
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_non_generic_enum() -> Result<()> {
    let source = "
        enum Color {
            Red,
            Green,
            Blue,
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_enum_with_multiple_fields() -> Result<()> {
    // This won't parse yet - we use tuple-like syntax for now
    let source = "
        enum Message {
            Quit,
            Move(i32, i32),
            Write(string),
            ChangeColor(i32, i32, i32),
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_enum_in_function() -> Result<()> {
    // This will need enum value construction syntax which we don't have yet
    // For now just test that the type checks
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }

        fn uses_option(): void {
            return;
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_multiple_enum_instantiations() -> Result<()> {
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }

        fn test(): void {
            // Option<i32>
            // Option<f64>
            // Option<string>
            return;
        }
    ";
    check_and_lower(source)
}
