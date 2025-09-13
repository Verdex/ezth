
mod data;
mod parsing;

use std::io::{self, Write};
use std::rc::Rc;

use crate::data::*;
use crate::parsing::parser;

fn main() {

    let (parse_thread, send_input, rec_output) = parser::init();

    print!("> ");

    loop {


        match io::stdout().flush() {
            Err(e) => panic!("encountered io error: {e}"),
            _ => { },
        }

        let input = match read() {
            Ok(input) => input,
            Err(e) => panic!("encountered io error: {e}"),
        };

        send_input.send(input).expect("encountered parse send error");
        let result = rec_output.recv().expect("encountered parse recv error");

        match result {
            ParseResult::Success(v) => { 
                println!("{:?}", v);
                print!("> ");
            },
            ParseResult::Fatal(i) => { println!("fatal at {}", i); },
            ParseResult::Incremental => { print!("| "); },
        }
    }
}

fn read() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s)
}