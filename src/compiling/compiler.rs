
use std::collections::HashMap;
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;
use crate::data::parse::*;
use super::alef;
use super::bet::{self, BetExpr, BetFun, BetStmt};
use super::gimel;

pub fn compile(input : Vec<Def>, ops: &HashMap<Rc<str>, usize>) -> Result<Vec<Fun<Data>>, Box<dyn std::error::Error>> {
    let input = input.into_iter().map(convert_def).collect::<Vec<_>>();
    let input = bet::compile(input)?;
    let input = gimel::compile(input, ops)?;
    let input = alef::compile(input)?;
    Ok(input)
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
        Expr::Number(n) => BetExpr::Data(n.parse::<f64>().unwrap()),
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