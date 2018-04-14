use compiler::*;
use self::AST::*;


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


pub fn check_op<'a, F1, F2>(
	expr: &AST<'a>,
	expected_op: char,
	left: F1, right: F2)
    where F1: Fn(&AST<'a>), F2: Fn(&AST<'a>)
{
	if let BinOp{loc: _, op, left: ref l, right: ref r} = *expr {
		assert_eq!(expected_op, op);

		left(l);
		right(r);
	} else {
		panic!("BinOp type expected: {:?}", expr);
	}
}


pub fn check_block<'a>(expr: &AST<'a>, checkers: &[Box<Fn(&AST<'a>)>]) {
    if let Block(_loc, ref lst) = *expr {
        if lst.len() != checkers.len() {
            panic!("Wrong elements count: {} expected: {:?}", checkers.len(), expr);
        }

        for (st, ch) in lst.iter().zip(checkers) {
            (ch)(&st);
        }
    } else {
        panic!("Block type expected: {:?}", expr);
    }
}


pub fn check_let<'a, F>(expr: &AST<'a>, expected_name: &str, check: F)
    where F: Fn(&AST<'a>)
{
    if let DeclVar{ loc: _, name, ref init } = *expr {
        assert_eq!(expected_name, name);

        check(init);
    } else {
        panic!("Assign type expected: {:?}", expr);
    }
}


pub fn check_assign<'a, F>(expr: &AST<'a>, expected_name: &str, check: F)
    where F: Fn(&AST<'a>)
{
    if let Assign{ loc: _, name, ref init } = *expr {
        assert_eq!(expected_name, name);

        check(init);
    } else {
        panic!("Assign type expected: {:?}", expr);
    }
}


pub fn check_return<'a, F>(expr: &AST<'a>, check: F)
    where F: Fn(&AST<'a>)
{
    if let Return(_loc, ref ret) = *expr {
        check(ret);
    } else {
        panic!("Return type expected: {:?}", expr);
    }
}


pub fn check_func<'a, F>(expr: &AST<'a>, expected_name: &str, expected_args: &[&str], check_body: F)
    where F: Fn(&AST<'a>)
{
    if let Func{ loc: _, name, ref args, ref body } = *expr {
        assert_eq!(expected_name, name);
        assert_eq!(expected_args, args as &[&str]);

        check_body(body);
    } else {
        panic!("Assign type expected: {:?}", expr);
    }
}

