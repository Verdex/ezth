
use std::thread::{ self, JoinHandle };
use std::sync::mpsc::{ self, Sender, Receiver };

use crate::data::*;
use super::lexer;
use super::parse_input::Input;

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


pub fn init() -> (JoinHandle<()>, Sender<String>, Receiver<ParseResult>) {
    let (in_send, in_rec) = mpsc::channel();
    let (out_send, out_rec) = mpsc::channel();

    let t = thread::spawn(move || parse(out_send, in_rec));

    (t, in_send, out_rec)
}

fn parse(send : Sender<ParseResult>, rec : Receiver<String>) {
    let mut input = Input::new(send.clone(), rec);

    match parse_expr(&mut input) {
        Ok(w) => {
            send.send(ParseResult::Success(ExprOrDef::Expr(w))).expect("Parser Output send fail");
        },
        Err(i) => {
            send.send(ParseResult::Fatal(i)).expect("Parser Output send fail");
        },
    }
}

fn parse_expr(input : &mut Input) -> Result<Expr, usize> {
    let e = if input.check(|l| l.eq(&Lexeme::Let))? {
        parse_let(input)?
    }
    else if matches!(input.peek()?, Lexeme::Number(_)) {
        Expr::Number(input.take()?.value())
    }
    else {
        todo!()
    };
    Ok(e)
}

fn parse_let(input : &mut Input) -> Result<Expr, usize> {
    let var = input.expect(|l| matches!(l, Lexeme::Symbol(_)))?;
    input.expect(|l| matches!(l, Lexeme::Equal))?;
    let val = Box::new(parse_expr(input)?);
    input.expect(|l| matches!(l, Lexeme::In))?;
    let body = Box::new(parse_expr(input)?);
    Ok(Expr::Let { var: var.value(), val, body })
}
