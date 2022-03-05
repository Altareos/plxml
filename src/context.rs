use super::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Context {
    dict: HashMap<String, Value>,
    parent: Option<Box<Context>>,
}

impl Context {
    pub fn new(parent: Option<Box<Context>>) -> Context {
        Context {
            dict: HashMap::new(),
            parent,
        }
    }

    pub fn assign(&mut self, key: String, value: Value) {
        self.dict.insert(key, value);
    }

    pub fn value(&self, key: &String) -> Option<&Value> {
        self.dict
            .get(key)
            .or(self.parent.as_ref().and_then(|p| p.value(key)))
    }

    pub fn take(&mut self, key: &String) -> Option<Value> {
        self.dict.remove(key)
    }
}
