use syn::visit_mut::{visit_type_mut, VisitMut};
use syn::{Item, ItemMod, Type};

use crate::BindgenOptions;

pub(super) fn remove_alias(item_mod: &mut ItemMod, options: &BindgenOptions) {
    if let Some((_, items)) = item_mod.content.as_mut() {
        let visitors: Vec<_> = items
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(index, item)| {
                if let Item::Type(alias_item) = item {
                    if alias_item.generics.params.is_empty() {
                        let ident = alias_item.ident.to_string();
                        if options.remove_alias.matches(&ident) {
                            return Some((
                                index,
                                Visitor {
                                    ident,
                                    ty: alias_item.ty.clone(),
                                },
                            ));
                        }
                    }
                }
                None
            })
            .collect();

        for (index, mut visitor) in visitors {
            items.remove(index);
            for item in items.iter_mut() {
                visitor.visit_item_mut(item);
            }
        }
    }
}

struct Visitor {
    ident: String,
    ty: Box<Type>,
}

impl VisitMut for Visitor {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::Path(type_path) = ty {
            if type_path.path.is_ident(&self.ident) {
                *ty = self.ty.as_ref().clone();
            }
        }

        visit_type_mut::<_>(self, ty)
    }
}
