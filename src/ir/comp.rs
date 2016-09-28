use super::annotations::Annotations;
use super::context::BindgenContext;
use super::context::TypeResolver;
use super::layout::Layout;
use super::item::{Item, ItemId};
use super::ty::{Type, RUST_DERIVE_IN_ARRAY_LIMIT};
use std::cell::Cell;
use std::cmp;
use parse::{ClangItemParser, ParseError};
use clang;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompKind {
    Struct,
    Union,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MethodKind {
    Static,
    Normal,
    Virtual,
}

/// A struct representing a C++ method, either static, normal, or virtual.
#[derive(Debug)]
pub struct Method {
    kind: MethodKind,
    /// The signature of the method. Take into account this is not a `Type`
    /// item, but a `Function` one.
    ///
    /// This is tricky and probably this field should be renamed.
    signature: ItemId,
    is_const: bool,
}

impl Method {
    fn new(kind: MethodKind, signature: ItemId, is_const: bool) -> Self {
        Method {
            kind: kind,
            signature: signature,
            is_const: is_const,
        }
    }

    pub fn kind(&self) -> MethodKind {
        self.kind
    }

    pub fn is_virtual(&self) -> bool {
        self.kind == MethodKind::Virtual
    }

    pub fn is_static(&self) -> bool {
        self.kind == MethodKind::Static
    }

    pub fn signature(&self) -> ItemId {
        self.signature
    }

    pub fn is_const(&self) -> bool {
        self.is_const
    }
}

/// A struct representing a C++ field.
#[derive(Clone, Debug)]
pub struct Field {
    /// The name of the field, empty if it's an unnamed bitfield width.
    name: Option<String>,
    /// The inner type.
    ty: ItemId,
    /// The doc comment on the field if any.
    comment: Option<String>,
    /// Annotations for this field, or the default.
    annotations: Annotations,
    /// If this field is a bitfield, and how many bits does it contain if it is.
    bitfield: Option<u32>,
    /// If the C++ field is marked as `mutable`
    mutable: bool,
}

impl Field {
    pub fn new(name: Option<String>,
               ty: ItemId,
               comment: Option<String>,
               annotations: Option<Annotations>,
               bitfield: Option<u32>,
               mutable: bool) -> Field {
        Field {
            name: name,
            ty: ty,
            comment: comment,
            annotations: annotations.unwrap_or_default(),
            bitfield: bitfield,
            mutable: mutable,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| &**n)
    }

    pub fn ty(&self) -> ItemId {
        self.ty
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|c| &**c)
    }

    pub fn bitfield(&self) -> Option<u32> {
        self.bitfield
    }

    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn annotations(&self) -> &Annotations {
        &self.annotations
    }
}


#[derive(Debug)]
pub struct CompInfo {
    /// Whether this is a struct or a union.
    kind: CompKind,
    /// The members of this struct or union.
    fields: Vec<Field>,
    /// The template parameters of this class. These are non-concrete, and
    /// should always be a Type(TypeKind::Named(name)), but still they need to
    /// be registered with an unique type id in the context.
    template_args: Vec<ItemId>,
    /// The method declarations inside this class, if in C++ mode.
    methods: Vec<Method>,
    /// Vector of classes this one inherits from.
    base_members: Vec<ItemId>,
    /// The parent reference template if any.
    ref_template: Option<ItemId>,
    /// The inner types that were declared inside this class, in something like:
    ///
    /// class Foo {
    ///     typedef int FooTy;
    ///     struct Bar {
    ///         int baz;
    ///     };
    /// }
    ///
    /// static Foo::Bar const = {3};
    inner_types: Vec<ItemId>,
    /// Set of static constants declared inside this class.
    inner_vars: Vec<ItemId>,
    /// Whether this type should generate an vtable (TODO: Should be able to
    /// look at the virtual methods and ditch this field).
    has_vtable: bool,
    /// Whether this type has destructor.
    has_destructor: bool,
    /// Whether this type has a base type with more than one member.
    ///
    /// TODO: We should be able to compute this.
    has_nonempty_base: bool,
    /// If this type has a template parameter which is not a type (e.g.: a size_t)
    has_non_type_template_params: bool,
    /// Whether this struct layout is packed.
    packed: bool,
    /// Whether this struct is anonymous.
    is_anonymous: bool,
    /// Used to know if we've found an opaque attribute that could cause us to
    /// generate a type with invalid layout. This is explicitly used to avoid us
    /// generating bad alignments when parsing types like max_align_t.
    ///
    /// It's not clear what the behavior should be here, if generating the item
    /// and pray, or behave as an opaque type.
    found_unknown_attr: bool,
    /// Used to detect if we've run in a can_derive_debug cycle while cycling
    /// around the template arguments.
    detect_derive_debug_cycle: Cell<bool>,
    /// Used to detect if we've run in a has_destructor cycle while cycling
    /// around the template arguments.
    detect_has_destructor_cycle: Cell<bool>,
}

