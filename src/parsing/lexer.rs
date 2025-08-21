
use std::rc::Rc;
use crate::data::*;

pub enum LexResult {
    Lexeme(Lexeme),
    Partial,
    End,
    Fatal,
}

pub struct Lexer {
    row : usize,
    col : usize,
    lines : Vec<Vec<char>>,
}

type RestorePoint = (usize, usize);

impl Lexer {
    pub fn new() -> Self {
        Lexer { lines: vec![], row: 0, col: 0 }
    }
    pub fn save(&self) -> RestorePoint {
        (self.row, self.col)
    }
    pub fn restore(&mut self, rp : RestorePoint) {
        self.row = rp.0;
        self.col = rp.1;
    }
    pub fn append(&mut self, input : &str)  {
        self.lines.push(input.chars().collect::<Vec<_>>());
    }
    pub fn lex(&mut self) -> LexResult {
        while self.row < self.lines.len() {
            while self.col < self.lines[self.row].len() && self.lines[self.row][self.col].is_whitespace() {
                self.col += 1;
            }
            self.row += 1;
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
