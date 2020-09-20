%token <string> IDENT
%token <int> INTEGER
%token <float> DOUBLE
%token <bool> BOOLEAN
%token FUNC END LET VAR IF THEN ELSE
%token PLUS MINUS MULT DIV AND OR
%token RARROW LPAREN RPAREN
%token EQUALS
%token SEMI COLON COMMA EOF

%type <bool> let_var
%type <string> ident func_ret_type basic_type ident_type
%type <Ast.binop> binop

%type <Ast.func_args> func_decl_arg

%type <Ast.decl> func_decl
%type <Ast.prog> program

%type <Ast.expr> simple_expr expr if_then_block if_then_else

%type <Ast.stmt> stmt bind
%type <Ast.block> block else_block

(* Ordered weakest to strongest precedence *)
%left OR
%left AND
%left PLUS MINUS
%left MULT DIV

%start program
%%

program :
    | f=list(func_decl) EOF { f }
    ;

stmt :
    | b=bind SEMI { b }
    | expr=expr SEMI { Ast.Expr (expr) }
    ;

block :
    (* Ideally, make this 's=separated_list(SEMI, stmt)' (and remove all other SEMI)
    but need to figure out how to handle the ASI to do this *)
    | COLON s=list(stmt) END { s }
    ;

let_var :
    | LET { false }
    | VAR { true }
    ;

basic_type :
    | t=ident { t }
    ;

ident_type :
    | COLON t=basic_type { t }
    ;

bind :
    | m=let_var v=ident t=option(ident_type) EQUALS e=expr 
    { Ast.LetBind (Location.create $startpos $endpos, m, v, Types.basicOpt t, e) }
    ;

func_ret_type :
    | RARROW t=basic_type { t }
    ;

func_decl :
    | FUNC name=ident LPAREN args=func_decl_args RPAREN rt=option(func_ret_type) bl=block SEMI
    { let ft, rt' = Types.basic_func (Ast.arg_types args) rt in 
      Ast.FuncDecl (Location.create $startpos $endpos, ft, name, args, rt', bl) }
    ;

func_decl_args :
    | a=separated_list(COMMA, func_decl_arg) { a }
    ;

func_decl_arg :
    | v=separated_nonempty_list(COMMA, ident) t=ident_type 
    { {Ast.location=Location.create $startpos $endpos; Ast.arg_vars=v; Ast.arg_type=Types.basic t} }
    ;

if_then :
    | IF ie=simple_expr THEN { ie }
    ;

if_then_else :
    (* 
    NOTE: basic_expr needed due to ambituity with binop when not using parens
    Wanted: (if <expr> then <expr> else <expr>) + <expr>
    -vs-
    Not wanted: if <expr> then <expr> else (<expr> + <expr>)
    *)
    | ie=if_then te=simple_expr ELSE ee=basic_expr 
    { Ast.IfThenElse (Location.create $startpos $endpos, Types.basic "", ie, te, ee) }
    ;

if_then_block :
    | ie=if_then COLON s=list(stmt) ebl=else_block 
    { Ast.IfThenBlock (Location.create $startpos $endpos, Types.basic "", ie, s, ebl) }
    ;

else_block :
    | END { [] }
    | ELSE itb=if_then_block { [Ast.Expr(itb)] }
    | ELSE ebl=block { ebl }
    ;

ident :
    | i=IDENT { i }
    ;

expr :
    | e=simple_expr { e }
    | itb=if_then_block { itb }
    ;

simple_expr :
    | lhs=simple_expr op=binop rhs=simple_expr 
    { Ast.BinOp (Location.create $startpos $endpos, Types.basic "", lhs, op, rhs) }
    | e=basic_expr { e }
    ;

basic_expr :
    | i=ident { Ast.Ident (Location.create $startpos $endpos, Types.basic "", i) }
    | i=INTEGER { Ast.Integer (Location.create $startpos $endpos, i) }
    | d=DOUBLE { Ast.Double (Location.create $startpos $endpos, d) }
    | b=BOOLEAN { Ast.Boolean (Location.create $startpos $endpos, b) }
    | LPAREN e=simple_expr RPAREN { e }
    | ite=if_then_else { ite }
    | name=ident LPAREN args=separated_list(COMMA, simple_expr) RPAREN 
    { Ast.FuncCall (Location.create $startpos $endpos, Types.basic "", name, args) }
    ;

%inline binop :
    | PLUS { Ast.Plus }
    | MINUS { Ast.Minus }
    | MULT { Ast.Mult }
    | DIV { Ast.Div }
    | AND { Ast.And }
    | OR { Ast.Or }
    ;

%%
