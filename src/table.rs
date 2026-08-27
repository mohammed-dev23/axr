use std::collections::HashMap;

use crate::value::Value;

pub struct Table(HashMap<String, Value>);

impl Table {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, key: &str, value: Value) -> bool {
        let is_new = !self.0.contains_key(key);
        self.0.insert(key.to_string(), value);
        is_new
    }

    pub fn add_table(&mut self, table: Table) {
        for (key, value) in table.0 {
            self.0.insert(key, value);
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.0.remove(key).is_some()
    }
}
