
use std::str::CharIndices;
use std::iter::Peekable;
use crate::data::lex::*;


type Input<'a> = Peekable<CharIndices<'a>>;

pub fn lex(input : &str) -> Result<Vec<Lexeme>, usize> {
    let mut input : Input = input.char_indices().peekable();
    let mut ret = vec![];

    loop { 
        match input.peek() {
            None => { return Ok(ret); },
            Some((_, c)) if c.is_whitespace() => {
                whitespace(&mut input)?;
            },
            Some((_, c)) if c.is_alphabetic() || *c == '_' => {
                ret.push(symbol(&mut input)?);
            },
            Some((_, c)) if c.is_numeric() => {
                ret.push(number(&mut input)?);
            },
            Some((_, c)) if punct_char(*c) => {
                ret.append(&mut punct(&mut input)?);
            },
            Some((i, _)) => { return Err(*i); },
        } 
    } 
}

fn whitespace(input : &mut Input) -> Result<(), usize> {
    while let Some((_, c)) = input.peek() && c.is_whitespace() {
        input.next().unwrap();
    }
    Ok(())
}

fn number(input : &mut Input) -> Result<Lexeme, usize> {
    let s = take_until(input, |c| c.is_numeric());
    let s = s.into_iter().collect::<String>();
    Ok(Lexeme::Number(s.into()))
}

fn symbol(input : &mut Input) -> Result<Lexeme, usize> {
    let s = take_until(input, |c| c.is_alphanumeric() || c == '_');
    let s = s.into_iter().collect::<String>();

    match s.as_str() {
        "def" => Ok(Lexeme::Def),
        "let" => Ok(Lexeme::Let),
        s => Ok(Lexeme::Symbol(s.into())),
    }
}

fn punct(input : &mut Input) -> Result<Vec<Lexeme>, usize> {
    let mut ret = vec![];
    while let Some((_, c)) = input.peek() && punct_char(*c) {
        match c {
            '=' => {
                input.next().unwrap();
                if let Some((_, c)) = input.peek() && *c == '>' {
                    ret.push(Lexeme::DRArrow);
                }
                else if let Some((_, c)) = input.peek() && punct_char(*c) {
                    ret.push(Lexeme::Equal);
                    continue;
                }
                else {
                    ret.push(Lexeme::Equal);
                    return Ok(ret);
                }
            },
            '(' => { ret.push(Lexeme::LParen); },
            ')' => { ret.push(Lexeme::RParen); },
            '{' => { ret.push(Lexeme::LCurl); },
            '}' => { ret.push(Lexeme::RCurl); },
            '[' => { ret.push(Lexeme::LSquare); },
            ']' => { ret.push(Lexeme::RSquare); },
            '.' => { ret.push(Lexeme::Dot); },
            ',' =>{ ret.push(Lexeme::Comma); },
            '|' => { ret.push(Lexeme::OrBar); },
            ';' => { ret.push(Lexeme::SemiColon); },
            _ => unreachable!(),
        }
        input.next().unwrap();
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
    input == '>' ||
    input == ';'
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
    fn should_parse_punctuation_stream() {
        use Lexeme::*;
        let mut w = "(){}[].,|==>".char_indices().peekable();
        let o = punct(&mut w).unwrap();
        assert_eq!(o, vec![LParen, RParen, LCurl, RCurl, LSquare, RSquare, Dot, Comma, OrBar, Equal, DRArrow]);
    }

    #[test]
    fn should_parse_double_right_arrow() {
        let mut input = "=>".char_indices().peekable();
        let o = punct(&mut input).unwrap();
        assert_eq!(o, vec![Lexeme::DRArrow])
    }

    #[test]
    fn should_parse_equal() {
        let mut input = "=".char_indices().peekable();
        let o = punct(&mut input).unwrap();
        assert_eq!(o, vec![Lexeme::Equal])
    }

    #[test]
    fn should_take_while() {
        let mut x = 0;
        let mut w = "blah".char_indices().peekable();
        let o = take_until(&mut w, |_| { x+=1; true });
        assert_eq!(x, 3);
        assert_eq!(o, vec!['b', 'l', 'a', 'h']);
    }
}
