use bumpalo::collections::{string::String, vec::Vec};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident<'bump> {
    pub name: String<'bump>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'bump> {
    UserDefined(Ident<'bump>),
    Unit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field<'bump> {
    pub name: Ident<'bump>,
    pub typ: Vec<'bump, Type<'bump>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'bump> {
    pub name: Ident<'bump>,
    pub params: Vec<'bump, Field<'bump>>,
    pub ret: Type<'bump>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decl<'bump> {
    Function(Function<'bump>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File<'bump> {
    pub path: PathBuf,
    pub content: Vec<'bump, Decl<'bump>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Package<'bump> {
    pub files: Vec<'bump, File<'bump>>,
}
