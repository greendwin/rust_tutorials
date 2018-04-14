
use compiler;
//use compiler::def::*;
use compiler::execute::context::*;

//use compiler::tests::checks::*;


fn execute_it<'a>(text: &'a str) -> ExecContext {
    let prog = compiler::parse(text, "<test>")
        .expect("parse should not fail");

    let mut ctx = ExecContext::new();
    compiler::execute(&prog, &mut ctx)
        .expect("execution should not fail");

    // return context so it could be checked for results
    return ctx;
}


#[test]
fn empty_program() {
    let ctx = execute_it("");

    assert!(ctx.scope.is_empty());
}

