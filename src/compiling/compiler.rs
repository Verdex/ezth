
use std::collections::HashMap;
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;
use crate::data::parse::*;
use super::alef;
use super::bet::{self, BetExpr, BetFun, BetStmt, BetTopLevel, BetCo};
use super::gimel;

pub fn compile(input : Vec<TopLevel>, ops: &HashMap<Rc<str>, usize>) -> Result<Vec<Fun<Local>>, Box<dyn std::error::Error>> {
    let input = input.into_iter().map(convert_top_level).collect::<Vec<_>>();
    let input = bet::compile(input)?;
    let input = gimel::compile(input, ops)?;
    let input = alef::compile(input)?;
    Ok(input)
}

fn convert_top_level(input : TopLevel) -> BetTopLevel {
    match input { 
        TopLevel::Def(d) => BetTopLevel::Fun(convert_def(d)),
        TopLevel::Pat(p) => BetTopLevel::Co(convert_pat(p)),
    }
}

fn convert_pat(input : Pat) -> BetCo {
    todo!()
}

fn convert_def(input : Def) -> BetFun {
    BetFun { 
        name: input.name, 
        params: input.params, 
        stmts: input.stmts.into_iter().map(convert_stmt).collect(), 
        body: convert_expr(input.body),
    }
}

fn convert_stmt(input : Stmt) -> BetStmt {
    match input { 
        Stmt::Let { var, val } => BetStmt::Let { var, val: convert_expr(*val) },
    }
}

fn convert_expr(input : Expr) -> BetExpr {
    match input { 
        Expr::Symbol(v) => BetExpr::Var(v),
        Expr::Number(n) => BetExpr::Local(Local::Number(n.parse::<f64>().unwrap())),
        Expr::Data(n, exprs) => {
            let mut exprs = exprs.into_iter().map(convert_expr).collect::<Vec<_>>();
            exprs.insert(0, BetExpr::Local(Local::Symbol(n)));
            BetExpr::Call(cons_op.into(), exprs)
        },
        Expr::Call{ f, params } => {
            if let Expr::Symbol(v) = *f {
                BetExpr::Call(v, params.into_iter().map(convert_expr).collect())
            }
            else {
                panic!("fun expr currently not supported")
            }
        },
    }
}

// TODO bet needs some sort of ifelse eq thing
// TODO one level up would have match => if else eq
// TODO compiler can convert spattern into nested match
// TODO need access index from data
// TODO need check kind
// TODO need items count