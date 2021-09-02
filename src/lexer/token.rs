#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // keywords
    Let,      // let
    Function, // fn
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
    Bool(bool), // true/false
    Int(i64),
    Float(f64),
    String(String),
}

impl Eq for Token {}

pub const EOF_TOKEN: Token = Token::EOF;

impl Token {
    pub fn from_str<T: Into<String>>(input: T) -> Token {
        let input = input.into();
        match input.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
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

    // pub fn is_operator(&self) -> bool {
    //     match self {
    //         Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => true,
    //         _ => false,
    //     }
    // }

    pub fn to_string(&self) -> String {
        match self {
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::LT => "<".to_string(),
            Token::GT => ">".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
            _ => "".to_string(),
        }
    }
}
