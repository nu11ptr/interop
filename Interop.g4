grammar Interop
	;

program: (let_bind ';' | func_bind ';')* EOF;

block: ':' stmt* 'end';

stmt: let_bind ';' | expr ';';

expr: simple_expr | func_lit;

basic_expr
	: IDENT
	| INTEGER
	| DOUBLE
	| BOOL
	| '(' simple_expr ')'
	| IDENT '(' simple_expr* ')'
	;

simple_expr
	: basic_expr
	| simple_expr ('*' | '/') simple_expr
	| simple_expr ('+' | '-') simple_expr
	| simple_expr 'and' simple_expr
	| simple_expr 'or' simple_expr
	;

// *** types ***

type_ident: IDENT;

// *** bindings ***

name_type: IDENT (':' type_ident)?;

let_bind: 'let' 'var'? name_type (',' name_type)* '=' expr;

rebind: IDENT '=' expr;

func_bind: 'func' IDENT func_body;

// *** func ***

func_lit: 'func' func_body;

func_body: '(' func_args? ')' ('->' type_ident)? block;

func_args: func_arg (',' func_arg)*;

func_arg: IDENT (',' IDENT)* ':' type_ident ('=' simple_expr)?;

// *** if ***

if_then: 'if' simple_expr 'then';

if_then_else: if_then simple_expr 'else' basic_expr;

if_then_block: if_then ':' stmt* else_block;

else_block: 'end' | 'else' if_then_block | 'else' block;

// *** Tokens - TODO ***

IDENT: [a-zA-Z]+;
INTEGER: [0-9]+;
DOUBLE: INTEGER '.' INTEGER;
BOOL: ('true' | 'false');
