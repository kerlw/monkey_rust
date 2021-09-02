use crate::lexer::token::Token;

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum Statement {
    LetStatement(Ident, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Statement::LetStatement(ident, expression) => {
                format!("let {} = {};", ident.0, expression.to_string())
            }
            Statement::ExpressionStatement(expr) => {
                expr.to_string()
                // format!("{};", expr.to_string())
            }
            _ => "".to_string(),
        }
    }
}

// TODO 这个结构可能是多余的定义，也许可以直接使用Vec<Statement>代替，后面再确认
#[derive(PartialEq, Debug, Clone, Eq)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(Ident),
    IfExpression(Box<Expression>, Vec<Statement>, Vec<Statement>),
    CallExpression(Box<Expression>, Vec<Expression>),
    FunctionExpression(Vec<Ident>, Vec<Statement>),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    PrefixExpression(Token, Box<Expression>),
    InfixExpression(Box<Expression>, Token, Box<Expression>), // left, operator, right
}

impl Eq for Expression {}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.0.clone(),
            Expression::IntLiteral(v) => v.to_string(),
            Expression::BoolLiteral(v) => v.to_string(),
            Expression::PrefixExpression(prefix, right) => {
                format!("({}{})", prefix.to_string(), right.to_string())
            }
            Expression::InfixExpression(left, operator, right) => {
                format!(
                    "({} {} {})",
                    left.to_string(),
                    operator.to_string(),
                    right.to_string()
                )
            }
            Expression::CallExpression(function, params) => {
                let params_str = params
                    .iter()
                    .map(|expr| expr.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", function.to_string(), params_str)
            }
            Expression::StringLiteral(v) => v.clone(),
            _ => "".to_string(),
        }
    }
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

impl Program {
    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for st in &self.statements {
            ret.push_str(&st.to_string());
        }
        return ret;
    }
}

impl Precedence {
    pub fn from_token(token: &Token) -> Self {
        match token {
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::LT | Token::GT => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
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
            1 => Precedence::Equals,
            2 => Precedence::LessGreater,
            3 => Precedence::Sum,
            4 => Precedence::Product,
            5 => Precedence::Prefix,
            6 => Precedence::Call,
            _ => Precedence::Lowest,
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
