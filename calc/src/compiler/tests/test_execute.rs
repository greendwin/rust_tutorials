
use compiler;
use compiler::*;


fn exec_with<'a>(ctx: &mut ExecContext<'a>, text: &'a str) {
    let prog = compiler::parse(text, "<test>")
        .expect("parse should not fail");

    compiler::execute(&prog, ctx)
        .expect("execution should not fail");
}


fn exec<'a>(text: &'a str) -> ExecContext {
    let mut ctx = ExecContext::new();
    exec_with(&mut ctx, text);

    // return context so it could be checked for results
    return ctx;
}


fn expect_error(text: &str, expected_words: &str) {
    let prog = compiler::parse(text, "<test>")
        .expect("parse should not fail");

    let mut ctx = ExecContext::new();
    let r = compiler::execute(&prog, &mut ctx);

    match r {
        Ok(_) => {
            panic!("program was expected to fail");
        }
        Err(e) => {
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

    assert_eq!(Val::Num(42), ctx.scope["x"]);
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
    ctx.scope.insert("x", Val::None);

    exec_with(&mut ctx, r#"
        x = 42;
    "#);

    assert_eq!(Val::Num(42), ctx.scope["x"]);
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

    assert_eq!(Val::Num(42), ctx.scope["b"]);
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

    assert_eq!(Val::Num(7), ctx.scope["x"]);
    assert_eq!(Val::Num(-1), ctx.scope["y"]);
}


#[test]
fn mul_div_mod() {
    let ctx = exec(r#"
        let x = 5 * 2;  // 10
        let y = x / 3;  // 3
        let z = x % 4;  // 2
    "#);

    assert_eq!(Val::Num(10), ctx.scope["x"]);
    assert_eq!(Val::Num(3), ctx.scope["y"]);
    assert_eq!(Val::Num(2), ctx.scope["z"]);
}

