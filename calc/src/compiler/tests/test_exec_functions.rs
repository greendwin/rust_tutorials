use compiler::*;
use super::test_execute::*;


#[test]
fn decl_func() {
    let ctx = exec(r#"
        fn test() {}
    "#);

    assert!(ctx.scope["test"].is_func());
}


#[test]
fn call_func() {
    let ctx = exec(r#"
        fn test() {
            return 42;
        }

        let x = test();
    "#);

    assert_eq!(Val::Num(42), ctx.scope["x"]);
}


#[test]
fn unexpected_return() {
    expect_error(r#"
        return 42;
    "#, "unexpected return statement");
}


#[test]
fn call_func_with_args() {
    let ctx = exec(r#"
        fn add(a, b) {
            return a + b;
        }

        let x = add(2, 3);
    "#);

    assert_eq!(Val::Num(5), ctx.scope["x"]);
}


#[test]
fn wrong_args_count() {
    expect_error(r#"
        fn foo(a) {}
        foo()
    "#, "wrong arguments count");
}


#[test]
fn call_native_funcs() {
    let mut ctx = ExecContext::new();

    ctx.decl_func("two", |_args| Val::Num(2));

    exec_with(&mut ctx, r#"
        let r = 2 * two();
    "#);

    assert_eq!(Val::Num(4), ctx.scope["r"]);
}

