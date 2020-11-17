use std::str::FromStr;
use proc_macro2::TokenStream;
use crate::HashMap;
use crate::codegen::helpers;
use crate::codegen::helpers::attributes;
use crate::codegen::{CodegenResult, CodeGenerator, ToRustTyOrOpaque};
use crate::codegen::utils;
use crate::ir::comp::*;
use crate::ir::ty::TypeKind;
use crate::ir::layout::Layout;
use crate::ir::context::BindgenContext;
use crate::ir::item::ItemCanonicalName;
use crate::ir::function::{Abi, Function};

/// This function takes a TokenStream, removes all "const" and replaces all "*"
/// with "&".
///
/// This is a bit hacky, but we need it because argument_type_id_to_rust_type
/// returns something like "const *MyClass" instead of "&MyClass" and the latter
/// is what we need to write into bindgen.rs. The less hacky way would be to
/// make argument_type_id_to_rust_type use "&" instead of "*" when assembling
/// the TokenStream, but doing this is tricky.
fn raw_pointer_to_reference(
    rust_source: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    // Todo: Check if the performance of this function is terrible if rust_source is long.
    use proc_macro2::*;
    rust_source
        .into_iter()
        .filter(|elem| match &elem {
            TokenTree::Ident(ident) => {
                let comp = Ident::new("const", ident.span());
                if comp == ident.clone() {
                    false
                } else {
                    true
                }
            }
            _ => (true),
        })
        .map(|elem| match &elem {
            TokenTree::Punct(punct) => {
                if punct.as_char() == '*' {
                    TokenTree::Punct(Punct::new('&', Spacing::Alone))
                } else {
                    elem
                }
            }
            _ => (elem),
        })
        .collect()
}

