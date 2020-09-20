grammar interop;

program: (func_bind ';')* EOF;

block: ':' (stmt ';')* 'end';

stmt: let_bind | expr;

expr: simple_expr | func_lit;

basic_expr:
	IDENT
	| INTEGER
	| DOUBLE
	| BOOL
	| '(' simple_expr ')'
	| IDENT '(' simple_expr* ')';

simple_expr:
	basic_expr
	| simple_expr ('*' | '/') simple_expr
	| simple_expr ('+' | '-') simple_expr
	| simple_expr 'and' simple_expr
	| simple_expr 'or' simple_expr;

type: IDENT;

name_type: IDENT (':' type)?;

let_bind: 'let' 'var'? name_type (',' name_type)* '=' expr;

rebind: IDENT '=' expr;

func_bind: 'func' IDENT func;

func_lit: 'func' func;

func: '(' func_args? ')' ('->' type)? block;

func_args: func_arg (',' func_arg)*;

func_arg: IDENT (',' IDENT)* ':' type ('=' simple_expr)?;

if_then: 'if' simple_expr 'then';

if_then_else: if_then simple_expr 'else' basic_expr;

if_then_block: if_then ':' stmt* else_block;

else_block: 'end' | 'else' if_then_block | 'else' block;
