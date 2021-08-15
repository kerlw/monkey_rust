use crate::lexer::token::{Token, TokenType};
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String {
        "".to_string()
    }
}

pub trait Statement {
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {}

pub struct Identifier {
    pub token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return if let TokenType::Ident(v) = &self.token.typ {
            v.clone()
        } else {
            "".to_string()
        };
    }
}

impl Expression for Identifier {}

pub struct LetStatement {
    pub(crate) token: Token,
    pub(crate) name: Identifier,
    pub(crate) value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> Self {
        LetStatement { token, name, value }
    }
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct OperatorExpression {
    // left:
}

impl Node for OperatorExpression {}
impl Expression for OperatorExpression {}

#[derive(Default)]
pub struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>,
}
