use compiler::*;


#[derive(Debug)]
pub enum FlowExc {
    Error(Error),
    Return(Val),
    // Break,
    // Continue,
}


pub type ExecResult = Result<Val, FlowExc>;


impl FlowExc {
    pub fn to_error(self) -> Error {
        match self {
            FlowExc::Error(err) => err,
            _ => panic!("Unexpected flow exception: {:?}", self),
        }
    }
}


impl Into<ExecResult> for Error {
    fn into(self) -> ExecResult {
        Err(FlowExc::Error(self))
    }
}

