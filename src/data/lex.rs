
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Def,
    Let, 
    Symbol(Rc<str>),
    Number(Rc<str>),
    LParen,
    RParen,
    LCurl,
    RCurl,
    LSquare,
    RSquare,
    Dot,
    Comma,
    OrBar,
    DRArrow,
    Equal,
    SemiColon,
    Colon,
    Pat,
}

impl Lexeme {
    pub fn value(&self) -> Rc<str> {
        match self {
            Lexeme::Symbol(x) => Rc::clone(x),
            Lexeme::Number(x) => Rc::clone(x),
            x => panic!("value failure {:?}", x),
        }
    }
}
