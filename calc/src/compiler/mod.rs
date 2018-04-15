
mod location;
mod error;
mod token;
mod ast;
mod value;

mod tokenizer;
mod parser;
mod execute;

#[cfg(test)]
mod tests;


pub use self::location::Loc;
pub use self::location::Location;
pub use self::token::Token;
pub use self::ast::AST;
pub use self::value::Val;

pub use self::parser::context::ParseContext;
pub use self::execute::context::ExecContext;

pub use self::error::Error;

pub type TokenizeResult<'a> = Result<Vec<Token<'a>>, Error<'a>>;
pub type ParseResult<'a> = Result<AST<'a>, Error<'a>>;
pub type ExecResult<'a> = Result<Val, Error<'a>>;


pub fn parse_expr<'a>(text: &'a str, filename: &'a str) -> ParseResult<'a> {
    use compiler::parser::context::ParseContext;

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


pub use self::execute::execute;

