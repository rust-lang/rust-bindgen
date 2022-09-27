use syn::{Item, ItemForeignMod};

pub(super) fn merge_extern_blocks(items: &mut Vec<Item>) {
    // Keep all the extern blocks in a different `Vec` for faster search.
    let mut foreign_mods = Vec::<ItemForeignMod>::new();

    for item in std::mem::take(items) {
        match item {
            Item::ForeignMod(ItemForeignMod {
                attrs,
                abi,
                brace_token,
                items: foreign_items,
            }) => {
                let mut exists = false;
                for foreign_mod in &mut foreign_mods {
                    // Check if there is a extern block with the same ABI and
                    // attributes.
                    if foreign_mod.attrs == attrs && foreign_mod.abi == abi {
                        // Merge the items of the two blocks.
                        foreign_mod.items.extend_from_slice(&foreign_items);
                        exists = true;
                        break;
                    }
                }
                // If no existing extern block had the same ABI and attributes, store
                // it.
                if !exists {
                    foreign_mods.push(ItemForeignMod {
                        attrs,
                        abi,
                        brace_token,
                        items: foreign_items,
                    });
                }
            }
            // If the item is not an extern block, we don't have to do anything.
            _ => items.push(item),
        }
    }

    // Move all the extern blocks alongside the rest of the items.
    for foreign_mod in foreign_mods {
        items.push(Item::ForeignMod(foreign_mod));
    }
}
