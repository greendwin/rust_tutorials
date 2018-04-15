
pub mod context;

use std::rc::Rc;
use compiler::*;
use self::AST::*;


pub fn execute(expr: &AST, ctx: &mut ExecContext) -> ExecResult {
    match *expr {
        Block{ ref body, .. } => {
            for expr in body {
                execute(expr, ctx)?;
            }
        }

        Func{ ref decl, .. } => {
            if ctx.scope.contains_key(&decl.name) {
                return expr.error(format!("'{}': redefinition", &decl.name));
            }

            ctx.set_var(&decl.name, Val::Func(Rc::clone(decl)));
        }

        DeclVar{ ref name, ref init, .. } => {
            let init = execute(init, ctx)?;

            if ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': redefinition", name));
            }

            ctx.set_var(name, init);
        }

        Assign{ ref name, ref init, .. } => {
            let init = execute(init, ctx)?;

            if !ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': undeclared variable", name));
            }

            ctx.set_var(name, init);
        }

        BinOp{ op, ref left, ref right, .. } => {
            let left = execute(left, ctx)?;
            let right = execute(right, ctx)?;

            if left.is_num() && right.is_num() {
                return exec_numeric_op(op, left, right);
            }

            // TODO: mismatched types
        }

        Num{ val, .. } => {
            return Ok(Val::Num(val));
        }

        Var{ ref name, .. } => {
            if !ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': undeclared variable", name));
            }

            return Ok(ctx.scope[name].clone());
        }

        _ => {
            return expr.error(format!("AST node not implemented: {:?}", expr));
        }
    }

    Ok(Val::None)
}


fn exec_numeric_op(op: char, left: Val, right: Val) -> ExecResult {
    let left = left.as_num().unwrap();
    let right = right.as_num().unwrap();

    let r = match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        '%' => left % right,

        _ => panic!("binary op '{}' not implemented", op),
    };

    Ok(Val::Num(r))
}

