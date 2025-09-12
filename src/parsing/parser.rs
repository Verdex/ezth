
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
    // TODO iterator that produces the next lexeme from the string or else wait on the receiver

    //let output = lexer::lex(input); 

    //todo!()
}

struct Input {
    lexemes : Vec<(usize, Lexeme)>,
    rec : Receiver<String>,
    send : Sender<ParseResult>,
}

impl Input {
    pub fn peek(&mut self) -> Result<&Lexeme, usize> {
        if self.lexemes.len() == 0 {
            self.wait()?;
        }
        Ok(&self.lexemes[0].1)
    }

    // Note:  Intended to use only for checking the index that an unexpected 
    // lexeme appears at.  If there aren't any lexemes ready then index 0 is
    // as good as any.
    pub fn peek_index(&self) -> usize {
        if self.lexemes.len() == 0 { 
            0
        }
        else {
            self.lexemes[0].0
        }
    }

    pub fn take(&mut self) -> Result<Lexeme, usize> {
        if self.lexemes.len() == 0 {
            self.wait()?;
        }
        Ok(self.lexemes.pop().unwrap().1)
    }

    fn wait(&mut self) -> Result<(), usize> {
        self.send.send(ParseResult::Incremental).expect("Parser Output send failure");
        let s = self.rec.recv().expect("Parser Input recv failure");
        let ls = lexer::lex(&s)?;
        self.lexemes = ls.into_iter().enumerate().collect();
        Ok(())
    }
}

