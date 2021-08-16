use crate::lexer::token::Token;
use std::any::Any;

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

pub enum Statement {
    LetStatement(Ident, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

pub enum Expression {
    Identifier(Ident),
    OperatorExpression(Box<Expression>, Token, Box<Expression>),
}

#[derive(Default)]
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}
