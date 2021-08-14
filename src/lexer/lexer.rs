use crate::lexer::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new<T: Into<String>>(input: T) -> Lexer {
        let mut ret = Lexer {
            input: input.into(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        ret.read_char();
        ret
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while (self.ch as char).is_ascii_alphabetic() {
            self.read_char();
        }
        String::from_utf8(self.input.as_bytes()[pos..self.position].to_vec()).unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ret = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Eq)
                } else {
                    Token::new(TokenType::Assign)
                }
            }
            b'+' => Token::new(TokenType::Plus),
            b'-' => Token::new(TokenType::Minus),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NotEq)
                } else {
                    Token::new(TokenType::Bang)
                }
            }
            b'*' => Token::new(TokenType::Asterisk),
            b'/' => Token::new(TokenType::Slash),
            b'<' => Token::new(TokenType::LT),
            b'>' => Token::new(TokenType::GT),
            b',' => Token::new(TokenType::Comma),
            b';' => Token::new(TokenType::Semicolon),

            b'(' => Token::new(TokenType::LParen),
            b')' => Token::new(TokenType::RParen),
            b'[' => Token::new(TokenType::LBracket),
            b']' => Token::new(TokenType::RBracket),
            b'{' => Token::new(TokenType::LBrace),
            b'}' => Token::new(TokenType::RBrace),
            0 => Token::new(TokenType::EOF),
            _ => {
                let ch = self.ch as char;

                // read_identifier和read_number_token中都进行了read_char，所以直接returns
                if ch.is_ascii_alphabetic() {
                    return Token::from_str(self.read_identifier());
                } else if ch.is_numeric() {
                    return self.read_number_token();
                } else {
                    Token::new(TokenType::Illegal)
                }
            }
        };
        self.read_char();
        return ret;
    }

    fn skip_whitespace(&mut self) {
        while (self.ch as char).is_whitespace() {
            self.read_char();
        }
    }

    fn read_number_token(&mut self) -> Token {
        let pos = self.position;
        let mut is_float = false;
        loop {
            let ch = self.ch as char;

            if ch.is_numeric() || (!is_float && ch == '.') {
                self.read_char();
                if ch == '.' {
                    is_float = true;
                }
            } else {
                break;
            }
        }

        let sub_str =
            String::from_utf8(self.input.as_bytes()[pos..self.position].to_vec()).unwrap();
        if is_float {
            Token::from_float(sub_str.parse::<f64>().unwrap())
        } else {
            Token::from_int(sub_str.parse::<i64>().unwrap())
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.as_bytes().len() {
            return 0;
        }
        return self.input.as_bytes()[self.read_position];
    }
}
