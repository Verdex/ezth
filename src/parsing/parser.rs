
use std::thread::{ self, JoinHandle };
use std::sync::mpsc::{ self, Sender, Receiver };

use crate::data::*;
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


pub enum ParseResult {
    Success,
    Incremental,
    Fatal,
}

pub fn init() -> (JoinHandle<()>, Sender<String>, Receiver<ParseResult>) {
    let (in_send, in_rec) = mpsc::channel();
    let (out_send, out_rec) = mpsc::channel();

    let t = thread::spawn(move || parse(out_send, in_rec));

    (t, in_send, out_rec)
}

fn parse(send : Sender<ParseResult>, rec : Receiver<String>) {

    //let output = lexer::lex(input); 

    //todo!()
}

