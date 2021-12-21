use std::collections::HashMap;

pub struct Stack(Vec<u8>);

pub struct Scope {
    pub symbols: Vec<String>,
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Void,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
}

impl SymbolTable {
    pub fn create_var(&mut self, name: &str, value: Value) {
        self.symbols.insert(name.into(), value);
    }

    pub fn get_value(&self, name: &str) -> Option<&Value> {
        self.symbols.get(name)
    }

    pub fn set_value(&mut self, name: &str, value: Value) {
        self.create_var(name, value);
    }
}
