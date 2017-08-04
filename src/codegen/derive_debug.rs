use ir::comp::{BitfieldUnit, CompKind, Field, FieldData, FieldMethods};
use ir::context::BindgenContext;
use ir::derive::CanTriviallyDeriveDebug;
use ir::item::{IsOpaque, Item, ItemCanonicalName};
use ir::ty::{TypeKind, RUST_DERIVE_IN_ARRAY_LIMIT};
use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::Token;

use syntax::tokenstream::TokenTree;

pub fn gen_debug_impl(
    ctx: &BindgenContext,
    fields: &[Field],
    item: &Item,
    kind: CompKind,
) -> Vec<ast::ImplItem> {
    let struct_name = item.canonical_name(ctx);
    let mut format_string = format!("{} {{{{ ", struct_name);
    let mut tokens: Vec<TokenTree> = Vec::new();

    if item.is_opaque(ctx, &()) {
        format_string.push_str("opaque");
    } else {
        match kind {
            CompKind::Union => {
                format_string.push_str("union");
            }
            CompKind::Struct => {
                let processed_fields = fields.iter().filter_map(|f| match f {
                    &Field::DataMember(ref fd) => {
                        gen_field_data_debug_impl(ctx, fd)
                    }
                    &Field::Bitfields(ref bu) => {
                        gen_bitfield_unit_debug_impl(ctx, bu)
                    }
                });


                for (i, (fstring, token)) in processed_fields.enumerate() {
                    if i > 0 {
                        format_string.push_str(", ");
                    }
                    if !token.is_empty() {
                        tokens.push(TokenTree::Token(DUMMY_SP, Token::Comma));
                        tokens.extend(token);
                    }
                    format_string.push_str(&fstring);
                }
            }
        }
    }

    format_string.push_str(" }}");

    let impl_ = quote_item!(ctx.ext_cx(),
                            impl X {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    write!(f, $format_string $tokens)
                                }    
                            });

    match impl_.unwrap().node {
        ast::ItemKind::Impl(_, _, _, _, _, ref items) => items.clone(),
        _ => unreachable!(),
    }
}

fn gen_field_data_debug_impl(
    ctx: &BindgenContext,
    data: &FieldData,
) -> Option<(String, Vec<TokenTree>)> {
    if let Some(name) = data.name() {
        gen_item_debug_impl(ctx, ctx.resolve_item(data.ty()), name)
    } else {
        None
    }
}

fn gen_bitfield_unit_debug_impl(
    ctx: &BindgenContext,
    data: &BitfieldUnit,
) -> Option<(String, Vec<TokenTree>)> {
    let mut format_string = String::new();
    let mut tokens = Vec::new();
    for (i, bu) in data.bitfields().iter().enumerate() {
        if i > 0 {
            format_string.push_str(", ");
            tokens.push(TokenTree::Token(DUMMY_SP, Token::Comma));
        }
        format_string.push_str(&format!("{} : {{:?}}", bu.name()));
        let name_ident = ctx.rust_ident_raw(bu.name());
        tokens.extend(quote_tokens!(ctx.ext_cx(), self.$name_ident()));
    }

    Some((format_string, tokens))
}

fn gen_item_debug_impl(
    ctx: &BindgenContext,
    item: &Item,
    name: &str,
) -> Option<(String, Vec<TokenTree>)> {
    let name_ident = ctx.rust_ident_raw(name);

    let ty = match item.as_type() {
        Some(ty) => ty,
        None => {
            return None;
        }
    };

    fn debug_print(
        ctx: &BindgenContext,
        name: &str,
        name_ident: ast::Ident,
    ) -> Option<(String, Vec<TokenTree>)> {
        Some((
            format!("{}: {{:?}}", name),
            quote_tokens!(ctx.ext_cx(), self.$name_ident),
        ))
    }

    match *ty.kind() {
        // Handle the simple cases.
        TypeKind::Void |
        TypeKind::NullPtr |
        TypeKind::Int(..) |
        TypeKind::Float(..) |
        TypeKind::Complex(..) |
        TypeKind::Function(..) |
        TypeKind::Enum(..) |
        TypeKind::Reference(..) |
        TypeKind::BlockPointer |
        TypeKind::UnresolvedTypeRef(..) |
        TypeKind::ObjCInterface(..) |
        TypeKind::ObjCId |
        TypeKind::Comp(..) |
        TypeKind::ObjCSel => debug_print(ctx, name, name_ident),

        TypeKind::TemplateInstantiation(ref inst) => {
            if inst.is_opaque(ctx, item) {
                Some((format!("{}: opaque", name), vec![]))
            } else {
                debug_print(ctx, name, name_ident)
            }
        } 

        // The generic is not required to implement Debug, so we can not debug print that type
        TypeKind::Named => {
            Some((format!("{}: Non-debuggable generic", name), vec![]))
        }

        TypeKind::Array(_, len) => {
            // Generics are not required to implement Debug
            if ctx.lookup_item_id_has_type_param_in_array(&item.id()) {
                Some((format!("{}: Array with length {}", name, len), vec![]))
            } else if len < RUST_DERIVE_IN_ARRAY_LIMIT {
                // The simple case
                debug_print(ctx, name, name_ident)
            } else {
                // Let's implement our own print function
                Some((
                    format!("{}: [{{}}]", name),
                    quote_tokens!(
                        ctx.ext_cx(),
                        self.$name_ident
                            .iter()
                            .enumerate()
                            .map(|(i, v)| format!("{}{:?}", if i > 0 { ", " } else { "" }, v))
                            .collect::<String>()),
                ))
            }
        }

        TypeKind::ResolvedTypeRef(t) |
        TypeKind::TemplateAlias(t, _) |
        TypeKind::Alias(t) => {
            // We follow the aliases
            gen_item_debug_impl(ctx, ctx.resolve_item(t), name)
        }

        TypeKind::Pointer(inner) => {
            let inner_type = ctx.resolve_type(inner).canonical_type(ctx);
            match *inner_type.kind() {
                TypeKind::Function(ref sig)
                    if !sig.can_trivially_derive_debug() =>
                {
                    Some((format!("{}: FunctionPointer", name), vec![]))
                }
                _ => debug_print(ctx, name, name_ident),
            }
        }

        TypeKind::Opaque => None,
    }
}
