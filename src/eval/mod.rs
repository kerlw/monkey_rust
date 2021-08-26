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
            (ObjectWrapper::Null, ObjectWrapper::Null) => {
                Ok(ObjectWrapper::Boolean(true))
            }
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
            (ObjectWrapper::Null, ObjectWrapper::Null) => {
                Ok(ObjectWrapper::Boolean(false))
            }
            _ => Ok(ObjectWrapper::Boolean(true)),
        }
    }
}
