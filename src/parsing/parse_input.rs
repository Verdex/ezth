
use std::iter::Peekable;
use crate::data::*;

pub struct Input {
    ls : Peekable<std::vec::IntoIter<Lexeme>>
}

impl Input {
    pub fn new(input : Vec<Lexeme>) -> Self {
        Input { ls: input.into_iter().peekable() }
    }

    pub fn check<F:Fn(&Lexeme) -> bool>(&mut self, f : F) -> Result<bool, ParseError> {
        match self.ls.peek() {
            Some(l) if f(l) => {
                self.ls.next().unwrap();
                Ok(true)
            },
            Some(_) => Ok(false),
            None => Err(ParseError::Eof),
        }
    }
    pub fn expect<F:Fn(&Lexeme) -> bool>(&mut self, f : F) -> Result<Lexeme, ParseError> {
        match self.ls.peek() {
            Some(l) if f(l) => {
                let l = self.ls.next().unwrap();
                Ok(l)
            },
            Some(_) => Err(ParseError::Fatal),
            None => Err(ParseError::Eof),
        }
    }
    pub fn peek(&mut self) -> Result<&Lexeme, ParseError> {
        match self.ls.peek() {
            Some(l) => Ok(l),
            None => Err(ParseError::Eof),
        }
    }
    pub fn take(&mut self) -> Result<Lexeme, ParseError> {
        match self.ls.next() {
            Some(l) => Ok(l),
            None => Err(ParseError::Eof),
        }
    }
}
