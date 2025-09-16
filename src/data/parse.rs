
use std::rc::Rc;

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