impl CompInfo {
    pub fn new(kind: CompKind) -> Self {
        CompInfo {
            kind: kind,
            fields: vec![],
            template_args: vec![],
            methods: vec![],
            base_members: vec![],
            ref_template: None,
            inner_types: vec![],
            inner_vars: vec![],
            has_vtable: false,
            has_destructor: false,
            has_nonempty_base: false,
            has_non_type_template_params: false,
            packed: false,
            is_anonymous: false,
            found_unknown_attr: false,
            detect_derive_debug_cycle: Cell::new(false),
            detect_has_destructor_cycle: Cell::new(false),
        }
    }

    pub fn can_derive_debug(&self, type_resolver: &TypeResolver, layout: Option<Layout>) -> bool {
        // We can reach here recursively via template parameters of a member,
        // for example.
        if self.detect_derive_debug_cycle.get() {
            warn!("Derive debug cycle detected!");
            return true;
        }

        if self.kind  == CompKind::Union {
            if type_resolver.options().unstable_rust {
                return false;
            }

            let layout = layout.unwrap_or_else(Layout::zero);
            let size_divisor = cmp::max(1, layout.align);
            return layout.size / size_divisor <= RUST_DERIVE_IN_ARRAY_LIMIT;
        }

        self.detect_derive_debug_cycle.set(true);

        let can_derive_debug =
            self.base_members.iter().all(|ty| {
                type_resolver.resolve_type(*ty)
                             .can_derive_debug(type_resolver)
            }) &&
            self.template_args.iter().all(|ty| {
                type_resolver.resolve_type(*ty)
                             .can_derive_debug(type_resolver)
            }) &&
            self.fields.iter().all(|field| {
                type_resolver.resolve_type(field.ty)
                             .can_derive_debug(type_resolver)
            }) &&
            self.ref_template.map_or(true, |template| {
                type_resolver.resolve_type(template)
                             .can_derive_debug(type_resolver)
            });

        self.detect_derive_debug_cycle.set(false);

        can_derive_debug
    }

    pub fn is_unsized(&self, type_resolver: &TypeResolver) -> bool {
        !self.has_vtable(type_resolver) && self.fields.is_empty() &&
            self.base_members.iter().all(|base| {
                type_resolver
                    .resolve_type(*base)
                    .canonical_type(type_resolver)
                    .is_unsized(type_resolver)
            }) &&
            self.ref_template.map_or(true, |template| {
                type_resolver.resolve_type(template).is_unsized(type_resolver)
            })
    }

    pub fn has_destructor(&self, type_resolver: &TypeResolver) -> bool {
        if self.detect_has_destructor_cycle.get() {
            warn!("Cycle detected looking for destructors");
            // Assume no destructor, since we don't have an explicit one.
            return false;
        }

        self.detect_has_destructor_cycle.set(true);

        let has_destructor = self.has_destructor || match self.kind {
            CompKind::Union => false,
            CompKind::Struct => {
                // NB: We can't rely on a type with type parameters
                // not having destructor.
                //
                // This is unfortunate, but...
                self.ref_template.as_ref().map_or(false, |t| {
                    type_resolver.resolve_type(*t).has_destructor(type_resolver)
                }) ||
                self.template_args.iter().any(|t| {
                    type_resolver.resolve_type(*t).has_destructor(type_resolver)
                }) ||
                self.base_members.iter().any(|t| {
                    type_resolver.resolve_type(*t).has_destructor(type_resolver)
                }) ||
                self.fields.iter().any(|field| {
                    type_resolver.resolve_type(field.ty)
                                 .has_destructor(type_resolver)
                })
            }
        };

        self.detect_has_destructor_cycle.set(false);

        has_destructor
    }

