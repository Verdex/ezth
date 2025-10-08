
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

    let mut top_level = vec![];
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

        let top = match parser::parse(&input) {
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

        match top {
            ReplTopLevel::Def(d) => { 
                top_level.push(TopLevel::Def(d)); 
            },
            ReplTopLevel::Expr(e) => { 
                let mut top_level = top_level.clone();
                let ops = ops::op_list();
                let op_map : HashMap<Rc<str>, usize> = ops.iter().enumerate().map(|(i, x)| (x.name(), i)).collect();

                top_level.insert(0, TopLevel::Def(Def { name: "main".into(), params: vec![], stmts: vec![], body: e}));

                match compiler::compile(top_level, &op_map) {
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
            ReplTopLevel::Pat(p) => {
                top_level.push(TopLevel::Pat(p));
            }
        }
    }

}

fn read(mut prev : String) -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    prev.push_str(&s);
    Ok(prev)
}