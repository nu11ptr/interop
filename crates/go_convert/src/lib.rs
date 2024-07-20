use bumpalo::{collections::vec::Vec, format, Bump};
use convert_case::{Case, Casing};
use gosyn::ast as goast;

#[derive(Default)]
pub struct GoConversion {
    bump: Bump,
}

impl GoConversion {
    pub fn convert(&self, pkg: &goast::Package) -> ast::Package<'_> {
        ast::Package {
            files: Vec::from_iter_in(pkg.files.iter().map(|f| self.convert_file(f)), &self.bump),
        }
    }

    fn convert_file(&self, file: &goast::File) -> ast::File<'_> {
        ast::File {
            path: file.path.clone(),
            content: Vec::from_iter_in(
                file.decl.iter().flat_map(|decl| self.convert_decl(decl)),
                &self.bump,
            ),
        }
    }

    fn convert_decl(&self, decl: &goast::Declaration) -> Option<ast::Decl<'_>> {
        match decl {
            goast::Declaration::Function(f) => Some(ast::Decl::Function(ast::Function {
                name: self.convert_ident(&f.name),
                // temp
                params: Vec::new_in(&self.bump),
                // temp
                ret: ast::Type::Unit,
            })),
            goast::Declaration::Type(_) => None,
            goast::Declaration::Const(_) => None,
            goast::Declaration::Variable(_) => None,
        }
    }

    fn convert_ident(&self, ident: &goast::Ident) -> ast::Ident<'_> {
        ast::Ident {
            name: format!(in &self.bump, "{}", ident.name.to_case(Case::Snake)),
        }
    }
}
