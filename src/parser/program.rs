use crate::lexer::token::Token;

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum Statement {
    LetStatement(Ident, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(Ident),
    IfExpression,
    CallExpression,
    FunctionExpression,
    IntLiteral(i64),
    PrefixExpression(Token, Box<Expression>),
    InfixExpression(Box<Expression>, Token, Box<Expression>),
}

impl Eq for Expression {}

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

    #[inline]
    pub fn to_i32(&self) -> i32 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Prefix => 5,
            Precedence::Call => 6,
        }
    }

    #[inline]
    pub fn from_i32(v: i32) -> Self {
        match v {
            _ => Precedence::Lowest,
            1 => Precedence::Equals,
            2 => Precedence::LessGreater,
            3 => Precedence::Sum,
            4 => Precedence::Product,
            5 => Precedence::Prefix,
            6 => Precedence::Call,
        }
    }

    #[inline]
    pub fn sub(&self, v: i32) -> Self {
        let val = self.to_i32() - v;
        if val >= Precedence::Call.to_i32() {
            Precedence::Call
        } else {
            Precedence::from_i32(val)
        }
    }

    #[inline]
    pub fn add(&self, v: i32) -> Self {
        self.sub(-v)
    }
}
