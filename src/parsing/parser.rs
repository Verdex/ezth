

use crate::data::lex::*;
use crate::data::parse::*;
use super::parse_input::Input;
use super::lexer;

/*
    def sym = expr

    match sym = ?

    pat sym = ? 


    Expr
    fun (sym_list) => expr
    sym
    expr.sym
    expr |> call_expr
    expr |index_pattern> call_expr
    (expr)
    expr(expr_list)
    let sym = expr in expr

    Pattern
    data (exact)
    _
    a
    $a
    cons(pattern_list)
    and
    or
    [| pattern_list |]
    {| next_pattern_list |}
    predicate
    match_with ?

    IndexPattern
    $0 (or something)

    : for data

*/


pub fn parse(input : &str) -> Result<ReplTopLevel, ParseError> {
    let input = match lexer::lex(input) {
        Err(i) => { return Err(ParseError::Lex(i)); },
        Ok(ls) => ls,
    };
    let mut input = Input::new(input);


    let result = 
    if input.check(|l| l.eq(&Lexeme::Def))? {
        ReplTopLevel::Def(parse_def(&mut input)?)
    }
    else if input.check(|l| l.eq(&Lexeme::Pat))? {
        ReplTopLevel::Pat(parse_pat(&mut input)?)
    }
    else {
        ReplTopLevel::Expr(parse_expr(&mut input)?)
    };

    Ok(result)
}

fn parse_def(input : &mut Input) -> Result<Def, ParseError> {
    let name = input.expect(|l| matches!(l, Lexeme::Symbol(_)))?.value();
    input.expect(|l| l.eq(&Lexeme::LParen))?;
    let params = parse_list(input, |input| input.expect(|l| matches!(l, Lexeme::Symbol(_))), Lexeme::RParen)?
                    .into_iter()
                    .map(|s| s.value())
                    .collect::<Vec<_>>();
    input.expect(|l| l.eq(&Lexeme::LCurl))?;
    let stmts = parse_stmts(input)?; 
    let body = parse_expr(input)?;
    input.expect(|l| l.eq(&Lexeme::RCurl))?;
    Ok(Def { name, params, stmts, body })
}

fn parse_stmts(input : &mut Input) -> Result<Vec<Stmt>, ParseError> {
    let mut ret = vec![];
    loop {
        if input.check(|l| l.eq(&Lexeme::Let))? {
            ret.push(parse_let(input)?);
        }
        else {
            return Ok(ret);
        }
    }
}

fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {
    let e = 
    if let Lexeme::Symbol(_) = input.peek()? {
        Expr::Symbol(input.take()?.value())
    }
    else if matches!(input.peek()?, Lexeme::Number(_)) {
        Expr::Number(input.take()?.value())
    }
    else if input.check(|x| x.eq(&Lexeme::Colon))? {
        parse_data(input)?
    }
    else if input.check(|x| x.eq(&Lexeme::LSquare))? {
        use crate::data::runtime;
        let items = parse_list(input, parse_expr, Lexeme::RSquare)?;
        Expr::Data(runtime::list_data.into(), items)
    }
    else {
        panic!("parse expr TODO {:?}", input.peek())
    };
    // TODO can have multiple after expr (also need a stop)
    parse_after_expr(input, e)
}

fn parse_after_expr(input : &mut Input, e : Expr) -> Result<Expr, ParseError> {
    match input.peek() {
        Err(ParseError::Eof) => { return Ok(e); },
        Err(err) => { return Err(err); },
        Ok(_) => { },
    }

    if input.check(|l| l.eq(&Lexeme::LParen))? {
        let params = parse_list(input, parse_expr, Lexeme::RParen)?;
        Ok(Expr::Call{ f: Box::new(e), params })
    }
    else {
        Ok(e)
    }
}

fn parse_let(input : &mut Input) -> Result<Stmt, ParseError> {
    let var = input.expect(|l| matches!(l, Lexeme::Symbol(_)))?;
    input.expect(|l| matches!(l, Lexeme::Equal))?;
    let val = Box::new(parse_expr(input)?);
    input.expect(|l| l.eq(&Lexeme::SemiColon))?;
    Ok(Stmt::Let { var: var.value(), val })
}

