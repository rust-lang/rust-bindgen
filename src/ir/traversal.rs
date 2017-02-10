//! Traversal of the graph of IR items and types.

use super::context::{BindgenContext, ItemId};
use super::item::ItemSet;

/// Collect all the type items referenced by this item.
pub trait Trace {
    /// If a particular type needs extra information beyond what it has in
    /// `self` and `context` to find its referenced type items, its
    /// implementation can define this associated type, forcing callers to pass
    /// the needed information through.
    type Extra;

    /// Add each type item referenced by `self` into the `types` set.
    fn trace(&self,
                     context: &BindgenContext,
                     types: &mut ItemSet,
                     extra: &Self::Extra);
}

/// An graph traversal of the transitive closure of references between items.
///
/// See `BindgenContext::whitelisted_items` for more information.
pub struct ItemTraversal<'ctx, 'gen>
    where 'gen: 'ctx,
{
    ctx: &'ctx BindgenContext<'gen>,

    /// The set of whitelisted items we have seen. If you think of traversing
    /// whitelisted items like GC tracing, this is the mark bits, and contains
    /// both black and gray items.
    seen: ItemSet,

    /// The set of whitelisted items that we have seen but have yet to iterate
    /// over and collect transitive references from. To return to the GC analogy,
    /// this is the mark stack, containing the set of gray items which we have
    /// not finished tracing yet.
    to_iterate: Vec<ItemId>,
}

impl<'ctx, 'gen> ItemTraversal<'ctx, 'gen>
    where 'gen: 'ctx,
{
    /// Begin a new traversal, starting from the given roots.
    pub fn new<R>(ctx: &'ctx BindgenContext<'gen>,
                  roots: R)
                  -> ItemTraversal<'ctx, 'gen>
        where R: IntoIterator<Item = ItemId>,
    {
        // Construct the ItemSet first. Because its underlying storage is a
        // BTreeSet, its iteration over its entries is ordered, and the roots
        // end up ordered as well. This contributes enables stable,
        // deterministic generated names in the bindings.
        let seen: ItemSet = roots.into_iter().collect();
        let roots: Vec<_> = seen.iter().cloned().collect();

        ItemTraversal {
            ctx: ctx,
            seen: seen,
            to_iterate: roots,
        }
    }
}

impl<'ctx, 'gen> Iterator for ItemTraversal<'ctx, 'gen>
    where 'gen: 'ctx,
{
    type Item = ItemId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = match self.to_iterate.pop() {
            None => return None,
            Some(id) => id,
        };

        debug_assert!(self.seen.contains(&id));
        debug_assert!(self.ctx.resolve_item_fallible(id).is_some());

        if self.ctx.options().whitelist_recursively {
            let mut sub_types = ItemSet::new();
            id.trace(self.ctx, &mut sub_types, &());

            for id in sub_types {
                if self.seen.insert(id) {
                    self.to_iterate.push(id);
                }
            }
        }

        Some(id)
    }
}
