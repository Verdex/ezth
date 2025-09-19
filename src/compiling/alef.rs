
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

#[derive(Debug)]
pub enum AlefError {
    LocalDoesNotExist { local: Rc<str>, fun: Rc<str> },
    LocalRedefined { local: Rc<str>, fun: Rc<str> },
    FunDoesNotExist { target: Rc<str>, src: Rc<str> },
    OpDoesNotExist { target: Rc<str>, src: Rc<str> },
}

pub fn compile(input : Vec<AlefFun>, op_map : &HashMap<Rc<str>, usize>) -> Result<Vec<Fun<Data>>, AlefError> {
    // TODO check for duplicate func names
    let map : HashMap<Rc<str>, usize> = input.iter().enumerate().map(|(i, x)| (Rc::clone(&x.name), i)).collect();
    input.into_iter().map(|f| compile_fun(f, &map, op_map)).collect()
}

fn compile_fun(f : AlefFun, fun_map : &HashMap<Rc<str>, usize>, op_map : &HashMap<Rc<str>, usize>) -> Result<Fun<Data>, AlefError> {
    let mut locals : HashMap<Rc<str>, usize> = f.params.into_iter().enumerate().map(|(i, x)| (Rc::clone(&x), i)).collect();
    let mut instrs = vec![];
    for stmt in f.stmts {
        match stmt {
            AlefStmt::Return => { instrs.push(Op::Return); },
            AlefStmt::ReturnVar(v) => { 
                let local = get_local(&locals, &v, &f.name)?;
                instrs.push(Op::ReturnLocal(local)); 
            },
            AlefStmt::Let { var, val } => {
                match val { 
                    AlefVal::Data(data) => { instrs.push(Op::PushLocal(data)); },
                    AlefVal::Var(v) => {
                        let local = get_local(&locals, &v, &f.name)?;
                        instrs.push(Op::Dup(local));
                    },
                    AlefVal::FunCall(fun, ps) => {
                        let fun = get_fun(fun_map, &fun, &f.name)?;
                        let params = ps.iter().map(|param| get_local(&locals, &param, &f.name)).collect::<Result<Vec<_>, _>>()?;
                        instrs.push(Op::Call(fun, params));
                        instrs.push(Op::PushRet);
                    },
                    AlefVal::LocalOp(op, ps) => {
                        let fun = get_op(fun_map, &op, &f.name)?;
                        let params = ps.iter().map(|param| get_local(&locals, &param, &f.name)).collect::<Result<Vec<_>, _>>()?;
                        instrs.push(Op::Gen(fun, params));
                        instrs.push(Op::PushRet);
                    },
                }
                if locals.contains_key(&var) {
                    return Err(AlefError::LocalRedefined{ local: Rc::clone(&var), fun: Rc::clone(&f.name) });
                }
                let l = locals.len();
                locals.insert(var, l);
            },
        }
    }

    Ok(Fun { name: f.name, instrs })
} 

fn get_local(map : &HashMap<Rc<str>, usize>, local : &Rc<str>, fun_name : &Rc<str>) -> Result<usize, AlefError> {
    match map.get(local) {
        Some(local) => Ok(*local),
        None => Err(AlefError::LocalDoesNotExist{ local: Rc::clone(local), fun: Rc::clone(fun_name) })
    }
}

fn get_fun(map : &HashMap<Rc<str>, usize>, target: &Rc<str>, src: &Rc<str>) -> Result<usize, AlefError> {
    match map.get(target) {
        Some(f) => Ok(*f),
        None => Err(AlefError::FunDoesNotExist{ target: Rc::clone(target), src: Rc::clone(src) })
    }
}

fn get_op(map : &HashMap<Rc<str>, usize>, target: &Rc<str>, src: &Rc<str>) -> Result<usize, AlefError> {
    match map.get(target) {
        Some(f) => Ok(*f),
        None => Err(AlefError::OpDoesNotExist{ target: Rc::clone(target), src: Rc::clone(src) })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use an_a_vm::*;

    #[test]
    fn should_assign_let_from_fun_call() {
        let fa = AlefFun{
            name: "fa".into(),
            params: vec!["a".into()],
            stmts: vec![
                AlefStmt::Let { var: "b".into(), val: AlefVal::Var("a".into()) },
                AlefStmt::ReturnVar("b".into()),
            ],
        };
        let fb = AlefFun{
            name: "fb".into(),
            params: vec![],
            stmts: vec![
                AlefStmt::Let { var: "a".into(), val: AlefVal::Data(19.0) },
                AlefStmt::Let { var: "b".into(), val: AlefVal::FunCall("fa".into(), vec!["a".into()]) },
                AlefStmt::ReturnVar("b".into()),
            ],
        };
        let fs = compile(vec![fa, fb], &HashMap::new()).unwrap();
        let mut vm : Vm<Data, ()> = Vm::new(fs, vec![]);

        let result = vm.run(1).unwrap().unwrap();

        assert_eq!(result, 19.0);
    }

    #[test]
    fn should_assign_let_from_let_from_value() {
        let f = AlefFun{
            name: "fun".into(),
            params: vec![],
            stmts: vec![
                AlefStmt::Let { var: "a".into(), val: AlefVal::Data(19.0) },
                AlefStmt::Let { var: "b".into(), val: AlefVal::Var("a".into()) },
                AlefStmt::ReturnVar("b".into()),
            ],
        };
        let fs = compile(vec![f], &HashMap::new()).unwrap();
        let mut vm : Vm<Data, ()> = Vm::new(fs, vec![]);

        let result = vm.run(0).unwrap().unwrap();

        assert_eq!(result, 19.0);
    }
}