impl Method {
    /// If self is an overloaded operator that this function recognizes, this
    /// function will put the correct wrapper into result and return true. Note
    /// that this function does not need to recognize all overloaded operators
    /// for Bindgen to work correctly, because this function "only" adds some
    /// syntactic sugar to make the Rust code that calls the C++ code using
    /// Bindgen look a bit nicer. A non-exhaustive list of ignored operators:
    /// * `operator[]` - We could translate this C++ operator to Index or
    ///   IndexMut, but in Rust usually the [] operator does the bounds
    ///   checking, but in C++ usually the caller does the bounds checking. This
    ///   might lead to bugs if the Rust caller thinks that the C++ operator
    ///   will do bounds checking.
    /// * `operator<, operator>, operator<=, operator>=` Rust does have the
    ///   "Ord" and "PartialOrd" Traits that are similar to those operators, but
    ///   those operators would require more coding work from the bindgen
    ///   Authors than other operators and I'm lazy.
    /// * `operator!=` In C++ you could (but you probably should not) define a
    ///   != Operator and a == Operator, so that a != b is not !(a == b). You
    ///   cannot do that in Rust, because Rustc translates a != b to !(a == b).
    ///   We ignore the != Operator that the C++ code defined and just use the
    ///   == Operator that the C++ code defined.
    fn try_codegen_operator(
        &self,
        ctx: &BindgenContext,
        ty_for_impl: &proc_macro2::TokenStream,
        result: &mut CodegenResult,
    ) {
        // We start by getting the name of the function.
        let function_item = ctx.resolve_item(self.signature());
        let function = function_item.expect_function();
        let name = function.name().to_owned();
        if !name.starts_with("operator") {
            return;
        }

        // We then get the type of the return value (ret_type) and the type of
        // the second argument (second_arg_type) (if a second argument exists).
        let function_name = ctx.rust_ident(function_item.canonical_name(ctx));
        let signature_item = ctx.resolve_item(function.signature());
        let signature = match *signature_item.expect_type().kind() {
            TypeKind::Function(ref sig) => sig,
            _ => panic!("How in the world?"),
        };
        let return_item = ctx.resolve_item(signature.return_type());
        let ret_type = return_item.to_rust_ty_or_opaque(ctx, &());
        let args = signature.argument_types();
        let second_arg_type = if args.len() < 2 {
            None
        } else {
            let temp = utils::argument_type_id_to_rust_type(
                ctx,
                signature.argument_types()[1].1,
            );
            Some(raw_pointer_to_reference(temp))
        };

        // We then check if the function name is in one of the following three
        // arrays or "operator==". If yes, we generate the correct wrapper and
        // add it to result.
        let assignment_operators = [
            ("operator+=", "AddAssign", "add_assign"),
            ("operator&=", "BitAndAssign", "bitand_assign"),
            ("operator|=", "BitOrAssign", "bitor_assign"),
            ("operator^=", "BitXorAssign", "bitxor_assign"),
            ("operator/=", "DivAssign", "div_assign"),
            ("operator%=", "RemAssign", "rem_assign"),
            ("operator<<=", "ShlAssign", "shl_assign"),
            ("operator>>=", "ShrAssign", "shr_assign"),
            ("operator-=", "SubAssign", "sub_assign"),
        ];
        let binary_operators = [
            ("operator+", "Add", "add"),
            ("operator&", "BitAnd", "bitand"),
            ("operator|", "BitOr", "bitor"),
            ("operator^", "BitXor", "bitxor"),
            ("operator/", "Div", "div"),
            ("operator%", "Rem", "rem"),
            ("operator<<", "Shl", "shl"),
            ("operator>>", "Shr", "shr"),
            ("operator-", "Sub", "sub"),
        ];
        let unary_operators =
            [("operator-", "Neg", "neg"), ("operator!", "Not", "not")];
        if name == "operator==" {
            result.push(quote!(
                impl PartialEq for #ty_for_impl {
                    fn eq(&self, rhs: &Self) -> bool {
                        unsafe {
                            #function_name(self, rhs)
                        }
                    }
                }
            ));
            return;
        }
        let prefix = ctx.trait_prefix();
        for el in assignment_operators.iter() {
            // The args.len() == 2 check shouldn't be neccessary but we still have it.
            if args.len() == 2 && name == el.0 {
                let rhs_type = second_arg_type.unwrap();
                let trait_name =
                    proc_macro2::TokenStream::from_str(el.1).unwrap();
                let func_name =
                    proc_macro2::TokenStream::from_str(el.2).unwrap();
                result.push(quote!(
                    impl ::#prefix::ops::#trait_name<#rhs_type> for #ty_for_impl {
                        fn #func_name(&mut self, rhs: #rhs_type) {
                            let retptr = unsafe {
                                #function_name(self, rhs)
                            };
                            assert_eq!(retptr, self as *mut #ty_for_impl, "The bindgen authors have no idea how to handle this case.");
                        }
                    }
                ));
                return;
            }
        }
        for el in binary_operators.iter() {
            // The args.len() == 2 check is needed, because "operator-" is both
            // the name of the binary minus and the unary minus operator (a.k.a
            // the negation operator).
            if args.len() == 2 && name == el.0 {
                let rhs_type = second_arg_type.unwrap();
                let trait_name =
                    proc_macro2::TokenStream::from_str(el.1).unwrap();
                let func_name =
                    proc_macro2::TokenStream::from_str(el.2).unwrap();
                result.push(quote!(
                    impl ::#prefix::ops::#trait_name<#rhs_type> for &#ty_for_impl {
                        type Output = #ret_type;
                        fn #func_name(self, rhs: #rhs_type) -> #ret_type {
                            unsafe {
                                #function_name(self, rhs)
                            }
                        }
                    }
                ));
                return;
            }
        }
        for el in unary_operators.iter() {
            // The args.len() == 1 check shouldn't be neccessary but we still have it.
            if args.len() == 1 && name == el.0 {
                let trait_name =
                    proc_macro2::TokenStream::from_str(el.1).unwrap();
                let func_name =
                    proc_macro2::TokenStream::from_str(el.2).unwrap();
                result.push(quote!(
                    impl ::#prefix::ops::#trait_name for &#ty_for_impl {
                        type Output = #ret_type;
                        fn #func_name(self) -> #ret_type {
                            unsafe {
                                #function_name(self)
                            }
                        }
                    }
                ));
                return;
            }
        }
    }
    pub fn name_this_method(&self, function: &Function, method_names: &mut HashMap<String, usize>) -> String {
        let mut name = match self.kind() {
            MethodKind::Constructor => "new".into(),
            MethodKind::Destructor => "destruct".into(),
            _ => function.name().to_owned(),
        };
        let count = {
            let count = method_names.entry(name.clone()).or_insert(0);
            *count += 1;
            *count - 1
        };
        if count != 0 {
            name.push_str(&count.to_string());
        }
        name
    }
}

