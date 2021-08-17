use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, EOF_TOKEN};
use crate::parser::program::{Expression, Ident, Precedence, Program, Statement};
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

    pub fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token == token {
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
            // println!("[parse loop] current token is {:?}", self.cur_token);
            if self.cur_token.is_eof() {
                break;
            }

            let statement = self.parse_statement()?;
            ret.statements.push(statement);

            self.next_token();
        }
        Ok(ret)
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        if let Token::Ident(_) = &self.peek_token {
            self.next_token();
        }
        let identifier = self.parse_identifier()?;

        if !self.expect_peek(Token::Assign) {
            return Err("no equal sign!".into());
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;
        Ok(Statement::LetStatement(identifier, value))
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.next_token();

        loop {
            if self.cur_token == Token::Semicolon {
                break;
            }
            self.next_token();
        }
        todo!()
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let ret = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement(ret))
    }

    fn parse_identifier(&mut self) -> Result<Ident> {
        match &self.cur_token {
            Token::Ident(v) => Ok(Ident(v.clone())),
            _ => Err("not a ident token".into()),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
        let mut left = match &self.cur_token {
            Token::Ident(_) => {
                let ident = self.parse_identifier()?;
                Ok(Expression::Identifier(ident))
            }
            Token::Int(_) => self.parse_int_literal(),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            _ => Err(format!("no prefix parse function for {:?}", self.cur_token)
                .as_str()
                .into()),
        }?;

        loop {
            if self.expect_peek(Token::Semicolon)
                || precedence >= Precedence::from_token(&self.peek_token)
            {
                break;
            }

            self.next_token();
            let is_infix = match self.cur_token {
                Token::Eq
                | Token::NotEq
                | Token::LT
                | Token::GT
                | Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk => true,
                _ => false,
            };

            if is_infix {
                left = self.parse_infix_expression(left)?;
            } else {
                return Ok(left);
            }
        }

        Ok(left)
    }

    fn parse_int_literal(&self) -> Result<Expression> {
        if let Token::Int(v) = self.cur_token {
            Ok(Expression::IntLiteral(v))
        } else {
            Err("Token::Int not found".into())
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression> {
        let token = self.cur_token.clone();
        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::PrefixExpression(token, Box::new(right)))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression> {
        let precedence = Precedence::from_token(&self.cur_token);
        let token = self.cur_token.clone();
        self.next_token();

        let right = match &token {
            Token::Plus => self.parse_expression(precedence.sub(1))?,
            _ => self.parse_expression(precedence)?,
        };
        Ok(Expression::InfixExpression(
            Box::new(left),
            token,
            Box::new(right),
        ))
    }
}
