
pub mod context;
pub mod result;

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
            if ctx.has_var(&decl.name) {
                return expr.error(format!("'{}': redefinition", &decl.name)).into();
            }

            ctx.set_var(&decl.name, Val::Func(Rc::clone(decl)));
        }

        DeclVar{ ref name, ref init, .. } => {
            let init = execute(ctx, init)?;

            if ctx.has_var(name) {
                return expr.error(format!("'{}': redefinition", name)).into();
            }

            ctx.set_var(name, init);
        }

        Assign{ ref name, ref init, .. } => {
            let init = execute(ctx, init)?;

            if !ctx.has_var(name) {
                return expr.error(format!("'{}': undeclared variable", name)).into();
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
                op, left, right)).into();
        }

        Num{ val, .. } => {
            return Ok(Val::Num(val));
        }

        Var{ ref name, .. } => {
            if let Some(val) = ctx.lookup_name(name) {
                return Ok(val.clone());
            }

            return expr.error(format!("'{}': undeclared variable", name)).into();
        }

        Call{ ref name, ref args, ref loc, .. } => {
            let mut values: Vec<Val> = Vec::new();
            for p in args {
                values.push(execute(ctx, p)?);
            }

            return exec_func_call(ctx, loc, name, values);
        }

        Return{ ref ret, .. } => {
            let ret = execute(ctx, ret)?;

            if !ctx.allow_return {
                return expr.error(format!("unexpected 'return' statement")).into();
            }

            return Err(FlowExc::Return(ret));
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
    let func_val = match ctx.lookup_name(name) {
        Some(val) => val,
        None => return loc.error(format!("'{}': undeclared function name", name)).into(),
    };

    match *func_val {
        Val::Func(ref decl) => {
            let mut ctx = ctx.new_nested(); // override it with nested context

            // allow return in nested context
            ctx.allow_return = true;

            if decl.args.len() != args.len() {
                return loc.error(
                    format!("wrong arguments count, expected {} args",
                            decl.args.len())).into();
            }

            for (arg_name, arg_val) in decl.args.iter().zip(args) {
                ctx.set_var(arg_name, arg_val);
            }

            let r = execute(&mut ctx, &decl.body);

            return match r {
                Err(FlowExc::Return(val)) => Ok(val),
                _ => r,
            };
        }

        Val::NativeFunc(ref decl) => {
            let r = (decl.callback)(args);
            return Ok(r);
        }

        _ => {
            loc.error(format!(
                "'{}': type mismatch, function expected, found '{}'",
                name, func_val)).into()
        }
    }
}


