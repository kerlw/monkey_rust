use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

#[test]
fn sign_test() {
    let mut lx = Lexer::new("={}[]+;");
    let expects: Vec<Token> = vec![
        Token::new(TokenType::Assign),
        Token::new(TokenType::LBrace),
        Token::new(TokenType::RBrace),
        Token::new(TokenType::LBracket),
        Token::new(TokenType::RBracket),
        Token::new(TokenType::Plus),
        Token::new(TokenType::Semicolon),
        Token::new(TokenType::EOF),
    ];

    for tk in expects {
        assert_eq!(tk, lx.next_token());
    }
}

#[test]
fn simple_test() {
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
        Token::new(TokenType::Let),
        Token::from_str("five"),
        Token::new(TokenType::Assign),
        Token::from_int(5),
        Token::new(TokenType::Semicolon),
        // line 2
        Token::new(TokenType::Let),
        Token::from_str("ten"),
        Token::new(TokenType::Assign),
        Token::from_int(10),
        Token::new(TokenType::Semicolon),
        // line 3-6
        Token::new(TokenType::Let),
        Token::from_str("add"),
        Token::new(TokenType::Assign),
        Token::new(TokenType::Function),
        Token::new(TokenType::LParen),
        Token::from_str("x"),
        Token::new(TokenType::Comma),
        Token::from_str("y"),
        Token::new(TokenType::RParen),
        Token::new(TokenType::LBrace),
        Token::from_str("x"),
        Token::new(TokenType::Plus),
        Token::from_str("y"),
        Token::new(TokenType::Semicolon),
        Token::new(TokenType::RBrace),
        Token::new(TokenType::Semicolon),
        // line 8
        Token::new(TokenType::Let),
        Token::from_str("result"),
        Token::new(TokenType::Assign),
        Token::from_str("add"),
        Token::new(TokenType::LParen),
        Token::from_str("five"),
        Token::new(TokenType::Comma),
        Token::from_str("ten"),
        Token::new(TokenType::RParen),
        Token::new(TokenType::Semicolon),
        // line 9
        Token::new(TokenType::Bang),
        Token::new(TokenType::Minus),
        Token::new(TokenType::Slash),
        Token::new(TokenType::Asterisk),
        Token::from_int(5),
        Token::new(TokenType::Semicolon),
        // line 10
        Token::from_int(5),
        Token::new(TokenType::LT),
        Token::from_int(10),
        Token::new(TokenType::GT),
        Token::from_int(5),
        Token::new(TokenType::Semicolon),
        // line 12-16
        Token::new(TokenType::If),
        Token::new(TokenType::LParen),
        Token::from_int(5),
        Token::new(TokenType::LT),
        Token::from_int(10),
        Token::new(TokenType::RParen),
        Token::new(TokenType::LBrace),
        Token::new(TokenType::Return),
        Token::new(TokenType::True),
        Token::new(TokenType::Semicolon),
        Token::new(TokenType::RBrace),
        Token::new(TokenType::Else),
        Token::new(TokenType::LBrace),
        Token::new(TokenType::Return),
        Token::new(TokenType::False),
        Token::new(TokenType::Semicolon),
        Token::new(TokenType::RBrace),
        // line 18-19
        Token::from_int(10),
        Token::new(TokenType::Eq),
        Token::from_int(10),
        Token::new(TokenType::Semicolon),
        Token::from_int(10),
        Token::new(TokenType::NotEq),
        Token::from_int(9),
        Token::new(TokenType::Semicolon),
        // end
        Token::new(TokenType::EOF),
    ];

    let mut lx = Lexer::new(input);
    for tk in expects {
        assert_eq!(tk, lx.next_token());
    }
}
