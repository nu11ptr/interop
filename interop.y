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
    : FuncDecl
    ;

Type
    : IDENT
    ;

OptComma
    : %empty
    | ','
    ;

// ** Function Decl ***

FuncDecl
    : "func" IDENT '(' FuncArgs ')' FuncBody
    ;

FuncArgs
    : %empty
    | FuncArgsInner OptComma
    ;

FuncArgsInner
    : FuncArg
    | FuncArgsInner ',' FuncArg
    ;

FuncArg
    : IDENT ':' Type DefaultVal
    ;

DefaultVal
    : %empty
    | '=' SimpleExpr
    ;

FuncBody
    : "->" SimpleExpr
    | FuncRetType Block "end"
    ;

FuncRetType
    : %empty
    | "->" Type
    ;

// *** Blocks ***

Block
    : ':' Exprs
    ;

Exprs
    : ExprSemi
    | Exprs ExprSemi
    ;

ExprSemi
    : Expr ';'
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
