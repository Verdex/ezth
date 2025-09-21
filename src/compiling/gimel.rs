
use super::alef::*;

pub enum GimelVal {
    Data(Data),
    Var(Rc<str>),
    Call(Rc<str>, Vec<Rc<str>>),
}

pub enum GimelStmt {
    Let { var : Rc<str>, val : GimelVal },
    ReturnVar(Rc<str>),
}

pub struct GimelFun {
    pub name : Rc<str>,
    pub params : Vec<Rc<str>>,
    pub stmts : Vec<GimelStmt>,
}

#[derive(Debug)]
pub enum GimelError {

}

impl std::fmt::Display for GimelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            _ => unreachable!(),
        }
    }
}

impl std::error::Error for GimelError { }

pub fn compile(input : Vec<GimelFun>) -> Result<Vec<AlefFun>, GimelError> {
    todo!()
}