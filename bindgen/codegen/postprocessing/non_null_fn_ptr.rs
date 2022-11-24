use std::fmt::Write;

use syn::{
    visit_mut::{
        visit_field_mut, visit_fn_arg_mut, visit_item_enum_mut,
        visit_item_struct_mut, visit_item_union_mut, visit_signature_mut,
        visit_variant_mut, VisitMut,
    },
    AngleBracketedGenericArguments, Field, FnArg, GenericArgument, Ident,
    ItemMod, Pat, PatIdent, Path, PathArguments, PathSegment, Type, TypePath,
};

use crate::regex_set::RegexSet;

pub(super) fn non_null_fn_ptr(
    item_mod: &mut ItemMod,
    options: &crate::BindgenOptions,
) {
    Visitor::new(&options.non_null_fn_ptr).visit_item_mod_mut(item_mod)
}

struct Visitor<'vis> {
    path: Vec<Ident>,
    regex_set: &'vis RegexSet,
}

impl<'vis> Visitor<'vis> {
    fn new(regex_set: &'vis RegexSet) -> Self {
        Self {
            path: Default::default(),
            regex_set,
        }
    }

    fn with_ident(&mut self, ident: Ident, f: impl FnOnce(&mut Self)) {
        self.path.push(ident);
        f(self);
        let _ = self.path.pop();
    }

    fn path_matches(&self) -> bool {
        let mut idents = self.path.iter();

        let mut path = String::new();

        let head = idents.next().expect("This should never be empty!");
        write!(path, "{}", head).unwrap();
        for ident in idents {
            write!(path, "::{}", ident).unwrap();
        }

        self.regex_set.matches(&path)
    }
}

impl<'vis> VisitMut for Visitor<'vis> {
    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        self.with_ident(item.ident.clone(), |this| {
            visit_item_struct_mut(this, item)
        });
    }

    fn visit_item_union_mut(&mut self, item: &mut syn::ItemUnion) {
        self.with_ident(item.ident.clone(), |this| {
            visit_item_union_mut(this, item)
        });
    }

    fn visit_item_enum_mut(&mut self, item: &mut syn::ItemEnum) {
        self.with_ident(item.ident.clone(), |this| {
            visit_item_enum_mut(this, item);
        });
    }

    fn visit_variant_mut(&mut self, variant: &mut syn::Variant) {
        self.with_ident(variant.ident.clone(), |this| {
            visit_variant_mut(this, variant);
        });
    }

    fn visit_signature_mut(&mut self, sig: &mut syn::Signature) {
        self.with_ident(sig.ident.clone(), |this| {
            visit_signature_mut(this, sig);
        });
    }

    fn visit_field_mut(&mut self, field: &mut Field) {
        if let Some(bare_fn) = extract_fn_pointer(&field.ty) {
            if let Some(ident) = &field.ident {
                let bare_fn = bare_fn.clone();
                self.with_ident(ident.clone(), |this| {
                    if this.path_matches() {
                        field.ty = bare_fn;
                    }
                });
            } else if self.path_matches() {
                field.ty = bare_fn.clone();
            }
        }

        visit_field_mut(self, field)
    }

    fn visit_fn_arg_mut(&mut self, fn_arg: &mut FnArg) {
        if let FnArg::Typed(syn::PatType { pat, ty, .. }) = fn_arg {
            if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                if let Some(bare_fn) = extract_fn_pointer(ty).cloned() {
                    self.with_ident(ident.clone(), |this| {
                        if this.path_matches() {
                            **ty = bare_fn;
                        }
                    })
                }
            }
        }

        visit_fn_arg_mut(self, fn_arg)
    }
}

fn extract_fn_pointer(ty: &Type) -> Option<&Type> {
    if let Type::Path(TypePath {
        qself: None,
        path: Path { segments, .. },
    }) = ty
    {
        let mut path = String::new();

        for segment in segments {
            write!(path, "::{}", segment.ident).unwrap();
        }

        if let "::std::option::Option" | "::core::option::Option" =
            path.as_str()
        {
            if let Some(PathSegment {
                arguments:
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }),
                ..
            }) = segments.last()
            {
                if args.len() == 1 {
                    if let GenericArgument::Type(arg_ty @ Type::BareFn(_)) =
                        &args[0]
                    {
                        return Some(arg_ty);
                    }
                }
            }
        }
    }

    None
}
