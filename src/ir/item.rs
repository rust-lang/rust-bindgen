use super::context::BindgenContext;
use super::item_kind::ItemKind;
use super::ty::{Type, TypeKind};
use super::function::Function;
use super::module::Module;
use super::annotations::Annotations;
use std::fmt;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use parse::{ClangItemParser, ClangSubItemParser, ParseError, ParseResult};
use clang;
use clangll;

/// A trait to get the canonical name from an item.
///
/// This is the trait that will eventually isolate all the logic related to name
/// mangling and that kind of stuff.
///
/// This assumes no nested paths, at some point I'll have to make it a more
/// complex thing.
///
/// This name is required to be safe for Rust, that is, is not expected to
/// return any rust keyword from here.
pub trait ItemCanonicalName {
    fn canonical_name(&self, ctx: &BindgenContext) -> String;
}

/// The same, but specifies the path that needs to be followed to reach an item.
///
/// To contrast with canonical_name, here's an example:
///
/// ```
/// namespace foo {
///     const BAR = 3;
/// }
/// ```
///
/// For bar, the canonical path is foo::BAR, while the canonical name is just
/// BAR.
pub trait ItemCanonicalPath {
    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String>;
}

/// A single identifier for an item.
///
/// TODO: Build stronger abstractions on top of this, like TypeId(ItemId), ...
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(usize);

impl fmt::Display for ItemId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "_bindgen_id_"));
        self.0.fmt(fmt)
    }
}

pub static NEXT_ITEM_ID: AtomicUsize = ATOMIC_USIZE_INIT;

impl ItemId {
    pub fn next() -> Self {
        ItemId(NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed))
    }
}

// Pure convenience
impl ItemCanonicalName for ItemId {
    fn canonical_name(&self, ctx: &BindgenContext) -> String {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        ctx.resolve_item(*self).canonical_name(ctx)
    }
}

impl ItemCanonicalPath for ItemId {
    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String> {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        ctx.resolve_item(*self).canonical_path(ctx)
    }
}

#[derive(Debug)]
pub struct Item {
    /// This item's id.
    id: ItemId,
    /// A doc comment over the item, if any.
    comment: Option<String>,
    /// Annotations extracted from the doc comment, or the default ones
    /// otherwise.
    annotations: Annotations,
    /// An item's parent id. This will most likely be a class where this item
    /// was declared, or a module, etc.
    ///
    /// All the items have a parent, except the root module, in which case the
    /// parent id is its own id.
    parent_id: ItemId,
    /// The item kind.
    kind: ItemKind,
}

impl Item {
    pub fn new(id: ItemId,
               comment: Option<String>,
               annotations: Option<Annotations>,
               parent_id: ItemId,
               kind: ItemKind) -> Self {
        debug_assert!(id != parent_id || kind.is_module());
        Item {
            id: id,
            parent_id: parent_id,
            comment: comment,
            annotations: annotations.unwrap_or_default(),
            kind: kind,
        }
    }

    pub fn id(&self) -> ItemId {
        self.id
    }

