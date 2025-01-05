use lalrpop_util::{lalrpop_mod, ParseError};
use lexer::{Lexer, TokenType};

#[cfg(test)]
mod test;

pub fn compile(input: &str) -> Result<Vec<ast::Decl<'_>>, ParseError<u32, TokenType, &str>> {
    let lexer = Lexer::new(input, false, true);

    lalrpop_mod!(interop);
    interop::FileParser::new().parse(input, lexer)
}
