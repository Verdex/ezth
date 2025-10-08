
use std::rc::Rc;
use crate::data::runtime::*;
use crate::data::parse::SPattern;
use super::gimel::{GimelFun, GimelStmt, GimelVal};


pub struct BetFun {
    pub name : Rc<str>,
    pub params : Vec<Rc<str>>,
    pub stmts : Vec<BetStmt>,
    pub body : BetExpr,
}

pub enum BetStmt {
    Let { var : Rc<str>, val : BetExpr },
}

pub enum BetExpr {
    Local(Local),
    Var(Rc<str>),
    Call(Rc<str>, Vec<BetExpr>),
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

pub fn compile(input : Vec<BetFun>) -> Result<Vec<GimelFun>, BetError> {
    input.into_iter().map(compile_fun).collect()
} 

fn compile_fun(f : BetFun) -> Result<GimelFun, BetError> {
    let mut i : usize = 0;

    let stmts = f.stmts.into_iter().map(|s| compile_stmt(&mut i, s)).collect::<Result<Vec<_>, _>>()?;
    let mut stmts = stmts.into_iter().flatten().collect::<Vec<_>>();

    let (mut ret_lets, ret_var) = compile_expr(&mut i, f.body)?;

    stmts.append(&mut ret_lets);
    stmts.push(GimelStmt::ReturnVar(ret_var));

    Ok(GimelFun { name: f.name, params: f.params, stmts })
}

fn compile_stmt(i : &mut usize, s : BetStmt) -> Result<Vec<GimelStmt>, BetError> {
    match s { 
        BetStmt::Let { var, val } => {
            let (mut lets, v) = compile_expr(i, val)?;
            lets.push(GimelStmt::Let { var, val: GimelVal::Var(v) });
            Ok(lets)
        },
    }
}

fn compile_expr(i : &mut usize, e : BetExpr) -> Result<(Vec<GimelStmt>, Rc<str>), BetError> { 
    let var = gen_sym(i);
    match e {
        BetExpr::Local(d) => Ok((vec![GimelStmt::Let { var: Rc::clone(&var), val: GimelVal::Local(d) }], var)),
        BetExpr::Var(v) => Ok((vec![], v)),
        BetExpr::Call(n, ps) => {
            let x = ps.into_iter().map(|p| compile_expr(i, p)).collect::<Result<Vec<_>, _>>()?;
            let (mut lets, ps) = x.into_iter().fold((vec![], vec![]), |(mut all_lets, mut ps), (mut lets, p)| {
                all_lets.append(&mut lets);
                ps.push(p);
                (all_lets, ps)
            });

            lets.push(GimelStmt::Let { var: Rc::clone(&var), val: GimelVal::Call(n, ps)});

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
    use super::super::gimel;

    #[test]
    fn should_denest() {

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
            stmts: vec![BetStmt::Let{var: "z".into(), val: BetExpr::Call("add".into(), vec![BetExpr::Var("a".into()), BetExpr::Var("b".into())])}],
            body: BetExpr::Call("add".into(), vec![BetExpr::Var("z".into()), BetExpr::Local(Local::Number(5.0))]),
        };

        let main = BetFun {
            name: "main".into(),
            params: vec![],
            stmts: vec![
                BetStmt::Let { var: "x".into(), val: BetExpr::Local(Local::Number(2.0)) },
                BetStmt::Let { var: "y".into(), val: BetExpr::Var("x".into()) },
                BetStmt::Let { var: "z".into(), val: BetExpr::Local(Local::Number(3.0)) },
                BetStmt::Let { 
                    var: "w".into(), 
                    val: BetExpr::Call("other".into(), vec![
                        BetExpr::Var("x".into()),
                        BetExpr::Call("add".into(), vec![BetExpr::Var("y".into()), BetExpr::Var("z".into())])
                    ])
                },
                BetStmt::Let { 
                    var: "a".into(), 
                    val: BetExpr::Call("add".into(), vec![
                        BetExpr::Call("other".into(), vec![
                            BetExpr::Call("add".into(), vec![
                                BetExpr::Var("y".into()),
                                BetExpr::Var("z".into())
                            ]),
                            BetExpr::Call("other".into(), vec![
                                BetExpr::Var("x".into()),
                                BetExpr::Var("x".into())
                            ])
                        ]),
                        BetExpr::Call("add".into(), vec![
                            BetExpr::Var("w".into()),
                            BetExpr::Var("x".into())
                        ])
                    ])
                },
            ],
            body: BetExpr::Call("add".into(), vec![
                BetExpr::Var("a".into()),
                BetExpr::Call("add".into(), vec![
                    BetExpr::Var("a".into()),
                    BetExpr::Call("add".into(), vec![
                        BetExpr::Var("a".into()),
                        BetExpr::Var("a".into())
                    ])
                ])
            ]) 
        };

        let ops : Vec<GenOp<Local, ()>> = vec![
            GenOp::Local
            {
                name: "add".into(), 
                op: |locals, params| { 
                    if let Local::Number(a) = locals[params[0]] && let Local::Number(b) = locals[params[1]] {
                        Ok(Some(Local::Number(a + b)))
                    }
                    else {
                        panic!("failure");
                    }
                }
            }
        ];
        let op_map : HashMap<Rc<str>, usize> = HashMap::from([("add".into(), 0)]);

        let gimel_fs = compile(vec![main, other]).unwrap();
        let alef_fs = gimel::compile(gimel_fs, &op_map).unwrap();
        let fs = alef::compile(alef_fs).unwrap();
        let mut vm : Vm<Local, ()> = Vm::new(fs, ops);

        let result = vm.run(0).unwrap().unwrap();

        assert!(matches!(result, Local::Number(132.0)));
    }
}