    pub fn parent_id(&self) -> ItemId {
        self.parent_id
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|c| &**c)
    }

    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut ItemKind {
        &mut self.kind
    }

    pub fn is_toplevel(&self, ctx: &BindgenContext) -> bool {
        // FIXME: Workaround for some types falling behind when parsing weird
        // stl classes, for example.
        if ctx.options().enable_cxx_namespaces &&
           self.kind().is_module() &&
           self.id() != ctx.root_module() {
            return false;
        }

        let mut parent = self.parent_id;
        loop {
            let parent_item = match ctx.resolve_item_fallible(parent) {
                Some(item) => item,
                None => return false,
            };

            if parent_item.id() == ctx.root_module() {
                return true;
            } else if ctx.options().enable_cxx_namespaces || !parent_item.kind().is_module() {
                return false;
            }

            parent = parent_item.parent_id();
        }
    }

    pub fn expect_type(&self) -> &Type {
        self.kind().expect_type()
    }

    pub fn expect_function(&self) -> &Function {
        self.kind().expect_function()
    }

    pub fn applicable_template_args(&self, ctx: &BindgenContext) -> Vec<ItemId> {
        let ty = match *self.kind() {
            ItemKind::Type(ref ty) => ty,
            _ => return vec![],
        };

        fn parent_contains(ctx: &BindgenContext,
                           parent_template_args: &[ItemId],
                           item: ItemId) -> bool {
            let item_ty = ctx.resolve_type(item);
            parent_template_args.iter().any(|parent_item| {
                let parent_ty = ctx.resolve_type(*parent_item);
                match (parent_ty.kind(), item_ty.kind()) {
                    (&TypeKind::Named(ref n, _), &TypeKind::Named(ref i, _)) => n == i,
                    _ => false,
                }
            })
        }

        match *ty.kind() {
            TypeKind::Named(..) => vec![self.id()],
            TypeKind::Array(inner, _) |
            TypeKind::Pointer(inner) |
            TypeKind::Reference(inner) |
            TypeKind::ResolvedTypeRef(inner) => {
                ctx.resolve_item(inner).applicable_template_args(ctx)
            }
            TypeKind::Alias(_, inner) => {
                let parent_args = ctx.resolve_item(self.parent_id())
                   .applicable_template_args(ctx);
                let inner = ctx.resolve_type(inner);
                // Avoid unused type parameters, sigh.
                parent_args.iter().cloned().filter(|arg| {
                    let arg = ctx.resolve_type(*arg);
                    arg.is_named() && inner.signature_contains_named_type(ctx, arg)
                }).collect()
            }
            // XXX Is this completely correct? Partial template specialization
            // is hard anyways, sigh...
            TypeKind::TemplateRef(_, ref args) => {
                args.clone()
            }
            // In a template specialization we've got all we want.
            TypeKind::Comp(ref ci) if ci.is_template_specialization() => {
                ci.template_args().iter().cloned().collect()
            }
            TypeKind::Comp(ref ci) => {
                let mut parent_template_args =
                    ctx.resolve_item(self.parent_id())
                       .applicable_template_args(ctx);

                for ty in ci.template_args() {
                    if !parent_contains(ctx, &parent_template_args, *ty) {
                        parent_template_args.push(*ty);
                    }
                }

                parent_template_args
            }
            _ => vec![],
        }
    }

    fn is_module(&self) -> bool {
        match self.kind {
            ItemKind::Module(..) => true,
            _ => false,
        }
    }

    pub fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    /// Whether this item should be hidden, either due to annotations, or due to
    /// other kind of configuration.
    pub fn is_hidden(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        self.annotations.hide() ||
            ctx.hidden_by_name(&self.real_canonical_name(ctx, false))
    }

    pub fn is_opaque(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        self.annotations.opaque() ||
            ctx.opaque_by_name(&self.real_canonical_name(ctx, false))
    }

    /// Get the canonical name without taking into account the replaces
    /// annotation.
    fn real_canonical_name(&self, ctx: &BindgenContext, count_namespaces: bool) -> String {
        let base_name = match *self.kind() {
            ItemKind::Type(ref ty) => {
                match *ty.kind() {
                    // If we're a template specialization, our name is our parent's
                    TypeKind::Comp(ref ci) if ci.is_template_specialization() => {
                        return ci.specialized_template().unwrap().canonical_name(ctx);
                    },
                    // Same as above
                    TypeKind::ResolvedTypeRef(inner) |
                    TypeKind::TemplateRef(inner, _) => {
                        return inner.canonical_name(ctx);
                    }
                    // If we're a named type, we don't need to mangle it, and we
                    // should be able to assert we're not top level.
                    TypeKind::Named(ref name, _) => {
                        return name.to_owned();
                    }
                    _ => {}
                }

                ty.name().map(ToOwned::to_owned)
                         .unwrap_or_else(|| format!("_bindgen_ty{}", self.id()))
            }
            ItemKind::Function(ref fun) => {
                let mut base = fun.name().to_owned();

                // We might need to deduplicate if we're a method.
                let parent = ctx.resolve_item(self.parent_id());
                if let ItemKind::Type(ref ty) = *parent.kind() {
                    if let TypeKind::Comp(ref ci) = *ty.kind() {
                        let mut count = 0;
                        let mut found = false;
                        for method in ci.methods() {
                            if method.signature() == self.id() {
                                found = true;
                                break;
                            }
                            let fun = ctx.resolve_item(method.signature())
                                         .expect_function();
                            if fun.name() == base {
                                count += 1;
                            }
                        }

                        assert!(found, "Method not found?");
                        if count != 0 {
                            base.push_str(&count.to_string());
                        }
                    }
                }
                base
            }
            ItemKind::Var(ref var) => {
                var.name().to_owned()
            }
            ItemKind::Module(ref module) => {
                module.name().map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("_bindgen_mod{}", self.id()))
            }
        };

        let parent = ctx.resolve_item(self.parent_id());
        let parent_is_namespace = parent.is_module();
        if self.is_toplevel(ctx) || (parent_is_namespace && count_namespaces) {
            return ctx.rust_mangle(&base_name).into_owned();
        }

        // TODO: allow modification of the mangling functions, maybe even per
        // item type?
        format!("{}_{}", parent.canonical_name(ctx), base_name)
    }

    pub fn as_module_mut(&mut self) -> Option<&mut Module> {
        match self.kind {
            ItemKind::Module(ref mut module) => Some(module),
            _ => None,
        }
    }

    pub fn can_derive_copy(&self, ctx: &BindgenContext) -> bool {
        self.expect_type().can_derive_copy(ctx, self)
    }

    pub fn can_derive_copy_in_array(&self, ctx: &BindgenContext) -> bool {
        self.expect_type().can_derive_copy_in_array(ctx, self)
    }
}

