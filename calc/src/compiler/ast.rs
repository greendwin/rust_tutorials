use super::location::*;


#[derive(Debug, PartialEq)]
pub enum AST<'a> {
    Block {
        loc: Loc<'a>,
        body: Vec<AST<'a>>,
    },

    // statements
    DeclVar {
        loc: Loc<'a>,
        name: &'a str,
        init: Box<AST<'a>>,
    },

    Assign {
        loc: Loc<'a>,
        name: &'a str,
        init: Box<AST<'a>>,
    },

    Return(Loc<'a>, Box<AST<'a>>),

    Func {
        loc: Loc<'a>,
        name: &'a str,
        args: Vec<&'a str>,
        body: Box<AST<'a>>,
    },

    // expression
    Num {
        loc: Loc<'a>,
        val: i32,
    },

    Var(Loc<'a>, &'a str),
	
    BinOp {
        loc: Loc<'a>,
        op: char,
        left: Box<AST<'a>>,
        right: Box<AST<'a>>,
    },
}


impl<'a> Location<'a> for AST<'a> {
    fn loc(&self) -> Loc<'a> {
        use self::AST::*;

        match *self {
            Block{ loc, .. } => loc,
            DeclVar{ loc, .. } => loc,
            Assign{ loc, .. } => loc,
            Return(loc, ..) => loc,
            Func{ loc, .. } => loc,
            Num{ loc, .. } => loc,
            Var(loc, ..) => loc,
            BinOp{loc, ..} => loc,
        }
    }
}

