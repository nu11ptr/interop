use std::{borrow::Cow, path::PathBuf};

use lexer::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub struct Ident<'input> {
    pub name: Cow<'input, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntLit {
    pub value: i32,
}

// *** Type ***

// TODO: Flesh this out
#[derive(Clone, Debug, PartialEq)]
pub enum Type<'input> {
    Simple(Ident<'input>),
}

// *** Expressions ***

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleExpr<'input> {
    Ident(Ident<'input>),
    IntLit(IntLit),
    // Expression in parens - should be rare
    Expr(Box<Expr<'input>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'input> {
    If(If<'input>),
    Func(FuncDecl<'input>),
    Simple(SimpleExpr<'input>),
}

// *** Block ***

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'input> {
    pub expr: Vec<Expr<'input>>,
    // else or end keyword
    pub else_or_end: TokenType,
}

// *** Function ***

#[derive(Clone, Debug, PartialEq)]
pub enum FuncBody<'input> {
    Expr(SimpleExpr<'input>),
    Block(Option<Type<'input>>, Block<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncArg<'input> {
    pub name: Ident<'input>,
    pub arg_type: Type<'input>,
    pub default_val: Option<SimpleExpr<'input>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncDecl<'input> {
    pub name: Ident<'input>,
    pub args: Vec<FuncArg<'input>>,
    pub body: FuncBody<'input>,
}

// *** If ***

#[derive(Clone, Debug, PartialEq)]
pub enum IfBody<'input> {
    // Special case for "else if" since "if" is not a simple expression
    If(Box<If<'input>>),
    Expr(SimpleExpr<'input>),
    Block(Block<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct If<'input> {
    pub condition: SimpleExpr<'input>,
    pub then_body: IfBody<'input>,
    pub else_body: Option<IfBody<'input>>,
}

// *** Top level ***

#[derive(Clone, Debug, PartialEq)]
pub enum Decl<'input> {
    Func(FuncDecl<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct File<'input> {
    pub path: PathBuf,
    pub decls: Vec<Decl<'input>>,
}
