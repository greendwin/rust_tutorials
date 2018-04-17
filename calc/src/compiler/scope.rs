use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use compiler::*;


pub type ValMap = HashMap<String, Val>;

#[derive(Debug, PartialEq)]
pub struct Scope {
    pub vals: RefCell<ValMap>,
    pub parent: Option<Rc<Scope>>,
}


impl Scope {
    pub fn new() -> Rc<Self> {
        Rc::new(Scope {
            vals: RefCell::new(HashMap::new()),
            parent: None,
        })
    }

    pub fn new_nested(other: &Rc<Self>) -> Rc<Self> {
        Rc::new(Scope {
            vals: RefCell::new(HashMap::new()),
            parent: Some(Rc::clone(other)),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.vals.borrow().is_empty()
    }

    pub fn get(&self, key: &str) -> Option<Val> {
        match self.vals.borrow().get(key) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn get_val(&self, key: &str) -> Val {
        self.get(key).unwrap()
    }
}
