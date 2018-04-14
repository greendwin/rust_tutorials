
mod def;
mod tokenizer;
mod parser;

#[cfg(test)]
mod tests;

use compiler::parser::context::ParseContext;


// pub type AST<'a> = def::AST<'a>;
pub type ParseResult<'a> = def::ParseResult<'a>;


pub fn parse_expr<'a>(text: &'a str, filename: &'a str) -> ParseResult<'a> {
	let tokens = tokenizer::tokenize(text, filename)?;
	let mut ctx = ParseContext::new(tokens);

	// parse expression
	let expr = parser::expr::parse_expr(&mut ctx)?;

	// make sure there is no data left
	ctx.match_eof()?;

	Ok(expr)
}


pub fn parse<'a>(text: &'a str, filename: &'a str) -> ParseResult<'a> {
	let tokens = tokenizer::tokenize(text, filename)?;
	let mut ctx = ParseContext::new(tokens);

	// parse expression
	let expr = parser::block::parse_block(&mut ctx)?;

	// make sure there is no data left
	ctx.match_eof()?;

	Ok(expr)
}

