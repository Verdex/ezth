
use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
    Lex(usize),
    Eof,
    Fatal,
}

#[derive(Debug, Clone)]
pub enum ExprOrDef {
    Expr(Expr),
    Def(Def),
}

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Rc<str>,
    pub params: Vec<Rc<str>>,
    pub stmts: Vec<Stmt>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { var: Rc<str>, val: Box<Expr> }, 
}

#[derive(Debug, Clone)]
pub enum Expr {
    Symbol(Rc<str>),
    Number(Rc<str>),
    Data(Rc<str>, Vec<Expr>),
    Call { f : Box<Expr>, params : Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum SPattern {
    Wild,
    Number(Rc<str>),
    CaptureVar(Rc<str>),
    Var(Rc<str>),
    Data(Rc<str>, Vec<SPattern>),
    And(Box<SPattern>, Box<SPattern>),
    Or(Box<SPattern>, Box<SPattern>),
    PathNext,
    Path(Vec<SPattern>),
    ListPath(Vec<SPattern>),
}