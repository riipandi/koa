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

    Ok(format!("{:#?}", ir))
}

#[test]
fn test_debug_non_generic_enum_ir() -> Result<()> {
    let source = "
        enum Color {
            Red,
            Green,
            Blue,
        }
    ";
    let ir = lower_to_ir(source)?;
    println!("{}", ir);
    Ok(())
}

#[test]
fn test_debug_enum_with_data_ir() -> Result<()> {
    let source = "
        enum Option {
            Some(i32),
            None,
        }
    ";
    let ir = lower_to_ir(source)?;
    println!("{}", ir);
    Ok(())
}

#[test]
fn test_debug_generic_enum_ir() -> Result<()> {
    let source = "
        enum Option<T> {
            Some(T),
            None,
        }
    ";
    let ir = lower_to_ir(source)?;
    println!("{}", ir);
    Ok(())
}

#[test]
fn test_debug_generic_result_enum_ir() -> Result<()> {
    let source = "
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    ";
    let ir = lower_to_ir(source)?;
    println!("{}", ir);
    Ok(())
}
