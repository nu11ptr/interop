use std::{env, fs};

use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let src = fs::read_to_string(filename).unwrap();

    let lexer = Lexer::new(&src, false, true);

    lalrpop_mod!(interop);
    let parse = interop::FileParser::new().parse(&src, lexer);

    match parse {
        Ok(ast) => println!("{ast:#?}"),
        e @ _ => println!("Parsing failed: {e:#?}"),
    }
}
