use super::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Context<'par> {
    dict: HashMap<String, Value>,
    parent: Option<&'par Context<'par>>,
}

impl<'par> Context<'par> {
    pub fn new(parent: Option<&'par Context<'par>>) -> Context<'par> {
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
