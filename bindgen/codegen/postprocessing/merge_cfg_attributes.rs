use crate::HashMap;
use crate::HashSet;
use itertools::Itertools;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::token::Unsafe;
use syn::Abi;
use syn::{
    Attribute, File, ForeignItem, Ident, Item, ItemConst, ItemEnum, ItemFn,
    ItemForeignMod, ItemImpl, ItemMod, ItemStatic, ItemStruct, ItemType,
    ItemUnion, ItemUse,
};

pub fn merge_cfg_attributes(file: &mut File) {
    let mut visitor = Visitor::new();
    visitor.visit_file(file);
}

struct SyntheticMod {
    attrs: AttributeSet,
    unsafety: Option<Unsafe>,
    abi: Option<Abi>,
    items: Vec<Item>,
}

impl SyntheticMod {
    pub fn new(attrs: AttributeSet) -> Self {
        Self {
            attrs,
            unsafety: None,
            abi: None,
            items: vec![],
        }
    }
}

#[derive(Default, Clone)]
struct AttributeSet {
    cfg_attrs: HashSet<Attribute>,
    cc_attrs: HashSet<Attribute>,
    other_attrs: HashSet<Attribute>,
    unsafety: Option<Unsafe>,
    abi: Option<Abi>,
}

impl AttributeSet {
    fn new(
        attrs: &[Attribute],
        unsafety: Option<Unsafe>,
        abi: Option<Abi>,
    ) -> Self {
        let mut attribute_set = AttributeSet::default();

        for attr in attrs {
            let target_set = if let Some(ident) = attr.path().get_ident() {
                match ident.to_string().as_str() {
                    "cfg" => &mut attribute_set.cfg_attrs,
                    "link" => &mut attribute_set.cc_attrs,
                    _ => &mut attribute_set.other_attrs,
                }
            } else {
                &mut attribute_set.other_attrs
            };
            target_set.insert(attr.clone());
        }
        attribute_set.unsafety = unsafety;
        attribute_set.abi = abi;

        attribute_set
    }

    fn extend(
        &mut self,
        attrs: &[Attribute],
        unsafety: Option<Unsafe>,
        abi: Option<Abi>,
    ) {
        let other = AttributeSet::new(attrs, unsafety, abi);
        self.other_attrs.extend(other.other_attrs);
        self.cfg_attrs.extend(other.cfg_attrs);
        self.cc_attrs.extend(other.cc_attrs);

        self.unsafety = other.unsafety.or(self.unsafety);
        self.abi = other.abi.or(self.abi.clone());
    }

    fn ident(&self) -> Ident {
        Ident::new(
            Itertools::intersperse(
                self.unsafety
                    .map(|r#unsafe| r#unsafe.to_token_stream().to_string())
                    .into_iter()
                    .chain(
                        self.abi
                            .as_ref()
                            .map(|abi| abi.to_token_stream().to_string()),
                    )
                    .chain(
                        self.cfg_attrs
                            .iter()
                            .chain(self.cc_attrs.iter())
                            .map(|attr| attr.to_token_stream().to_string())
                            .sorted(),
                    ),
                "_".to_string(),
            )
            .collect::<String>()
            .replace(|c: char| !c.is_alphanumeric(), "_")
            .chars()
            .coalesce(|a, b| {
                if a == '_' && b == '_' {
                    Ok(a)
                } else {
                    Err((a, b))
                }
            })
            .collect::<String>()
            .trim_matches('_'),
            Span::call_site(),
        )
    }
}

struct Visitor {
    synthetic_mods: HashMap<Ident, SyntheticMod>,
    new_items: Vec<Item>,
}

impl Visitor {
    fn new() -> Self {
        Self {
            synthetic_mods: HashMap::default(),
            new_items: Vec::new(),
        }
    }

