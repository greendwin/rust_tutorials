use compiler::*;
use compiler::tokenizer;
use self::Token::*;


const FILENAME: &str = "<test>";


fn loc(line: i32) -> Loc {
    Loc::new(FILENAME, line)
}


fn tokenize(text: &str) -> Vec<Token> {
    tokenizer::tokenize(text, FILENAME)
        .expect("text should compile")
}


#[test]
fn empty_stream() {
    let tkn = tokenize("");

    assert_eq!(vec![Eof(loc(1))], tkn);
}


#[test]
fn parse_int() {
    let tkn = tokenize("42");

    assert_eq!(vec![Int(loc(1), 42), Eof(loc(1))], tkn);
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
        Eof(loc(4)),
    ], tkn);
}


#[test]
fn breackets_and_ops() {
    let symbols = "{(+-/%*&|=)};";
    let tkn = tokenize(symbols);

    let expected: Vec<Token<'static>> = symbols.chars().map(|ch| {
        Symbol(loc(1), ch)
    }).chain(vec![Eof(loc(1))]).collect();

    assert_eq!(expected, tkn);
}


#[test]
fn parse_ident() {
    let tkn = tokenize("x");

    assert_eq!(vec![Ident(loc(1), "x"), Eof(loc(1))], tkn);
}


#[test]
fn skip_comments() {
    let symbols = r#"
        abc // skip
        def
    "#;
    let tkn = tokenize(symbols);

    let expected = vec![
        Ident(loc(2), "abc"),
        Ident(loc(3), "def"),
        Eof(loc(4)),
    ];

    assert_eq!(expected, tkn);
}

