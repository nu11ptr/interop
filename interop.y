%token IDENT

%token STRING_LIT
%token INTEGER_LIT
%token CHAR_LIT

%token L_PAREN
%token R_PAREN
%token SEMI
%token COLON
%token ASSIGN
%token COMMA
%token R_ARROW

%token TRUE
%token FALSE

%token FUNC
%token END

%token IF
%token THEN
%token ELSE

%start File

%%

File
    : Decl SEMI
    | File Decl
    ;

Decl
    : FuncDecl
    ;

Type
    : Ident
    ;

// ** Function Decl ***

FuncDecl
    : FUNC Ident FuncArgs FuncBody
    ;

FuncArgs
    : L_PAREN R_PAREN
    | L_PAREN FuncArgsInner FuncArgOptComma R_PAREN
    ;

FuncArgsInner
    : FuncArg
    | FuncArgsInner COMMA FuncArg
    ;

FuncArgOptComma
    :
    | COMMA
    ;

FuncArg
    : FuncArgNames COLON Type DefaultVal
    ;

FuncArgNames
    : Ident
    | FuncArgNames COMMA Ident
    ;

DefaultVal
    :
    | ASSIGN SimpleExpr
    ;

FuncBody
    : R_ARROW SimpleExpr
    | FuncRetType LastBlock
    ;

FuncRetType
    :
    | R_ARROW Type
    ;

LastBlock
    : COLON Exprs END
    ;

// *** Expressions ***

Ident
    : IDENT
    ;

IntLit
    : INTEGER_LIT
    ;

StringLit
    : STRING_LIT
    ;

CharLit
    : CHAR_LIT
    ;

BoolLit
    : TRUE
    | FALSE
    ;

Exprs
    : Expr SEMI
    | Exprs Expr
    ;

SimpleExpr
    : Ident
    | IntLit
    | StringLit
    | CharLit
    | BoolLit
    | L_PAREN Expr R_PAREN
    ;

Expr
    : SimpleExpr
    ;

%%
