use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType, EOF_TOKEN};
use crate::parser::program::{Expression, Identifier, LetStatement, Program, Statement};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

mod program;
#[cfg(test)]
mod test;

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

pub struct ParseError {
    info: String,
}

impl From<&str> for ParseError {
    fn from(s: &str) -> Self {
        ParseError { info: s.to_owned() }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut ret = Parser {
            l,
            cur_token: EOF_TOKEN,
            peek_token: EOF_TOKEN,
        };
        ret.next_token();
        ret.next_token();

        ret
    }

    pub fn next_token(&mut self) {
        std::mem::swap(&mut self.cur_token, &mut self.peek_token);
        self.peek_token = self.l.next_token();
    }

    pub fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token.typ == token {
            self.next_token();
            true
        } else {
            eprintln!(
                "{}:{} parser error: expect next token to be {:?}, got {:?} instead",
                file!(),
                line!(),
                token,
                self.peek_token
            );
            false
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut ret = Program::default();
        loop {
            if self.cur_token.is_eof() {
                break;
            }

            let statement = self.parse_statement()?;
            ret.statements.push(statement);
        }
        Ok(ret)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>> {
        match self.cur_token.typ {
            TokenType::Let => self.parse_let_statement(),
            _ => Err("no equal sign!".into()),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>> {
        let token = self.cur_token.clone();
        let identifier = self.parse_identifier()?;

        if !self.expect_peek(TokenType::Assign) {
            return Err("no equal sign!".into());
        }

        let value = self.parse_expression()?;

        let st = LetStatement::new(token, identifier, value);
        Ok(Box::new(st))
    }

    fn parse_identifier(&mut self) -> Result<Identifier> {
        if let TokenType::Ident(v) = self.peek_token.typ.borrow() {
            self.next_token();
            let ident = Identifier {
                token: self.cur_token.clone(),
            };
            Ok(ident)
        } else {
            return Err("not a ident token".into());
        }
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>> {
        match self.cur_token.typ {
            TokenType::Int(v) => {
                if self.peek_token.is_operator() {
                    return self.parse_operator_expression();
                } else if self.expect_peek(TokenType::Semicolon) {

                }
                Err("".into())
            }
            _ => Err("".into()),
        }
    }

    fn parse_operator_expression(&mut self) -> Result<Box<dyn Expression>> {
        Err("".into())
    }
}