    pub fn can_derive_copy(&self, type_resolver: &TypeResolver, item: &Item) -> bool {
        // NOTE: Take into account that while unions in C and C++ are copied by
        // default, the may have an explicit destructor in C++, so we can't
        // defer this check just for the union case.
        if self.has_destructor(type_resolver) {
            return false;
        }

        if self.kind == CompKind::Union {
            if !type_resolver.options().unstable_rust {
                return true;
            }

            // https://github.com/rust-lang/rust/issues/36640
            if !self.template_args.is_empty() ||
                self.ref_template.is_some() ||
                !item.applicable_template_args(type_resolver).is_empty() {
                return false;
            }
        }

        // With template args, use a safe subset of the types,
        // since copyability depends on the types itself.
        self.ref_template.as_ref().map_or(true, |t| {
            type_resolver.resolve_item(*t).can_derive_copy(type_resolver)
        }) &&
        self.base_members.iter().all(|t| {
            type_resolver.resolve_item(*t).can_derive_copy(type_resolver)
        }) &&
        self.fields.iter().all(|field| {
            type_resolver.resolve_item(field.ty)
                         .can_derive_copy(type_resolver)
        })
    }

    pub fn is_template_specialization(&self) -> bool {
        self.ref_template.is_some()
    }

    pub fn specialized_template(&self) -> Option<ItemId> {
        self.ref_template
    }

