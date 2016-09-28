use super::comp::CompInfo;
use super::enum_ty::Enum;
use super::function::FunctionSig;
use super::item::{Item, ItemId};
use super::int::IntKind;
use super::layout::Layout;
use super::context::BindgenContext;
use super::context::TypeResolver;
use parse::{ClangItemParser, ParseResult, ParseError};
use clang::{self, Cursor};

#[derive(Debug)]
pub struct Type {
    /// The name of the type, or None if it was an unnamed struct or union.
    name: Option<String>,
    /// The layout of the type, if known.
    layout: Option<Layout>,
    /// Whether this type is marked as opaque.
    opaque: bool,
    /// Whether this type is marked as hidden.
    hide: bool,
    /// The inner kind of the type
    kind: TypeKind,
    /// Whether this type is const-qualified.
    is_const: bool,
}

pub const RUST_DERIVE_IN_ARRAY_LIMIT: usize = 32usize;

impl Type {
    pub fn as_comp(&self) -> Option<&CompInfo> {
        match self.kind {
            TypeKind::Comp(ref ci) => Some(ci),
            _ => None,
        }
    }

    pub fn new(name: Option<String>,
               layout: Option<Layout>,
               kind: TypeKind,
               is_const: bool) -> Self {
        Type {
            name: name,
            layout: layout,
            opaque: false,
            hide: false,
            kind: kind,
            is_const: is_const,
        }
    }

