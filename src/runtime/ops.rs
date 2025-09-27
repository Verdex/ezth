
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

fn cons(globals: &mut Vec<Global>, locals: &[Local], params : &[usize] ) -> Result<Option<Local>, Box<dyn std::error::Error>> {
    // TODO 
    // first param had better indicate a symbol in locals
    // arbitrary many params can follow
    todo!()
}