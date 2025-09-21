
use std::rc::Rc;
use std::collections::HashMap;
use crate::data::runtime::*;
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

pub fn compile(input : Vec<GimelFun>, op_map : &HashMap<Rc<str>, usize>) -> Result<Vec<AlefFun>, GimelError> {
    input.into_iter().map(|f| compile_fun(f, op_map)).collect()
}

fn compile_fun(input : GimelFun, op_map : &HashMap<Rc<str>, usize>)  -> Result<AlefFun, GimelError> {
    let stmts = input.stmts.into_iter().map(|x| compile_stmt(x, op_map)).collect::<Result<Vec<_>, _>>()?;
    Ok(AlefFun{ name: input.name, params: input.params, stmts })
}

fn compile_stmt(input : GimelStmt, op_map : &HashMap<Rc<str>, usize>) -> Result<AlefStmt, GimelError> {
    todo!()

}