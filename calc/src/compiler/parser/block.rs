use std::rc::Rc;
use compiler::*;
use super::expr::parse_expr;
use self::AST::*;


pub fn parse_block(ctx: &mut ParseContext) -> ParseResult {
    let loc = ctx.loc().clone();    // store block beginning
    let mut body: Vec<AST> = Vec::new();

    while let Token::Ident(_, name) = *ctx.token() {
        let st = match name {
            "let"    => parse_st_let(ctx)?,
            "return" => parse_st_return(ctx)?,
            "fn"     => parse_st_fn(ctx)?,
            _        => {
                if ctx.get_next().is_symbol('=') {
                    parse_st_assign(ctx)?
                } else {
                    let r = parse_expr(ctx)?;
                    ctx.match_symbol(';')?;
                    r
                }
            }
        };

        body.push(st);
    }

    Ok(Block{
        loc: loc.clone(),
        body,
    })
}


fn parse_st_let(ctx: &mut ParseContext) -> ParseResult {
    // let <ident> = <expr> ;
    ctx.match_keyword("let")?;
    let (loc, name) = ctx.match_ident()?;

    ctx.match_symbol('=')?;
    let expr = parse_expr(ctx)?;

    ctx.match_symbol(';')?;

    Ok(DeclVar {
        loc,
        name: String::from(name),
        init: Box::new(expr),
    }) 
}


fn parse_st_return(ctx: &mut ParseContext) -> ParseResult {
    // return <expr> ;
    let loc = ctx.match_keyword("return")?;
    let expr = parse_expr(ctx)?;

    ctx.match_symbol(';')?;

    Ok(Return{ loc, ret: Box::new(expr) })
}


fn parse_st_assign(ctx: &mut ParseContext) -> ParseResult {
    // <ident> = <expr> ;
    let (loc, name) = ctx.match_ident()?;

    ctx.match_symbol('=')?;
    let expr = parse_expr(ctx)?;

    ctx.match_symbol(';')?;

    Ok(Assign {
        loc,
        name: String::from(name),
        init: Box::new(expr),
    }) 
}


fn parse_st_fn(ctx: &mut ParseContext) -> ParseResult {
    // fn <name> ( <arg1>, <arg2>, ... ) { <block> }
    ctx.match_keyword("fn")?;
    let (loc, name) = ctx.match_ident()?;
    let mut args: Vec<String> = Vec::new();

    ctx.match_symbol('(')?;
    if !ctx.token().is_symbol(')') {
        loop {
            let (_, arg_name) = ctx.match_ident()?;
            args.push(String::from(arg_name));

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
        loc,
        decl: Rc::new(FuncDecl {
            name: String::from(name),
            args, body,
        }),
    })
}

