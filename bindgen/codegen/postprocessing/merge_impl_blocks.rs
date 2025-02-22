use syn::{
    visit_mut::{visit_file_mut, visit_item_mod_mut, VisitMut},
    File, Item, ItemImpl, ItemMod,
};

pub(super) fn merge_impl_blocks(file: &mut File) {
    Visitor.visit_file_mut(file);
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_file_mut(&mut self, file: &mut File) {
        visit_items(&mut file.items);
        visit_file_mut(self, file);
    }

    fn visit_item_mod_mut(&mut self, item_mod: &mut ItemMod) {
        if let Some((_, ref mut items)) = item_mod.content {
            visit_items(items);
        }
        visit_item_mod_mut(self, item_mod);
    }
}

fn visit_items(items: &mut Vec<Item>) {
    // Keep all the impl blocks in a different `Vec` for faster search.
    let mut impl_blocks = Vec::<ItemImpl>::new();

    for item in std::mem::take(items) {
        if let Item::Impl(ItemImpl {
            attrs,
            defaultness,
            unsafety,
            impl_token,
            generics,
            trait_: None, // don't merge `impl <Trait> for T` blocks
            self_ty,
            brace_token,
            items: impl_block_items,
        }) = item
        {
            let mut exists = false;
            for impl_block in &mut impl_blocks {
                // Check if there is an equivalent impl block
                if impl_block.attrs == attrs &&
                    impl_block.unsafety == unsafety &&
                    impl_block.generics == generics &&
                    impl_block.self_ty == self_ty
                {
                    // Merge the items of the two blocks.
                    impl_block.items.extend_from_slice(&impl_block_items);
                    exists = true;
                    break;
                }
            }
            // If no matching impl block was found, store it.
            if !exists {
                impl_blocks.push(ItemImpl {
                    attrs,
                    defaultness,
                    unsafety,
                    impl_token,
                    generics,
                    trait_: None,
                    self_ty,
                    brace_token,
                    items: impl_block_items,
                });
            }
        } else {
            // If the item is not an mergeable impl block, we don't have to do
            // anything and just push it back.
            items.push(item);
        }
    }

    // Move all the impl blocks alongside the rest of the items.
    for impl_block in impl_blocks {
        items.push(Item::Impl(impl_block));
    }
}
