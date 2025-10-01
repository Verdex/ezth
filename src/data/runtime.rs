
use std::rc::Rc;

pub const cons_op : &'static str = "cons";
pub const list_data : &'static str = "list";

#[derive(Clone, Debug)]
pub enum Local {
    Number(f64),
    Ref(usize),
    Symbol(Rc<str>),
}

#[derive(Debug)]
pub enum Global {
    Number(f64),
    Ref(usize),
    Data(Rc<str>, Vec<Global>),
}