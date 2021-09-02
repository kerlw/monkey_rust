use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;

#[test]
fn sign_test() {
    let mut lx = Lexer::new("={}[]+;");
    let expects: Vec<Token> = vec![
        Token::Assign,
        Token::LBrace,
        Token::RBrace,
        Token::LBracket,
        Token::RBracket,
        Token::Plus,
        Token::Semicolon,
        Token::EOF,
    ];

    for tk in expects {
        assert_eq!(tk, lx.next_token());
    }
}

#[test]
fn simple_token_test() {
    let input = r"let five = 5;
    let ten  = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;
    ";

    let expects: Vec<Token> = vec![
        // line 1
        Token::Let,
        Token::from_str("five"),
        Token::Assign,
        Token::from_int(5),
        Token::Semicolon,
        // line 2
        Token::Let,
        Token::from_str("ten"),
        Token::Assign,
        Token::from_int(10),
        Token::Semicolon,
        // line 3-6
        Token::Let,
        Token::from_str("add"),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::from_str("x"),
        Token::Comma,
        Token::from_str("y"),
        Token::RParen,
        Token::LBrace,
        Token::from_str("x"),
        Token::Plus,
        Token::from_str("y"),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        // line 8
        Token::Let,
        Token::from_str("result"),
        Token::Assign,
        Token::from_str("add"),
        Token::LParen,
        Token::from_str("five"),
        Token::Comma,
        Token::from_str("ten"),
        Token::RParen,
        Token::Semicolon,
        // line 9
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::from_int(5),
        Token::Semicolon,
        // line 10
        Token::from_int(5),
        Token::LT,
        Token::from_int(10),
        Token::GT,
        Token::from_int(5),
        Token::Semicolon,
        // line 12-16
        Token::If,
        Token::LParen,
        Token::from_int(5),
        Token::LT,
        Token::from_int(10),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::Bool(true),
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::Bool(false),
        Token::Semicolon,
        Token::RBrace,
        // line 18-19
        Token::from_int(10),
        Token::Eq,
        Token::from_int(10),
        Token::Semicolon,
        Token::from_int(10),
        Token::NotEq,
        Token::from_int(9),
        Token::Semicolon,
        // end
        Token::EOF,
    ];

    let mut lx = Lexer::new(input);
    for tk in expects {
        assert_eq!(tk, lx.next_token());
    }
}

#[test]
fn test_string_token() {
    let cases = [
        ("\"hello\"", Token::String("hello".to_string())),
        ("\"hello \\\"world\\\"\"", Token::String("hello \\\"world\\\"".to_string())),
    ];

    for (input, expect) in cases {
        let mut lx = Lexer::new(input);
        assert_eq!(lx.next_token(), expect)
    }
}