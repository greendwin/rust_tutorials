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


// TODO: misplaced: return, break, continue
// TODO: scopes
