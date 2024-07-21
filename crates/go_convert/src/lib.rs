use std::cell::RefCell;

use ast::IdentIdx;
use bumpalo::{collections::Vec, format, Bump};
use convert_case::{Case, Casing};
use gosyn::ast as goast;

pub struct GoConversion<'bump> {
    builder: RefCell<ast::AstBuilder<'bump>>,
    bump: &'bump Bump,
}

impl<'bump> GoConversion<'bump> {
    pub fn new(bump: &'bump Bump) -> Self {
        Self {
            builder: RefCell::new(ast::AstBuilder::default()),
            bump,
        }
    }

    pub fn convert(&self, pkg: &goast::Package) -> ast::Package<'_> {
        let mut files = Vec::with_capacity_in(pkg.files.len(), self.bump);
        for file in &pkg.files {
            files.push(self.convert_file(file));
        }

        ast::Package { files }
    }

    fn convert_file(&self, file: &goast::File) -> ast::File<'_> {
        ast::File {
            path: file.path.clone(),
            content: Vec::from_iter_in(
                file.decl.iter().flat_map(|decl| self.convert_decl(decl)),
                self.bump,
            ),
        }
    }

    fn convert_decl(&self, decl: &goast::Declaration) -> Option<ast::Decl<'_>> {
        match decl {
            goast::Declaration::Function(f) => {
                // temp
                let ret = self
                    .builder
                    .borrow_mut()
                    .types
                    .get_or_insert(ast::Type::Unit);
                Some(ast::Decl::Function(ast::Function {
                    name: self.convert_ident(&f.name),
                    // temp
                    params: Vec::new_in(self.bump),
                    ret,
                }))
            }
            goast::Declaration::Type(_) => None,
            goast::Declaration::Const(_) => None,
            goast::Declaration::Variable(_) => None,
        }
    }

    fn convert_ident(&self, ident: &goast::Ident) -> IdentIdx {
        let ident = ast::Ident {
            name: format!(in &self.bump, "{}", ident.name.to_case(Case::Snake)),
        };
        self.builder.borrow_mut().idents.get_or_insert(ident)
    }
}
