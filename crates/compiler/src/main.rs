use pest::Parser;
use pest_derive::Parser;
use std::{env, fs};

#[derive(Parser)]
#[grammar = "../../interop.pest"]
pub struct InteropParser;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let src = fs::read_to_string(filename).unwrap();
    let parse = InteropParser::parse(Rule::grammar, &src);

    println!("{:#?}", parse);
}
