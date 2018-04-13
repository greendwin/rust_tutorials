

#[test]
fn empty() {
	let statements = parse_it(r#""#);
	assert_eq!(0, statements.len());
}


#[test]
fn let_statement() {
	let statements = parse_it(r#"
		let x = 5;
	"#);

	assert_eq!(1, statements.len());
	check_let(statements[0], "x", |x| check_num(x, 5));
}


#[test]
fn assign_statement() {
	let statements = parse_it(r#"
		x = 5;
	"#);

	assert_eq!(1, statements.len());
	check_assign(statements[0], "x", |x| check_num(x, 5));
}


#[test]
fn fn_statement() {
	let statements = parse_it(r#"
		fn foo() {}
	"#);

	assert_eq!(1, statements.len());
	check_fn(statements[0], "foo", vec![], |x| ());
}
