
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Local {
    Number(f64),
    Ref(usize),
    Symbol(Rc<str>),
}

pub enum Heap {
    Number(f64),
    Ref(usize),
    Data(Rc<str>, Vec<Local>),
}