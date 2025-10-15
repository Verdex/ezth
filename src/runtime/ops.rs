
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;
use crate::runtime::error::*;

pub fn op_list() -> Vec<GenOp<Local, Global>> {
    vec![ 
        GenOp::Local { name: "add".into(), op: add },
        GenOp::DynGlobal { name: cons_op.into(), op: cons },
        // TODO data_access 
        GenOp::DynGlobal { name: "data_access".into(), op: data_access },
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
}

fn data_access(globals: &mut Vec<Global>, locals: &[Local], params: &[usize]) -> Result<Option<Local>, Box<dyn std::error::Error>> {
    // params = ref of the data, index to get 
    // TODO checks
    //
    // Note:  This basically works because while creating a Data the 'fields' always get populated
    // with Ref or Number

    if let Local::Ref(addr) = &locals[params[0]] && 
       let Local::Number(index) = &locals[params[1]] {
        
        // TODO grab the global, it had better either be data or a ref to data
        // does it have an index that looks good?
        // that had better be a number or ref
        // return that
        todo!()
    }
    else {
        // TODO what about number mismatch?
        return Err(Box::new(RuntimeError::Type { src: "data_access", expected: "Local::Ref" }));
    }
}

// TODO do a set_branch_if_kind_eq or something.  Maybe just a general purpose eq op


fn local_to_global(local : &Local) -> Global {
    match local {
        Local::Number(x) => Global::Number(*x),
        Local::Ref(addr) => Global::Ref(*addr),
        Local::Symbol(s) => Global::Data(Rc::clone(s), vec![]),
    }
}

