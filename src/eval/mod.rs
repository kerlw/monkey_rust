use crate::parser::Result;
use std::any::Any;

pub mod evaluator;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum ObjectWrapper {
    Null,
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl ObjectWrapper {
    pub fn type_str(&self) -> &str {
        match self {
            ObjectWrapper::Integer(_) => "int",
            ObjectWrapper::Float(_) => "float",
            ObjectWrapper::Boolean(_) => "bool",
            ObjectWrapper::Null => "NULL",
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
}
