use compiler::*;
use self::AST::*;


pub fn check_var(expr: &AST, expected: &str) {
	if let Var{ ref name, .. } = *expr {
		assert_eq!(expected, name);
	} else {
		panic!("Var type expected: {:?}", expr);
	}
}


pub fn check_num(expr: &AST, expected: i32) {
	if let Num{ val, .. } = *expr {
		assert_eq!(expected, val);
	} else {
		panic!("Num type expected: {:?}", expr);
	}
}


pub fn check_op<F1, F2>(
	expr: &AST,
	expected_op: char,
	left: F1, right: F2)
    where F1: Fn(&AST), F2: Fn(&AST)
{
	if let BinOp{loc: _, op, left: ref l, right: ref r} = *expr {
		assert_eq!(expected_op, op);

		left(l);
		right(r);
	} else {
		panic!("BinOp type expected: {:?}", expr);
	}
}


pub fn check_let<F>(expr: &AST, expected_name: &str, check: F)
    where F: Fn(&AST)
{
    if let DeclVar{ ref name, ref init, .. } = *expr {
        assert_eq!(expected_name, name);

        check(init);
    } else {
        panic!("Assign type expected: {:?}", expr);
    }
}


pub fn check_assign<F>(expr: &AST, expected_name: &str, check: F)
    where F: Fn(&AST)
{
    if let Assign{ ref name, ref init, .. } = *expr {
        assert_eq!(expected_name, name);

        check(init);
    } else {
        panic!("Assign type expected: {:?}", expr);
    }
}


pub fn check_return<F>(expr: &AST, check: F)
    where F: Fn(&AST)
{
    if let Return{ ref ret, .. } = *expr {
        check(ret);
    } else {
        panic!("Return type expected: {:?}", expr);
    }
}


pub fn check_func<F>(expr: &AST, expected_name: &str, expected_args: &[&str], check_body: F)
    where F: Fn(&AST)
{
    if let Func{ ref decl, .. } = *expr {
        assert_eq!(expected_name, decl.name);

        if expected_args.len() != decl.args.len() {
            panic!("wrong arguments count, expected {}: {:?}", expected_args.len(), decl.args);
        }
        for (exp, x) in expected_args.iter().zip(&decl.args) {
            assert_eq!(*exp, x);
        }

        check_body(&decl.body);
    } else {
        panic!("Assign type expected: {:?}", expr);
    }
}


pub fn check_block(expr: &AST, checkers: &[Box<Fn(&AST)>]) {
    if let Block{ ref body, .. } = *expr {
        if body.len() != checkers.len() {
            panic!("Wrong elements count: {} expected: {:?}", checkers.len(), expr);
        }

        for (st, ch) in body.iter().zip(checkers) {
            ch(&st);
        }
    } else {
        panic!("Block type expected: {:?}", expr);
    }
}


pub fn check_call(expr: &AST, expected_name: &str, args_checkers: &[Box<Fn(&AST)>]) {
    if let Call{ ref name, ref args, .. } = *expr {
        assert_eq!(expected_name, name);

        if args.len() != args_checkers.len() {
            panic!("Wrong elements count: {} expected: {:?}", args_checkers.len(), expr);
        }

        for (st, ch) in args.iter().zip(args_checkers) {
            ch(&st);
        }
    } else {
        panic!("FuncCall type expected: {:?}", expr);
    }
}


