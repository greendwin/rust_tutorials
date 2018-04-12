
use compiler::def::*;
use compiler::error::*;


fn tokenize<'a>(text: &'a str, filename: &'a str) -> Result<Vec<Token<'a>>, ParseError<'a>> {
	use self::Token::*;

	let mut r: Vec<Token> = Vec::new();
	let mut loc = Loc {
		filename: filename,
		line: 1,
	};

	const EOF: char = '\0';

	enum State {
		Default,
		Int,
	}
	let mut state = State::Default;
	let mut cur_int = 0;

	for ch in text.chars().chain(vec![EOF]) {
		match state {
			State::Default => {
				if ch.is_whitespace() {
					if ch == '\n' {
						loc.line += 1;
					}
					continue;
				}

				if ch.is_digit(10) {
					state = State::Int;
					cur_int = ch.to_digit(10).unwrap() as i32;
					continue;
				}

				match ch {
					'(' => r.push(LeftBr(loc)),
					')' => r.push(RightBr(loc)),
					'+' => r.push(Plus(loc)),
					'-' => r.push(Minus(loc)),
					'*' => r.push(Mul(loc)),
					'/' => r.push(Div(loc)),
					'%' => r.push(Mod(loc)),
					EOF => return Ok(r),
					_ => return loc.error(format!("'{}': unexpected symbol", ch)),
				}
			}

			State::Int => {
				if ch.is_digit(10) {
					cur_int *= 10;
					cur_int += ch.to_digit(10).unwrap() as i32;
					continue;
				}

				r.push(Int(loc, cur_int));
				state = State::Default;

				if ch.is_whitespace() {
					if ch == '\n' {
						loc.line += 1;
					}
					continue;
				}

				if ch == EOF {
					return Ok(r);
				}

				return loc.error(format!("'{}': invalid symbol in integer constant", ch));
			}
		}
	}

	return Ok(r);
}


#[cfg(test)]
mod tests {
	use super::*;
	use compiler::def::Token::*;

	const FILENAME: &str = "<test>";

	fn loc(line: i32) -> Loc<'static> {
		Loc {
			filename: FILENAME,
			line: line,
		}
	}

	fn tokenize(text: &str) -> Vec<Token> {
		super::tokenize(text, FILENAME)
			.expect(&format!("text should compile: '{}'", text))
	}

	#[test]
	fn empty_stream() {
		let tkn = tokenize("");

		assert_eq!(Vec::<Token>::new(), tkn);
	}

	#[test]
	fn parse_int() {
		let tkn = tokenize("42");

		assert_eq!(vec![Int(loc(1), 42)], tkn);
	}

	#[test]
	fn calc_lines() {
		let tkn = tokenize(r#"
			4
			2
		"#);

		assert_eq!(vec![
			Int(loc(2), 4),
			Int(loc(3), 2),
		], tkn);
	}

	#[test]
	fn breackets_and_ops() {
		let tkn = tokenize(r#"(+-/%*)"#);

		assert_eq!(vec![
			LeftBr(loc(1)),
			Plus(loc(1)),
			Minus(loc(1)),
			Div(loc(1)),
			Mod(loc(1)),
			Mul(loc(1)),
			RightBr(loc(1)),
		], tkn);
	}

	// TODO: indentifier
}