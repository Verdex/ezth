
use std::collections::HashMap;
use std::rc::Rc;
use crate::data::*;
use an_a_vm::data::*;

pub fn compile_def(env : &mut HashMap<Rc<str>, usize>, d : Def) -> Result<Vec<Op<f64>>, Box<dyn std::error::Error>> {

    todo!()
}