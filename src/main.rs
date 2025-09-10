
mod data;
mod parsing;

use std::io::{self, Write};
use std::rc::Rc;

use crate::parsing::parser;

fn main() {

    let (parse_thread, send_input, rec_output) = parser::init();


    loop {

        print!("> ");

        match io::stdout().flush() {
            Err(e) => panic!("encountered io error: {e}"),
            _ => { },
        }

        let input = match read() {
            Ok(input) => input,
            Err(e) => panic!("encountered io error: {e}"),
        };

        send_input.send(input).expect("encountered parse send error");

    }
}

fn read() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s)
}