use crate::eval::environment::Environment;
use crate::eval::ObjectWrapper;
use crate::lexer::token::Token;
use crate::parser::program::{Expression, Ident, Statement};
use crate::parser::Result;
use std::rc::Rc;

pub struct Evaluator<'a> {
    statements: &'a Vec<Statement>,
    env: Environment,
}

impl<'a> Evaluator<'a> {
    pub fn new(statements: &'a Vec<Statement>) -> Self {
        Evaluator {
            statements,
            env: Environment::default(),
        }
    }

    pub fn with_env(statements: &'a Vec<Statement>, env: Environment) -> Self {
        Evaluator { statements, env }
    }

    pub fn eval(&mut self) -> Result<ObjectWrapper> {
        match self.eval_statements(&self.statements) {
            Err(e) => Ok(ObjectWrapper::ErrorObject(e.to_string())),
            Ok(ret) => Ok(ret),
        }
    }

    fn eval_statements(&mut self, statements: &Vec<Statement>) -> Result<ObjectWrapper> {
        let mut ret = ObjectWrapper::Null;
        for st in statements {
            ret = self.eval_statement(st)?;
            log::trace!("eval_statement result: {:}", ret);
            if let ObjectWrapper::ReturnValue(v) = ret {
                return Ok(*v);
            }
        }
        Ok(ret)
    }

    fn eval_block_statements(&mut self, statements: &Vec<Statement>) -> Result<ObjectWrapper> {
        let mut ret = ObjectWrapper::Null;
        for st in statements {
            ret = self.eval_statement(st)?;
            if let ObjectWrapper::ReturnValue(_) = ret {
                break;
            }
        }
        Ok(ret)
    }

    fn eval_statement(&mut self, statement: &Statement) -> Result<ObjectWrapper> {
        match statement {
            Statement::ReturnStatement(expr) => self.eval_return_statement(expr),
            Statement::ExpressionStatement(expr) => self.eval_expression(expr),
            Statement::LetStatement(ident, expression) => {
                let value = self.eval_expression(expression)?;
                self.env.set(&ident.0, value);
                Ok(ObjectWrapper::Null)
            } // _ => Ok(ObjectWrapper::Null),
        }
    }

    fn eval_return_statement(&mut self, expression: &Expression) -> Result<ObjectWrapper> {
        let ret = self.eval_expression(expression)?;
        Ok(ObjectWrapper::ReturnValue(Box::new(ret)))
    }

    fn eval_expression(&mut self, expression: &Expression) -> Result<ObjectWrapper> {
        match expression {
            Expression::Identifier(ident) => {
                if self.env.contains(&ident.0) {
                    Ok(self.env.get(&ident.0).unwrap().clone())
                } else {
                    Err(format!("identifier not found: {}", &ident.0).into())
                }
            }
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
            Expression::FunctionExpression(params, body) => Ok(ObjectWrapper::FunctionObject(
                Rc::new(params.clone()),
                Rc::new(body.clone()),
                self.env.clone(),
            )),
            Expression::CallExpression(func, params) => self.eval_call_expression(func, params),
            _ => Ok(ObjectWrapper::Null),
        }
    }

    fn eval_infix_expression(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
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
        operator: &Token,
        expr: &Expression,
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
        condition: &Expression,
        consequence: &Vec<Statement>,
        alternative: &Vec<Statement>,
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

    fn eval_call_expression(
        &mut self,
        func: &Expression,
        params: &Vec<Expression>,
    ) -> Result<ObjectWrapper> {
        let real_params = params
            .iter()
            .map(|expr| self.eval_expression(expr))
            .collect::<Result<Vec<ObjectWrapper>>>()?;
        match func {
            Expression::Identifier(ident) => {
                let obj = self.env.get(&ident.0);
                if let Some(ObjectWrapper::FunctionObject(params_ident, body, env_func)) = obj {
                    let params_ident = params_ident.clone();
                    let body = body.clone();
                    let env = env_func.clone();
                    self.do_eval_function_call(&params_ident, &real_params, &body, env)
                } else {
                    Err(format!("function not found: {}", &ident.0).into())
                }
            }
            Expression::FunctionExpression(params_ident, body) => {
                self.do_eval_function_call(params_ident, &real_params, body, self.env.clone())
            }
            _ => Err("invalid call expression.".into()),
        }
    }

    fn do_eval_function_call(
        &mut self,
        params_ident: &Vec<Ident>,
        params: &Vec<ObjectWrapper>,
        body: &Vec<Statement>,
        mut env: Environment,
    ) -> Result<ObjectWrapper> {
        if params.len() != params_ident.len() {
            return Err(format!(
                "Invalid params, expect {} got {}",
                params_ident.len(),
                params.len()
            )
            .into());
        }
        params
            .iter()
            .zip(params_ident.iter())
            .for_each(|(obj, param_ident)| {
                env.set(&param_ident.0, obj.clone());
            });
        let mut evaluator = Evaluator::with_env(body, env);
        evaluator.eval()
    }
}