    pub fn kind(&self) -> &TypeKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut TypeKind {
        &mut self.kind
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|name| &**name)
    }

    pub fn is_comp(&self) -> bool {
        match self.kind {
            TypeKind::Comp(..) => true,
            _ => false,
        }
    }

    pub fn is_named(&self) -> bool {
        match self.kind {
            TypeKind::Named(..) => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self.kind {
            TypeKind::Function(..) => true,
            _ => false,
        }
    }

    pub fn is_builtin_or_named(&self) -> bool {
        match self.kind {
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Function(..) |
            TypeKind::Array(..) |
            TypeKind::Reference(..) |
            TypeKind::Pointer(..) |
            TypeKind::BlockPointer |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Named(..) => true,
            _ => false,
        }
    }

    /// Creates a new named type, with name `name`.
    pub fn named(name: String, default: Option<ItemId>) -> Self {
        assert!(!name.is_empty());
        // TODO: stop duplicating the name, it's stupid.
        let kind = TypeKind::Named(name.clone(), default);
        Self::new(Some(name), None, kind, false)
    }

    pub fn is_integer_literal(&self) -> bool {
        match *self.kind() {
            TypeKind::Int(..) => true,
            _ => false,
        }
    }

    pub fn is_const(&self) -> bool {
        self.is_const
    }

    pub fn layout(&self, type_resolver: &TypeResolver) -> Option<Layout> {
        use std::mem;

        self.layout.or_else(|| {
            match self.kind {
                TypeKind::Comp(ref ci)
                    => ci.layout(type_resolver),
                // FIXME(emilio): This is a hack for anonymous union templates.
                // Use the actual pointer size!
                TypeKind::Pointer(..) |
                TypeKind::BlockPointer
                    => Some(Layout::new(mem::size_of::<*mut ()>(), mem::align_of::<*mut ()>())),
                TypeKind::ResolvedTypeRef(inner)
                    => type_resolver.resolve_type(inner).layout(type_resolver),
                _ => None,
            }
        })
    }

    pub fn is_opaque(&self, _type_resolver: &TypeResolver) -> bool {
        self.opaque
    }

    pub fn can_derive_debug(&self, type_resolver: &TypeResolver) -> bool {
        !self.is_opaque(type_resolver) && match self.kind {
            TypeKind::Array(t, len) => {
                len <= RUST_DERIVE_IN_ARRAY_LIMIT &&
                type_resolver.resolve_type(t).can_derive_debug(type_resolver)
            }
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::Alias(_, t) => {
                type_resolver.resolve_type(t).can_derive_debug(type_resolver)
            }
            TypeKind::Comp(ref info) => {
                info.can_derive_debug(type_resolver, self.layout(type_resolver))
            }
            _   => true,
        }
    }

    // For some reason, deriving copies of an array of a type that is not known
    // to be copy is a compile error. e.g.:
    //
    // #[derive(Copy)]
    // struct A<T> {
    //     member: T,
    // }
    //
    // is fine, while:
    //
    // #[derive(Copy)]
    // struct A<T> {
    //     member: [T; 1],
    // }
    //
    // is an error.
    //
    // That's the point of the existence of can_derive_copy_in_array().
    pub fn can_derive_copy_in_array(&self, type_resolver: &TypeResolver, item: &Item) -> bool {
        match self.kind {
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::Alias(_, t) |
            TypeKind::Array(t, _) => {
                type_resolver.resolve_item(t)
                             .can_derive_copy_in_array(type_resolver)
            }
            TypeKind::Named(..) => false,
            _ => self.can_derive_copy(type_resolver, item),
        }
    }

    pub fn can_derive_copy(&self, type_resolver: &TypeResolver, item: &Item) -> bool {
        !self.is_opaque(type_resolver) && match self.kind {
            TypeKind::Array(t, len) => {
                len <= RUST_DERIVE_IN_ARRAY_LIMIT &&
                type_resolver.resolve_item(t).can_derive_copy_in_array(type_resolver)
            }
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateRef(t, _) |
            TypeKind::Alias(_, t) => {
                type_resolver.resolve_item(t).can_derive_copy(type_resolver)
            }
            TypeKind::Comp(ref info) => {
                info.can_derive_copy(type_resolver, item)
            }
            _ => true,
        }
    }

    pub fn has_vtable(&self, type_resolver: &TypeResolver) -> bool {
        // FIXME: Can we do something about template parameters? Huh...
        match self.kind {
            TypeKind::TemplateRef(t, _) |
            TypeKind::Alias(_, t) |
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::Array(t, _) => {
                type_resolver.resolve_type(t).has_vtable(type_resolver)
            }
            TypeKind::Comp(ref info) => {
                info.has_vtable(type_resolver)
            }
            _ => false,
        }

    }

    pub fn has_destructor(&self, type_resolver: &TypeResolver) -> bool {
        self.is_opaque(type_resolver) || match self.kind {
            TypeKind::TemplateRef(t, _) |
            TypeKind::Alias(_, t) |
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::Array(t, _) => {
                type_resolver.resolve_type(t).has_destructor(type_resolver)
            }
            TypeKind::Comp(ref info) => {
                info.has_destructor(type_resolver)
            }
            _ => false,
        }
    }

    pub fn signature_contains_named_type(&self,
                                         type_resolver: &TypeResolver,
                                         ty: &Type) -> bool {
        debug_assert!(ty.is_named());
        let name = match *ty.kind() {
            TypeKind::Named(ref name, _) => name,
            _ => unreachable!(),
        };

        match self.kind {
            TypeKind::Named(ref this_name, _)
                => this_name == name,
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::Array(t, _) |
            TypeKind::Pointer(t) |
            TypeKind::Alias(_, t)
                => type_resolver.resolve_type(t)
                                .signature_contains_named_type(type_resolver, ty),
            TypeKind::Function(ref sig) => {
                sig.argument_types().iter().any(|&(_, arg)| {
                    type_resolver.resolve_type(arg)
                                 .signature_contains_named_type(type_resolver, ty)
                }) ||
                type_resolver.resolve_type(sig.return_type())
                             .signature_contains_named_type(type_resolver, ty)
            },
            TypeKind::TemplateRef(_inner, ref template_args) => {
                template_args.iter().any(|arg| {
                    type_resolver.resolve_type(*arg)
                                 .signature_contains_named_type(type_resolver, ty)
                })
            }
            TypeKind::Comp(ref ci)
                => ci.signature_contains_named_type(type_resolver, ty),
            _   => false,
        }
    }

    pub fn canonical_type<'tr>(&'tr self, type_resolver: &'tr TypeResolver) -> &'tr Type {
        match self.kind {
            TypeKind::Named(..) |
            TypeKind::Array(..) |
            TypeKind::Comp(..) |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::BlockPointer |
            TypeKind::Pointer(..) => self,

            TypeKind::ResolvedTypeRef(inner) |
            TypeKind::Alias(_, inner) |
            TypeKind::TemplateRef(inner, _)
                => type_resolver.resolve_type(inner).canonical_type(type_resolver),

            TypeKind::UnresolvedTypeRef(..)
                => unreachable!("Should have been resolved after parsing!"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FloatKind {
    Float,
    Double,
    LongDouble,
}

/// The different kinds of types that we can parse.
///
/// TODO: The name in the Alias and Named kinds is a bit unsound, should be in
/// type.name?
#[derive(Debug)]
pub enum TypeKind {
    /// The void type.
    Void,
    /// The nullptr_t type.
    NullPtr,
    /// A compound type, that is, a class, struct, or union.
    Comp(CompInfo),
    /// An integer type, of a given kind. `bool` and `char` are also considered
    /// integers.
    Int(IntKind),
    /// A floating point type.
    Float(FloatKind),
    /// A type alias, with a name, that points to another type.
    Alias(String, ItemId),
    /// An array of a type and a lenght.
    Array(ItemId, usize),
    /// A function type, with a given signature.
    Function(FunctionSig),
    /// An enum type.
    Enum(Enum),
    /// A pointer to a type. The bool field represents whether it's const or
    /// not.
    Pointer(ItemId),
    /// A pointer to an Apple block.
    BlockPointer,
    /// A reference to a type, as in: int& foo().
    Reference(ItemId),
    /// A reference to a template, with different template parameter names. To
    /// see why this is needed, check out the creation of this variant in
    /// `Type::from_clang_ty`.
    TemplateRef(ItemId, Vec<ItemId>),

    /// A reference to a yet-to-resolve type. This stores the clang cursor
    /// itself, and postpones its resolution.
    ///
    /// These are gone in a phase after parsing where these are mapped to
    /// already known types, and are converted to ResolvedTypeRef.
    ///
    /// see tests/headers/typeref.hpp to see somewhere where this is a problem.
    UnresolvedTypeRef(clang::Type, Option<clang::Cursor>, /* parent_id */ Option<ItemId>),
    ResolvedTypeRef(ItemId),

    /// A named type, that is, a template parameter, with an optional default
    /// type.
    Named(String, Option<ItemId>),
}

impl Type {
    pub fn is_unsized(&self, type_resolver: &TypeResolver) -> bool {
        match self.kind {
            TypeKind::Void => true,
            TypeKind::Comp(ref ci) => ci.is_unsized(type_resolver),
            TypeKind::Array(inner, size) => {
                size == 0 ||
                type_resolver.resolve_type(inner).is_unsized(type_resolver)
            }
            TypeKind::ResolvedTypeRef(inner) |
            TypeKind::Alias(_, inner) |
            TypeKind::TemplateRef(inner, _)
                => type_resolver.resolve_type(inner).is_unsized(type_resolver),
            TypeKind::Named(..) |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::NullPtr |
            TypeKind::BlockPointer |
            TypeKind::Pointer(..) => false,

            TypeKind::UnresolvedTypeRef(..)
                => unreachable!("Should have been resolved after parsing!"),
        }
    }

    pub fn from_clang_ty(potential_id: ItemId,
                         ty: &clang::Type,
                         location: Option<Cursor>,
                         parent_id: Option<ItemId>,
                         ctx: &mut BindgenContext) -> Result<ParseResult<Self>, ParseError> {
        use clangll::*;
        if let Some(ty) = ctx.builtin_or_resolved_ty(parent_id, ty, location) {
            debug!("{:?} already resolved: {:?}", ty, location);
            return Ok(ParseResult::AlreadyResolved(ty));
        }

        let layout = ty.fallible_layout().ok();
        let cursor = ty.declaration();
        let mut name = cursor.spelling();

        debug!("from_clang_ty: {:?}, ty: {:?}, loc: {:?}", potential_id, ty, location);
        debug!("currently_parsed_types: {:?}", ctx.currently_parsed_types);

        let canonical_ty = ty.canonical_type();
        let kind = match ty.kind() {
            CXType_Unexposed if *ty != canonical_ty &&
                                canonical_ty.kind() != CXType_Invalid => {
                debug!("Looking for canonical type: {:?}", canonical_ty);
                return Self::from_clang_ty(potential_id, &canonical_ty,
                                           location, parent_id, ctx);
            }
            CXType_Unexposed |
            CXType_Invalid => {
                // For some reason Clang doesn't give us any hint
                // in some situations where we should generate a
                // function pointer (see
                // tests/headers/func_ptr_in_struct.h), so we do a
                // guess here trying to see if it has a valid return
                // type.
                if ty.ret_type().kind() != CXType_Invalid {
                    let signature =
                        try!(FunctionSig::from_ty(ty, &location.unwrap_or(cursor), ctx));
                    TypeKind::Function(signature)
                // Same here, with template specialisations we can safely assume
                // this is a Comp(..)
                } else if ty.num_template_args() > 0 {
                    debug!("Template specialization: {:?}", ty);
                    let complex =
                        CompInfo::from_ty(potential_id, ty, location, ctx)
                                .expect("C'mon");
                    TypeKind::Comp(complex)
                } else if let Some(location) = location {
                    match location.kind() {
                        CXCursor_ClassTemplatePartialSpecialization |
                        CXCursor_ClassTemplate => {
                            name = location.spelling();
                            let complex =
                                CompInfo::from_ty(potential_id, ty, Some(location), ctx)
                                        .expect("C'mon");
                            TypeKind::Comp(complex)
                        }
                        CXCursor_TemplateRef => {
                            let referenced = location.referenced();
                            return Self::from_clang_ty(potential_id,
                                                       &referenced.cur_type(),
                                                       Some(referenced.cur_type().declaration()),
                                                       parent_id,
                                                       ctx);
                        }
                        CXCursor_TypeRef => {
                            let referenced = location.referenced();
                            return Ok(ParseResult::AlreadyResolved(
                                    Item::from_ty_or_ref_with_id(potential_id,
                                                                 referenced.cur_type(),
                                                                 Some(referenced.cur_type().declaration()),
                                                                 parent_id,
                                                                 ctx)));
                        }
                        _ => {
                            if ty.kind() == CXType_Unexposed {
                                warn!("Unexposed type {:?}, recursing inside, loc: {:?}", ty, location);
                                return Err(ParseError::Recurse);
                            }

                            error!("invalid type {:?}", ty);
                            return Err(ParseError::Continue);
                        }
                    }
                } else {
                    // TODO: Don't duplicate this!
                    if ty.kind() == CXType_Unexposed {
                        warn!("Unexposed type {:?}, recursing inside", ty);
                        return Err(ParseError::Recurse);
                    }

                    error!("invalid type `{}`", ty.spelling());
                    return Err(ParseError::Continue);
                }
            }
            // NOTE: We don't resolve pointers eagerly because the pointee type
            // might not have been parsed, and if it contains templates or
            // something else we might get confused, see the comment inside
            // TypeRef.
            //
            // We might need to, though, if the context is already in the
            // process of resolving them.
            CXType_MemberPointer |
            CXType_Pointer => {
                let inner =
                    Item::from_ty_or_ref(ty.pointee_type(), location,
                                         parent_id, ctx);
                TypeKind::Pointer(inner)
            }
            CXType_BlockPointer => {
                TypeKind::BlockPointer
            }
            // XXX: RValueReference is most likely wrong, but I don't think we
            // can even add bindings for that, so huh.
            CXType_RValueReference |
            CXType_LValueReference => {
                let inner =
                    Item::from_ty_or_ref(ty.pointee_type(), location,
                                         parent_id, ctx);
                TypeKind::Reference(inner)
            }
            // XXX DependentSizedArray is wrong
            CXType_VariableArray |
            CXType_DependentSizedArray |
            CXType_IncompleteArray => {
                let inner = Item::from_ty(&ty.elem_type(), location, parent_id, ctx)
                                .expect("Not able to resolve array element?");
                TypeKind::Pointer(inner)
            }
            CXType_FunctionNoProto |
            CXType_FunctionProto => {
                let signature = try!(FunctionSig::from_ty(ty, &location.unwrap_or(cursor), ctx));
                TypeKind::Function(signature)
            }
            CXType_Typedef  => {
                let inner = cursor.typedef_type();
                let inner =
                    Item::from_ty_or_ref(inner, location, parent_id, ctx);
                TypeKind::Alias(ty.spelling(), inner)
            }
            CXType_Enum => {
                let enum_ = Enum::from_ty(ty, ctx)
                                .expect("Not an enum?");
                TypeKind::Enum(enum_)
            }
            CXType_Record => {
                let complex = CompInfo::from_ty(potential_id, ty, location, ctx)
                                    .expect("Not a complex type?");
                TypeKind::Comp(complex)
            }
            CXType_ConstantArray => {
                let inner = Item::from_ty(&ty.elem_type(), location, parent_id, ctx)
                                .expect("Not able to resolve array element?");
                TypeKind::Array(inner, ty.array_size())
            }
            CXType_Elaborated => {
                return Self::from_clang_ty(potential_id, &ty.named(),
                                           location, parent_id, ctx);
            }
            _ => {
                error!("unsupported type {:?} at {:?}", ty, location);
                return Err(ParseError::Continue);
            }
        };

        let name = if name.is_empty() { None } else { Some(name) };
        let is_const = ty.is_const();

        let ty = Type::new(name, layout, kind, is_const);
        // TODO: maybe declaration.canonical()?
        Ok(ParseResult::New(ty, Some(cursor.canonical())))
    }
}
