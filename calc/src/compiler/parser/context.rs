
use compiler::def::*;


pub struct ParseContext<'a> {
	tokens: Vec<Token<'a>>,
	offset: usize,	// cur token index
}


impl<'a> Location<'a> for ParseContext<'a> {
    fn loc(&self) -> Loc<'a> {
        self.tokens[self.offset].loc()
    }
}


type MatchResult<'a> = Result<(), ParseError<'a>>;


impl<'a> ParseContext<'a> {
	pub fn new(tokens: Vec<Token<'a>>) -> Self {
		ParseContext {
			tokens,
			offset: 0,
		}
	}

	pub fn token(&self) -> &Token<'a> {
		&self.tokens[self.offset]
	}

	pub fn error_unexpected_token<T>(&self) -> Result<T, ParseError<'a>> {
		self.error(format!("'{}': unexpected token", self.token()))
	}

	pub fn match_eof(&self) -> MatchResult<'a> {
		if let Token::Eof(_loc) = *self.token() {
			// don't increment offset since it's the end

			Ok(())
		} else {
			self.error_unexpected_token()
		}
	}

	pub fn match_symbol(&mut self, expected: char) -> MatchResult<'a> {
		if let Token::Symbol(_loc, ch) = *self.token() {
			if ch == expected {
				self.match_any();
				return Ok(());
			}
		} 

		self.error(format!("'{}': unexpected token, expected symbol '{}'", self.token(), expected))
	}

	pub fn match_any(&mut self) {
		if let Token::Eof(_loc) = *self.token() {
			panic!("Trying to match Eof");
		}

		debug_assert!(self.offset < self.tokens.len());
		self.offset += 1;
	}
}
