use crate::eval::ObjectWrapper;
use lazy_static::lazy_static;
use std::collections::HashMap;
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
