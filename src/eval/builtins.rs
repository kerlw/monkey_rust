use crate::eval::ObjectWrapper;
use crate::parser::Result;
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

        maps.insert(
            "len".to_string(),
            ObjectWrapper::BuiltinFn(1, |args: Vec<ObjectWrapper>| -> Result<ObjectWrapper> {
                if args.len() != 1 {
                    return Err(
                        format!("Wrong number of arguments, expect 1 got {}", args.len()).into(),
                    );
                }
                match &args[0] {
                    ObjectWrapper::String(v) => Ok(ObjectWrapper::Integer(v.len() as i64)),
                    ObjectWrapper::Array(array) => Ok(ObjectWrapper::Integer(array.len() as i64)),
                    _ => Err(format!(
                        "Argument to `len` not supported, got {}",
                        args[0].type_str()
                    )
                    .into()),
                }
            }),
        );

        maps.insert(
            "first".to_string(),
            ObjectWrapper::BuiltinFn(1, |args: Vec<ObjectWrapper>| -> Result<ObjectWrapper> {
                if args.len() != 1 {
                    return Err(
                        format!("Wrong number of arguments, expect 1 got {}", args.len()).into(),
                    );
                }

                if let ObjectWrapper::Array(array) = &args[0] {
                    if array.len() > 0 {
                        Ok(array[0].clone())
                    } else {
                        Ok(ObjectWrapper::Null)
                    }
                } else {
                    return Err(format!(
                        "Argument to 'first' must be ARRAY, got {:?}",
                        args[0].type_str()
                    )
                    .into());
                }
            }),
        );

        maps.insert(
            "last".to_string(),
            ObjectWrapper::BuiltinFn(1, |args: Vec<ObjectWrapper>| -> Result<ObjectWrapper> {
                if args.len() != 1 {
                    return Err(
                        format!("Wrong number of arguments, expect 1 got {}", args.len()).into(),
                    );
                }

                if let ObjectWrapper::Array(array) = &args[0] {
                    if array.len() > 0 {
                        Ok(array[array.len() - 1].clone())
                    } else {
                        Ok(ObjectWrapper::Null)
                    }
                } else {
                    return Err(format!(
                        "Argument to 'last' must be ARRAY, got {:?}",
                        args[0].type_str()
                    )
                    .into());
                }
            }),
        );

        maps.insert(
            "rest".to_string(),
            ObjectWrapper::BuiltinFn(1, |args: Vec<ObjectWrapper>| -> Result<ObjectWrapper> {
                if args.len() != 1 {
                    return Err(
                        format!("Wrong number of arguments, expect 1 got {}", args.len()).into(),
                    );
                }

                if let ObjectWrapper::Array(array) = &args[0] {
                    if array.len() > 0 {
                        Ok(ObjectWrapper::Array(
                            array[0..array.len() - 1]
                                .iter()
                                .map(|ele| ele.clone())
                                .collect(),
                        ))
                    } else {
                        Ok(ObjectWrapper::Null)
                    }
                } else {
                    return Err(format!(
                        "Argument to 'rest' must be ARRAY, got {:?}",
                        args[0].type_str()
                    )
                    .into());
                }
            }),
        );
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

    pub fn contains(&self, ident: &str) -> bool {
        self.builtins.clone().lock().unwrap().contains_key(ident)
    }

    // pub fn set(&mut self, ident: &str, obj: ObjectWrapper) -> Option<ObjectWrapper> {
    //     self.builtins
    //         .clone()
    //         .lock()
    //         .unwrap()
    //         .insert(ident.to_string(), obj)
    // }

    pub fn get(&self, ident: &str) -> Option<ObjectWrapper> {
        // 最后的结果如果不clone会导致无法返回，因此get_mut已经失去意义
        self.builtins.clone().lock().unwrap().get(ident).cloned()
    }

    // pub fn get_mut(&mut self, ident: &str) -> Option<&mut ObjectWrapper> {
    //     self.builtins.clone().lock().unwrap().get_mut(ident)
    // }
}
