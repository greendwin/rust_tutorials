
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
            // TODO: error on redefine
            let init = execute(init, ctx)?;

            ctx.scope.insert(name, init);
        }

        Assign{ name, ref init, .. } => {
            // TODO: error on undefined
            let init = execute(init, ctx)?;

            ctx.scope.insert(name, init);
        }

        Num{ val, .. } => {
            return Ok(Val::Num(val));
        }

        _ => {
            return expr.error(format!("AST node not implemented: {:?}", expr));
        }
    }

    Ok(Val::None)
}

