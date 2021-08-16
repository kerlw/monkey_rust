use crate::lexer::token::Token;

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

pub enum Statement {
    LetStatement(Ident, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

pub enum Expression {
    Identifier(Ident),
    IfExpression,
    CallExpression,
    FunctionExpression,
    IntLiteral(i64),
    PrefixExpression(Token, Box<Expression>),
    InfixExpression(Box<Expression>, Token, Box<Expression>),
}

#[derive(PartialEq, PartialOrd, Debug, Eq, Clone)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -x or !x
    Call,        // my_func(x)
}

#[derive(Default)]
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Precedence {
    pub fn from_token(token: &Token) -> Self {
        match token {
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::LT | Token::GT => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}
