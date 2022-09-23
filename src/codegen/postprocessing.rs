use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

use crate::BindgenOptions;

macro_rules! decl_postprocessing {
    ($($ty:ty),*) => {
        pub(crate) fn postprocessing(
            items: Vec<TokenStream>,
            options: &BindgenOptions,
        ) -> TokenStream {
            // Whether any of the enabled options requires `syn`.
            let require_syn = $(<$ty as PostProcessing>::should_run(options))||*;

            if !require_syn {
                return items.into_iter().collect();
            }

            let module_wrapped_tokens =
                quote!(mod wrapper_for_sorting_hack { #( #items )* });

            // This syn business is a hack, for now. This means that we are re-parsing already
            // generated code using `syn` (as opposed to `quote`) because `syn` provides us more
            // control over the elements.
            // One caveat is that some of the items coming from `quote`d output might have
            // multiple items within them. Hence, we have to wrap the incoming in a `mod`.
            // The two `unwrap`s here are deliberate because
            //      The first one won't panic because we build the `mod` and know it is there
            //      The second one won't panic because we know original output has something in
            //      it already.
            let mut items =
                syn::parse2::<syn::ItemMod>(module_wrapped_tokens)
                    .unwrap()
                    .content
                    .unwrap()
                    .1;

            $(if <$ty as PostProcessing>::should_run(options) {
                <$ty as PostProcessing>::run(&mut items);
            })*

            let synful_items = items
                .into_iter()
                .map(|item| item.into_token_stream());

            quote! { #( #synful_items )* }
        }
    };
}

decl_postprocessing! {
    MergeExternBlocks,
    SortSemantically
}

trait PostProcessing {
    fn should_run(options: &BindgenOptions) -> bool;

    fn run(items: &mut Vec<Item>);
}

struct SortSemantically;

impl PostProcessing for SortSemantically {
    #[inline]
    fn should_run(options: &BindgenOptions) -> bool {
        options.sort_semantically
    }

    fn run(items: &mut Vec<Item>) {
        items.sort_by_key(|item| match item {
            Item::Type(_) => 0,
            Item::Struct(_) => 1,
            Item::Const(_) => 2,
            Item::Fn(_) => 3,
            Item::Enum(_) => 4,
            Item::Union(_) => 5,
            Item::Static(_) => 6,
            Item::Trait(_) => 7,
            Item::TraitAlias(_) => 8,
            Item::Impl(_) => 9,
            Item::Mod(_) => 10,
            Item::Use(_) => 11,
            Item::Verbatim(_) => 12,
            Item::ExternCrate(_) => 13,
            Item::ForeignMod(_) => 14,
            Item::Macro(_) => 15,
            Item::Macro2(_) => 16,
            _ => 18,
        });
    }
}

struct MergeExternBlocks;

impl PostProcessing for MergeExternBlocks {
    #[inline]
    fn should_run(options: &BindgenOptions) -> bool {
        options.merge_extern_blocks
    }

    fn run(items: &mut Vec<Item>) {
        // Keep all the extern blocks in a different `Vec` for faster search.
        let mut foreign_mods = Vec::<syn::ItemForeignMod>::new();

        for item in std::mem::take(items) {
            match item {
                Item::ForeignMod(syn::ItemForeignMod {
                    attrs,
                    abi,
                    brace_token,
                    items: foreign_items,
                }) => {
                    let mut exists = false;
                    for foreign_mod in &mut foreign_mods {
                        // Check if there is a extern block with the same ABI and
                        // attributes.
                        if foreign_mod.attrs == attrs && foreign_mod.abi == abi
                        {
                            // Merge the items of the two blocks.
                            foreign_mod.items.extend_from_slice(&foreign_items);
                            exists = true;
                            break;
                        }
                    }
                    // If no existing extern block had the same ABI and attributes, store
                    // it.
                    if !exists {
                        foreign_mods.push(syn::ItemForeignMod {
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

        // Move all the extern blocks alongiside the rest of the items.
        for foreign_mod in foreign_mods {
            items.push(Item::ForeignMod(foreign_mod));
        }
    }
}
