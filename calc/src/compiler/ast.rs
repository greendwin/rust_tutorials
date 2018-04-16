use std::rc::Rc;
use super::location::*;


#[derive(Debug, PartialEq)]
pub enum AST {
    Block {
        loc: Loc,
        body: Vec<AST>,
    },

    // statements
    DeclVar {
        loc: Loc,
        name: String,
        init: Box<AST>,
    },

    Assign {
        loc: Loc,
        name: String,
        init: Box<AST>,
    },

    Return {
        loc: Loc,
        ret: Box<AST>,
    },

    Func {
        loc: Loc,
        decl: Rc<FuncDecl>,
    },

    // expression
    Num {
        loc: Loc,
        val: i32,
    },

    Var {
        loc: Loc,
        name: String,
    },

    Call {
        loc: Loc,
        name: String,
        args: Vec<AST>,
    },
	
    BinOp {
        loc: Loc,
        op: char,
        left: Box<AST>,
        right: Box<AST>,
    },
}


#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    pub name: String,
    pub args: Vec<String>,
    pub body: AST,
}


impl Location for AST {
    fn loc(&self) -> &Loc {
        use self::AST::*;

        match *self {
            Block   { ref loc, .. } => loc,
            DeclVar { ref loc, .. } => loc,
            Assign  { ref loc, .. } => loc,
            Return  { ref loc, .. } => loc,
            Func    { ref loc, .. } => loc,
            Num     { ref loc, .. } => loc,
            Var     { ref loc, .. } => loc,
            Call    { ref loc, .. } => loc,
            BinOp   { ref loc, .. } => loc,
        }
    }
}

