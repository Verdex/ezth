
use std::error::Error;
use std::rc::Rc;
use an_a_vm::data::*;
use crate::data::runtime::*;
use crate::runtime::error::*;

pub fn op_list() -> Vec<GenOp<Local, Global>> {
    vec![ 
        GenOp::Local { name: add_op.into(), op: add },
        GenOp::DynGlobal { name: cons_op.into(), op: cons },
        GenOp::DynGlobal { name: index_op.into(), op: data_access },
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
       let Local::Number(index) = &locals[params[1]] &&
       let Global::Data(kind, items) = deref_until_not_ref(globals, *addr)? &&
       let index = *index as usize &&
       items.len() > index {

        match &items[index] {
            Global::Number(x) => Ok(Some(Local::Number(*x))),
            Global::Ref(x) => Ok(Some(Local::Ref(*x))),
            _ => Err(Box::new(RuntimeError::Words("data_access needs to find a number or ref"))),
        }
    }
    else {
        return Err(Box::new(RuntimeError::Words("some problem with data_access")));
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

fn deref_until_not_ref(globals: &[Global], mut addr : usize) -> Result<&Global, Box<dyn Error>> {
    let mut seen = std::collections::HashSet::new();
    loop {
        if !seen.insert(addr) {
            return Err(Box::new(RuntimeError::Words("deref until not ref encountered loop")));
        }

        if globals.len() <= addr {
            return Err(Box::new(RuntimeError::Words("deref until not ref encountered a ref out of range")));
        }

        match &globals[addr] {
            x @ Global::Number(_) => { return Ok(x); },
            x @ Global::Data(_, _) => { return Ok(x); },
            Global::Ref(x) => {
                addr = *x;
            }
        }
    }
}



