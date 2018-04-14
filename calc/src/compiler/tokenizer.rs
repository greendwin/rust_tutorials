use std::str::Chars;
use compiler::*;
use self::Token::*;


struct TokenizeContext<'a> {
    stream: Chars<'a>,
    cur_text: &'a str,  // rest of the text beginning from cur
    cur: Option<char>,
    loc: Loc<'a>,
}


impl<'a> TokenizeContext<'a> {
    fn new(text: &'a str, filename: &'a str) -> Self {
        TokenizeContext {
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
}


impl<'a> Location<'a> for TokenizeContext<'a> {
    fn loc(&self) -> Loc<'a> {
        self.loc
    }
}


fn parse_int<'a>(ctx: &mut TokenizeContext<'a>) -> Token<'a> {
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

    Int(loc, r)
}


fn is_word_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}


fn parse_ident<'a>(ctx: &mut TokenizeContext<'a>) -> Token<'a> {
    let loc = ctx.loc;
    let text = ctx.cur_text;

    debug_assert!(is_word_start(ctx.cur.unwrap()));
    let mut len = 1;
    ctx.next();
    
    while let Some(ch) = ctx.cur {
        if !is_word_start(ch) && !ch.is_digit(10) {
            break;
        }

        len += 1;
        ctx.next();
    }

    Ident(loc, &text[..len])
}


pub fn tokenize<'a>(text: &'a str, filename: &'a str) -> TokenizeResult<'a> {
    let mut ctx = TokenizeContext::new(text, filename);
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
            '(' | ')' | '[' | ']' | '{' | '}' |
            ':' | '?' | ',' | '.' | ';' | '=' |
            '+' | '-' | '*' | '/' | '%' |
            '!' | '~' | '|' | '&' => {
                r.push(Symbol(ctx.loc, ch));
                ctx.next();
            }

            _ => {
                return ctx.error(format!("'{}': unexpected symbol", ch));
            }
        }
    }

    r.push(Eof(ctx.loc)); // finish this stream by Eof token
    return Ok(r);
}

