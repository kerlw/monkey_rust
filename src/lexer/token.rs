#[warn(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // keywords
    Let,      // let
    Function, // fn
    True,     // true
    False,    // false
    If,       // if
    Else,     //else
    Return,   //return

    // sign
    Assign,    // =
    Plus,      // +
    Minus,     // -
    Bang,      // !
    Asterisk,  // *
    Slash,     // /
    LT,        // <
    GT,        // >
    Comma,     // ,
    Semicolon, // ;
    Eq,        // ==
    NotEq,     // !=

    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    LBrace,   // {
    RBrace,   // }

    Ident(String),
    Int(i64),
    Float(f64),
}

pub const EOF_TOKEN: Token = Token::EOF;

impl Token {
    pub fn from_str<T: Into<String>>(input: T) -> Token {
        let input = input.into();
        match input.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(input),
        }
    }

    pub fn from_int(input: i64) -> Token {
        Token::Int(input)
    }

    pub fn from_float(input: f64) -> Token {
        Token::Float(input)
    }

    pub fn is_eof(&self) -> bool {
        *self == Token::EOF
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => true,
            _ => false,
        }
    }
}
