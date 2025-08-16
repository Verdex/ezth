
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
    pub fn append(&mut self, input : &'a str)  {
        self.input.push(input);
    }
    pub fn lex(&mut self) -> LexResult {
        LexResult::End
    }
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
