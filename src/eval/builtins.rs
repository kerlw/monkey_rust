use crate::eval::ObjectWrapper;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::parser::Result;
use std::sync::{Arc, Mutex};

impl Into<ObjectWrapper> for bool {
    fn into(self) -> ObjectWrapper {
        ObjectWrapper::Boolean(self)
    }
}

impl Into<ObjectWrapper> for i64 {
    fn into(self) -> ObjectWrapper {
        ObjectWrapper::Integer(self)
    }
}

impl Into<ObjectWrapper> for f64 {
    fn into(self) -> ObjectWrapper {
        ObjectWrapper::Float(self)
    }
}

impl Into<ObjectWrapper> for String {
    fn into(self) -> ObjectWrapper {
        ObjectWrapper::String(self)
    }
}

lazy_static! {
    static ref BUILTINS_INS: Builtins = {
        let ret = Builtins {
            builtins: Arc::new(Mutex::new(HashMap::default())),
        };
        let builtins = ret.builtins.clone();
        let mut maps = builtins.lock().unwrap();
        maps.insert("PI".to_string(), std::f64::consts::PI.into());

        maps.insert("len".to_string(), ObjectWrapper::BuiltinFn(1, |args: Vec<ObjectWrapper>| -> Result<ObjectWrapper> {
            if args.len() != 1 {
                return Err(format!("Wrong number of arguments, expect 1 got {}", args.len()).into());
            }
            match &args[0] {
                ObjectWrapper::String(v) => Ok(ObjectWrapper::Integer(v.len() as i64)),
                _ => Err(format!("Argument to `len` not supported, got {}", args[0].type_str()).into()),
            }
        }));
        ret
    };
}

pub struct Builtins {
    builtins: Arc<Mutex<HashMap<String, ObjectWrapper>>>,
}

impl Builtins {
    pub fn instance_ref() -> &'static Self {
        &BUILTINS_INS
    }
}
