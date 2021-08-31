use crate::eval::environment::Environment;
use crate::eval::ObjectWrapper;
use crate::lexer::token::Token;
use crate::parser::program::{Expression, Program, Statement};
use crate::parser::Result;
use env_logger::Env;

pub struct Evaluator<'prog, 'env>
where
    'prog: 'env,
{
    program: &'prog Program,
    env: Environment<'env>,
}

impl<'prog, 'env> Evaluator<'prog, 'env>
where
    'prog: 'env,
{
    pub fn new(program: &'prog Program) -> Self {
        Evaluator {
            program,
            env: Environment::default(),
        }
    }

    pub fn eval(&mut self) -> Result<ObjectWrapper> {
        match self.eval_statements(&self.program.statements) {
            Err(e) => Ok(ObjectWrapper::ErrorObject(e.to_string())),
            Ok(ret) => Ok(ret),
        }
    }

    fn eval_statements(&mut self, statements: &'prog Vec<Statement>) -> Result<ObjectWrapper> {
        let mut ret = ObjectWrapper::Null;
        for st in statements {
            ret = self.eval_statement(st)?;
            if let ObjectWrapper::ReturnValue(v) = ret {
                return Ok(*v);
            }
        }
        Ok(ret)
    }

    fn eval_block_statements(&mut self, statements: &'prog Vec<Statement>) -> Result<ObjectWrapper> {
        let mut ret = ObjectWrapper::Null;
        for st in statements {
            ret = self.eval_statement(st)?;
            if let ObjectWrapper::ReturnValue(_) = ret {
                break;
            }
        }
        Ok(ret)
    }

    fn eval_statement(&mut self, statement: &'prog Statement) -> Result<ObjectWrapper> {
        match statement {
            Statement::ReturnStatement(expr) => self.eval_return_statement(expr),
            Statement::ExpressionStatement(expr) => self.eval_expression(expr),
            Statement::LetStatement(ident, expression) => {
                let value = self.eval_expression(expression)?;
                self.env.set(&ident.0, value);
                Ok(ObjectWrapper::Null)
            }
            _ => Ok(ObjectWrapper::Null),
        }
    }

    fn eval_return_statement(&mut self, expression: &'prog Expression) -> Result<ObjectWrapper> {
        let ret = self.eval_expression(expression)?;
        Ok(ObjectWrapper::ReturnValue(Box::new(ret)))
    }

    fn eval_expression(&mut self, expression: &'prog Expression) -> Result<ObjectWrapper> {
        match expression {
            Expression::Identifier(ident) => {
                if self.env.contains(&ident.0) {
                    Ok(self.env.get(&ident.0).unwrap().clone())
                } else {
                    Err(format!("identifier not found: {}", &ident.0).into())
                }
            },
            Expression::IntLiteral(v) => Ok(ObjectWrapper::Integer(v.clone())),
            Expression::BoolLiteral(v) => Ok(ObjectWrapper::Boolean(v.clone())),
            Expression::InfixExpression(left, operator, right) => {
                self.eval_infix_expression(left, operator, right)
            }
            Expression::PrefixExpression(operator, right) => {
                self.eval_prefix_expression(operator, right)
            }
            Expression::IfExpression(condition, consequence, alternative) => {
                self.eval_if_expression(condition, consequence, alternative)
            }
            _ => Ok(ObjectWrapper::Null),
        }
    }

    fn eval_infix_expression(
        &mut self,
        left: &'prog Expression,
        operator: &'prog Token,
        right: &'prog Expression,
    ) -> Result<ObjectWrapper> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

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

    fn eval_prefix_expression(
        &mut self,
        operator: &'prog Token,
        expr: &'prog Expression,
    ) -> Result<ObjectWrapper> {
        match operator {
            Token::Bang => match self.eval_expression(expr)? {
                ObjectWrapper::Boolean(v) => Ok(ObjectWrapper::Boolean(!v)),
                ObjectWrapper::Integer(v) => Ok(ObjectWrapper::Boolean(v == 0)),
                _ => Ok(ObjectWrapper::Boolean(false)),
            },
            Token::Minus => match self.eval_expression(expr)? {
                ObjectWrapper::Integer(v) => Ok(ObjectWrapper::Integer(-v)),
                ObjectWrapper::Float(f) => Ok(ObjectWrapper::Float(-f)),
                _ => Err(format!("cannot eval {} after '-'.", operator.to_string()).into()),
            },
            _ => Ok(ObjectWrapper::Null),
        }
    }

    fn eval_if_expression(
        &mut self,
        condition: &'prog Expression,
        consequence: &'prog Vec<Statement>,
        alternative: &'prog Vec<Statement>,
    ) -> Result<ObjectWrapper> {
        let cond = self.eval_expression(condition)?;
        if let ObjectWrapper::Boolean(v) = cond {
            if v {
                self.eval_block_statements(consequence)
            } else {
                self.eval_block_statements(alternative)
            }
        } else {
            return Err("Invalid 'if' condition.".into());
        }
    }
}
