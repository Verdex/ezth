
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;
use crate::runtime::error::*;

pub fn op_list() -> Vec<GenOp<Local, Global>> {
    vec![ 
        GenOp::Local { name: "add".into(), op: add },
        GenOp::DynGlobal { name: cons_op.into(), op: cons },
    ]
}

fn add(locals: &mut Vec<Local>, params: &[usize]) -> Result<Option<Local>, Box<dyn std::error::Error>> {
    // TODO param count check
    if let Local::Number(a) = locals[params[0]] &&
       let Local::Number(b) = locals[params[1]] 

    {
        Ok(Some(Local::Number(a + b)))
    }
    else {
        Err(Box::new(RuntimeError::Type { src: "add", expected: "Local::Number"}))
    }
}

fn cons(globals: &mut Vec<Global>, locals: &[Local], params : &[usize]) -> Result<Option<Local>, Box<dyn std::error::Error>> {
    // TODO param count
    // TODO locals count

    if let Local::Symbol(s) = &locals[params[0]] {
        let ps = params[1..].iter().map(|x| &locals[*x]).map(local_to_global).collect::<Vec<_>>();
        let addr = globals.len();
        globals.push(Global::Data(Rc::clone(&s), ps));
        Ok(Some(Local::Ref(addr)))
    }
    else {
        return Err(Box::new(RuntimeError::Type { src: cons_op, expected: "Local::Symbol"}));
    }

    // TODO 
    // first param had better indicate a symbol in locals
    // arbitrary many params can follow
}

fn local_to_global(local : &Local) -> Global {
    match local {
        Local::Number(x) => Global::Number(*x),
        Local::Ref(addr) => Global::Ref(*addr),
        Local::Symbol(s) => Global::Data(Rc::clone(s), vec![]),
    }
}