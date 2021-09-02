use crate::lexer::token::Token;

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
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b',' => Token::Comma,
            b';' => Token::Semicolon,

            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,

            b'"' => self.read_string(),
            0 => Token::EOF,
            _ => {
                let ch = self.ch as char;

                // read_identifier和read_number_token中都进行了read_char，所以直接returns
                if ch.is_ascii_alphabetic() {
                    return Token::from_str(self.read_identifier());
                } else if ch.is_numeric() {
                    return self.read_number_token();
                } else {
                    Token::Illegal
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

    fn read_string(&mut self) -> Token {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'\\' && self.peek_char() == b'"' {
                self.read_char();
            } else if self.ch == b'"' {
                break;
            }
        }
        return Token::String(
            String::from_utf8(self.input.as_bytes()[pos..self.position].to_vec()).unwrap(),
        );
    }
}
