use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;
use compiler::*;


pub type ValMap = HashMap<String, Val>;

pub struct Scope {
    pub vals: ValMap,
    pub parent: Option<Rc<Scope>>,
}


pub struct ExecContext {
    pub scope: Rc<Scope>,
    pub allow_return: bool,
}


impl Scope {
    pub fn new() -> Rc<Self> {
        Rc::new(Scope {
            vals: HashMap::new(),
            parent: None,
        })
    }

    pub fn new_nested(parent: &Rc<Scope>) -> Rc<Self> {
        Rc::new(Scope {
            vals: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        })
    }
}


impl<'a> Index<&'a str> for Scope {
    type Output = Val 

    #[inline]
    fn index(&self, key: &str) -> &Val{
        &self.vals[key]
    }
}


impl ExecContext {
    pub fn new() -> Self {
        ExecContext {
            scope: Scope::new(),
            allow_return: false,
        }
    }

    pub fn new_nested(&self) -> Self {
        ExecContext {
            scope: Scope::new_nested(&self.scope),
            allow_return: false,
        }
    }

    pub fn lookup_name(&self, name: &str) -> Option<&Val> {
        let mut scope = &self.scope;

        loop {
            if scope.vals.contains_key(name) {
                return Some(&scope.vals[name]);
            }

            if let Some(ref parent) = scope.parent {
                scope = parent;
            } else {
                return None;
            }
        }
    }

    pub fn has_var(&self, name: &str) -> bool {
        self.scope.vals.contains_key(name)
    }

    pub fn set_var(&mut self, name: &str, val: Val) {
        let scope = Rc::get_mut(&mut self.scope).unwrap();
        scope.vals.insert(String::from(name), val);
    }

    pub fn decl_func<T>(&mut self, name: &str, func: T) 
        where T: Fn(Vec<Val>) -> Val + 'static
    {
        self.set_var(name, Val::new_func(name, func));
    }
}

