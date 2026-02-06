use koa::lexer::{Lexer, TokenKind};

#[test]
fn test_tokenize_simple() {
    let input = "fn main() { return 0; }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    
    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    assert_eq!(kinds, vec![
        TokenKind::Fn,
        TokenKind::Ident,
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LBrace,
        TokenKind::Return,
        TokenKind::IntLiteral,
        TokenKind::Semicolon,
        TokenKind::RBrace,
        TokenKind::EOF,
    ]);
}

#[test]
fn test_types() {
    let input = "let x: i32 = 42; let y: string = \"hello\";";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    
    assert!(tokens.iter().any(|t| t.kind == TokenKind::I32));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::String));
}

#[test]
fn test_doc_comments() {
    let input = "/// Hello\nfn main() {}";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    
    assert_eq!(tokens[0].kind, TokenKind::DocComment);
    assert_eq!(tokens[1].kind, TokenKind::Fn);
}

#[test]
fn test_range_operator() {
    let input = "1..10 1..=10";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    
    assert_eq!(tokens[0].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[1].kind, TokenKind::DotDot);
    assert_eq!(tokens[2].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[3].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[4].kind, TokenKind::DotDotEqual);
    assert_eq!(tokens[5].kind, TokenKind::IntLiteral);
}
