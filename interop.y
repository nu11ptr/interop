%token IDENT

%token STRING_LIT
%token INTEGER_LIT
%token CHAR_LIT

%token L_PAREN  '('
%token R_PAREN  ')'
%token SEMI     ';'
%token COLON    ':'
%token ASSIGN   '='
%token COMMA    ','
%token R_ARROW  "->"
%token DOT      '.'

%token TRUE     "true"
%token FALSE    "false"

%token FUNC     "func"
%token END      "end"

%token IF       "if"
%token THEN     "then"
%token ELSE     "else"

%start File

%%

File
    : Decl ';'
    | File Decl
    ;

Decl
    : Func
    ;

Type
    : IDENT
    ;

OptComma
    : %empty
    | ','
    ;

// ** Function ***

Closure
    : "func" '(' FuncArgs ')' FuncBody
    ;

Func
    : "func" IDENT '(' FuncArgs ')' FuncBody
    ;

FuncArgs
    : %empty
    | FuncArgsNoVal OptComma
    | FuncArgsDefaultVal OptComma
    | FuncArgsNoVal ',' FuncArgsDefaultVal OptComma
    ;

FuncArgsNoVal
    : FuncArg
    | FuncArgsNoVal ',' FuncArg
    ;

FuncArg
    : IDENT ':' Type
    ;

FuncArgsDefaultVal
    : FuncArgDefaultVal
    | FuncArgsDefaultVal ',' FuncArgDefaultVal

FuncArgDefaultVal
    : FuncArg '=' SimpleExpr
    ;

FuncBody
    : SimpleExpr
    | "->" SimpleExpr
    | FuncRetType Block "end"
    ;

FuncRetType
    : %empty
    | "->" Type
    ;

// *** Blocks ***

Block
    : ':' StmtOrExprs
    ;

StmtOrExprs
    : StmtOrExpr ';'
    | StmtOrExprs StmtOrExpr ';'
    ;

StmtOrExpr
    : Expr
    | Func
    ;

// *** If ***

If
    : "if" Disjunction "then" Block "end"
    | "if" Disjunction "then" Block "else" ElseBody
    ;

ElseBody
    : Block "end"
    | If
    ;

// *** Call ***

CallArgs
    : %empty
    | PosCallArgs OptComma
    | NamedCallArgs OptComma
    | PosCallArgs ',' NamedCallArgs OptComma
    ;

PosCallArgs
    : SimpleExpr
    | PosCallArgs ',' SimpleExpr
    ;

NamedCallArgs
    : NamedCallArg
    | NamedCallArgs ',' NamedCallArg
    ;

NamedCallArg
    : IDENT '=' SimpleExpr
    ;

// *** Expressions ***

Expr
    : If
    | Closure
    | SimpleExpr
    ;

SimpleExpr  
    : "if" Disjunction "then" Disjunction "else" SimpleExpr
    | Disjunction
    ;

Disjunction
    : Disjunction "or" Conjunction
    | Conjunction
    ;

Conjunction
    : Conjunction "and" Inversion
    | Inversion
    ;

Inversion
    : "not" Inversion
    | Primary
    ;

Primary
    : Primary '.' IDENT
    | Primary '(' CallArgs ')'
    | Atom
    ;

Atom
    : INTEGER_LIT
    | STRING_LIT
    | CHAR_LIT
    | "true"
    | "false"
    | IDENT
    | '(' Expr ')'
    ;

%%
