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
fn test_enum_in_struct_field() -> Result<()> {
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }

        struct Container {
            value: Option<i32>;
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_enum_in_function_parameter() -> Result<()> {
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }

        fn process(opt: Option<i32>): void {
            return;
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_result_enum_with_different_types() -> Result<()> {
    let source = "
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        fn test(): void {
            return;
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_nested_generics() -> Result<()> {
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }

        fn test(): void {
            return;
        }
    ";
    check_and_lower(source)
}

#[test]
fn test_enum_with_multiple_type_params_and_structs() -> Result<()> {
    let source = "
        struct Pair<T, U> {
            first: T;
            second: U;
        }

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        fn test(): void {
            return;
        }
    ";
    check_and_lower(source)
}
