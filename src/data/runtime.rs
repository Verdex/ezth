
use std::rc::Rc;

pub const eq_op : &'static str = "eq";
pub const add_op : &'static str = "add";
pub const cons_op : &'static str = "cons";
pub const index_op : &'static str = "index";
pub const list_data : &'static str = "list";

#[derive(Clone, Debug)]
pub enum Local {
    Number(f64),
    Ref(usize),
    Symbol(Rc<str>),
    Bool(bool),
}

#[derive(Debug)]
pub enum Global {
    Number(f64),
    Ref(usize),
    Data(Rc<str>, Vec<Global>),
    Bool(bool),
}
