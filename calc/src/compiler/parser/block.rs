use compiler::*;
use super::expr::parse_expr;
use self::AST::*;


pub fn parse_block<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
    let loc = ctx.loc();    // store block beginning
    let mut lst: Vec<AST<'a>> = Vec::new();

    while let Token::Ident(_loc, name) = *ctx.token() {
        let st = match name {
            "let"    => parse_st_let(ctx)?,
            "return" => parse_st_return(ctx)?,
            "fn"     => parse_st_fn(ctx)?,
            _        => parse_st_assign(ctx)?,
        };

        lst.push(st);
    }

    Ok(Block(loc, lst))
}


fn parse_st_let<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
    // let <ident> = <expr> ;
    ctx.match_keyword("let")?;
    let (loc, name) = ctx.match_ident()?;

    ctx.match_symbol('=')?;
    let expr = parse_expr(ctx)?;

    ctx.match_symbol(';')?;

    Ok(DeclVar {
        loc, name,
        init: Box::new(expr),
    }) 
}


fn parse_st_return<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
    // return <expr> ;
    let loc = ctx.match_keyword("return")?;
    let expr = parse_expr(ctx)?;

    ctx.match_symbol(';')?;

    Ok(Return(loc, Box::new(expr)))
}


fn parse_st_assign<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
    // <ident> = <expr> ;
    let (loc, name) = ctx.match_ident()?;

    ctx.match_symbol('=')?;
    let expr = parse_expr(ctx)?;

    ctx.match_symbol(';')?;

    Ok(Assign {
        loc, name,
        init: Box::new(expr),
    }) 
}


fn parse_st_fn<'a>(ctx: &mut ParseContext<'a>) -> ParseResult<'a> {
    // fn <name> ( <arg1>, <arg2>, ... ) { <block> }
    ctx.match_keyword("fn")?;
    let (loc, name) = ctx.match_ident()?;
    let mut args: Vec<&'a str> = Vec::new();

    ctx.match_symbol('(')?;
    if !ctx.token().is_symbol(')') {
        loop {
            let (_, arg_name) = ctx.match_ident()?;
            args.push(arg_name);

            if !ctx.token().is_symbol(',') {
                break;
            }
            ctx.match_symbol(',')?;
        }
    }
    ctx.match_symbol(')')?;

    ctx.match_symbol('{')?;
    let body = parse_block(ctx)?;
    ctx.match_symbol('}')?;

    Ok(Func{
        loc, name, args,
        body: Box::new(body),
    })
}

