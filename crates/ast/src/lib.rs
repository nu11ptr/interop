use std::path::PathBuf;

use lexer::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntLit {
    pub value: i32,
}

// *** Type ***

// TODO: Flesh this out
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Simple(Ident),
}

// *** Expressions ***

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleExpr {
    Ident(Ident),
    IntLit(IntLit),
    // Expression in parens - should be rare
    Expr(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    If(If),
    Simple(SimpleExpr),
}

// *** Block ***

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub expr: Vec<Expr>,
    // else or end keyword
    pub else_or_end: TokenType,
}

// *** Function ***

#[derive(Clone, Debug, PartialEq)]
pub enum FuncBody {
    Expr(SimpleExpr),
    Block(Option<Type>, Block),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncArg {
    pub name: Ident,
    pub arg_type: Type,
    pub default_val: Option<SimpleExpr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncDecl {
    pub name: Ident,
    pub args: Vec<FuncArg>,
    pub body: FuncBody,
}

// *** If ***

#[derive(Clone, Debug, PartialEq)]
pub enum IfBody {
    // Special case for "else if" since "if" is not a simple expression
    If(Box<If>),
    Expr(SimpleExpr),
    Block(Block),
}

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    pub condition: SimpleExpr,
    pub then_body: IfBody,
    pub else_body: Option<IfBody>,
}

// *** Top level ***

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Func(FuncDecl),
}

#[derive(Clone, Debug, PartialEq)]
pub struct File {
    pub path: PathBuf,
    pub decls: Vec<Decl>,
}
