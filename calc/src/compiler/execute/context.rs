use std::collections::HashMap;
use compiler::*;


pub struct ExecContext<'a> {
    pub scope: HashMap<&'a str, Val>,
}


impl<'a> ExecContext<'a> {
    pub fn new() -> Self {
        ExecContext {
            scope: HashMap::new(),
        }
    }
}

