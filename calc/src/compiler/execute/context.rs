use compiler::*;


pub struct Var<'a> {
    name: &'a str,
    value: Value,
}


pub struct ExecContext<'a> {
    pub scope: Vec<Var<'a>>,
}


impl<'a> ExecContext<'a> {
    pub fn new() -> Self {
        ExecContext {
            scope: Vec::new(),
        }
    }
}

