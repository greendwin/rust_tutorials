use compiler::*;
use self::AST::*;


pub fn parse_expr<'a>(ctx: &mut ParseContext<'a>) -> ParseResult {
	parse_expr_add(ctx)
}


fn parse_expr_add<'a>(ctx: &mut ParseContext<'a>) -> ParseResult {
	let mut left = parse_expr_mul(ctx)?;

	while let Token::Symbol(_, op) = *ctx.token() {
		if op != '+' && op != '-' {
			break;
		}

        let loc = ctx.match_any();
		let right = parse_expr_mul(ctx)?;

		left = BinOp {
			loc: loc.clone(),
            op,
			left: Box::new(left),
			right: Box::new(right),
		};
	}

	return Ok(left);
}


fn parse_expr_mul<'a>(ctx: &mut ParseContext<'a>) -> ParseResult {
	let mut left = parse_val(ctx)?;

	while let Token::Symbol(_, op) = *ctx.token() {
		if op != '*' && op != '/' && op != '%' {
			break;
		}

        let loc = ctx.match_any();
		let right = parse_val(ctx)?;

		left = BinOp {
			loc, op,
			left: Box::new(left),
			right: Box::new(right),
		};
	}

	return Ok(left);
}


fn parse_val<'a>(ctx: &mut ParseContext<'a>) -> ParseResult {
	match *ctx.token() {
		Token::Ident(_, name) => {
            if ctx.get_next().is_symbol('(') {
                return parse_func_call(ctx);
            }

            let loc = ctx.match_any();

			Ok(Var{
               loc,
               name: String::from(name),
            })
		}

		Token::Int(_, val) => {
            let loc = ctx.match_any();

			Ok(Num{loc, val})
		}

        Token::Str(_, val) => {
            let loc = ctx.match_any();

			Ok(Str{
                loc,
                val: literal_to_string(val),
            })
        }

		Token::Symbol(_, '(') => {
			ctx.match_any();
			let expr = parse_expr(ctx)?;
			ctx.match_symbol(')')?;
			Ok(expr)
		}

		_ => ctx.error_unexpected_token(),
	}
}


fn literal_to_string(literal: &str) -> String {
    debug_assert!(literal.len() >= 2);
    let mut r = String::with_capacity(literal.len());
    let mut escape = false;

    for ch in literal[1..literal.len()-1].chars() {
        if escape {
            let ch = match ch {
                '\\' => '\\',
                '\"' => '\"',
                'n'  => '\n',
                't'  => '\t',
                _    => panic!("unexpected escape sequence: '\\{}'", ch),
            };

            r.push(ch);
            escape = false;
            continue;
        }

        if ch == '\\' {
            escape = true;
            continue;
        }

        r.push(ch);
    }

    return r;
}


fn parse_func_call<'a>(ctx: &mut ParseContext<'a>) -> ParseResult {
    let (loc, name) = ctx.match_ident()?;
    let mut args: Vec<AST> = Vec::new();

    ctx.match_symbol('(')?;

    if !ctx.token().is_symbol(')') {
        loop {
            let expr = parse_expr(ctx)?;
            args.push(expr);

            if !ctx.token().is_symbol(',') {
                break;
            }
            ctx.match_symbol(',')?;
        }
    }

    ctx.match_symbol(')')?;

    Ok(Call{
       loc,
       name: String::from(name),
       args,
    })
}

