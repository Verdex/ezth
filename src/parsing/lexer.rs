
use std::rc::Rc;
use crate::data::*;

pub enum LexResult {
    Lexeme(Vec<Lexeme>),
    Fatal(usize),
    End,
}

#[derive(Clone, Copy)]
enum Mode {
    Init,
    Symbol,
}

struct L {
    mode : Mode,
    results : Vec<Lexeme>,    
    fatal : Option<usize>,
    partial : Vec<char>,
}

pub fn lex(input : &str) -> LexResult {
    let result = input.char_indices().fold( L { mode: Mode::Init, results: vec![], fatal: None, partial: vec![] }, 
        |mut l, c| {
            let (index, c) = c;
            if l.fatal.is_some() {
                return l;
            }

            let mode = l.mode;

            match (c, mode) {
                (c, Mode::Init) if c.is_whitespace() => { },
                (c, Mode::Init) if c.is_alphabetic() => {
                    l.mode = Mode::Symbol;
                    l.partial.push(c);
                },
                (c, Mode::Symbol) => c.is_alphanumeric() || c == '_' => {
                    l.partial.push(c);
                },
                (c, Mode::Symbol) => c.is_whitespace() {
                    l.mode == Mode::Init;
                },

                _ => todo!(),
            }
            l
        });

    LexResult::End
}

#[cfg(test)]
mod test {
    use super::*;

}