    // Computes the layout of this type.
    //
    // This is called as a fallback under some circumstances where LLVM doesn't
    // give us the correct layout.
    // If we're a union without known layout, we try to compute it from our
    // members. This is not ideal, but clang fails to report the size for
    // these kind of unions, see test/headers/template_union.hpp
    pub fn layout(&self, type_resolver: &TypeResolver) -> Option<Layout> {
        use std::cmp;

        // We can't do better than clang here, sorry.
        if self.kind == CompKind::Struct {
            return None;
        }

        let mut max_size = 0;
        let mut max_align = 0;
        for field in &self.fields {
            let field_layout = type_resolver.resolve_type(field.ty)
                                            .layout(type_resolver);

            if let Some(layout) = field_layout {
                max_size = cmp::max(max_size, layout.size);
                max_align = cmp::max(max_align, layout.align);
            }
        }

        Some(Layout::new(max_size, max_align))
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn template_args(&self) -> &[ItemId] {
        &self.template_args
    }

    pub fn has_non_type_template_params(&self) -> bool {
        self.has_non_type_template_params
    }

    pub fn has_vtable(&self, type_resolver: &TypeResolver) -> bool {
        self.has_vtable || self.base_members().iter().any(|base| {
            type_resolver
                .resolve_type(*base)
                .has_vtable(type_resolver)
        }) || self.ref_template.map_or(false, |template| {
            type_resolver.resolve_type(template).has_vtable(type_resolver)
        })
    }

    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    pub fn kind(&self) -> CompKind {
        self.kind
    }

    pub fn base_members(&self) -> &[ItemId] {
        &self.base_members
    }

    pub fn from_ty(potential_id: ItemId,
                   ty: &clang::Type,
                   location: Option<clang::Cursor>,
                   ctx: &mut BindgenContext) -> Result<Self, ParseError> {
        use clangll::*;
        // Sigh... For class templates we want the location, for
        // specialisations, we want the declaration...  So just try both.
        //
        // TODO: Yeah, this code reads really bad.
        let mut cursor = ty.declaration();
        let mut kind = Self::kind_from_cursor(&cursor);
        if kind.is_err() {
            if let Some(location) = location {
                kind = Self::kind_from_cursor(&location);
                cursor = location;
            }
        }

        let kind = try!(kind);

        debug!("CompInfo::from_ty({:?}, {:?})", kind, cursor);


        let mut ci = CompInfo::new(kind);
        ci.is_anonymous = cursor.is_anonymous();
        ci.template_args = match ty.num_template_args() {
            // In forward declarations and not specializations, etc, they are in
            // the ast, we'll meet them in CXCursor_TemplateTypeParameter
            -1 => vec![],
            len => {
                let mut list = Vec::with_capacity(len as usize);
                for i in 0..len {
                    let arg_type = ty.template_arg_type(i);
                    if arg_type.kind() != CXType_Invalid {
                        let type_id =
                            Item::from_ty_or_ref(arg_type, None, None, ctx);

                        list.push(type_id);
                    } else {
                        ci.has_non_type_template_params = true;
                        warn!("warning: Template parameter is not a type");
                    }
                }

                list
            }
        };

        ci.ref_template = Item::parse(cursor.specialized(), None, ctx).ok();

        let mut maybe_anonymous_struct_field = None;
        cursor.visit(|cur, _other| {
            if cur.kind() != CXCursor_FieldDecl {
                if let Some((ty, ref _clang_ty)) = maybe_anonymous_struct_field {
                    let field = Field::new(None, ty, None, None, None, false);
                    ci.fields.push(field);
                }
                maybe_anonymous_struct_field = None;
            }

            match cur.kind() {
                CXCursor_FieldDecl => {
                    match maybe_anonymous_struct_field.take() {
                        Some((ty, clang_ty)) => {
                            let mut used = false;
                            cur.visit(|child, _| {
                                if child.cur_type() == clang_ty {
                                    used = true;
                                }
                                CXChildVisit_Continue
                            });
                            if !used {
                                let field = Field::new(None, ty, None, None, None, false);
                                ci.fields.push(field);
                            }
                        },
                        None => {}
                    }

                    let bit_width = cur.bit_width();
                    let field_type =
                        Item::from_ty_or_ref(cur.cur_type(), Some(*cur), Some(potential_id), ctx);
                    let comment = cur.raw_comment();
                    let annotations = Annotations::new(cur);
                    let name = cur.spelling();
                    let is_mutable = cursor.is_mutable_field();

                    // Name can be empty if there are bitfields, for example,
                    // see tests/headers/struct_with_bitfields.h
                    assert!(!name.is_empty() || bit_width.is_some(),
                            "Empty field name?");

                    let name = if name.is_empty() { None } else { Some(name) };

                    let field = Field::new(name, field_type, comment,
                                           annotations, bit_width, is_mutable);
                    ci.fields.push(field);

                    // No we look for things like attributes and stuff.
                    cur.visit(|cur, _| {
                        if cur.kind() == CXCursor_UnexposedAttr {
                            ci.found_unknown_attr = true;
                        }
                        CXChildVisit_Continue
                    });

                }
                CXCursor_UnexposedAttr => {
                    ci.found_unknown_attr = true;
                }
                CXCursor_EnumDecl |
                CXCursor_TypeAliasDecl |
                CXCursor_TypedefDecl |
                CXCursor_StructDecl |
                CXCursor_UnionDecl |
                CXCursor_ClassTemplate |
                CXCursor_ClassDecl => {
                    let inner = Item::parse(*cur, Some(potential_id), ctx)
                                    .expect("Inner ClassDecl");
                    if !ci.inner_types.contains(&inner) {
                        ci.inner_types.push(inner);
                    }
                    // A declaration of an union or a struct without name could
                    // also be an unnamed field, unfortunately.
                    if cur.spelling().is_empty() && cur.kind() != CXCursor_EnumDecl {
                        maybe_anonymous_struct_field = Some((inner, cur.cur_type()));
                    }
                }
                CXCursor_PackedAttr => {
                    ci.packed = true;
                }
                CXCursor_TemplateTypeParameter => {
                    // Yes! You can arrive here with an empty template parameter
                    // name! Awesome, isn't it?
                    //
                    // see tests/headers/empty_template_param_name.hpp
                    if cur.spelling().is_empty() {
                        return CXChildVisit_Continue;
                    }

                    let default_type =
                        Item::from_ty(&cur.cur_type(), Some(*cur), Some(potential_id), ctx).ok();

                    let param = Item::named_type(cur.spelling(), default_type, potential_id, ctx);
                    ci.template_args.push(param);
                }
                CXCursor_CXXBaseSpecifier => {
                    if !ci.has_vtable {
                        ci.has_vtable = cur.is_virtual_base();
                    }
                    let type_id = Item::from_ty(&cur.cur_type(), None, None, ctx)
                                        .expect("BaseSpecifier");
                    ci.base_members.push(type_id);
                }
                CXCursor_CXXMethod => {
                    let is_virtual = cur.method_is_virtual();
                    let is_static = cur.method_is_static();
                    debug_assert!(!(is_static && is_virtual), "How?");

                    if !ci.has_vtable {
                        ci.has_vtable = is_virtual;
                    }

                    let linkage = cur.linkage();
                    if linkage != CXLinkage_External {
                        return CXChildVisit_Continue;
                    }

                    if cur.access_specifier() == CX_CXXPrivate {
                        return CXChildVisit_Continue;
                    }

                    let visibility = cur.visibility();
                    if visibility != CXVisibility_Default {
                        return CXChildVisit_Continue;
                    }

                    if cur.is_inlined_function() && !ctx.options().keep_inline_functions {
                        return CXChildVisit_Continue;
                    }

                    let spelling = cur.spelling();
                    if spelling.starts_with("operator") {
                        return CXChildVisit_Continue;
                    }

                    // This used to not be here, but then I tried generating
                    // stylo bindings with this (without path filters), and
                    // cried a lot with a method in gfx/Point.h
                    // (ToUnknownPoint), that somehow was causing the same type
                    // to be inserted in the map two times.
                    //
                    // I couldn't make a reduced test case, but anyway...
                    // Methods of template functions not only use to be inlined,
                    // but also instantiated, and we wouldn't be able to call
                    // them, so just bail out.
                    if !ci.template_args.is_empty() {
                        return CXChildVisit_Continue;
                    }

                    // NB: This gets us an owned `Function`, not a `FunctionSig`.
                    let method_signature = Item::parse(*cur, Some(potential_id), ctx)
                                                .expect("CXXMethod");
                    let is_const = cur.method_is_const();
                    let method_kind = if is_static {
                        MethodKind::Static
                    } else if is_virtual {
                        MethodKind::Virtual
                    } else {
                        MethodKind::Normal
                    };
                    ci.methods.push(Method::new(method_kind, method_signature, is_const));
                }
                CXCursor_Destructor => {
                    if cur.method_is_virtual() {
                        // FIXME: Push to the method list?
                        ci.has_vtable = true;
                    }
                    ci.has_destructor = true;
                }
                CXCursor_NonTypeTemplateParameter => {
                    ci.has_non_type_template_params = true;
                }
                CXCursor_VarDecl => {
                    let linkage = cur.linkage();
                    if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
                        return CXChildVisit_Continue;
                    }

                    let visibility = cur.visibility();
                    if visibility != CXVisibility_Default {
                        return CXChildVisit_Continue;
                    }

                    let item = Item::parse(*cur, Some(potential_id), ctx)
                                    .expect("VarDecl");
                    ci.inner_vars.push(item);
                }
                // Intentionally not handled
                CXCursor_CXXAccessSpecifier |
                CXCursor_CXXFinalAttr |
                CXCursor_Constructor |
                CXCursor_FunctionTemplate |
                CXCursor_ConversionFunction => {}
                _ => {
                    warn!("unhandled composite member `{}` (kind {}) in `{}` ({})",
                          cur.spelling(), cur.kind(), cursor.spelling(),
                          cur.location());
                }
            }
            CXChildVisit_Continue
        });

        if let Some((ty, _)) = maybe_anonymous_struct_field {
            let field = Field::new(None, ty, None, None, None, false);
            ci.fields.push(field);
        }

        Ok(ci)
    }