fn parse_data(input : &mut Input) -> Result<Expr, ParseError> {
    let name = input.expect(|l| matches!(l, Lexeme::Symbol(_)))?.value();
    if input.check(|l| l.eq(&Lexeme::LParen))? {
        let params = parse_list(input, parse_expr, Lexeme::RParen)?;
        Ok(Expr::Data(name, params))
    }
    else {
        Ok(Expr::Data(name, vec![]))
    }
}

fn parse_list<T, F : Fn(&mut Input) -> Result<T, ParseError>>(input : &mut Input, f : F, end : Lexeme) 
    -> Result<Vec<T>, ParseError> {

    let mut ret = vec![];
    if input.check(|l| l.eq(&end))? {
        return Ok(vec![]);
    }
    ret.push(f(input)?);
    while input.check(|l| l.eq(&end))? == false {
        input.expect(|l| l.eq(&Lexeme::Comma))?;
        ret.push(f(input)?);
    }
    Ok(ret)
}

fn parse_pat(input : &mut Input) -> Result<Pat, ParseError> {
    let name = input.expect(|l| matches!(l, Lexeme::Symbol(_)))?.value();
    input.expect(|l| l.eq(&Lexeme::LCurl))?;
    let body = parse_spattern(input)?;
    input.expect(|l| l.eq(&Lexeme::RCurl))?;
    Ok(Pat { name, body })
}

fn parse_spattern(input : &mut Input) -> Result<SPattern, ParseError> {
    let p = 
    if let Lexeme::Symbol(s) = input.peek()? && s.as_ref() == "_" {
        input.take()?;
        SPattern::Wild
    }
    else if let Lexeme::Symbol(_) = input.peek()? {
        SPattern::CaptureVar(input.take()?.value())
    }
    else if input.check(|x| x.eq(&Lexeme::Dollar))? {
        let name = input.expect(|x| matches!(x, Lexeme::Symbol(_)))?.value();
        SPattern::Var(name)
    }
    else if matches!(input.peek()?, Lexeme::Number(_)) {
        SPattern::Number(input.take()?.value())
    }
    else if input.check(|x| x.eq(&Lexeme::Colon))? {
        parse_spattern_data(input)?
    }
    else if input.check(|x| x.eq(&Lexeme::LSquare))? {
        use crate::data::runtime;
        let items = parse_list(input, parse_spattern, Lexeme::RSquare)?;
        SPattern::Data(runtime::list_data.into(), items)
    }
    else if input.check(|x| x.eq(&Lexeme::LOrSquare))? {
        let items = parse_list(input, parse_spattern, Lexeme::ROrSquare)?;
        SPattern::ListPath(items)
    }
    else if input.check(|x| x.eq(&Lexeme::LOrCurl))? {
        let items = parse_list(input, parse_spattern, Lexeme::ROrCurl)?;
        SPattern::Path(items)
    }
    else {
        panic!("parse spattern TODO {:?}", input.peek())
    };
    parse_after_spattern(input, p)
}

fn parse_spattern_data(input : &mut Input) -> Result<SPattern, ParseError> {
    let name = input.expect(|l| matches!(l, Lexeme::Symbol(_)))?.value();

    if input.check(|x| x.eq(&Lexeme::LParen))? {
        let ps = parse_list(input, parse_spattern, Lexeme::RParen)?;
        Ok(SPattern::Data(name, ps))
    }
    else {
        Ok(SPattern::Data(name, vec![]))
    }
}

fn parse_after_spattern(input : &mut Input, p : SPattern) -> Result<SPattern, ParseError> {
    match input.peek() {
        Err(ParseError::Eof) => { return Ok(p); },
        Err(err) => { return Err(err); },
        Ok(_) => { },
    }

    // TODO and / or parsing
    Ok(p)
    /*if input.check(|l| l.eq(&Lexeme::LParen))? {
        let params = parse_call_params(input)?;
        Ok(Expr::Call{ f: Box::new(e), params })
    }
    else {
        Ok(e)
    }*/
}