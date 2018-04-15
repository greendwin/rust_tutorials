use std::rc::Rc;
use compiler::*;


#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    None,
    Num(i32),
    Func(Rc<FuncDecl>),
}


impl Val {
    pub fn is_num(&self) -> bool {
        match *self {
            Val::Num(..) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match *self {
            Val::Func{..} => true,
            _ => false,
        }
    }

    pub fn as_num(&self) -> Option<i32> {
        match *self {
            Val::Num(val) => Some(val),
            _ => None,
        }
    }
}

