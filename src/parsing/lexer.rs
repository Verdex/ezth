
use std::rc::Rc;
use crate::data::*;

pub enum LexResult {
    Lexeme(Lexeme),
    Partial,
    End,
    Fatal,
}

pub struct Lexer<'a> {
    line_index : usize, 
    current_index : usize,
    lines : Vec<&'a str>,
}

struct RestorePoint {
    line_index : usize,
    current_index : usize,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Lexer { lines: vec![], line_index: 0, current_index: 0 }
    }
    pub fn save(&self) -> RestorePoint {
        RestorePoint { line_index: self.line_index, current_index: self.current_index }
    }
    pub fn restore(&mut self, rp : RestorePoint) {
        self.line_index = rp.line_index;
        self.current_index = rp.current_index;
    }
    pub fn append(&mut self, input : &'a str)  {
        self.lines.push(input);
    }
    pub fn lex(&mut self) -> LexResult {
        self.lines[self.line_index]
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
