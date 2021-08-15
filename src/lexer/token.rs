#[warn(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) typ: TokenType,
}

pub const EOF_TOKEN: Token = Token {
    typ: TokenType::EOF,
};

impl Token {
    pub fn new(typ: TokenType) -> Token {
        Token { typ }
    }

    pub fn from_str<T: Into<String>>(input: T) -> Token {
        let input = input.into();
        match input.as_str() {
            "fn" => Token::new(TokenType::Function),
            "let" => Token::new(TokenType::Let),
            "true" => Token::new(TokenType::True),
            "false" => Token::new(TokenType::False),
            "if" => Token::new(TokenType::If),
            "else" => Token::new(TokenType::Else),
            "return" => Token::new(TokenType::Return),
            _ => Token {
                typ: TokenType::Ident(input),
            },
        }
    }

    pub fn from_int(input: i64) -> Token {
        Token {
            typ: TokenType::Int(input),
        }
    }

    pub fn from_float(input: f64) -> Token {
        Token {
            typ: TokenType::Float(input),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.typ == TokenType::EOF
    }

    pub fn is_operator(&self) -> bool {
        match self.typ {
            TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => true,
            _ => false,
        }
    }
}
