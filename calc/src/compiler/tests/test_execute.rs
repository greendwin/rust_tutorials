
use compiler;
use compiler::*;


pub fn exec_with(ctx: &mut ExecContext, text: &str) {
    let prog = compiler::parse(text, "<test>")
        .map_err(|err| {
            panic!("parse should not fail: {}", err);
        })
        .unwrap();

    compiler::execute(ctx, &prog)
        .map_err(|err| {
            panic!("execution should not fail: {}", err.to_error());
        })
        .unwrap();
}


pub fn exec(text: &str) -> ExecContext {
    let mut ctx = ExecContext::new();
    exec_with(&mut ctx, text);

    // return context so it could be checked for results
    return ctx;
}


pub fn expect_error(text: &str, expected_words: &str) {
    let prog = compiler::parse(text, "<test>")
        .map_err(|err| {
            panic!("parse should not fail: {}", err);
        })
        .unwrap();

    let mut ctx = ExecContext::new();
    let r = compiler::execute(&mut ctx, &prog);

    match r {
        Ok(_) => {
            panic!("program was expected to fail");
        }
        Err(e) => {
            let e = e.to_error();
            let description = e.description.to_lowercase();

            for word in expected_words.split(" ") {
                if !description.contains(word) {
                    panic!("missing word '{}': '{}'", word, e.description);
                }
            }
        }
    }
}


#[test]
fn empty_program() {
    let ctx = exec("");

    assert!(ctx.scope.is_empty());
}


#[test]
fn decl_var() {
    let ctx = exec(r#"
        let x = 42;
    "#);

    assert_eq!(Val::Num(42), ctx.scope.get_val("x"));
}


#[test]
fn decl_var_check_duplicate() {
    expect_error(r#"
        let x = 42;
        let x = 42;
    "#, "'x' redefinition");
}


#[test]
fn assign_val() {
    let mut ctx = ExecContext::new();
    ctx.set_var("x", Val::None);

    exec_with(&mut ctx, r#"
        x = 42;
    "#);

    assert_eq!(Val::Num(42), ctx.scope.get_val("x"));
}


#[test]
fn assign_val_undeclared() {
    expect_error(r#"
        x = 42;
    "#, "'x' undeclared");
}


#[test]
fn read_variable() {
    let ctx = exec(r#"
        let a = 42;
        let b = a;
    "#);

    assert_eq!(Val::Num(42), ctx.scope.get_val("b"));
}


#[test]
fn read_val_undeclared() {
    expect_error(r#"
        let y = x;
    "#, "'x' undeclared");
}


#[test]
fn add_sub() {
    let ctx = exec(r#"
        let x = 2 + 5;  // = 7
        let y = x - 8;  // = -1
    "#);

    assert_eq!(Val::Num(7), ctx.scope.get_val("x"));
    assert_eq!(Val::Num(-1), ctx.scope.get_val("y"));
}


#[test]
fn mul_div_mod() {
    let ctx = exec(r#"
        let x = 5 * 2;  // 10
        let y = x / 3;  // 3
        let z = x % 4;  // 2
    "#);

    assert_eq!(Val::Num(10), ctx.scope.get_val("x"));
    assert_eq!(Val::Num(3), ctx.scope.get_val("y"));
    assert_eq!(Val::Num(2), ctx.scope.get_val("z"));
}


#[test]
fn add_mismatched_types() {
    expect_error(r#"
        fn foo() {}
        let x = foo + 5;
    "#, "type mismatch foo 5");
}

