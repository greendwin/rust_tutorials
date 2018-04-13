
mod def;
mod tokenizer;
mod parser;

#[cfg(test)]
mod tests;


// pub type AST<'a> = def::AST<'a>;
pub type ParseResult<'a> = def::ParseResult<'a>;


pub fn parse_expr<'a>(text: &'a str, filename: &'a str) -> ParseResult<'a> {
	let tokens = tokenizer::tokenize(text, filename)?;
	let mut ctx = parser::context::ParseContext::new(tokens);

	// parse expression
	let expr = parser::expr::parse_expr(&mut ctx);

	// make sure there is no data left
	ctx.match_eof()?;

	return expr;
}
