
use std::thread::{ self, JoinHandle };
use std::sync::mpsc::{ self, Sender, Receiver };
use crate::data::*;
use super::lexer;

pub struct Input {
    lexemes : Vec<(usize, Lexeme)>,
    rec : Receiver<String>,
    send : Sender<ParseResult>,
}

impl Input {
    pub fn new(send : Sender<ParseResult>, rec : Receiver<String>) -> Self {
        Input { lexemes: vec![], send, rec }
    }

    pub fn check<F:Fn(&Lexeme) -> bool>(&mut self, f : F) -> Result<bool, usize> {
        if self.lexemes.len() == 0 {
            self.wait()?;
        }
        if f(&self.lexemes[0].1) {
            self.lexemes.remove(0);
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
    pub fn expect<F:Fn(&Lexeme) -> bool>(&mut self, f : F) -> Result<Lexeme, usize> {
        if self.lexemes.len() == 0 {
            self.wait()?;
        }
        if f(&self.lexemes[0].1) {
            let l = self.lexemes.remove(0);
            Ok(l.1)
        }
        else {
            Err(self.lexemes[0].0)
        }
    }
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
