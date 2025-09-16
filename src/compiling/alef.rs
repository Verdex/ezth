
// lets, defs, ops, calls

use std::collections::HashMap;
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;

pub enum AlefStmt {

}

pub struct AlefFun {
    name : Rc<str>,
    params : Vec<Rc<str>>,
    stmts : Vec<AlefStmt>,
}

pub fn compile(input : Vec<AlefFun>) -> Result<(Vec<Fun<Data>>, HashMap<Rc<str>, usize>), Box<dyn std::error::Error>> {

    todo!()
}