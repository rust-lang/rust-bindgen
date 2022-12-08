use proc_macro2::Ident;
use syn::{
    visit_mut::{visit_item_mod_mut, VisitMut},
    Item, ItemMod,
};

pub(super) fn sort_semantically(item_mod: &mut ItemMod) {
    Visitor.visit_item_mod_mut(item_mod)
}

fn sorting_structure(item: &Item) -> (Option<&Ident>, usize) {
    let ident = match item {
        Item::Use(_) => (None, 0),
        Item::Type(i) => (Some(&i.ident), 1),
        Item::Struct(i) => (Some(&i.ident), 2),
        Item::Impl(_) => (None, 4),
        Item::Const(i) => (Some(&i.ident), 3),
        Item::Enum(i) => (Some(&i.ident), 5),
        Item::Union(i) => (Some(&i.ident), 6),
        Item::Static(i) => (Some(&i.ident), 7),
        Item::Trait(i) => (Some(&i.ident), 8),
        Item::TraitAlias(i) => (Some(&i.ident), 9),
        Item::Fn(i) => (Some(&i.sig.ident), 10),
        Item::Mod(i) => (Some(&i.ident), 11),
        Item::Verbatim(_) => (None, 12),
        Item::ExternCrate(i) => (Some(&i.ident), 13),
        Item::ForeignMod(_) => (None, 14),
        Item::Macro(_) => (None, 15),
        Item::Macro2(_) => (None, 16),
        _ => (None, 17),
    };
    ident
}

fn prepare_item_data_structure(item: &Item) -> (String, usize) {
    let (item_ident_string, item_sort_position) = match sorting_structure(item)
    {
        (Some(item), pos) => (item.to_string(), pos),
        //push None to the bottom using clever trick of ZZzzzzs
        (None, pos) => ("ZZZ".to_string(), pos),
    };
    (item_ident_string, item_sort_position)
}

fn get_sortable_items(items: &Vec<Item>) -> Vec<(&Item, String, usize)> {
    let mut output = Vec::new();
    for item in items {
        let (ident, pos) = prepare_item_data_structure(&item);
        output.push((item, ident, pos));
    }
    output
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_item_mod_mut(&mut self, item_mod: &mut ItemMod) {
        if let Some((_, ref mut items)) = item_mod.content {
            let mut sortable_items = get_sortable_items(items);
            sortable_items
                .sort_by_key(|sortable| (sortable.2, sortable.clone().1));
        }
        visit_item_mod_mut(self, item_mod)
    }
}
