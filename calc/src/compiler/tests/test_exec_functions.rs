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


#[test]
fn access_vals_from_parent() {
    let ctx = exec(r#"
        fn x2(val) {
            return val * 2;
        }

        fn test(val) {
            return x2(x2(val));
        }

        let r = test(5); // = 20
    "#);

    assert_eq!(Val::Num(20), ctx.scope["r"]);
}


#[test]
fn store_vals_by_ref() {
    let ctx = exec(r#"
        let x = 5;

        fn test() {
            return x * 5;
        }

        x = 10;
        let r = test(); // = 50
    "#);

    assert_eq!(Val::Num(50), ctx.scope["r"]);
}


#[test]
fn capture_vals() {
    let ctx = exec(r#"
        fn make_func(x) {
            fn mul(y) {
                return x * y;
            }
            return mul;
        }

        let mul_10 = make_func(10);
        let mul_20 = make_func(20);

        let x = mul_10(42); // 420
        let y = mul_20(42); // 840
    "#);

    assert_eq!(Val::Num(420), ctx.scope["x"]);
    assert_eq!(Val::Num(840), ctx.scope["y"]);
}


#[test]
fn capture_vals_dynamic() {
    let ctx = exec(r#"
        let x = 5;

        fn foo() {
            let y = x;  // wow!
            let x = 10;
            return x * y;
        }

        let r = foo();
    "#);

    assert_eq!(Val::Num(50), ctx.scope["r"]);
}


/*
#[test]
fn hello_world() {
    let mut ctx = ExecContext::new();
    let mut out = String::new();

    ctx.decl_func("println", |args| {
        for val in args {
            out.push_str(val.as_str().unwrap());
        }

        Val::None
    });
    
    exec_with(&mut ctx, r#"
        fn main() {
            println("Hello world!");
        }
    "#);

    ctx.exec_func("main", vec![])
        .expect("'main' method should not fail");

    assert_eq!("Hello world!", out);
}
*/


// TODO: nested methods may fail! (e.g.: input args validation)
// TODO: standard library
