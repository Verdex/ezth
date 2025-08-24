
use std::rc::Rc;
use std::str::CharIndices;
use std::iter::Peekable;
use crate::data::*;


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

type Input<'a> = Peekable<CharIndices<'a>>;
type LexResult = Result<Lexeme, usize>;

pub fn lex(input : &str) -> Result<Vec<Lexeme>, usize> {
    let w : Input = input.char_indices().peekable();



    Err(0)
}

fn symbol(input : &mut Input) -> LexResult {
    let s = take_until(input, |c| c.is_alphanumeric() || c == '_');
    Ok(Lexeme::Symbol(s.into_iter().collect::<String>().into()))
}

// Note:  Only call this function when you know the first char is what you want
fn take_until<F : FnMut(char) -> bool>(input : &mut Input, mut p : F) -> Vec<char> {
    let mut ret = vec![input.next().unwrap().1];

    loop {
        match input.peek() {
            Some((_, c)) if p(*c) => {
                ret.push(*c);
                input.next().unwrap();
            },
            Some(_) => { return ret; },
            None => { return ret; },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_take_while() {
        let mut x = 0;
        let mut w = "blah".char_indices().peekable();
        let o = take_until(&mut w, |_| { x+=1; true });
        assert_eq!(x, 3);
        assert_eq!(o, vec!['b', 'l', 'a', 'h']);
    }
}
