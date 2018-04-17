use std::rc::Rc;
use compiler::*;


pub struct ExecContext {
    pub scope: Rc<Scope>,
    pub allow_return: bool,
}


impl ExecContext {
    pub fn new() -> Self {
        ExecContext {
            scope: Scope::new(),
            allow_return: false,
        }
    }

    pub fn new_with(&self, scope: &Rc<Scope>) -> Self {
        ExecContext {
            scope: Scope::new_nested(scope),
            allow_return: false,
        }
    }

    pub fn lookup_name(&self, name: &str) -> Option<Val> {
        let mut scope = &self.scope;

        loop {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }

            if let Some(ref parent) = scope.parent {
                scope = parent;
            } else {
                return None;
            }
        }
    }

    pub fn has_var(&self, name: &str) -> bool {
        self.scope.vals.borrow().contains_key(name)
    }

    pub fn set_var(&mut self, name: &str, val: Val) {
        self.scope.vals.borrow_mut().insert(String::from(name), val);
    }

    pub fn decl_func<T>(&mut self, name: &str, func: T) 
        where T: Fn(Vec<Val>) -> Val + 'static
    {
        self.set_var(name, Val::new_func(name, func));
    }
}

