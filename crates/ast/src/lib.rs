use std::{borrow::Cow, path::PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct Ident<'input> {
    pub name: Cow<'input, str>,
}

impl<'input> Ident<'input> {
    pub fn from_str(ident: &'input str) -> Self {
        Self {
            name: Cow::Borrowed(ident),
        }
    }

    pub fn from_string(ident: String) -> Self {
        Self {
            name: Cow::Owned(ident),
        }
    }
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
}

// *** Function ***

#[derive(Clone, Debug, PartialEq)]
pub enum FuncBody<'input> {
    Expr(SimpleExpr<'input>),
    // TODO: Move type into block? (can be used by type inference later, if needed)
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
pub enum ThenBody<'input> {
    // Lack of 'If' prevents: if <cond> then if <cond> then <expr>
    Expr(SimpleExpr<'input>),
    Block(Block<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElseBody<'input> {
    // Special case for "else if" since "if" is not a simple expression
    If(Box<If<'input>>),
    Expr(SimpleExpr<'input>),
    Block(Block<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct If<'input> {
    pub cond: SimpleExpr<'input>,
    pub then_body: ThenBody<'input>,
    pub else_body: Option<ElseBody<'input>>,
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
