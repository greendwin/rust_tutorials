use std::collections::HashMap;
use compiler::*;


pub struct ExecContext {
    pub scope: HashMap<String, Val>,
}


impl ExecContext {
    pub fn new() -> Self {
        ExecContext {
            scope: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: &str, val: Val) {
        self.scope.insert(String::from(name), val);
    }
}

