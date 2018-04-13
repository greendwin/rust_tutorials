
use compiler::def::*;
use compiler::def::AST::*;

use compiler::parser::context::*;


pub fn parse_expr<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
	parse_expr_add(ctx)
}


fn parse_expr_add<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
	let mut left = parse_expr_mul(ctx)?;

	while let Token::Symbol(loc, op) = *ctx.token() {
		if op != '+' && op != '-' {
			break;
		}

		ctx.match_any();
		let right = parse_expr_mul(ctx)?;

		left = BinOp {
			loc, op,
			left: Box::new(left),
			right: Box::new(right),
		};
	}

	return Ok(left);
}


fn parse_expr_mul<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
	let mut left = parse_val(ctx)?;

	while let Token::Symbol(loc, op) = *ctx.token() {
		if op != '*' && op != '/' && op != '%' {
			break;
		}

		ctx.match_any();
		let right = parse_val(ctx)?;

		left = BinOp {
			loc, op,
			left: Box::new(left),
			right: Box::new(right),
		};
	}

	return Ok(left);
}


fn parse_val<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
	match *ctx.token() {
		Token::Ident(loc, name) => {
			ctx.match_any();

			Ok(Var(loc, name))
		}

		Token::Int(loc, val) => {
			ctx.match_any();

			Ok(Num(loc, val))
		}

		Token::Symbol(_loc, '(') => {
			ctx.match_any();
			let expr = parse_expr(ctx)?;
			ctx.match_symbol(')')?;
			Ok(expr)
		}

		_ => ctx.error_unexpected_token(),
	}
}
