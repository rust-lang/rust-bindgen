//! Intermediate representation for modules (AKA C++ namespaces).

use clang;
use parse::{ClangSubItemParser, ParseError, ParseResult};
use parse_one;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use super::context::{BindgenContext, ItemId};

/// A module, as in, a C++ namespace.
#[derive(Clone, Debug)]
pub struct Module {
    /// The name of the module, or none if it's anonymous.
    name: Option<String>,
    /// The children of this module, just here for convenience.
    children_ids: Vec<ItemId>,
    /// The set of submodules for this module. We use this to make sure that
    /// there are never duplicate submodules with the same name.
    submodules: RefCell<HashMap<Option<String>, ItemId>>,
}

impl Module {
    /// Construct a new `Module`.
    pub fn new(name: Option<String>) -> Self {
        Module {
            name: name,
            children_ids: vec![],
            submodules: RefCell::new(HashMap::new()),
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

    /// Get the submodule named `submodule`, if one exists.
    pub fn submodule<Q>(&self, submodule: &Q) -> Option<ItemId>
        where Option<String>: Borrow<Q>,
              Q: Hash + Eq
    {
        self.submodules.borrow().get(submodule).cloned()
    }

    /// Add a new submodule with the given `name` and `id`.
    pub fn add_submodule(&self, name: Option<String>, id: ItemId) {
        let mut submodules = self.submodules.borrow_mut();
        submodules.insert(name.into(), id);
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
