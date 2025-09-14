
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

#[derive(Debug)]
pub enum ParseError {
    Lex(usize),
    Eof,
    Fatal,
}

#[derive(Debug)]
pub enum ExprOrDef {
    Expr(Expr),
    Def(Def),
}

#[derive(Debug)]
pub struct Def {
    pub name: Rc<str>,
    pub params: Vec<Rc<str>>,
    pub stmts: Vec<Stmt>,
    pub body: Expr,
}

#[derive(Debug)]
pub enum Stmt {
    Let { var: Rc<str>, val: Box<Expr> }, 
}

#[derive(Debug)]
pub enum Expr {
    Symbol(Rc<str>),
    Number(Rc<str>),
    Call { f : Box<Expr>, params : Vec<Expr> },
}