    fn kind_from_cursor(cursor: &clang::Cursor) -> Result<CompKind, ParseError> {
        use clangll::*;
        Ok(match cursor.kind() {
            CXCursor_UnionDecl => CompKind::Union,
            CXCursor_ClassDecl |
            CXCursor_StructDecl => CompKind::Struct,
            CXCursor_ClassTemplatePartialSpecialization |
            CXCursor_ClassTemplate => {
                match cursor.template_kind() {
                    CXCursor_UnionDecl => CompKind::Union,
                    _ => CompKind::Struct,
                }
            }
            _ => {
                warn!("Unknown kind for comp type: {:?}", cursor);
                return Err(ParseError::Continue);
            }
        })
    }

    pub fn signature_contains_named_type(&self,
                                         type_resolver: &TypeResolver,
                                         ty: &Type) -> bool {
        // We don't generate these, so rather don't make the codegen step to
        // think we got it covered.
        if self.has_non_type_template_params() {
            return false;
        }
        self.template_args.iter().any(|arg| {
            type_resolver.resolve_type(*arg)
                         .signature_contains_named_type(type_resolver, ty)
        })
    }

    pub fn inner_types(&self) -> &[ItemId] {
        &self.inner_types
    }

    pub fn inner_vars(&self) -> &[ItemId] {
        &self.inner_vars
    }

    pub fn found_unknown_attr(&self) -> bool {
        self.found_unknown_attr
    }

    pub fn packed(&self) -> bool {
        self.packed
    }

    /// Returns whether this type needs an explicit vtable because it has
    /// virtual methods and none of its base classes has already a vtable.
    pub fn needs_explicit_vtable(&self, type_resolver: &TypeResolver) -> bool {
        self.has_vtable(type_resolver) && !self.base_members.iter().any(|base| {
            // NB: Ideally, we could rely in all these types being `comp`, and
            // life would be beautiful.
            //
            // Unfortunately, given the way we implement --match-pat, and also
            // that you can inherit from templated types, we need to handle
            // other cases here too.
            type_resolver
                .resolve_type(*base)
                .canonical_type(type_resolver)
                .as_comp().map_or(false, |ci| {
                    ci.has_vtable(type_resolver)
                })
        })
    }
}
