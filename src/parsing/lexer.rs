
use std::rc::Rc;
use std::str::CharIndices;
use std::iter::Peekable;
use crate::data::*;


type Input<'a> = Peekable<CharIndices<'a>>;

pub fn lex(input : &str) -> Result<Vec<Lexeme>, usize> {
    let mut input : Input = input.char_indices().peekable();
    let mut ret = vec![];

    loop { 
        match input.peek() {
            None => { return Ok(ret); },
            Some((_, c)) if c.is_alphabetic() || *c == '_' => {
                ret.push(symbol(&mut input)?);
            },
            Some((_, c)) if punct_char(*c) => {
                ret.append(&mut punct(&mut input)?);
            },
            _ => todo!(),
        } 
    } 

    Err(0)
}

fn symbol(input : &mut Input) -> Result<Lexeme, usize> {
    let s = take_until(input, |c| c.is_alphanumeric() || c == '_');
    let s = s.into_iter().collect::<String>();

    match s.as_str() {
        "def" => Ok(Lexeme::Def),
        s => Ok(Lexeme::Symbol(s.into())),
    }
}

fn punct(input : &mut Input) -> Result<Vec<Lexeme>, usize> {
    let s = take_until(input, |c| punct_char(c));
    let mut s = s.into_iter().collect::<String>();
    let mut remainder = vec![];

    let mut ret = vec![];
    while !s.is_empty() {
        match s.as_str() {
            "=>" => { ret.push(Lexeme::DRArrow); },
            "(" => { 
                ret.push(Lexeme::LParen); 
                s.remove(0);
                s.remove(0);
            },
            ")" => { 
                ret.push(Lexeme::RParen); 
                s.remove(0);
            },
            "{" => { 
                ret.push(Lexeme::LCurl); 
                s.remove(0);
            },
            "}" => { 
                ret.push(Lexeme::RCurl); 
                s.remove(0);
            },
            "[" => { 
                ret.push(Lexeme::LSquare); 
                s.remove(0);
            },
            "]" => { 
                ret.push(Lexeme::RSquare); 
                s.remove(0);
            },
            "." => { 
                ret.push(Lexeme::Dot); 
                s.remove(0);
            },
            "," => { 
                ret.push(Lexeme::Comma); 
                s.remove(0);
            },
            "|" => { 
                ret.push(Lexeme::OrBar); 
                s.remove(0);
            },
            _ => { 
                remainder.insert(0, s.pop().unwrap());
            },
        }
        if s.is_empty() {
            s = remainder.into_iter().collect::<String>();
            remainder = vec![];
        }
    }

    Ok(ret)
}

fn punct_char(input : char) -> bool {
    input == '(' ||
    input == ')' ||
    input == '{' ||
    input == '}' ||
    input == '[' ||
    input == ']' ||
    input == '.' ||
    input == ',' ||
    input == '|' ||
    input == '=' ||
    input == '>' 
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
