//! Helpers for code generation that don't need macro expansion.

use proc_macro2::{Ident, Span};

use crate::ir::context::BindgenContext;
use crate::ir::layout::Layout;

pub(crate) mod attributes {
    use proc_macro2::{Ident, Span, TokenStream};
    use std::{borrow::Cow, str::FromStr};

    pub(crate) fn repr(which: &str) -> TokenStream {
        let which = Ident::new(which, Span::call_site());
        quote! {
            #[repr( #which )]
        }
    }

    pub(crate) fn repr_list(which_ones: &[&str]) -> TokenStream {
        let which_ones = which_ones
            .iter()
            .map(|one| TokenStream::from_str(one).expect("repr to be valid"));
        quote! {
            #[repr( #( #which_ones ),* )]
        }
    }

    pub(crate) fn derives(which_ones: &[&str]) -> TokenStream {
        let which_ones = which_ones
            .iter()
            .map(|one| TokenStream::from_str(one).expect("derive to be valid"));
        quote! {
            #[derive( #( #which_ones ),* )]
        }
    }

    pub(crate) fn inline() -> TokenStream {
        quote! {
            #[inline]
        }
    }

    pub(crate) fn must_use() -> TokenStream {
        quote! {
            #[must_use]
        }
    }

    pub(crate) fn non_exhaustive() -> TokenStream {
        quote! {
            #[non_exhaustive]
        }
    }

    pub(crate) fn doc(comment: &str) -> TokenStream {
        if comment.is_empty() {
            quote!()
        } else {
            quote!(#[doc = #comment])
        }
    }

    pub(crate) fn link_name<const MANGLE: bool>(name: &str) -> TokenStream {
        // LLVM mangles the name by default but it's already mangled.
        // Prefixing the name with \u{1} should tell LLVM to not mangle it.
        let name: Cow<'_, str> = if MANGLE {
            name.into()
        } else {
            format!("\u{1}{name}").into()
        };

        quote! {
            #[link_name = #name]
        }
    }
}