impl ClangItemParser for Item {
    fn builtin_type(kind: TypeKind, is_const: bool, ctx: &mut BindgenContext) -> ItemId {
        // Feel free to add more here, I'm just lazy.
        match kind {
            TypeKind::Void |
            TypeKind::Int(..) |
            TypeKind::Pointer(..) |
            TypeKind::Float(..) => {},
            _ => panic!("Unsupported builtin type"),
        }

        let ty = Type::new(None, None, kind, is_const);
        let id = ItemId::next();
        let module = ctx.root_module();
        ctx.add_item(Item::new(id, None, None, module, ItemKind::Type(ty)),
                     None, None);
        id
    }


    fn parse(cursor: clang::Cursor,
             parent_id: Option<ItemId>,
             context: &mut BindgenContext) -> Result<ItemId, ParseError> {
        use ir::function::Function;
        use ir::module::Module;
        use ir::var::Var;

        if !cursor.is_valid() {
            return Err(ParseError::Continue);
        }

        let comment = cursor.raw_comment();
        let annotations = Annotations::new(&cursor);

        // FIXME: The current_module logic is not really accurate. We should be
        // able to index modules by their Cursor, and locate the proper module
        // for a given item.
        //
        // We don't support modules properly though, so there's no rush for
        // this.
        let current_module = context.current_module();
        macro_rules! try_parse {
            ($what:ident) => {
                match $what::parse(cursor, context) {
                    Ok(ParseResult::New(item, declaration)) => {
                        let id = ItemId::next();
                        context.add_item(Item::new(id, comment, annotations,
                                                   parent_id.unwrap_or(current_module),
                                                   ItemKind::$what(item)),
                                         declaration,
                                         Some(cursor));
                        return Ok(id);
                    }
                    Ok(ParseResult::AlreadyResolved(id)) => {
                        return Ok(id);
                    }
                    Err(ParseError::Recurse) => return Err(ParseError::Recurse),
                    Err(ParseError::Continue) => {},
                }
            }
        }

        try_parse!(Module);

        // NOTE: Is extremely important to parse functions and vars **before**
        // types.  Otherwise we can parse a function declaration as a type
        // (which is legal), and lose functions to generate.
        //
        // In general, I'm not totally confident this split between
        // ItemKind::Function and TypeKind::FunctionSig is totally worth it, but
        // I guess we can try.
        try_parse!(Function);
        try_parse!(Var);

        // Types are sort of special, so to avoid parsing template classes
        // twice, handle them separately.
        {
            let definition = cursor.definition();
            let applicable_cursor = if definition.is_valid() {
                definition
            } else {
                cursor
            };
            match Self::from_ty(&applicable_cursor.cur_type(),
                                Some(applicable_cursor), parent_id, context)
            {
                Ok(ty) => return Ok(ty),
                Err(ParseError::Recurse) => return Err(ParseError::Recurse),
                Err(ParseError::Continue) => {},
            }
        }

        // Guess how does clang treat extern "C" blocks?
        if cursor.kind() == clangll::CXCursor_UnexposedDecl {
            Err(ParseError::Recurse)
        } else {
            error!("Unhandled cursor kind: {}", ::clang::kind_to_str(cursor.kind()));
            Err(ParseError::Continue)
        }
    }

