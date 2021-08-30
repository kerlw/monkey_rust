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
        if let ObjectWrapper::ReturnValue(v) = ret {
            return Ok(*v);
        }
    }
    Ok(ret)
}

fn eval_block_statements(statements: &Vec<Statement>) -> Result<ObjectWrapper> {
    let mut ret = ObjectWrapper::Null;
    for st in statements {
        ret = eval_statement(st)?;
        if let ObjectWrapper::ReturnValue(_) = ret {
            break;
        }
    }
    Ok(ret)
}

fn eval_statement(statement: &Statement) -> Result<ObjectWrapper> {
    match statement {
        Statement::ReturnStatement(expr) => eval_return_statement(expr),
        Statement::ExpressionStatement(expr) => eval_expression(expr),
        _ => Ok(ObjectWrapper::Null),
    }
}

fn eval_return_statement(expression: &Expression) -> Result<ObjectWrapper> {
    let ret = eval_expression(expression)?;
    Ok(ObjectWrapper::ReturnValue(Box::new(ret)))
}

fn eval_expression(expression: &Expression) -> Result<ObjectWrapper> {
    match expression {
        Expression::IntLiteral(v) => Ok(ObjectWrapper::Integer(v.clone())),
        Expression::BoolLiteral(v) => Ok(ObjectWrapper::Boolean(v.clone())),
        Expression::InfixExpression(left, operator, right) => {
            eval_infix_expression(left, operator, right)
        }
        Expression::PrefixExpression(operator, right) => eval_prefix_expression(operator, right),
        Expression::IfExpression(condition, consequence, alternative) => {
            eval_if_expression(condition, consequence, alternative)
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
        Token::Eq => left.eq(&right),
        Token::GT => left.great_than(&right),
        Token::LT => left.less_than(&right),
        Token::NotEq => left.not_eq(&right),
        Token::Asterisk => left.multi(&right),
        _ => Ok(ObjectWrapper::Null),
    }
}

fn eval_prefix_expression(operator: &Token, expr: &Expression) -> Result<ObjectWrapper> {
    match operator {
        Token::Bang => match eval_expression(expr)? {
            ObjectWrapper::Boolean(v) => Ok(ObjectWrapper::Boolean(!v)),
            ObjectWrapper::Integer(v) => Ok(ObjectWrapper::Boolean(v == 0)),
            _ => Ok(ObjectWrapper::Boolean(false)),
        },
        Token::Minus => match eval_expression(expr)? {
            ObjectWrapper::Integer(v) => Ok(ObjectWrapper::Integer(-v)),
            ObjectWrapper::Float(f) => Ok(ObjectWrapper::Float(-f)),
            _ => Err(format!("cannot eval {} after '-'.", operator.to_string()).into()),
        },
        _ => Ok(ObjectWrapper::Null),
    }
}

fn eval_if_expression(
    condition: &Expression,
    consequence: &Vec<Statement>,
    alternative: &Vec<Statement>,
) -> Result<ObjectWrapper> {
    let cond = eval_expression(condition)?;
    if let ObjectWrapper::Boolean(v) = cond {
        if v {
            eval_block_statements(consequence)
        } else {
            eval_block_statements(alternative)
        }
    } else {
        return Err("Invalid 'if' condition.".into());
    }
}
