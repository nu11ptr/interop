use std::{borrow::Cow, path::PathBuf};

// *** Identifiers ***

#[derive(Clone, Debug, PartialEq)]
pub struct Ident<'input> {
    pub name: Cow<'input, str>,
}

impl AsRef<str> for Ident<'_> {
    fn as_ref(&self) -> &str {
        &self.name
    }
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

// *** Literals ***

#[derive(Clone, Debug, PartialEq)]
pub struct IntLit {
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolLit(pub bool);

// *** String literal ***

#[derive(Clone, Debug, PartialEq)]
pub struct StringLit<'input> {
    pub unparsed: Cow<'input, str>,
    parsed: Option<Cow<'input, str>>,
}

impl<'input> StringLit<'input> {
    pub fn from_str(s: &'input str, has_escapes: bool) -> Self {
        let unparsed = Cow::Borrowed(s);

        // if no escapes, then we are already parsed
        let parsed = if has_escapes {
            None
        } else {
            Some(unparsed.clone())
        };

        Self { unparsed, parsed }
    }

    pub fn from_string(s: String, has_escapes: bool) -> Self {
        let unparsed: Cow<'_, str> = Cow::Owned(s);

        // if no escapes, then we are already parsed
        let parsed = if has_escapes {
            None
        } else {
            Some(unparsed.clone())
        };

        Self { unparsed, parsed }
    }
}

// *** Char literal ***

#[derive(Clone, Debug, PartialEq)]
pub struct CharLit<'input> {
    pub unparsed: Cow<'input, str>,
    parsed: Option<char>,
}

impl<'input> CharLit<'input> {
    pub fn from_str(s: &'input str, has_escapes: bool) -> Self {
        let unparsed = Cow::Borrowed(s);

        // if no escapes, then we are already parsed
        let parsed = if has_escapes {
            None
        } else {
            Some(unparsed.chars().nth(1).unwrap())
        };

        Self { unparsed, parsed }
    }

    pub fn from_string(s: String, has_escapes: bool) -> Self {
        let unparsed: Cow<'_, str> = Cow::Owned(s);

        // if no escapes, then we are already parsed
        let parsed = if has_escapes {
            None
        } else {
            Some(unparsed.chars().nth(1).unwrap())
        };

        Self { unparsed, parsed }
    }
}

// *** Type ***

// TODO: Flesh this out
#[derive(Clone, Debug, PartialEq)]
pub enum Type<'input> {
    Simple(Ident<'input>),
}

impl AsRef<str> for Type<'_> {
    fn as_ref(&self) -> &str {
        match self {
            Type::Simple(ident) => ident.as_ref(),
        }
    }
}

// *** Expressions ***

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'input> {
    If(If<'input>),
    Simple(SimpleExpr<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleExpr<'input> {
    Ident(Ident<'input>),
    IntLit(IntLit),
    StringLit(StringLit<'input>),
    CharLit(CharLit<'input>),
    BoolLit(BoolLit),
    Field(Box<Field<'input>>),
    Call(Box<Call<'input>>),
    IfThenElse(Box<IfThenElse<'input>>),
    BoolCond(Box<BoolCond<'input>>),
    // Expression in parens - should be rare
    Expr(Box<Expr<'input>>),
}

// *** Bool Conditional ***

#[derive(Clone, Debug, PartialEq)]
pub enum BoolCond<'input> {
    Not(SimpleExpr<'input>),
    And(SimpleExpr<'input>, SimpleExpr<'input>),
    Or(SimpleExpr<'input>, SimpleExpr<'input>),
}

// *** If/Then/Else ***

#[derive(Clone, Debug, PartialEq)]
pub struct IfThenElse<'input> {
    pub cond: SimpleExpr<'input>,
    pub then: SimpleExpr<'input>,
    pub else_: SimpleExpr<'input>,
}

// *** Field ***

#[derive(Clone, Debug, PartialEq)]
pub struct Field<'input> {
    pub target: SimpleExpr<'input>,
    pub field: Ident<'input>,
}

// *** Call ***

#[derive(Clone, Debug, PartialEq)]
pub struct Call<'input> {
    pub target: SimpleExpr<'input>,
    pub args: Vec<CallArg<'input>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallArg<'input> {
    pub name: Option<Ident<'input>>,
    pub expr: SimpleExpr<'input>,
}

// *** Block ***

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'input> {
    pub stmt_or_exprs: Vec<StmtOrExpr<'input>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtOrExpr<'input> {
    Func(Func<'input>),
    Expr(Expr<'input>),
}

// *** Function ***

#[derive(Clone, Debug, PartialEq)]
pub struct Func<'input> {
    pub name: Ident<'input>,
    pub args: Vec<FuncArg<'input>>,
    pub body: FuncBody<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncArg<'input> {
    pub name: Ident<'input>,
    pub arg_type: Type<'input>,
    pub default_val: Option<SimpleExpr<'input>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FuncBody<'input> {
    Expr(SimpleExpr<'input>),
    // TODO: Move type into block? (can be used by type inference later, if needed)
    Block(Option<Type<'input>>, Block<'input>),
}

// *** If ***

#[derive(Clone, Debug, PartialEq)]
pub struct If<'input> {
    pub cond: SimpleExpr<'input>,
    pub then_body: Block<'input>,
    pub else_body: Option<ElseBody<'input>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElseBody<'input> {
    // Special case for "else if" since "if" is not a simple expression
    If(Box<If<'input>>),
    Block(Block<'input>),
}

// *** Top level ***

#[derive(Clone, Debug, PartialEq)]
pub enum Decl<'input> {
    Func(Func<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct File<'input> {
    pub path: PathBuf,
    pub decls: Vec<Decl<'input>>,
}
