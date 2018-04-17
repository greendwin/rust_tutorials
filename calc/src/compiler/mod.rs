
mod location;
mod error;
mod token;
mod ast;
mod value;
mod scope;

mod tokenizer;
mod parser;
mod execute;

#[cfg(test)]
mod tests;


pub use self::location::Loc;
pub use self::location::Location;
pub use self::token::Token;
pub use self::ast::AST;
pub use self::ast::FuncDecl;

pub use self::scope::Scope;
pub use self::value::Val;

pub use self::parser::context::ParseContext;
pub use self::execute::context::ExecContext;

pub use self::error::Error;

pub type TokenizeResult<'a> = Result<Vec<Token<'a>>, Error>;
pub type ParseResult = Result<AST, Error>;

pub use self::execute::result::FlowExc;
pub use self::execute::result::ExecResult;


pub fn parse_expr(text: &str, filename: &str) -> ParseResult {
    use compiler::parser::context::ParseContext;

	let tokens = tokenizer::tokenize(text, filename)?;
	let mut ctx = ParseContext::new(tokens);

	// parse expression
	let expr = parser::expr::parse_expr(&mut ctx)?;

	// make sure there is no data left
	ctx.match_eof()?;

	Ok(expr)
}


pub fn parse(text: &str, filename: &str) -> ParseResult {
	let tokens = tokenizer::tokenize(text, filename)?;
	let mut ctx = ParseContext::new(tokens);

	// parse expression
	let expr = parser::block::parse_block(&mut ctx)?;

	// make sure there is no data left
	ctx.match_eof()?;

	Ok(expr)
}


pub use self::execute::execute;

