
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

#[derive(Debug)]
pub enum BetError {

}

impl std::fmt::Display for BetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            _ => unreachable!(),
        }
    }
}

impl std::error::Error for BetError { }

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
    stmts.push(AlefStmt::ReturnVar(ret_var));

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
        BetExpr::LocalOp(n, ps) => {
            // TODO is `i` actually being processed correctly
            let x = ps.into_iter().map(|p| compile_expr(i, p)).collect::<Result<Vec<_>, _>>()?;
            let (mut lets, ps) = x.into_iter().fold((vec![], vec![]), |(mut all_lets, mut ps), (mut lets, p)| {
                all_lets.append(&mut lets);
                ps.push(p);
                (all_lets, ps)
            });

            lets.push(AlefStmt::Let { var: Rc::clone(&var), val: AlefVal::LocalOp(n, ps)});

            Ok((lets, var))
        },
    }
}

fn gen_sym(i : &mut usize) -> Rc<str> { *i += 1; format!("temp_{i}").into() }

#[cfg(test)]
mod test {
    use an_a_vm::*;
    use an_a_vm::data::*;
    use std::collections::HashMap;
    use super::*;
    use super::super::alef;

    #[test]
    fn should_denest() {
        // op with op inside
        // op with call inside
        // call with call inside
        // call with op inside
        // inside inside inside inside
        // data param
        // var param
        // return with all that

        /*
            let x = 2
            let y = x
            let z = 3
            let w = other(x, add(y, z))
            let a = add(other(add(y, z), other(x, x)), add(w, y))

            return add(a, add(a, add(a, a)))

        */

        let other = BetFun { 
            name: "other".into(),
            params: vec!["a".into(), "b".into()],
            stmts: vec![BetStmt::Let{var: "z".into(), val: BetExpr::LocalOp("add".into(), vec![BetExpr::Var("a".into()), BetExpr::Var("b".into())])}],
            body: BetExpr::LocalOp("add".into(), vec![BetExpr::Var("z".into()), BetExpr::Data(5.0)]),
        };

        let main = BetFun {
            name: "main".into(),
            params: vec![],
            stmts: vec![],
            body: BetExpr::FunCall("other".into(), vec![BetExpr::Data(1.0), BetExpr::Data(2.0)]),
        };

        /*let main = BetFun {
            name: "main".into(),
            params: vec![],
            stmts: vec![
                AlefStmt::Let { var: "a".into(), val: AlefVal::Data(19.0) },
                AlefStmt::Let { var: "b".into(), val: AlefVal::Data(2.0) },
                AlefStmt::Let { var: "c".into(), val: AlefVal::LocalOp("add".into(), vec!["a".into(), "b".into()]) },
                AlefStmt::ReturnVar("c".into()),
            ],
        };*/
        let ops : Vec<GenOp<Data, ()>> = vec![
            GenOp::Local{name: "add".into(), op: |locals, params| { Ok(Some(locals[params[0]] + locals[params[1]])) }}
        ];
        let op_map : HashMap<Rc<str>, usize> = HashMap::from([("add".into(), 0)]);

        let alef_fs = compile(vec![other]).unwrap();

        let fs = alef::compile(alef_fs, &op_map).unwrap();
        let mut vm : Vm<Data, ()> = Vm::new(fs, ops);

        let result = vm.run(0).unwrap().unwrap();

        assert_eq!(result, 21.0);
    }
}