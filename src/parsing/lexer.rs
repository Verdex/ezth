
use std::rc::Rc;
use std::str::CharIndices;
use crate::data::*;

pub enum LexResult {
    Lexeme(Lexeme),
    Partial,
    End,
    Fatal,
}

pub struct Lexer<'a> {
    line_index : usize, 
    lines : Vec<&'a str>,
    current : Option<CharIndices<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Lexer { lines: vec![], line_index: 0, current: None }
    }
    pub fn save(&self) -> Self {
        Lexer { lines: self.lines.clone(), line_index: self.line_index, current: self.current.clone() }
    }
    pub fn restore(&mut self, rp : Self ) {
        *self = rp;
    }
    pub fn append(&mut self, input : &'a str)  {
        self.lines.push(input);
    }
    pub fn lex(&mut self) -> LexResult {
        if self.line_index >= self.lines.len() {
            return LexResult::End;
        }
        LexResult::End
    }
}

fn lex_def(l : &mut Lexer) -> LexResult {
    LexResult::End
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blarg() {
        let mut x = Lexer { input : vec![] };
        x.append("blah");
        x.append("blah");

    }
}
