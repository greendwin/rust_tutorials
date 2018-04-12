
use std::str::Chars;

use compiler::def::*;
use compiler::error::*;


struct TokenizerContext<'a> {
    stream: Chars<'a>,
    cur_text: &'a str,  // rest of the text beginning from cur
    cur: Option<char>,
    loc: Loc<'a>,
}


impl<'a> TokenizerContext<'a> {
    fn new(text: &'a str, filename: &'a str) -> TokenizerContext<'a> {
        TokenizerContext {
            stream: text.chars(),
            cur_text: text,
            cur: None,
            loc: Loc {
                filename: filename,
                line: 1,
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        // update cur text (since cur symbol already cropped from the stream)
        self.cur_text = self.stream.as_str();
        self.cur = self.stream.next();

        if let Some(ch) = self.cur {
            if ch == '\n' {
                self.loc.line += 1;
            }
        }

        return self.cur;
    }

    pub fn error<T>(&self, description: String) -> ParseResult<'a, T> {
        Err(ParseError::new(
            self.loc.filename,
            self.loc.line,
            description))
    }
}


fn parse_int<'a>(ctx: &mut TokenizerContext<'a>) -> Token<'a> {
    let loc = ctx.loc;
    let mut r = 0;
    
    while let Some(ch) = ctx.cur {
        if !ch.is_digit(10) {
            break;
        }

        r *= 10;
        r += ch.to_digit(10).unwrap() as i32;

        ctx.next();
    }

    Token::Int(loc, r)
}


fn is_word_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}


fn parse_ident<'a>(ctx: &mut TokenizerContext<'a>) -> Token<'a> {
    let loc = ctx.loc;
    let text = ctx.cur_text;

    assert!(is_word_start(ctx.cur.unwrap()));
    let mut len = 1;
    ctx.next();
    
    while let Some(ch) = ctx.cur {
        if !is_word_start(ch) && !ch.is_digit(10) {
            break;
        }

        len += 1;
        ctx.next();
    }

    Token::Ident(loc, &text[..len])
}


pub fn tokenize<'a>(text: &'a str, filename: &'a str) -> ParseResult<'a, Vec<Token<'a>>> {
    use self::Token::*;

    let mut ctx = TokenizerContext::new(text, filename);
    let mut r: Vec<Token> = Vec::new();
    
    // pop first char
    ctx.next();

    while let Some(ch) = ctx.cur {
        if ch.is_whitespace() {
            ctx.next();
            continue;
        }

        if ch.is_digit(10) {
            r.push(parse_int(&mut ctx));
            continue;
        }

        if is_word_start(ch) {
           r.push(parse_ident(&mut ctx));
           continue;
        }

        match ch {
            '(' | ')' | '[' | ']' |
            ':' | '?' | ',' | '.' | 
            '+' | '-' | '*' | '/' |
            '%' | '!' | '~' => {
                r.push(Symbol(ctx.loc, ch));
                ctx.next();
            }

            _ => {
                return ctx.error(format!("'{}': unexpected symbol", ch));
            }
        }
    }

    r.push(Eof); // finish this stream by Eof token
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

        assert_eq!(vec![Eof], tkn);
    }

    #[test]
    fn parse_int() {
        let tkn = tokenize("42");

        assert_eq!(vec![Int(loc(1), 42), Eof], tkn);
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
            Eof,
        ], tkn);
    }

    #[test]
    fn breackets_and_ops() {
        let symbols = "(+-/%*)";
        let tkn = tokenize(symbols);

        let expected: Vec<Token<'static>> = symbols.chars().map(|ch| {
            Symbol(loc(1), ch)
        }).chain(vec![Eof]).collect();

        assert_eq!(expected, tkn);
    }

    #[test]
    fn identifier() {
        let tkn = tokenize("x");

        assert_eq!(vec![Ident(loc(1), "x"), Eof], tkn);
    }
}
