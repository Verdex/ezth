
mod data;
mod parsing;

use std::io::{self, Write};
use std::rc::Rc;

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

        prev_line = String::new();
    }
}

fn read(mut prev : String) -> io::Result<Rc<str>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    prev.push_str(&s);
    Ok(prev.into())
}