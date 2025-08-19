
use std::rc::Rc;

pub enum Lexeme {
    Def,
    Symbol(Rc<str>),
    LParen,
    RParen,
}