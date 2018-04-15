
use compiler;
use compiler::*;


fn execute_it_with<'a>(ctx: &mut ExecContext<'a>, text: &'a str) {
    let prog = compiler::parse(text, "<test>")
        .expect("parse should not fail");

    compiler::execute(&prog, ctx)
        .expect("execution should not fail");
}


fn execute_it<'a>(text: &'a str) -> ExecContext {
    let mut ctx = ExecContext::new();
    execute_it_with(&mut ctx, text);

    // return context so it could be checked for results
    return ctx;
}


#[test]
fn empty_program() {
    let ctx = execute_it("");

    assert!(ctx.scope.is_empty());
}


#[test]
fn decl_var() {
    let ctx = execute_it(r#"
        let x = 42;
    "#);

    assert_eq!(Val::Num(42), ctx.scope["x"]);
}


#[test]
fn assign_val() {
    let mut ctx = ExecContext::new();
    ctx.scope.insert("x", Val::None);

    execute_it_with(&mut ctx, r#"
        x = 42;
    "#);

    assert_eq!(Val::Num(42), ctx.scope["x"]);
}

