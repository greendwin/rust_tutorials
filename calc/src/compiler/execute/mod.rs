pub mod context;

// use compiler::*;


pub enum Value {
    None,
}


pub struct ExecContext {
    scope: Vec<Value>,
}


impl ExecContext {
    pub fn new() -> Self {
        ExecContext {
            scope: Vec::new(),
        }
    }
}

