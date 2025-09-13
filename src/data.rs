
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Def,
    Let, 
    In,
    Symbol(Arc<str>),
    Number(Arc<str>),
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

impl Lexeme {
    pub fn value(&self) -> Arc<str> {
        match self {
            Lexeme::Symbol(x) => Arc::clone(x),
            Lexeme::Number(x) => Arc::clone(x),
            x => panic!("value failure {:?}", x),
        }
    }
}

pub enum ParseError {
    Eof,
    Fatal,
}

#[derive(Debug)]
pub enum ExprOrDef {
    Expr(Expr)
    // fun def
}

#[derive(Debug)]
pub enum Expr {
    Symbol(Arc<str>),
    Number(Arc<str>),
    Let { var: Arc<str>, val: Box<Expr>, body: Box<Expr> }, 
    Call { f : Box<Expr>, params : Vec<Expr> },
}