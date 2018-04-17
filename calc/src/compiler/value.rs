use std::rc::Rc;
use std::fmt;
use compiler::*;


#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    None,
    Num(i32),
    Str(Rc<String>),
    Func{
        decl: Rc<FuncDecl>,
        scope: Rc<Scope>,
    },
    NativeFunc(Rc<NativeFuncDecl>),
}


pub type CallbackType = Fn(Vec<Val>) -> Val;

pub struct NativeFuncDecl {
    pub name: String,
    pub callback: Box<CallbackType>,
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

    pub fn as_str(&self) -> Option<&String> {
        match *self {
            Val::Str(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn new_func<T>(name: &str, callback: T) -> Val 
        where T: Fn(Vec<Val>) -> Val + 'static
    {
        Val::NativeFunc(
            Rc::new(NativeFuncDecl {
                name: String::from(name),
                callback: Box::new(callback),
            }))
    }
}


impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val::None => write!(f, "None"),
            Val::Num(val) => write!(f, "{}", val),
            Val::Str(ref val) => write!(f, r#""{}""#, val),
            Val::Func{ref decl, ..} => write!(f, "fn {}", decl.name),
            Val::NativeFunc(ref decl) => write!(f, "native fn {}", decl.name),
        }
    }
}


impl fmt::Debug for NativeFuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Val::NativeFunc("{}", ..)"#, self.name)
    }
}


impl PartialEq for NativeFuncDecl {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name
            && &*self.callback as *const CallbackType == &*other.callback as *const CallbackType;
    }
}

