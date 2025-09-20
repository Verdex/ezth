
use std::rc::Rc;
use std::collections::HashMap;
use crate::data::runtime::*;
use super::alef::{AlefFun, AlefStmt, AlefVal};


pub struct BetFun {
    name : Rc<str>,
    params : Vec<Rc<str>>,
    stmts : Vec<BetStmt>,
    body : BetExpr,
}

pub enum BetStmt {
    Let { var : Rc<str>, val : BetExpr },
}

pub enum BetExpr {
    Data(Data),
    Var(Rc<str>),
    Call(Rc<str>, Vec<BetExpr>),
}

pub enum BetError {

}

// TODO unwrap nested fun calls into lets
// TODO split calls into fun calls and op calls
pub fn compile(input : Vec<BetFun>, op_map : &HashMap<Rc<str>, usize>) -> Result<Vec<AlefFun>, BetError> {
    input.into_iter().map(|f| compile_fun(f, op_map)).collect()
} 

fn compile_fun(f : BetFun, op_map : &HashMap<Rc<str>, usize>) -> Result<AlefFun, BetError> {
    let mut i : usize = 0;

    todo!()
}

fn compile_expr(i : &mut usize, e : BetExpr, op_map : &HashMap<Rc<str>, usize>) -> Result<(Vec<AlefStmt>, AlefVal), BetError> { 
    match e {
        BetExpr::Data(d) => Ok((vec![], AlefVal::Data(d))),
        BetExpr::Var(v) => todo!(),
        BetExpr::Call(n, ps) => todo!(),
    }
}

fn gen_sym(i : &mut usize) -> Rc<str> { *i += 1; format!("temp_{i}").into() }