/// The `ffi_safe` argument should be true if this is a type that the user might
/// reasonably use, e.g. not struct padding, where the `__BindgenOpaqueArray` is
/// just noise.
/// TODO: Should this be `MaybeUninit`, since padding bytes are effectively
/// uninitialized?
pub(crate) fn blob(
    ctx: &BindgenContext,
    layout: Layout,
    ffi_safe: bool,
) -> syn::Type {
    let opaque = layout.opaque();

    // FIXME(emilio, #412): We fall back to byte alignment, but there are
    // some things that legitimately are more than 8-byte aligned.
    //
    // Eventually we should be able to `unwrap` here, but...
    let ty = opaque.known_rust_type_for_array().unwrap_or_else(|| {
        warn!("Found unknown alignment on code generation!");
        syn::parse_quote! { u8 }
    });

    let data_len = opaque.array_size().unwrap_or(layout.size);

    if data_len == 1 {
        ty
    } else if ffi_safe && ctx.options().rust_features().min_const_generics {
        ctx.generated_opaque_array();
        if ctx.options().enable_cxx_namespaces {
            syn::parse_quote! { root::__BindgenOpaqueArray<#ty, #data_len> }
        } else {
            syn::parse_quote! { __BindgenOpaqueArray<#ty, #data_len> }
        }
    } else {
        // This is not FFI safe as an argument; the struct above is
        // preferable.
        syn::parse_quote! { [ #ty ; #data_len ] }
    }
}

/// Integer type of the same size as the given `Layout`.
pub(crate) fn integer_type(layout: Layout) -> Option<syn::Type> {
    Layout::known_type_for_size(layout.size)
}

pub(crate) const BITFIELD_UNIT: &str = "__BindgenBitfieldUnit";

/// Generates a bitfield allocation unit type for a type with the given `Layout`.
pub(crate) fn bitfield_unit(ctx: &BindgenContext, layout: Layout) -> syn::Type {
    let size = layout.size;
    let bitfield_unit_name = Ident::new(BITFIELD_UNIT, Span::call_site());
    let ty = syn::parse_quote! { #bitfield_unit_name<[u8; #size]> };

    if ctx.options().enable_cxx_namespaces {
        return syn::parse_quote! { root::#ty };
    }

    ty
}

pub(crate) mod ast_ty {
    use crate::ir::context::BindgenContext;
    use crate::ir::function::FunctionSig;
    use crate::ir::layout::Layout;
    use crate::ir::ty::{FloatKind, IntKind};
    use crate::ir::var::LiteralRadix;
    use crate::RustTarget;
    use proc_macro2::TokenStream;
    use std::str::FromStr;

    pub(crate) fn c_void(ctx: &BindgenContext) -> syn::Type {
        // ctypes_prefix takes precedence
        match ctx.options().ctypes_prefix {
            Some(ref prefix) => {
                let prefix = TokenStream::from_str(prefix.as_str()).unwrap();
                syn::parse_quote! { #prefix::c_void }
            }
            None => {
                if ctx.options().use_core {
                    syn::parse_quote! { ::core::ffi::c_void }
                } else {
                    syn::parse_quote! { ::std::os::raw::c_void }
                }
            }
        }
    }

    pub(crate) fn raw_type(ctx: &BindgenContext, name: &str) -> syn::Type {
        let ident = ctx.rust_ident_raw(name);
        match ctx.options().ctypes_prefix {
            Some(ref prefix) => {
                let prefix = TokenStream::from_str(prefix.as_str()).unwrap();
                syn::parse_quote! { #prefix::#ident }
            }
            None => {
                if ctx.options().use_core &&
                    ctx.options().rust_features().core_ffi_c
                {
                    syn::parse_quote! { ::core::ffi::#ident }
                } else {
                    syn::parse_quote! { ::std::os::raw::#ident }
                }
            }
        }
    }

    pub(crate) fn int_kind_rust_type(
        ctx: &BindgenContext,
        ik: IntKind,
        layout: Option<Layout>,
    ) -> syn::Type {
        match ik {
            IntKind::Bool => syn::parse_quote! { bool },
            IntKind::Char { .. } => raw_type(ctx, "c_char"),
            // The following is used only when an unusual command-line
            // argument is used. bindgen_cchar16_t is not a real type;
            // but this allows downstream postprocessors to distinguish
            // this case and do something special for C++ bindings
            // containing the C++ type char16_t.
            IntKind::Char16 => syn::parse_quote! { bindgen_cchar16_t },
            IntKind::SChar => raw_type(ctx, "c_schar"),
            IntKind::UChar => raw_type(ctx, "c_uchar"),
            IntKind::Short => raw_type(ctx, "c_short"),
            IntKind::UShort => raw_type(ctx, "c_ushort"),
            IntKind::Int => raw_type(ctx, "c_int"),
            IntKind::UInt => raw_type(ctx, "c_uint"),
            IntKind::Long => raw_type(ctx, "c_long"),
            IntKind::ULong => raw_type(ctx, "c_ulong"),
            IntKind::LongLong => raw_type(ctx, "c_longlong"),
            IntKind::ULongLong => raw_type(ctx, "c_ulonglong"),
            IntKind::WChar => {
                let layout =
                    layout.expect("Couldn't compute wchar_t's layout?");
                Layout::known_type_for_size(layout.size)
                    .expect("Non-representable wchar_t?")
            }

            IntKind::I8 => syn::parse_quote! { i8 },
            IntKind::U8 => syn::parse_quote! { u8 },
            IntKind::I16 => syn::parse_quote! { i16 },
            IntKind::U16 => syn::parse_quote! { u16 },
            IntKind::I32 => syn::parse_quote! { i32 },
            IntKind::U32 => syn::parse_quote! { u32 },
            IntKind::I64 => syn::parse_quote! { i64 },
            IntKind::U64 => syn::parse_quote! { u64 },
            IntKind::Custom { name, .. } => {
                syn::parse_str(name).expect("Invalid integer type.")
            }
            IntKind::U128 => {
                if true {
                    syn::parse_quote! { u128 }
                } else {
                    // Best effort thing, but wrong alignment
                    // unfortunately.
                    syn::parse_quote! { [u64; 2] }
                }
            }
            IntKind::I128 => {
                if true {
                    syn::parse_quote! { i128 }
                } else {
                    syn::parse_quote! { [u64; 2] }
                }
            }
        }
    }

    pub(crate) fn float_kind_rust_type(
        ctx: &BindgenContext,
        fk: FloatKind,
        layout: Option<Layout>,
    ) -> syn::Type {
        // TODO: we probably should take the type layout into account more
        // often?
        //
        // Also, maybe this one shouldn't be the default?
        match (fk, ctx.options().convert_floats) {
            (FloatKind::Float16, _) => {
                // TODO: do f16 when rust lands it
                ctx.generated_bindgen_float16();
                if ctx.options().enable_cxx_namespaces {
                    syn::parse_quote! { root::__BindgenFloat16 }
                } else {
                    syn::parse_quote! { __BindgenFloat16 }
                }
            }
            (FloatKind::Float, true) => syn::parse_quote! { f32 },
            (FloatKind::Double, true) => syn::parse_quote! { f64 },
            (FloatKind::Float, false) => raw_type(ctx, "c_float"),
            (FloatKind::Double, false) => raw_type(ctx, "c_double"),
            (FloatKind::LongDouble, _) => {
                if let Some(layout) = layout {
                    match layout.size {
                        4 => syn::parse_quote! { f32 },
                        8 => syn::parse_quote! { f64 },
                        // TODO(emilio): If rust ever gains f128 we should
                        // use it here and below.
                        _ => super::integer_type(layout)
                            .unwrap_or(syn::parse_quote! { f64 }),
                    }
                } else {
                    debug_assert!(
                        false,
                        "How didn't we know the layout for a primitive type?"
                    );
                    syn::parse_quote! { f64 }
                }
            }
            (FloatKind::Float128, _) => {
                if true {
                    syn::parse_quote! { u128 }
                } else {
                    syn::parse_quote! { [u64; 2] }
                }
            }
        }
    }

    fn integer_with_radix(
        val: u64,
        is_negative: bool,
        radix: &LiteralRadix,
    ) -> TokenStream {
        let sign = if is_negative { "-" } else { "" };
        let val = match radix {
            LiteralRadix::Binary => format!("{sign}0b{val:b}"),
            LiteralRadix::Octal => format!("{sign}0o{val:o}"),
            LiteralRadix::Hexadecimal => format!("{sign}0x{val:x}"),
            LiteralRadix::Decimal => format!("{sign}{val}"),
        };
        TokenStream::from_str(val.as_str())
            .expect("val was infallibly constructed")
    }

    pub(crate) fn int_expr(
        val: i64,
        radix: Option<&LiteralRadix>,
    ) -> TokenStream {
        // Don't use quote! { #val } because that adds the type suffix.
        match radix {
            None | Some(LiteralRadix::Decimal) => {
                let val = proc_macro2::Literal::i64_unsuffixed(val);
                quote!(#val)
            }
            Some(radix) => {
                integer_with_radix(val.unsigned_abs(), val.is_negative(), radix)
            }
        }
    }

    pub(crate) fn uint_expr(
        val: u64,
        radix: Option<&LiteralRadix>,
    ) -> TokenStream {
        // Don't use quote! { #val } because that adds the type suffix.
        match radix {
            None | Some(LiteralRadix::Decimal) => {
                let val = proc_macro2::Literal::u64_unsuffixed(val);
                quote!(#val)
            }
            Some(radix) => integer_with_radix(val, false, radix),
        }
    }

    pub(crate) fn cstr_expr(mut string: String) -> TokenStream {
        string.push('\0');
        let b = proc_macro2::Literal::byte_string(string.as_bytes());
        quote! {
            #b
        }
    }

    pub(crate) fn float_expr(
        ctx: &BindgenContext,
        f: f64,
    ) -> Result<TokenStream, ()> {
        if f.is_finite() {
            let val = proc_macro2::Literal::f64_unsuffixed(f);

            return Ok(quote!(#val));
        }

        let prefix = ctx.trait_prefix();
        let rust_target = ctx.options().rust_target;

        if f.is_nan() {
            // FIXME: This should be done behind a `RustFeature` instead
            #[allow(deprecated)]
            let tokens = if rust_target >= RustTarget::Stable_1_43 {
                quote! {
                    f64::NAN
                }
            } else {
                quote! {
                    ::#prefix::f64::NAN
                }
            };
            return Ok(tokens);
        }

        if f.is_infinite() {
            let tokens = if f.is_sign_positive() {
                // FIXME: This should be done behind a `RustFeature` instead
                #[allow(deprecated)]
                if rust_target >= RustTarget::Stable_1_43 {
                    quote! {
                        f64::INFINITY
                    }
                } else {
                    quote! {
                        ::#prefix::f64::INFINITY
                    }
                }
            } else {
                // FIXME: This should be done behind a `RustFeature` instead
                #[allow(deprecated)]
                // Negative infinity
                if rust_target >= RustTarget::Stable_1_43 {
                    quote! {
                        f64::NEG_INFINITY
                    }
                } else {
                    quote! {
                        ::#prefix::f64::NEG_INFINITY
                    }
                }
            };
            return Ok(tokens);
        }

        warn!("Unknown non-finite float number: {f:?}");
        Err(())
    }

    pub(crate) fn arguments_from_signature(
        signature: &FunctionSig,
        ctx: &BindgenContext,
    ) -> Vec<TokenStream> {
        let mut unnamed_arguments = 0;
        signature
            .argument_types()
            .iter()
            .map(|&(ref name, _ty)| {
                let name = if let Some(ref name) = *name {
                    ctx.rust_ident(name)
                } else {
                    unnamed_arguments += 1;
                    ctx.rust_ident(format!("arg{unnamed_arguments}"))
                };
                quote! { #name }
            })
            .collect()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn integer_with_radix_outputs_correct_tokens() {
            use super::LiteralRadix as R;
            struct Ar {
                v: u64,
                n: bool,
                r: R,
            }
            let inputs_and_expected_results = &[
                (Ar { v: 0b0, n: false, r: R::Binary }, quote! { 0b0 }),
                (Ar { v: 0o0, n: false, r: R::Octal }, quote! { 0o0 }),
                (Ar { v: 0, n: false, r: R::Decimal }, quote! { 0 }),
                (Ar { v: 0x0, n: false, r: R::Hexadecimal }, quote! { 0x0 }),

                (Ar { v: 0b1, n: false, r: R::Binary }, quote! { 0b1 }),
                (Ar { v: 0o1, n: false, r: R::Octal }, quote! { 0o1 }),
                (Ar { v: 1, n: false, r: R::Decimal }, quote! { 1 }),
                (Ar { v: 0x1, n: false, r: R::Hexadecimal }, quote! { 0x1 }),

                (Ar { v: 0b1, n: true, r: R::Binary }, quote! { -0b1 }),
                (Ar { v: 0o1, n: true, r: R::Octal }, quote! { -0o1 }),
                (Ar { v: 1, n: true, r: R::Decimal }, quote! { -1 }),
                (Ar { v: 0x1, n: true, r: R::Hexadecimal }, quote! { -0x1 }),

                (Ar { v: 0b1000000000000000000000000000000000000000000000000000000000000000, n: false, r: R::Binary }, quote! { 0b1000000000000000000000000000000000000000000000000000000000000000 }),
                (Ar { v: 0o1000000000000000000000, n: false, r: R::Octal }, quote! { 0o1000000000000000000000 }),
                (Ar { v: 9223372036854775808, n: false, r: R::Decimal }, quote! { 9223372036854775808 }),
                (Ar { v: 0x8000000000000000, n: false, r: R::Hexadecimal }, quote! { 0x8000000000000000 }),

                (Ar { v: 0b1000000000000000000000000000000000000000000000000000000000000000, n: true, r: R::Binary }, quote! { -0b1000000000000000000000000000000000000000000000000000000000000000 }),
                (Ar { v: 0o1000000000000000000000, n: true, r: R::Octal }, quote! { -0o1000000000000000000000 }),
                (Ar { v: 9223372036854775808, n: true, r: R::Decimal }, quote! { -9223372036854775808 }),
                (Ar { v: 0x8000000000000000, n: true, r: R::Hexadecimal }, quote! { -0x8000000000000000 }),

                (Ar { v: u64::MAX, n: false, r: R::Binary }, quote! { 0b1111111111111111111111111111111111111111111111111111111111111111 }),
                (Ar { v: u64::MAX, n: false, r: R::Octal }, quote! { 0o1777777777777777777777 }),
                (Ar { v: u64::MAX, n: false, r: R::Decimal }, quote! { 18446744073709551615 }),
                (Ar { v: u64::MAX, n: false, r: R::Hexadecimal }, quote! { 0xffffffffffffffff }),
            ];
            for (i, e) in inputs_and_expected_results {
                assert_eq!(
                    integer_with_radix(i.v, i.n, &i.r).to_string(),
                    e.to_string()
                );
            }
        }

        #[test]
        fn int_expr_outputs_correct_tokens() {
            use super::LiteralRadix as R;
            let values_and_expected_results = &[
                (
                    0,
                    (
                        quote! { 0b0 },
                        quote! { 0o0 },
                        quote! { 0 },
                        quote! { 0x0 },
                    ),
                ),
                (
                    1,
                    (
                        quote! { 0b1 },
                        quote! { 0o1 },
                        quote! { 1 },
                        quote! { 0x1 },
                    ),
                ),
                (
                    -1,
                    (
                        quote! { -0b1 },
                        quote! { -0o1 },
                        quote! { -1 },
                        quote! { -0x1 },
                    ),
                ),
                (
                    i64::MIN,
                    (
                        quote! { -0b1000000000000000000000000000000000000000000000000000000000000000 },
                        quote! { -0o1000000000000000000000 },
                        quote! { -9223372036854775808 },
                        quote! { -0x8000000000000000 },
                    ),
                ),
                (
                    i64::MAX,
                    (
                        quote! { 0b111111111111111111111111111111111111111111111111111111111111111 },
                        quote! { 0o777777777777777777777 },
                        quote! { 9223372036854775807 },
                        quote! { 0x7fffffffffffffff },
                    ),
                ),
            ];

            for (val, e) in values_and_expected_results {
                assert_eq!(
                    int_expr(*val, Some(&R::Binary)).to_string(),
                    e.0.to_string()
                );
                assert_eq!(
                    int_expr(*val, Some(&R::Octal)).to_string(),
                    e.1.to_string()
                );
                assert_eq!(int_expr(*val, None).to_string(), e.2.to_string());
                assert_eq!(
                    int_expr(*val, Some(&R::Decimal)).to_string(),
                    e.2.to_string()
                );
                assert_eq!(
                    int_expr(*val, Some(&R::Hexadecimal)).to_string(),
                    e.3.to_string()
                );
            }
        }

        #[test]
        fn uint_expr_outputs_correct_tokens() {
            use super::LiteralRadix as R;
            let values_and_expected_results = &[
                (
                    0,
                    (
                        quote! { 0b0 },
                        quote! { 0o0 },
                        quote! { 0 },
                        quote! { 0x0 },
                    ),
                ),
                (
                    1,
                    (
                        quote! { 0b1 },
                        quote! { 0o1 },
                        quote! { 1 },
                        quote! { 0x1 },
                    ),
                ),
                (
                    u64::MAX,
                    (
                        quote! { 0b1111111111111111111111111111111111111111111111111111111111111111 },
                        quote! { 0o1777777777777777777777 },
                        quote! { 18446744073709551615 },
                        quote! { 0xffffffffffffffff },
                    ),
                ),
            ];

            for (val, e) in values_and_expected_results {
                assert_eq!(
                    uint_expr(*val, Some(&R::Binary)).to_string(),
                    e.0.to_string()
                );
                assert_eq!(
                    uint_expr(*val, Some(&R::Octal)).to_string(),
                    e.1.to_string()
                );
                assert_eq!(uint_expr(*val, None).to_string(), e.2.to_string());
                assert_eq!(
                    uint_expr(*val, Some(&R::Decimal)).to_string(),
                    e.2.to_string()
                );
                assert_eq!(
                    uint_expr(*val, Some(&R::Hexadecimal)).to_string(),
                    e.3.to_string()
                );
            }
        }
    }
}
