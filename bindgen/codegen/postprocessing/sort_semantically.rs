use syn::Item;

pub(super) fn sort_semantically(items: &mut [Item]) {
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
