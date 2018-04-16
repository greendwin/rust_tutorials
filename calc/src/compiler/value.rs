use std::rc::Rc;
use std::fmt;
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


impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val::None => write!(f, "None"),
            Val::Num(val) => write!(f, "{}", val),
            Val::Func(ref decl) => write!(f, "fn {}", decl.name),
        }
    }
}
