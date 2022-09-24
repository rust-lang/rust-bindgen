use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

use crate::BindgenOptions;

mod merge_extern_blocks;
mod sort_semantically;

use merge_extern_blocks::merge_extern_blocks;
use sort_semantically::sort_semantically;

struct PostProcessingPass {
    should_run: fn(&BindgenOptions) -> bool,
    run: fn(&mut Vec<Item>),
}

// TODO: This can be a const fn when mutable references are allowed in const
// context.
macro_rules! pass {
    ($pass:ident) => {
        PostProcessingPass {
            should_run: |options| options.$pass,
            run: |items| $pass(items),
        }
    };
}

const PASSES: &[PostProcessingPass] =
    &[pass!(merge_extern_blocks), pass!(sort_semantically)];

pub(crate) fn postprocessing(
    items: Vec<TokenStream>,
    options: &BindgenOptions,
) -> TokenStream {
    let require_syn = PASSES.iter().any(|pass| (pass.should_run)(options));
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
    let (_, mut items) = syn::parse2::<syn::ItemMod>(module_wrapped_tokens)
        .unwrap()
        .content
        .unwrap();

    for pass in PASSES {
        if (pass.should_run)(options) {
            (pass.run)(&mut items);
        }
    }

    let synful_items = items.into_iter().map(|item| item.into_token_stream());

    quote! { #( #synful_items )* }
}