    fn visit_file(&mut self, file: &mut File) {
        self.visit_items(&mut file.items);

        for (
            ref mut ident,
            SyntheticMod {
                ref mut attrs,
                ref mut unsafety,
                ref mut abi,
                ref mut items,
            },
        ) in self.synthetic_mods.drain()
        {
            let cfg_attrs = attrs.cfg_attrs.iter().collect::<Vec<_>>();
            let cc_attrs = attrs.cc_attrs.iter().collect::<Vec<_>>();
            let synthetic_mod = if abi.is_some() {
                quote! {
                    #(#cfg_attrs)*
                    pub mod #ident {
                        #(#cc_attrs)*
                        #unsafety #abi {
                            #(#items)*
                        }
                    }
                }
            } else {
                quote! {
                    #(#cfg_attrs)*
                    pub mod #ident {
                        #(#items)*
                    }
                }
            };

            self.new_items.push(Item::Verbatim(quote! {
                #synthetic_mod

                #(#cfg_attrs)*
                pub use #ident::*;
            }));
        }

        file.items = std::mem::take(&mut self.new_items);
    }

    fn visit_items(&mut self, items: &mut Vec<Item>) {
        for mut item in std::mem::take(items) {
            match &mut item {
                Item::Const(ItemConst { ref mut attrs, .. }) |
                Item::Struct(ItemStruct { ref mut attrs, .. }) |
                Item::Enum(ItemEnum { ref mut attrs, .. }) |
                Item::Union(ItemUnion { ref mut attrs, .. }) |
                Item::Type(ItemType { ref mut attrs, .. }) |
                Item::Use(ItemUse { ref mut attrs, .. }) |
                Item::Static(ItemStatic { ref mut attrs, .. }) |
                Item::Mod(ItemMod { ref mut attrs, .. }) |
                Item::Impl(ItemImpl { ref mut attrs, .. }) |
                Item::Fn(ItemFn { ref mut attrs, .. }) => {
                    let attr_set = AttributeSet::new(attrs, None, None);
                    *attrs = attr_set.other_attrs.iter().cloned().collect();
                    self.insert_item_into_mod(attr_set, item);
                }
                Item::ForeignMod(foreign_mod) => {
                    self.visit_foreign_mod(foreign_mod);
                }
                _ => {
                    self.new_items.push(item);
                }
            }
        }
    }

    fn visit_foreign_mod(&mut self, foreign_mod: &mut ItemForeignMod) {
        for mut foreign_item in std::mem::take(&mut foreign_mod.items) {
            // When MSRV >= 1.79.0 we can return &Vec::new() in the generic case as this wll get lifetime extended,
            // see also https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html#extending-automatic-temporary-lifetime-extension.
            let mut _attrs = vec![];
            let (inner_attrs, inner_unsafety, inner_abi) =
                match &mut foreign_item {
                    ForeignItem::Fn(f) => {
                        (&mut f.attrs, f.sig.unsafety, f.sig.abi.clone())
                    }
                    ForeignItem::Static(s) => (&mut s.attrs, None, None),
                    ForeignItem::Type(t) => (&mut t.attrs, None, None),
                    ForeignItem::Macro(m) => (&mut m.attrs, None, None),
                    _ => (&mut _attrs, None, None),
                };

            let mut attr_set = AttributeSet::new(
                &foreign_mod.attrs,
                foreign_mod.unsafety,
                Some(foreign_mod.abi.clone()),
            );
            attr_set.extend(inner_attrs, inner_unsafety, inner_abi);
            *inner_attrs = attr_set.other_attrs.iter().cloned().collect();

            self.insert_item_into_mod(
                attr_set,
                Item::Verbatim(quote! { #foreign_item }),
            );
        }
    }

    fn insert_item_into_mod(&mut self, attr_set: AttributeSet, item: Item) {
        if !attr_set.cfg_attrs.is_empty() || !attr_set.cc_attrs.is_empty() {
            let ident = attr_set.ident();
            let synthetic_mod = self
                .synthetic_mods
                .entry(ident)
                .or_insert_with(|| SyntheticMod::new(attr_set.clone()));
            synthetic_mod.items.push(item);
            synthetic_mod.unsafety = attr_set.unsafety;
            synthetic_mod.abi = attr_set.abi.clone();
            synthetic_mod.attrs = attr_set;
        } else {
            self.new_items.push(item);
        }
    }
}