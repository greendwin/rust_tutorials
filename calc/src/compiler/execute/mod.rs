
pub mod context;

use std::rc::Rc;
use compiler::*;
use self::AST::*;


pub fn execute(ctx: &mut ExecContext, expr: &AST) -> ExecResult {
    match *expr {
        Block{ ref body, .. } => {
            for expr in body {
                execute(ctx, expr)?;
            }
        }

        Func{ ref decl, .. } => {
            if ctx.scope.contains_key(&decl.name) {
                return expr.error(format!("'{}': redefinition", &decl.name));
            }

            ctx.set_var(&decl.name, Val::Func(Rc::clone(decl)));
        }

        DeclVar{ ref name, ref init, .. } => {
            let init = execute(ctx, init)?;

            if ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': redefinition", name));
            }

            ctx.set_var(name, init);
        }

        Assign{ ref name, ref init, .. } => {
            let init = execute(ctx, init)?;

            if !ctx.scope.contains_key(name) {
                return expr.error(format!("'{}': undeclared variable", name));
            }

            ctx.set_var(name, init);
        }

        BinOp{ op, ref left, ref right, .. } => {
            let left = execute(ctx, left)?;
            let right = execute(ctx, right)?;

            if left.is_num() && right.is_num() {
                return exec_numeric_op(op, left, right);
            }

            return expr.error(format!(
                "'{}': type mismatch, numbers expected (found: '{}' and '{}')",
                op, left, right));
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

        Call{ ref name, ref args, ref loc, .. } => {
            let mut values: Vec<Val> = Vec::new();
            for p in args {
                values.push(execute(ctx, p)?);
            }

            return exec_func_call(ctx, loc, name, values);
        }

        _ => {
            return expr.error(format!("not implemented: {:#?}", expr));
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


fn exec_func_call(ctx: &mut ExecContext, loc: &Loc, name: &String, args: Vec<Val>) -> ExecResult {
    if !ctx.scope.contains_key(name) {
        return loc.error(format!("'{}': undeclared function name", name));
    }

    let func_val = &ctx.scope[name];
    if let Val::Func(ref decl) = *func_val {
        let mut ctx = ctx.new_nested(); // override it with nested context

        for (arg_name, arg_val) in decl.args.iter().zip(args) {
            ctx.set_var(arg_name, arg_val);
        }

        execute(&mut ctx, &decl.body)
    } else {
        loc.error(format!(
            "'{}': type mismatch, function expected: {}",
            name, func_val))
    }
}