trait MethodCodegen {
    fn codegen_method<'a>(
        &self,
        ctx: &BindgenContext,
        methods: &mut Vec<proc_macro2::TokenStream>,
        method_names: &mut HashMap<String, usize>,
        result: &mut CodegenResult<'a>,
        parent: &CompInfo,
        safe_class_interface: bool,
        layout: Option<Layout>,
        ty_for_impl: &proc_macro2::TokenStream,
    );
}

impl MethodCodegen for Method {
    fn codegen_method<'a>(
        &self,
        ctx: &BindgenContext,
        methods: &mut Vec<proc_macro2::TokenStream>,
        method_names: &mut HashMap<String, usize>,
        result: &mut CodegenResult<'a>,
        _parent: &CompInfo,
        safe_class_interface: bool,
        layout: Option<Layout>,
        ty_for_impl: &proc_macro2::TokenStream,
    ) {
        assert!({
            let cc = &ctx.options().codegen_config;
            match self.kind() {
                MethodKind::Constructor => cc.constructors(),
                MethodKind::Destructor => cc.destructors(),
                MethodKind::VirtualDestructor { .. } => cc.destructors(),
                MethodKind::Static |
                MethodKind::Normal |
                MethodKind::Virtual { .. } => cc.methods(),
            }
        });

        // TODO(emilio): We could generate final stuff at least.
        if self.is_virtual() {
            return; // FIXME
        }

        // First of all, output the actual function.
        let function_item = ctx.resolve_item(self.signature());
        if function_item.is_blacklisted(ctx) {
            // We shouldn't emit a method declaration if the function is blacklisted
            return;
        }
        function_item.codegen(ctx, result, &());

        let function = function_item.expect_function();
        let signature_item = ctx.resolve_item(function.signature());

        let signature = match *signature_item.expect_type().kind() {
            TypeKind::Function(ref sig) => sig,
            _ => panic!("How in the world?"),
        };

        if let (Abi::ThisCall, false) =
            (signature.abi(), ctx.options().rust_features().thiscall_abi)
        {
            return;
        }
        // Do not generate variadic methods, since rust does not allow
        // implementing them, and we don't do a good job at it anyway.
        if signature.is_variadic() {
            return;
        }

        let name = self.name_this_method(function, method_names);

        let function_name = ctx.rust_ident(function_item.canonical_name(ctx));
        let mut args = utils::fnsig_arguments(ctx, signature);
        let mut ret = utils::fnsig_return_ty(ctx, signature);

        if !self.is_static() && !self.is_constructor() {
            args[0] = if self.is_const() {
                quote! { &self }
            } else {
                quote! { &mut self }
            };
        }

        // If it's a constructor, we always return `Self`, and we inject the
        // "this" parameter, so there's no need to ask the user for it.
        //
        // Note that constructors in Clang are represented as functions with
        // return-type = void.
        if self.is_constructor() {
            args.remove(0);
            ret = quote! { -> Self };
        }

        let mut exprs =
            helpers::ast_ty::arguments_from_signature(&signature, ctx);

        let mut stmts = vec![];
        let prefix = ctx.trait_prefix();

        // If it's a constructor, we need to insert an extra parameter with a
        // variable called `__bindgen_tmp` we're going to create.
        if self.is_constructor() && !safe_class_interface {
            let tmp_variable_decl = if ctx
                .options()
                .rust_features()
                .maybe_uninit
            {
                exprs[0] = quote! {
                    __bindgen_tmp.as_mut_ptr()
                };
                quote! {
                    let mut __bindgen_tmp = ::#prefix::mem::MaybeUninit::uninit()
                }
            } else {
                exprs[0] = quote! {
                    &mut __bindgen_tmp
                };
                quote! {
                    let mut __bindgen_tmp = ::#prefix::mem::uninitialized()
                }
            };
            stmts.push(tmp_variable_decl);
        } else if self.is_constructor() && safe_class_interface {
            let size = layout.unwrap().size;
            let align = layout.unwrap().align;
            assert!(size != 0, "alloc is undefined if size == 0");
            stmts.push(quote!(
                let ret = Self{ptr: ::#prefix::alloc::alloc(::#prefix::alloc::Layout::from_size_align(#size, #align).unwrap()) as *mut ::#prefix::ffi::c_void}
            ));
        } else if !self.is_static() {
            assert!(!exprs.is_empty());
            exprs[0] = quote! {
                self
            };
        };
        if safe_class_interface && !self.is_static() {
            exprs[0] = if self.is_constructor() {
                quote! (
                    ret.ptr as * mut #ty_for_impl
                )
            } else if self.is_const() {
                quote! (
                    self.ptr as * const #ty_for_impl
                )
            } else {
                quote!(
                    self.ptr as * mut #ty_for_impl
                )
            };
        }

        let call = quote! {
            #function_name (#( #exprs ),* )
        };

        stmts.push(call);

        if self.is_constructor() && !safe_class_interface {
            stmts.push(if ctx.options().rust_features().maybe_uninit {
                quote! {
                    __bindgen_tmp.assume_init()
                }
            } else {
                quote! {
                    __bindgen_tmp
                }
            })
        } else if self.is_constructor() && safe_class_interface {
            stmts.push(quote!(
                ret
            ));
        }

        let block = quote! {
            #( #stmts );*
        };

        let mut attrs = vec![];
        attrs.push(attributes::inline());

        if signature.must_use() &&
            ctx.options().rust_features().must_use_function
        {
            attrs.push(attributes::must_use());
        }

        let name = ctx.rust_ident(&name);
        if safe_class_interface {
            methods.push(quote! {
                #(#attrs)*
                pub fn #name ( #( #args ),* ) #ret {
                    unsafe {
                        #block
                    }
                }
            });
        } else {
            methods.push(quote! {
                #(#attrs)*
                pub unsafe fn #name ( #( #args ),* ) #ret {
                    #block
                }
            });
        }
    }
}
impl CompInfo {
    /// Writes all methods of this Class into result
    pub fn codegen_methods(
        &self,
        ctx: &BindgenContext,
        result: &mut CodegenResult,
        ty_for_impl: &TokenStream,
        layout: Option<Layout>,
        methods: &mut Vec<proc_macro2::TokenStream>,
        safe_class_interface: bool
    ) {
        let mut method_names = Default::default();
        if ctx.options().codegen_config.methods() {
            for method in self.methods() {
                assert!(method.kind() != MethodKind::Constructor);
                method.try_codegen_operator(ctx, &ty_for_impl, result);
                method.codegen_method(
                    ctx,
                    methods,
                    &mut method_names,
                    result,
                    self,
                    safe_class_interface,
                    layout,
                    ty_for_impl,
                );
            }
        }

        if ctx.options().codegen_config.constructors() {
            for sig in self.constructors() {
                Method::new(
                    MethodKind::Constructor,
                    *sig,
                    /* const */
                    false,
                )
                .codegen_method(
                    ctx,
                    methods,
                    &mut method_names,
                    result,
                    self,
                    safe_class_interface,
                    layout,
                    ty_for_impl,
                );
            }
        }

        if !safe_class_interface && ctx.options().codegen_config.destructors() {
            if let Some((kind, destructor)) = self.destructor() {
                debug_assert!(kind.is_destructor());
                Method::new(kind, destructor, false).codegen_method(
                    ctx,
                    methods,
                    &mut method_names,
                    result,
                    self,
                    safe_class_interface,
                    layout,
                    ty_for_impl,
                );
            }
        }
    }
}