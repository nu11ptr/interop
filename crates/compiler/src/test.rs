use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

// *** If ***

fn if_parser(src: &str) -> ast::If<'_> {
    let lexer = Lexer::new(src, false, false);

    lalrpop_mod!(interop);
    interop::IfParser::new().parse(src, lexer).expect("if node")
}

#[test]
fn if_with_simple_then_and_else() {
    let src = r"if test then 123 else 456";
    let _actual = if_parser(src);
}

#[test]
fn if_then_block_with_simple_else() {
    let src = r"
        if test then:
            blah
            123
        else 456";
    let _actual = if_parser(src);
}

#[test]
fn if_simple_then_with_else_block() {
    let src = r"
        if test then blah else:
            456
            blah
        end";
    let _actual = if_parser(src);
}

#[test]
fn if_then_and_else_block() {
    let src = r"
        if test then:
            blah
            123
        else:
            456
            blah
        end";
    let _actual = if_parser(src);
}

#[test]
fn if_then_block() {
    let src = r"
        if test then:
            blah
            123
        end";
    let _actual = if_parser(src);
}

#[test]
fn if_then_simple() {
    let src = r"if test then blah";
    let _actual = if_parser(src);
}

#[test]
fn if_with_simple_then_else_and_else_if() {
    let src = r"if test then 123 else if test2 then 5 else 3";
    let _actual = if_parser(src);
}

#[test]
fn if_then_else_and_else_if_block() {
    let src = r"
        if test then:
            blah
            123
        else if test2 then:
            456
            blah
        else:
            blah
            789
        end";
    let _actual = if_parser(src);
}
