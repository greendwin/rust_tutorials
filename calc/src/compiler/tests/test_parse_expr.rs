use compiler;
use compiler::def::*;
use compiler::tests::checks::*;


fn parse_expr(text: &str) -> AST {
	compiler::parse_expr(text, "<test>")
		.expect("parse_expr should not fail")
}


#[test]
fn expr_ident() {
	let expr = parse_expr("x");

	check_var(&expr, "x");
}


#[test]
fn expr_num() {
	let expr = parse_expr("42");

	check_num(&expr, 42);
}


#[test]
fn expr_binops() {
	for op in "+-*/%".chars() {
		let text = format!("42 {} x", op);
		let expr = parse_expr(&text);

		check_binop(
			&expr, op,
			|l| check_num(l, 42),
			|r| check_var(r, "x"));
	}
}


#[test]
fn expr_binops_priority() {
	let expr = parse_expr("5*x + 9 - 7*b");

	check_binop(
		&expr, '-',
		|l| check_binop(
			l, '+',
			|l1| check_binop(
				l1, '*',
				|l2| check_num(l2, 5),
				|r2| check_var(r2, "x")),
			|r1| check_num(r1, 9)),
		|r| check_binop(
			r, '*',
			|l1| check_num(l1, 7),
			|r1| check_var(r1, "b")));
}


#[test]
fn expr_binops_stack_add() {
	let expr = parse_expr("a + b - c");

	check_binop(
		&expr, '-',
		|l| check_binop(
			l, '+',
			|l1| check_var(l1, "a"),
			|r1| check_var(r1, "b")),
		|r| check_var(r, "c"));
}


#[test]
fn expr_binops_stack_mul() {
	let expr = parse_expr("a * b / c % d");

	check_binop(
		&expr, '%',
		|l| check_binop(
			l, '/',
			|l1| check_binop(
				l1, '*',
				|l2| check_var(l2, "a"),
				|r2| check_var(r2, "b")),
			|r1| check_var(r1, "c")),
		|r| check_var(r, "d"));
}


#[test]
fn parentheses() {
	let expr = parse_expr("a * (b + c)");

	check_binop(
		&expr, '*',
		|l| check_var(l, "a"),
		|r| check_binop(
			r, '+',
			|l1| check_var(l1, "b"),
			|r1| check_var(r1, "c")));
}
