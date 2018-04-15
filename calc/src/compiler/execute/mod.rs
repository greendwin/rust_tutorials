
pub mod context;

use compiler::*;
use self::AST::*;


pub fn execute<'a>(expr: &AST<'a>, ctx: &mut ExecContext<'a>) -> ExecResult<'a> {
    match *expr {
        Block{ ref body, .. } => {
            for expr in body {
                execute(expr, ctx)?;
            }
        }

        DeclVar{ name, ref init, .. } => {
            let init = execute(init, ctx)?;

            if ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': redefinition", name));
            }

            ctx.scope.insert(name, init);
        }

        Assign{ name, ref init, .. } => {
            let init = execute(init, ctx)?;

            if !ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': undeclared variable", name));
            }

            ctx.scope.insert(name, init);
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

        Var{ name, .. } => {
            if !ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': undeclared variable", name));
            }

            return Ok(ctx.scope[name]);
        }

        _ => {
            return expr.error(format!("AST node not implemented: {:?}", expr));
        }
    }

    Ok(Val::None)
}


fn exec_numeric_op<'a>(op: char, left: Val, right: Val) -> ExecResult<'a> {
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

