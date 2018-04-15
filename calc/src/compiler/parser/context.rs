use compiler::*;


pub struct ParseContext<'a> {
	tokens: Vec<Token<'a>>,
	offset: usize,	// cur token index
}


impl<'a> Location for ParseContext<'a> {
    fn loc(&self) -> &Loc {
        self.tokens[self.offset].loc()
    }
}


type MatchResult<'a, T=()> = Result<T, Error>;


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

    pub fn error_unexpected_token<T>(&self) -> Result<T, Error> {
        if let Token::Eof(_) = *self.token() {
		    self.error_str("unexpected end of file")
        } else {
    		self.error(format!("'{}': unexpected token", self.token()))
        }
	}

	pub fn match_eof(&self) -> MatchResult<'a> {
		if let Token::Eof(_) = *self.token() {
			// don't increment offset since it's the end

			Ok(())
		} else {
			self.error_unexpected_token()
		}
	}

	pub fn match_symbol(&mut self, expected: char) -> MatchResult<'a> {
		if self.token().is_symbol(expected) {
			self.match_any();
			return Ok(());
		} 

		self.error(format!("'{}': unexpected token, expected symbol '{}'", self.token(), expected))
	}

    pub fn match_ident(&mut self) -> MatchResult<'a, (Loc, &'a str)> {
        if let Token::Ident(_, name) = *self.token() {
            let loc = self.loc().clone();
            self.match_any();

            Ok((loc, name))
        } else {
		    self.error(format!("'{}': identifier expected", self.token()))
        }
    }

    pub fn match_keyword(&mut self, expected: &str) -> MatchResult<'a, Loc> {
        if let Token::Ident(_, name) = *self.token() {
            if name == expected {
                let loc = self.loc().clone();
                self.match_any();

                return Ok(loc);
            }
        }
        
        self.error(format!("'{}': expected keyword '{}'", self.token(), expected))
}

	pub fn match_any(&mut self) {
		if let Token::Eof(_) = *self.token() {
			panic!("Trying to match Eof");
		}

		debug_assert!(self.offset < self.tokens.len());
		self.offset += 1;
	}
}

