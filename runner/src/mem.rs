use std::collections::HashMap;

pub struct Stack(Vec<u8>);

pub struct Scope {
    symbols: Vec<String>,
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Vec<Value>>,
}

pub enum Value {
    Void,
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl SymbolTable {
    fn create_var() {}
    fn remove_var() {}
    fn create_function() {}
}
