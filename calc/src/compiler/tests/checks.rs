use compiler::def::*;
use compiler::def::AST::*;


pub fn check_var(expr: &AST, expected: &str) {
	if let Var(_loc, name) = *expr {
		assert_eq!(expected, name);
	} else {
		panic!("Var type expected: {:?}", expr);
	}
}


pub fn check_num(expr: &AST, expected: i32) {
	if let Num(_loc, val) = *expr {
		assert_eq!(expected, val);
	} else {
		panic!("Num type expected: {:?}", expr);
	}
}


pub fn check_binop<'a, Fl, Fr>(
	expr: &AST<'a>,
	expected_op: char,
	left: Fl, right: Fr) 
	where
		Fl: Fn(&AST<'a>),
		Fr: Fn(&AST<'a>)
{
	if let BinOp{loc: _, op, left: ref l, right: ref r} = *expr {
		assert_eq!(expected_op, op);

		(left)(&l);
		(right)(&r);
	} else {
		panic!("BinOp type expected: {:?}", expr);
	}
}