    fn from_ty_or_ref(ty: clang::Type,
                      location: Option<clang::Cursor>,
                      parent_id: Option<ItemId>,
                      context: &mut BindgenContext) -> ItemId {
        Self::from_ty_or_ref_with_id(ItemId::next(), ty, location, parent_id, context)
    }

    fn from_ty_or_ref_with_id(potential_id: ItemId,
                              ty: clang::Type,
                              location: Option<clang::Cursor>,
                              parent_id: Option<ItemId>,
                              context: &mut BindgenContext) -> ItemId {
        debug!("from_ty_or_ref_with_id: {:?} {:?}, {:?}, {:?}", potential_id, ty, location, parent_id);

        if context.collected_typerefs() {
            debug!("refs already collected, resolving directly");
            return Self::from_ty_with_id(potential_id, &ty, location, parent_id, context)
                        .expect("Unable to resolve type");
        }

        if let Some(ty) = context.builtin_or_resolved_ty(parent_id, &ty, location) {
            debug!("{:?} already resolved: {:?}", ty, location);
            return ty;
        }

        debug!("New unresolved type reference: {:?}, {:?}", ty, location);

        let is_const = ty.is_const();
        let kind = TypeKind::UnresolvedTypeRef(ty, location, parent_id);
        let current_module = context.current_module();
        context.add_item(Item::new(potential_id, None, None,
                                   parent_id.unwrap_or(current_module),
                                   ItemKind::Type(Type::new(None, None, kind, is_const))),
                         Some(clang::Cursor::null()),
                         None);
        potential_id
    }


    fn from_ty(ty: &clang::Type,
               location: Option<clang::Cursor>,
               parent_id: Option<ItemId>,
               context: &mut BindgenContext) -> Result<ItemId, ParseError> {
        Self::from_ty_with_id(ItemId::next(), ty, location, parent_id, context)
    }

    fn from_ty_with_id(id: ItemId,
                       ty: &clang::Type,
                       location: Option<clang::Cursor>,
                       parent_id: Option<ItemId>,
                       context: &mut BindgenContext) -> Result<ItemId, ParseError> {
        use clangll::*;

        let decl = {
            let decl = ty.declaration();
            let definition = decl.definition();
            if definition.is_valid() {
                definition
            } else {
                decl
            }
        };

        let comment =
            decl.raw_comment()
                .or_else(|| location.as_ref().and_then(|l| l.raw_comment()));
        let annotations =
            Annotations::new(&decl)
                      .or_else(|| location.as_ref().and_then(|l| Annotations::new(l)));

        if let Some(ref replaced) = annotations.as_ref().and_then(|a| a.use_instead_of()) {
            context.replace(replaced, id);
        }

        if let Some(ty) = context.builtin_or_resolved_ty(parent_id, ty, location) {
            return Ok(ty);
        }

        // First, check we're not recursing.
        let mut valid_decl = decl.kind() != CXCursor_NoDeclFound;
        let declaration_to_look_for = if valid_decl {
            decl.canonical()
        } else if location.is_some() && location.unwrap().kind() == CXCursor_ClassTemplate {
            valid_decl = true;
            location.unwrap()
        } else {
            decl
        };

        if valid_decl {
            if let Some(&(_, item_id)) = context.currently_parsed_types.iter().find(|&&(d, _)| d == declaration_to_look_for) {
                debug!("Avoiding recursion parsing type: {:?}", ty);
                return Ok(item_id);
            }
        }

        let current_module = context.current_module();
        if valid_decl {
            context.currently_parsed_types.push((declaration_to_look_for, id));
        }

        let result = Type::from_clang_ty(id, ty, location, parent_id, context);
        let ret = match result {
            Ok(ParseResult::AlreadyResolved(ty)) => Ok(ty),
            Ok(ParseResult::New(item, declaration)) => {
                context.add_item(Item::new(id, comment, annotations,
                                           parent_id.unwrap_or(current_module),
                                           ItemKind::Type(item)),
                                 declaration,
                                 location);
                Ok(id)
            }
            Err(ParseError::Continue) => Err(ParseError::Continue),
            Err(ParseError::Recurse) => {
                debug!("Item::from_ty recursing in the ast");
                let mut result = Err(ParseError::Recurse);
                if let Some(ref location) = location {
                    // Need to pop here, otherwise we'll get stuck.
                    //
                    // TODO: Find a nicer interface, really. Also, the
                    // declaration_to_look_for suspiciously shares a lot of
                    // logic with ir::context, so we should refactor that.
                    if valid_decl {
                        let (popped_decl, _) = context.currently_parsed_types.pop().unwrap();
                        assert_eq!(popped_decl, declaration_to_look_for);
                    }

                    location.visit(|cur, _other| {
                        use clangll::*;
                        result = Item::from_ty_with_id(id, ty, Some(*cur), parent_id, context);
                        match result {
                            Ok(..) => CXChildVisit_Break,
                            Err(ParseError::Recurse) => CXChildVisit_Recurse,
                            Err(ParseError::Continue) => CXChildVisit_Continue,
                        }
                    });

                    if valid_decl {
                        context.currently_parsed_types.push((declaration_to_look_for, id));
                    }
                }
                // If we have recursed into the AST all we know, and we still
                // haven't found what we've got, let's
                // just make a named type.
                //
                // This is what happens with some template members, for example.
                //
                // FIXME: Maybe we should restrict this to things with parent?
                // It's harmless, but if we restrict that, then
                // tests/headers/nsStyleAutoArray.hpp crashes.
                if let Err(ParseError::Recurse) = result {
                    Ok(Self::named_type_with_id(id, ty.spelling(),
                                                None,
                                                parent_id.unwrap_or(context.current_module()),
                                                context))
                } else {
                    result
                }
            }
        };

        if valid_decl {
            let (popped_decl, _) = context.currently_parsed_types.pop().unwrap();
            assert_eq!(popped_decl, declaration_to_look_for);
        }

        ret
    }

