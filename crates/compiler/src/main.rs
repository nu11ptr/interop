use std::{env, fs};

use compiler::codegen_go;
use compiler::compile;

fn main() {
    // TODO: Replace with clap
    let filename = env::args().nth(1).unwrap();
    let src = fs::read_to_string(filename).unwrap();

    match compile(&src) {
        Ok(ast) => {
            //println!("{ast:#?}");

            let mut codegen = codegen_go::GoCodegen::new();
            let code = codegen.gen_code(&ast);
            println!("{code}");
        }
        e @ _ => println!("Parsing failed: {e:#?}"),
    }
}
