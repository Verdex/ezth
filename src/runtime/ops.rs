
use an_a_vm::data::*;
use crate::data::runtime::*;

pub fn op_list() -> Vec<GenOp<Data, Heap>> {
    vec![ 
        GenOp::Local { name: "add".into(), op: add }
    ]
}

fn add(locals: &mut Vec<Data>, params: &[usize]) -> Result<Option<Data>, Box<dyn std::error::Error>> {
    Ok(Some(locals[params[0]] + locals[params[1]]))
}
