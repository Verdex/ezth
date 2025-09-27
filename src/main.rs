
mod data;
mod parsing;
mod compiling;
mod runtime;

use std::io::{self, Write};
use std::rc::Rc;
use std::collections::HashMap;
use an_a_vm::*;
use an_a_vm::data::*;
use crate::data::runtime::*;
use crate::data::parse::*;
use crate::parsing::parser;
use crate::compiling::compiler;
use crate::runtime::ops;


fn main() {

    let mut defs = vec![];
    let mut prev_line = String::new();

    loop {

        if prev_line.is_empty() {
            print!("> ");
        }
        else {
            print!("| ");
        }

        match io::stdout().flush() {
            Err(e) => panic!("encountered io error: {e}"),
            _ => { },
        }

        let input = match read(prev_line) {
            Ok(input) => input,
            Err(e) => panic!("encountered io error: {e}"),
        };

        let def_or_exprs = match parser::parse(&input) {
            Ok(x) => x,
            Err(e) => {

                if matches!(e, ParseError::Eof) {
                    prev_line = input.to_string();
                    continue;
                }

                prev_line = String::new();

                println!("parse error");
                continue;
            }
        };

        prev_line = String::new();

        match def_or_exprs {
            ExprOrDef::Def(d) => { 
                defs.push(d); 
            },
            ExprOrDef::Expr(e) => { 
                let mut defs = defs.clone();
                let ops = ops::op_list();
                let op_map : HashMap<Rc<str>, usize> = ops.iter().enumerate().map(|(i, x)| (x.name(), i)).collect();

                defs.insert(0, Def { name: "main".into(), params: vec![], stmts: vec![], body: e});

                match compiler::compile(defs, &op_map) {
                    Ok(fs) => {
                        let mut vm = Vm::new(fs, ops);

                        let output = vm.run(0).unwrap().unwrap();
                        println!("{:?}", output);

                        let globals = vm.with_globals(vec![]);
                        println!("{:?}", globals);
                    },
                    Err(e) => { println!("{}", e); },
                }

            },
        }
    }

}

fn read(mut prev : String) -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    prev.push_str(&s);
    Ok(prev)
}