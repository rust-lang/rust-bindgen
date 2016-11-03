//! Intermediate representation for modules (AKA C++ namespaces).

use clang;
use parse::{ClangSubItemParser, ParseError, ParseResult};
use parse_one;
use super::context::BindgenContext;
use super::item::ItemId;

/// A module, as in, a C++ namespace.
#[derive(Clone, Debug)]
pub struct Module {
    /// The name of the module, or none if it's anonymous.
    name: Option<String>,
    /// The children of this module, just here for convenience.
    children_ids: Vec<ItemId>,
}

impl Module {
    /// Construct a new `Module`.
    pub fn new(name: Option<String>) -> Self {
        Module {
            name: name,
            children_ids: vec![],
        }
    }

    /// Get this module's name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| &**n)
    }

    /// Get a mutable reference to this module's children.
    pub fn children_mut(&mut self) -> &mut Vec<ItemId> {
        &mut self.children_ids
    }

    /// Get this module's children.
    pub fn children(&self) -> &[ItemId] {
        &self.children_ids
    }
}

impl ClangSubItemParser for Module {
    fn parse(cursor: clang::Cursor,
             ctx: &mut BindgenContext)
             -> Result<ParseResult<Self>, ParseError> {
        use clangll::*;
        match cursor.kind() {
            CXCursor_Namespace => {
                let module_id = ctx.module(cursor);
                ctx.with_module(module_id, |ctx, children| {
                    cursor.visit(|cursor| {
                        parse_one(ctx, cursor, Some(module_id), children)
                    })
                });

                Ok(ParseResult::AlreadyResolved(module_id))
            }
            _ => Err(ParseError::Continue),
        }
    }
}
