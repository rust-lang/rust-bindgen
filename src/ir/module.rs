use super::context::BindgenContext;
use super::item::ItemId;
use clang;
use parse::{ClangSubItemParser, ParseError, ParseResult};
use parse_one;

/// A module, as in, a C++ namespace.
#[derive(Clone, Debug)]
pub struct Module {
    /// The name of the module, or none if it's anonymous.
    name: Option<String>,
    /// The children of this module, just here for convenience.
    children_ids: Vec<ItemId>,
}

impl Module {
    pub fn new(name: Option<String>) -> Self {
        Module {
            name: name,
            children_ids: vec![],
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| &**n)
    }

    pub fn children_mut(&mut self) -> &mut Vec<ItemId> {
        &mut self.children_ids
    }

    pub fn children(&self) -> &[ItemId] {
        &self.children_ids
    }
}

impl ClangSubItemParser for Module {
    fn parse(cursor: clang::Cursor, ctx: &mut BindgenContext) -> Result<ParseResult<Self>, ParseError> {
        use clangll::*;
        match cursor.kind() {
            CXCursor_Namespace => {
                let module_id = ctx.module(cursor);
                ctx.with_module(module_id, |ctx, children| {
                    cursor.visit(|cursor, _| parse_one(ctx, *cursor, Some(module_id), children))
                });

                Ok(ParseResult::AlreadyResolved(module_id))
            }
            _ => Err(ParseError::Continue)
        }
    }
}
