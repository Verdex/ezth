

use crate::data::*;
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


*/


pub fn parse(input : &str) -> Result<ExprOrDef, ParseError> {
    let input = match lexer::lex(input) {
        Err(i) => { return Err(ParseError::Lex(i)); },
        Ok(ls) => ls,
    };
    let mut input = Input::new(input);

    let e = parse_expr(&mut input)?;

    Ok(ExprOrDef::Expr(e))
}

fn parse_stmt(input : &mut Input) -> Result<Stmt, ParseError> {
    if input.check(|l| l.eq(&Lexeme::Let))? {
        parse_let(input)
    }
    else {
        panic!("parse expr TODO {:?}", input.peek())
    }
}

fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {
    let e = 
    if let Lexeme::Symbol(v) = input.peek()? {
        Expr::Symbol(input.take()?.value())
    }
    else if matches!(input.peek()?, Lexeme::Number(_)) {
        Expr::Number(input.take()?.value())
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
        let params = parse_call_params(input)?;
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

fn parse_call_params(input : &mut Input) -> Result<Vec<Expr>, ParseError> {
    let mut ret = vec![];
    if input.check(|l| l.eq(&Lexeme::RParen))? {
        return Ok(vec![]);
    }
    ret.push(parse_expr(input)?);
    while input.check(|l| l.eq(&Lexeme::RParen))? == false {
        input.expect(|l| l.eq(&Lexeme::Comma))?;
        ret.push(parse_expr(input)?);
    }
    Ok(ret)
}