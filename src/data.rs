
use std::rc::Rc;

#[derive(Debug, PartialEq)]
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
    Equal,
}