    /// A named type is a template parameter, e.g., the "T" in Foo<T>. They're
    /// always local so it's the only exception when there's no declaration for
    /// a type.
    ///
    /// It must have an id, and must not be the current module id. Ideally we
    /// could assert the parent id is a Comp(..) type, but that info isn't
    /// available yet.
    fn named_type_with_id<S>(id: ItemId,
                             name: S,
                             default: Option<ItemId>,
                             parent_id: ItemId,
                             context: &mut BindgenContext) -> ItemId
        where S: Into<String>
    {
        // see tests/headers/const_tparam.hpp
        // and tests/headers/variadic_tname.hpp
        let name = name.into().replace("const ", "").replace(".", "");

        context.add_item(Item::new(id, None, None, parent_id,
                                   ItemKind::Type(Type::named(name, default))),
                         None,
                         None);

        id
    }

    fn named_type<S>(name: S,
                     default: Option<ItemId>,
                     parent_id: ItemId,
                     context: &mut BindgenContext) -> ItemId
        where S: Into<String>
    {
        Self::named_type_with_id(ItemId::next(), name, default, parent_id, context)
    }
}

impl ItemCanonicalName for Item {
    fn canonical_name(&self, ctx: &BindgenContext) -> String {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        if let Some(other_canon_type) = self.annotations.use_instead_of() {
            return other_canon_type.to_owned();
        }
        self.real_canonical_name(ctx, ctx.options().enable_cxx_namespaces)
    }
}

impl ItemCanonicalPath for Item {
    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String> {
        if !ctx.options().enable_cxx_namespaces {
            return vec![self.canonical_name(ctx)];
        }

        if self.id() == ctx.root_module() {
            match self.kind {
                ItemKind::Module(ref module) => {
                    return vec![module.name().unwrap().into()]
                }
                _ => panic!("Something has wrong horribly wrong"),
            }
        }

        // TODO: This duplicates too much logic with real_canonical_name.
        if let ItemKind::Type(ref ty) = *self.kind() {
            match *ty.kind() {
                TypeKind::Comp(ref ci) if ci.is_template_specialization() => {
                    return ci.specialized_template().unwrap().canonical_path(ctx);
                },
                TypeKind::ResolvedTypeRef(inner) |
                TypeKind::TemplateRef(inner, _) => {
                    return inner.canonical_path(ctx);
                }
                TypeKind::Named(ref name, _) => {
                    return vec![name.clone()];
                }
                _ => {}
            }
        }

        let mut parent_path = self.parent_id().canonical_path(&ctx);
        parent_path.push(self.real_canonical_name(ctx, true));

        parent_path
    }
}
