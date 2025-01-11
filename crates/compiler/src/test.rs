use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

use crate::ParseResult;

// *** function ***

fn func_parser(src: &str) -> ParseResult<ast::Func<'_>> {
    let lexer = Lexer::new(src, false, false);

    lalrpop_mod!(interop);
    interop::FuncParser::new().parse(src, lexer)
}

#[test]
fn func_arrow_type_block() {
    let src = r"
        func my_func() -> Int:
            123
        end";
    let _actual = func_parser(src).expect("func node");
}

#[test]
fn func_with_args_block() {
    let src = r"
        func my_func(a: Int, b: String) -> Int:
            123
        end";
    let _actual = func_parser(src).expect("func node");
}

#[test]
fn func_nested_func_block() {
    let src = r"
        func my_func2():
            func my_func5() println
        end";
    let _actual = func_parser(src).expect("func node");
}

#[test]
fn func_no_type_block() {
    let src = r"
        func my_func2():
            123
        end";
    let _actual = func_parser(src).expect("func node");
}

#[test]
fn func_arrow_no_type() {
    let src = r"
        func my_func3() -> 123";
    let _actual = func_parser(src).expect("func node");
}

#[test]
fn func_no_type() {
    let src = r"
        func my_func4() println";
    let _actual = func_parser(src).expect("func node");
}

#[test]
fn func_no_parens() {
    let src = r"
        func my_func4:
            println
        end";
    let actual = func_parser(src);
    assert!(actual.is_err());
}

// *** function args ***

fn func_args_parser(src: &str) -> ParseResult<Vec<ast::FuncArg<'_>>> {
    let lexer = Lexer::new(src, false, false);

    lalrpop_mod!(interop);
    interop::FuncArgsParser::new().parse(src, lexer)
}

#[test]
fn func_args_comma_list_all_types() {
    let src = r"a: Int, b: Int";
    let _actual = func_args_parser(src).expect("function arg node");
}

#[test]
fn func_args_comma_list_one_type_default_val() {
    let src = r"a: Int, b: Int = 123";
    let _actual = func_args_parser(src).expect("function arg node");
}

#[test]
fn func_args_missing_type() {
    let src = r"a, b: Int";
    let actual = func_args_parser(src);
    assert!(actual.is_err());
}

#[test]
fn func_args_empty() {
    let src = r"";
    let actual = func_args_parser(src);
    assert!(actual.is_err());
}

#[test]
fn func_args_pos_arg_after_default_val() {
    let src = r"a: Int = 123, b: Int";
    let actual = func_args_parser(src);
    assert!(actual.is_err());
}

// *** Expr ***

fn expr_parser(src: &str) -> ParseResult<ast::Expr<'_>> {
    let lexer = Lexer::new(src, false, false);

    lalrpop_mod!(interop);
    interop::ExprParser::new().parse(src, lexer)
}

#[test]
fn field_basic() {
    let src = r"test.field";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn field_on_call() {
    let src = r"test().field";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn field_on_wrapped_expr() {
    let src = r"(test).field";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_no_args() {
    let src = r"test()";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_from_field() {
    let src = r"test.field()";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_on_call() {
    let src = r"test()()";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_on_wrapped_expr() {
    let src = r"(test)()";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_pos_args() {
    let src = r#"test(a, 123, "test")"#;
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_named_args() {
    let src = r#"test(a=a, b=123, c="test")"#;
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_pos_and_named_args() {
    let src = r#"test(a, b=123, c="test")"#;
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn call_pos_after_named_args() {
    let src = r#"test(a=123, b="test", a)"#;
    let actual = expr_parser(src);
    assert!(actual.is_err());
}

#[test]
fn if_then_else_block() {
    let src = r"
        if test then:
            blah
            123
        else:
            456
            blah
        end";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn if_then_block() {
    let src = r"
        if test then:
            blah
            123
        end";
    let _actual = expr_parser(src).expect("if node");
}

#[test]
fn if_then_else_else_if_block() {
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
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn if_then_else_else_if_nested_block() {
    let src = r"
        if test then:
            blah
            if test then 5 else 4
        else if test2 then:
            456
            blah
        else:
            if test then:
                blah
                123
            else:
                blah
            end

            789
        end";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn if_then_block_with_simple_else() {
    let src = r"
        if test then:
            blah
            123
        else 456";
    let actual = expr_parser(src);
    assert!(actual.is_err());
}

#[test]
fn if_simple_then_with_else_block() {
    let src = r"
        if test then blah else:
            456
            blah
        end";
    let actual = expr_parser(src);
    assert!(actual.is_err());
}

#[test]
fn if_then_else_basic() {
    let src = r"if test then 123 else 456";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn if_then_else_minus_else() {
    let src = r"if test then blah";
    let actual = expr_parser(src);
    assert!(actual.is_err());
}

#[test]
fn if_then_else_compound_then() {
    let src = r"if test then 123 else if test2 then 5 else 3";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn if_then_else_compound_cond() {
    let src = r"if if test2 then 5 else 3 then 123 else 456";
    let actual = expr_parser(src);
    assert!(actual.is_err());
}

#[test]
fn if_then_else_compound_wrapped_cond() {
    let src = r"if (if test2 then 5 else 3) then 123 else 456";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn bool_cond_or() {
    let src = r"a or b";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn bool_cond_and() {
    let src = r"a and b";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn bool_cond_not() {
    let src = r"not a";
    let _actual = expr_parser(src).expect("expr node");
}

#[test]
fn bool_cond_compound() {
    let src = r"not a or b and c";
    let _actual = expr_parser(src).expect("expr node");
}
