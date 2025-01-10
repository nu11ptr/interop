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
%token DOT

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
    : FUNC Ident L_PAREN FuncArgs R_PAREN FuncBody
    ;

FuncArgs
    : %empty
    | FuncArgsInner FuncArgOptComma
    ;

FuncArgsInner
    : FuncArg
    | FuncArgsInner COMMA FuncArg
    ;

FuncArgOptComma
    : %empty
    | COMMA
    ;

FuncArg
    : Ident COLON Type DefaultVal
    ;

DefaultVal
    : %empty
    | ASSIGN SimpleExpr
    ;

FuncBody
    : R_ARROW SimpleExpr
    | FuncRetType Block END
    ;

FuncRetType
    : %empty
    | R_ARROW Type
    ;

// *** Blocks ***

Block
    : COLON Exprs
    ;

Exprs
    : Expr SEMI
    | Exprs Expr SEMI
    ;

// *** If ***

If
    : IF Disjunction THEN Block END
    | IF Disjunction THEN Block ELSE ElseBody
    ;

ElseBody
    : Block END
    | If
    ;

// *** Call ***

CallArgs
    : %empty
    | PosCallArgs CallArgOptComma
    | NamedCallArgs CallArgOptComma
    | PosCallArgs COMMA NamedCallArgs CallArgOptComma
    ;

PosCallArgs
    : SimpleExpr
    | PosCallArgs COMMA SimpleExpr
    ;

NamedCallArgs
    : NamedCallArg
    | NamedCallArgs COMMA NamedCallArg
    ;

NamedCallArg
    : Ident ASSIGN SimpleExpr
    ;

CallArgOptComma
    : %empty
    | COMMA
    ;

// *** Ident and Literals *** /

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

// *** Expressions ***

Expr
    : If
    | SimpleExpr
    ;

SimpleExpr  
    : IF Disjunction THEN Disjunction ELSE SimpleExpr
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
    : Primary DOT Ident
    | Primary L_PAREN CallArgs R_PAREN
    | Atom
    ;

Atom
    : IntLit
    | StringLit
    | CharLit
    | BoolLit
    | Ident
    | L_PAREN Expr R_PAREN
    ;

%%
