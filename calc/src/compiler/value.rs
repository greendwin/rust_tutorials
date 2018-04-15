

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Val {
    None,
    Num(i32),
}


impl Val {
//    pub fn is_none(&self) -> bool {
//        *self == Val::None
//    }

    pub fn is_num(&self) -> bool {
        match *self {
            Val::Num(_val) => true,
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
