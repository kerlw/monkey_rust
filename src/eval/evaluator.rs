use crate::eval::ObjectWrapper;
use crate::lexer::token::Token;
use crate::parser::program::{Expression, Program, Statement};
use crate::parser::Result;

pub fn eval(program: &Program) -> Result<ObjectWrapper> {
    eval_statements(&program.statements)
}

fn eval_statements(statements: &Vec<Statement>) -> Result<ObjectWrapper> {
    let mut ret = ObjectWrapper::Null;
    for st in statements {
        ret = eval_statement(st)?;
    }
    Ok(ret)
}

fn eval_statement(statement: &Statement) -> Result<ObjectWrapper> {
    match statement {
        Statement::ExpressionStatement(expr) => eval_expression(expr),
        _ => Ok(ObjectWrapper::Null),
    }
}

fn eval_expression(expression: &Expression) -> Result<ObjectWrapper> {
    match expression {
        Expression::IntLiteral(v) => Ok(ObjectWrapper::Integer(v.clone())),
        Expression::BoolLiteral(v) => Ok(ObjectWrapper::Boolean(v.clone())),
        Expression::InfixExpression(left, operator, right) => {
            eval_infix_expression(left, operator, right)
        }
        _ => Ok(ObjectWrapper::Null),
    }
}

fn eval_infix_expression(
    left: &Expression,
    operator: &Token,
    right: &Expression,
) -> Result<ObjectWrapper> {
    let left = eval_expression(left)?;
    let right = eval_expression(right)?;

    match operator {
        Token::Plus => left.add(&right),
        _ => Ok(ObjectWrapper::Null),
    }
}
