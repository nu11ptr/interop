use bumpalo::collections::{String, Vec};
use indexmap::IndexSet;
use std::{hash::Hash, ops::Index, path::PathBuf};

// *** Nodetracker ***

pub struct NodeTracker<I, N> {
    nodes: IndexSet<N>,
    phantom: std::marker::PhantomData<I>,
}

// Implement manually so that N doesn't require Default
impl<I, N> Default for NodeTracker<I, N> {
    fn default() -> Self {
        Self {
            nodes: IndexSet::default(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<I: From<usize>, N: Hash + Eq> NodeTracker<I, N> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: IndexSet::with_capacity(capacity),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get_or_insert(&mut self, node: N) -> I {
        self.nodes.insert_full(node).0.into()
    }
}

impl<I: Into<usize>, N> Index<I> for NodeTracker<I, N> {
    type Output = N;

    fn index(&self, index: I) -> &Self::Output {
        &self.nodes[index.into()]
    }
}

// *** Nodetracker Indices ***

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct TypeIdx(u32);

impl From<usize> for TypeIdx {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<TypeIdx> for usize {
    fn from(value: TypeIdx) -> Self {
        value.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct IdentIdx(u32);

impl From<usize> for IdentIdx {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<IdentIdx> for usize {
    fn from(value: IdentIdx) -> Self {
        value.0 as usize
    }
}

// *** AstBuilder ***

#[derive(Default)]
pub struct AstBuilder<'bump> {
    pub types: NodeTracker<TypeIdx, Type<'bump>>,
    pub idents: NodeTracker<IdentIdx, Ident<'bump>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Ident<'bump> {
    pub name: String<'bump>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Type<'bump> {
    Custom {
        typ: IdentIdx,
        type_params: Vec<'bump, TypeIdx>,
    },
    Integer,
    Float,
    Boolean,
    Unit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: IdentIdx,
    pub typ: TypeIdx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'bump> {
    pub name: IdentIdx,
    pub params: Vec<'bump, Field>,
    pub ret: TypeIdx,
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
