
mod data;
mod parsing;
mod compiler;

use std::io::{self, Write};

use crate::data::*;
use crate::parsing::parser;




fn main() {

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

        println!("{:?}", def_or_exprs);
    }

}

fn read(mut prev : String) -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    prev.push_str(&s);
    Ok(prev)
}