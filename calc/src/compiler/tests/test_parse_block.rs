use compiler;
use compiler::*;
use super::checks::*;


fn parse_it<'a>(text: &'a str) -> AST<'a> {
    compiler::parse(text, "<test>")
        .expect("parse should not fail")
}


#[test]
fn empty() {
	let block = parse_it("");

    check_block(&block, &[]);
}


#[test]
fn let_statement() {
	let block = parse_it("let x = 5;");

    check_block(
        &block, &[
            Box::new(|st| check_let(st, "x", |p| check_num(p, 5)))
        ]);
}


#[test]
fn assign_statement() {
	let block = parse_it("x = 5;");

    check_block(
        &block, &[
            Box::new(|st| check_assign(st, "x", |p| check_num(p, 5)))
        ]);
}


#[test]
fn fn_statement() {
	let block = parse_it("fn foo() {}");

    check_block(
        &block, &[
            Box::new(|st| check_func(st, "foo", &[], |_x| ()))
        ]);
}


#[test]
fn function_body () {
	let block = parse_it(r#"
        fn foo(a, b) {
            return a * b;
        }
    "#);

    check_block(
        &block, &[Box::new(
            |expr| check_func(
                expr, "foo", &["a", "b"],
                |body| check_block(body, &[Box::new(
                    |p| check_return(
                        p, |ret| check_op(
                            ret, '*',
                            |l| check_var(l, "a"),
                            |r| check_var(r, "b")))),
                ]))),
        ]);
}


