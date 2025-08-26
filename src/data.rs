
use std::rc::Rc;

pub enum Lexeme {
    Def,
    Symbol(Rc<str>),
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
}