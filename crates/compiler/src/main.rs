use std::{env, fs};

use compiler::compile;

fn main() {
    // TODO: Replace with clap
    let filename = env::args().nth(1).unwrap();
    let src = fs::read_to_string(filename).unwrap();

    match compile(&src) {
        Ok(ast) => println!("{ast:#?}"),
        e @ _ => println!("Parsing failed: {e:#?}"),
    }
}
