
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Def,
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
}

pub enum ExprOrDef {
    Expr(Expr)
    // type def
    // fun def
}

pub enum Expr {
    Number,
    Let { var: Rc<str>, val: Box<Expr>, body: Box<Expr> }, 
}