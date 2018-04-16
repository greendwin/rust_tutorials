use std::collections::HashMap;
use compiler::*;


pub struct ExecContext {
    pub allow_return: bool,
    pub scope: HashMap<String, Val>,
}


impl ExecContext {
    pub fn new() -> Self {
        ExecContext {
            allow_return: false,
            scope: HashMap::new(),
        }
    }

    pub fn new_nested(&self) -> Self {
        Self::new()
    }

    pub fn set_var(&mut self, name: &str, val: Val) {
        self.scope.insert(String::from(name), val);
    }

    pub fn decl_func<T>(&mut self, name: &str, func: T) 
        where T: Fn(Vec<Val>) -> Val + 'static
    {
        self.scope.insert(
            String::from(name),
            Val::new_func(name, func));
    }
}

