
use std::rc::Rc;
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
    FunCall(Rc<str>, Vec<BetExpr>),
    LocalOp(Rc<str>, Vec<BetExpr>),
}

pub enum BetError {

}

// TODO unwrap nested fun calls into lets
pub fn compile(input : Vec<BetFun>) -> Result<Vec<AlefFun>, BetError> {
    input.into_iter().map(compile_fun).collect()
} 

fn compile_fun(f : BetFun) -> Result<AlefFun, BetError> {
    let mut i : usize = 0;

    let stmts = f.stmts.into_iter().map(|s| compile_stmt(&mut i, s)).collect::<Result<Vec<_>, _>>()?;
    let mut stmts = stmts.into_iter().flatten().collect::<Vec<_>>();

    let (mut ret_lets, ret_var) = compile_expr(&mut i, f.body)?;

    stmts.append(&mut ret_lets);
    stmts.push(AlefStmt::Let { var: gen_sym(&mut i), val: AlefVal::Var(ret_var) });

    Ok(AlefFun { name: f.name, params: f.params, stmts })
}

fn compile_stmt(i : &mut usize, s : BetStmt) -> Result<Vec<AlefStmt>, BetError> {
    match s { 
        BetStmt::Let { var, val } => {
            let (mut lets, v) = compile_expr(i, val)?;
            lets.push(AlefStmt::Let { var, val: AlefVal::Var(v) });
            Ok(lets)
        },
    }
}

fn compile_expr(i : &mut usize, e : BetExpr) -> Result<(Vec<AlefStmt>, Rc<str>), BetError> { 
    let var = gen_sym(i);
    match e {
        BetExpr::Data(d) => Ok((vec![AlefStmt::Let { var: Rc::clone(&var), val: AlefVal::Data(d) }], var)),
        BetExpr::Var(v) => Ok((vec![], v)),
        BetExpr::FunCall(n, ps) => {
            // TODO is `i` actually being processed correctly
            let x = ps.into_iter().map(|p| compile_expr(i, p)).collect::<Result<Vec<_>, _>>()?;
            let (mut lets, ps) = x.into_iter().fold((vec![], vec![]), |(mut all_lets, mut ps), (mut lets, p)| {
                all_lets.append(&mut lets);
                ps.push(p);
                (all_lets, ps)
            });

            lets.push(AlefStmt::Let { var: Rc::clone(&var), val: AlefVal::FunCall(n, ps)});

            Ok((lets, var))
        },
        _ => todo!(),
    }
}

fn gen_sym(i : &mut usize) -> Rc<str> { *i += 1; format!("temp_{i}").into() }
