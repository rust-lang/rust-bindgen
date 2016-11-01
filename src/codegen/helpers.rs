//! Helpers for code generation that don't need macro expansion.

use aster;
use ir::layout::Layout;
use syntax::ast;
use syntax::ptr::P;


pub mod attributes {
    use aster;
    use syntax::ast;

    pub fn repr(which: &str) -> ast::Attribute {
        aster::AstBuilder::new().attr().list("repr").words(&[which]).build()
    }

    pub fn repr_list(which_ones: &[&str]) -> ast::Attribute {
        aster::AstBuilder::new().attr().list("repr").words(which_ones).build()
    }

    pub fn derives(which_ones: &[&str]) -> ast::Attribute {
        aster::AstBuilder::new().attr().list("derive").words(which_ones).build()
    }

    pub fn inline() -> ast::Attribute {
        aster::AstBuilder::new().attr().word("inline")
    }

    pub fn doc(comment: &str) -> ast::Attribute {
        aster::AstBuilder::new().attr().doc(comment)
    }

    pub fn link_name(name: &str) -> ast::Attribute {
        aster::AstBuilder::new().attr().name_value("link_name").str(name)
    }
}

/// Generates a proper type for a field or type with a given `Layout`, that is,
/// a type with the correct size and alignment restrictions.
pub struct BlobTyBuilder {
    layout: Layout,
}

impl BlobTyBuilder {
    pub fn new(layout: Layout) -> Self {
        BlobTyBuilder {
            layout: layout,
        }
    }

    pub fn build(self) -> P<ast::Ty> {
        use std::cmp;

        let ty_name = match self.layout.align {
            8 => "u64",
            4 => "u32",
            2 => "u16",
            1 | _ => "u8",
        };
        let data_len = if ty_name == "u8" {
            self.layout.size
        } else {
            self.layout.size / cmp::max(self.layout.align, 1)
        };

        let inner_ty = aster::AstBuilder::new().ty().path().id(ty_name).build();
        if data_len == 1 {
            inner_ty
        } else {
            aster::ty::TyBuilder::new().array(data_len).build(inner_ty)
        }
    }
}
