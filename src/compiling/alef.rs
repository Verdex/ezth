
// lets, defs, ops, calls

use std::collections::HashMap;
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;

pub enum AlefVal {
    Data(Data),
    Var(Rc<str>),
    FunCall(Rc<str>, Vec<Rc<str>>),
    LocalOp(Rc<str>, Vec<Rc<str>>),
}

pub enum AlefStmt {
    Let { var : Rc<str>, val : AlefVal },
    Return,
    ReturnVar(Rc<str>),
}

pub struct AlefFun {
    name : Rc<str>,
    params : Vec<Rc<str>>,
    stmts : Vec<AlefStmt>,
}

pub fn compile(input : Vec<AlefFun>) -> Result<(Vec<Fun<Data>>, HashMap<Rc<str>, usize>), Box<dyn std::error::Error>> {

    todo!()
}