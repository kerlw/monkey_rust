use super::parser::program::{Expression, Ident, Statement};
use crate::eval::environment::Environment;
use crate::parser::Result;
use std::any::Any;
use std::rc::Rc;

pub mod environment;
pub mod evaluator;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectWrapper {
    Null,
    Integer(i64),
    Float(f64),
    Boolean(bool),
    ReturnValue(Box<ObjectWrapper>),
    ErrorObject(String),
    FunctionObject(Rc<Vec<Ident>>, Rc<Vec<Statement>>),
}

fn ensure_compare_with_same_type(one: &ObjectWrapper, two: &ObjectWrapper) -> Result<()> {
    if !one.type_str().eq(two.type_str()) {
        return Err(format!(
            "Cannot compare between different types:{} and {}",
            one.type_str(),
            two.type_str()
        )
        .into());
    }
    Ok(())
}

impl ObjectWrapper {
    pub fn type_str(&self) -> &str {
        match self {
            ObjectWrapper::Null => "NULL",
            ObjectWrapper::Integer(_) => "int",
            ObjectWrapper::Float(_) => "float",
            ObjectWrapper::Boolean(_) => "bool",
            ObjectWrapper::ReturnValue(_) => "return_value",
            ObjectWrapper::ErrorObject(_) => "error",
            ObjectWrapper::FunctionObject(_, _) => "function",
            _ => "untyped",
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self> {
        match self {
            ObjectWrapper::Integer(one) => match other {
                ObjectWrapper::Integer(two) => Ok(ObjectWrapper::Integer(one + two)),
                ObjectWrapper::Float(two) => Ok(ObjectWrapper::Float((*one as f64) + two)),
                _ => Err(format!("int cannot '+' with type {}.", other.type_str()).into()),
            },
            _ => Err(format!("type {} dose not support '+' operation.", self.type_str()).into()),
        }
    }

    pub fn eq(&self, other: &Self) -> Result<Self> {
        ensure_compare_with_same_type(self, other)?;

        match (self, other) {
            (ObjectWrapper::Integer(one), ObjectWrapper::Integer(two)) => {
                Ok(ObjectWrapper::Boolean(one == two))
            }
            (ObjectWrapper::Float(one), ObjectWrapper::Float(two)) => {
                Ok(ObjectWrapper::Boolean(one == two))
            }
            (ObjectWrapper::Boolean(one), ObjectWrapper::Boolean(two)) => {
                Ok(ObjectWrapper::Boolean(one == two))
            }
            (ObjectWrapper::Null, ObjectWrapper::Null) => Ok(ObjectWrapper::Boolean(true)),
            _ => Ok(ObjectWrapper::Boolean(false)),
        }
    }

    pub fn not_eq(&self, other: &Self) -> Result<Self> {
        ensure_compare_with_same_type(self, other)?;

        match (self, other) {
            (ObjectWrapper::Integer(one), ObjectWrapper::Integer(two)) => {
                Ok(ObjectWrapper::Boolean(one != two))
            }
            (ObjectWrapper::Float(one), ObjectWrapper::Float(two)) => {
                Ok(ObjectWrapper::Boolean(one != two))
            }
            (ObjectWrapper::Boolean(one), ObjectWrapper::Boolean(two)) => {
                Ok(ObjectWrapper::Boolean(one != two))
            }
            (ObjectWrapper::Null, ObjectWrapper::Null) => Ok(ObjectWrapper::Boolean(false)),
            _ => Ok(ObjectWrapper::Boolean(true)),
        }
    }

    pub fn multi(&self, other: &Self) -> Result<Self> {
        ensure_compare_with_same_type(self, other)?;

        match (self, other) {
            (ObjectWrapper::Integer(one), ObjectWrapper::Integer(two)) => {
                Ok(ObjectWrapper::Integer(one * two))
            }
            (ObjectWrapper::Float(one), ObjectWrapper::Float(two)) => {
                Ok(ObjectWrapper::Float(one * two))
            }
            _ => Err(format!(
                "'*' is not support between {} and {}",
                self.type_str(),
                other.type_str()
            )
            .into()),
        }
    }

    pub fn divide(&self, other: &Self) -> Result<Self> {
        ensure_compare_with_same_type(self, other)?;

        match (self, other) {
            (ObjectWrapper::Integer(one), ObjectWrapper::Integer(two)) => {
                Ok(ObjectWrapper::Integer(one / two))
            }
            (ObjectWrapper::Float(one), ObjectWrapper::Float(two)) => {
                Ok(ObjectWrapper::Float(one / two))
            }
            _ => Err(format!(
                "'/' is not support between {} and {}",
                self.type_str(),
                other.type_str()
            )
            .into()),
        }
    }

    pub fn great_than(&self, other: &Self) -> Result<Self> {
        ensure_compare_with_same_type(self, other)?;

        match (self, other) {
            (ObjectWrapper::Integer(one), ObjectWrapper::Integer(two)) => {
                Ok(ObjectWrapper::Boolean(one > two))
            }
            (ObjectWrapper::Float(one), ObjectWrapper::Float(two)) => {
                Ok(ObjectWrapper::Boolean(one > two))
            }
            _ => Err(format!(
                "'>' is not support between {} and {}",
                self.type_str(),
                other.type_str()
            )
            .into()),
        }
    }

    pub fn less_than(&self, other: &Self) -> Result<Self> {
        ensure_compare_with_same_type(self, other)?;

        match (self, other) {
            (ObjectWrapper::Integer(one), ObjectWrapper::Integer(two)) => {
                Ok(ObjectWrapper::Boolean(one > two))
            }
            (ObjectWrapper::Float(one), ObjectWrapper::Float(two)) => {
                Ok(ObjectWrapper::Boolean(one < two))
            }
            _ => Err(format!(
                "'<' is not support between {} and {}",
                self.type_str(),
                other.type_str()
            )
            .into()),
        }
    }
}
