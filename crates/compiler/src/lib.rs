use lalrpop_util::{lalrpop_mod, ParseError};
use lexer::{Lexer, TokenType};

pub mod codegen_go;
#[cfg(test)]
mod test;

pub type ParseResult<T> = Result<T, ParseError<u32, TokenType, &'static str>>;

pub fn compile(input: &str) -> ParseResult<Vec<ast::Decl<'_>>> {
    let lexer = Lexer::new(input, false, true);

    lalrpop_mod!(interop);
    interop::FileParser::new().parse(input, lexer)
}
