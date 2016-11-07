//! Bindgen's core intermediate representation type.

use clang;
use parse::{ClangItemParser, ClangSubItemParser, ParseError, ParseResult};
use regex::Regex;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};
use super::annotations::Annotations;
use super::context::BindgenContext;
use super::function::Function;
use super::item_kind::ItemKind;
use super::module::Module;
use super::ty::{Type, TypeKind};
use super::type_collector::{ItemSet, TypeCollector};

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
    /// Get the canonical name for this item.
    fn canonical_name(&self, ctx: &BindgenContext) -> String;
}

/// The same, but specifies the path that needs to be followed to reach an item.
///
/// To contrast with canonical_name, here's an example:
///
/// ```c++
/// namespace foo {
///     const BAR = 3;
/// }
/// ```
///
/// For bar, the canonical path is `vec!["foo", "BAR"]`, while the canonical
/// name is just `"BAR"`.
pub trait ItemCanonicalPath {
    /// Get the canonical path for this item.
    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String>;
}

/// A trait for iterating over an item and its parents and up its ancestor chain
/// up to (but not including) the implicit root module.
pub trait ItemAncestors {
    /// Get an iterable over this item's ancestors.
    fn ancestors<'a, 'b>(&self,
                         ctx: &'a BindgenContext<'b>)
                         -> ItemAncestorsIter<'a, 'b>;
}

/// An iterator over an item and its ancestors.
pub struct ItemAncestorsIter<'a, 'b>
    where 'b: 'a,
{
    item: ItemId,
    ctx: &'a BindgenContext<'b>,
}

impl<'a, 'b> Iterator for ItemAncestorsIter<'a, 'b>
    where 'b: 'a,
{
    type Item = ItemId;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.ctx.resolve_item(self.item);
        if item.parent_id() == self.item {
            None
        } else {
            self.item = item.parent_id();
            Some(item.id())
        }
    }
}

/// A single identifier for an item.
///
/// TODO: Build stronger abstractions on top of this, like TypeId(ItemId)?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(usize);

impl ItemId {
    /// Allocate the next `ItemId`.
    pub fn next() -> Self {
        static NEXT_ITEM_ID: AtomicUsize = ATOMIC_USIZE_INIT;
        let next_id = NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed);
        ItemId(next_id)
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

impl ItemAncestors for ItemId {
    fn ancestors<'a, 'b>(&self,
                         ctx: &'a BindgenContext<'b>)
                         -> ItemAncestorsIter<'a, 'b> {
        ItemAncestorsIter {
            item: *self,
            ctx: ctx,
        }
    }
}

impl ItemAncestors for Item {
    fn ancestors<'a, 'b>(&self,
                         ctx: &'a BindgenContext<'b>)
                         -> ItemAncestorsIter<'a, 'b> {
        self.id().ancestors(ctx)
    }
}

impl TypeCollector for ItemId {
    type Extra = ();

    fn collect_types(&self,
                     ctx: &BindgenContext,
                     types: &mut ItemSet,
                     extra: &()) {
        ctx.resolve_item(*self).collect_types(ctx, types, extra);
    }
}

impl TypeCollector for Item {
    type Extra = ();

    fn collect_types(&self,
                     ctx: &BindgenContext,
                     types: &mut ItemSet,
                     _extra: &()) {
        if self.is_hidden(ctx) || types.contains(&self.id()) {
            return;
        }

        match *self.kind() {
            ItemKind::Type(ref ty) => {
                types.insert(self.id());
                if !self.is_opaque(ctx) {
                    ty.collect_types(ctx, types, self);
                }
            }
            _ => {} // FIXME.
        }
    }
}

/// An item is the base of the bindgen representation, it can be either a
/// module, a type, a function, or a variable (see `ItemKind` for more
/// information).
///
/// Items refer to each other by `ItemId`. Every item has its parent's
/// id. Depending on the kind of item this is, it may also refer to other items,
/// such as a compound type item referring to other types. Collectively, these
/// references form a graph.
///
/// The entry-point to this graph is the "root module": a meta-item used to hold
/// all top-level items.
///
/// An item may have a comment, and annotations (see the `annotations` module).
///
/// Note that even though we parse all the types of annotations in comments, not
/// all of them apply to every item. Those rules are described in the
/// `annotations` module.
#[derive(Debug)]
pub struct Item {
    /// This item's id.
    id: ItemId,

    /// The item's local id, unique only amongst its siblings.  Only used for
    /// anonymous items.
    ///
    /// Lazily initialized in local_id().
    ///
    /// Note that only structs, unions, and enums get a local type id. In any
    /// case this is an implementation detail.
    local_id: Cell<Option<usize>>,

    /// The next local id to use for a child..
    next_child_local_id: Cell<usize>,

    /// A cached copy of the canonical name, as returned by `canonical_name`.
    ///
    /// This is a fairly used operation during codegen so this makes bindgen
    /// considerably faster in those cases.
    canonical_name_cache: RefCell<Option<String>>,

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
    /// Construct a new `Item`.
    pub fn new(id: ItemId,
               comment: Option<String>,
               annotations: Option<Annotations>,
               parent_id: ItemId,
               kind: ItemKind)
               -> Self {
        debug_assert!(id != parent_id || kind.is_module());
        Item {
            id: id,
            local_id: Cell::new(None),
            next_child_local_id: Cell::new(1),
            canonical_name_cache: RefCell::new(None),
            parent_id: parent_id,
            comment: comment,
            annotations: annotations.unwrap_or_default(),
            kind: kind,
        }
    }

    /// Get this `Item`'s identifier.
    pub fn id(&self) -> ItemId {
        self.id
    }

    /// Get this `Item`'s parent's identifier.
    ///
    /// For the root module, the parent's ID is its own ID.
    pub fn parent_id(&self) -> ItemId {
        self.parent_id
    }

    /// Get this `Item`'s comment, if it has any.
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|c| &**c)
    }

    /// What kind of item is this?
    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    /// Get a mutable reference to this item's kind.
    pub fn kind_mut(&mut self) -> &mut ItemKind {
        &mut self.kind
    }

    /// Get an identifier that differentiates this item from its siblings.
    ///
    /// This should stay relatively stable in the face of code motion outside or
    /// below this item's lexical scope, meaning that this can be useful for
    /// generating relatively stable identifiers within a scope.
    pub fn local_id(&self, ctx: &BindgenContext) -> usize {
        if self.local_id.get().is_none() {
            let parent = ctx.resolve_item(self.parent_id);
            let local_id = parent.next_child_local_id.get();
            parent.next_child_local_id.set(local_id + 1);
            self.local_id.set(Some(local_id));
        }
        self.local_id.get().unwrap()
    }

    /// Returns whether this item is a top-level item, from the point of view of
    /// bindgen.
    ///
    /// This point of view changes depending on whether namespaces are enabled
    /// or not. That way, in the following example:
    ///
    /// ```c++
    /// namespace foo {
    ///     static int var;
    /// }
    /// ```
    ///
    /// `var` would be a toplevel item if namespaces are disabled, but won't if
    /// they aren't.
    ///
    /// This function is used to determine when the codegen phase should call
    /// `codegen` on an item, since any item that is not top-level will be
    /// generated by its parent.
    pub fn is_toplevel(&self, ctx: &BindgenContext) -> bool {
        // FIXME: Workaround for some types falling behind when parsing weird
        // stl classes, for example.
        if ctx.options().enable_cxx_namespaces && self.kind().is_module() &&
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
            } else if ctx.options().enable_cxx_namespaces ||
                      !parent_item.kind().is_module() {
                return false;
            }

            parent = parent_item.parent_id();
        }
    }

    /// Get a reference to this item's underlying `Type`. Panic if this is some
    /// other kind of item.
    pub fn expect_type(&self) -> &Type {
        self.kind().expect_type()
    }

    /// Get a reference to this item's underlying `Type`, or `None` if this is
    /// some other kind of item.
    pub fn as_type(&self) -> Option<&Type> {
        self.kind().as_type()
    }

    /// Get a reference to this item's underlying `Function`. Panic if this is
    /// some other kind of item.
    pub fn expect_function(&self) -> &Function {
        self.kind().expect_function()
    }

    /// Checks whether an item contains in its "type signature" some named type.
    ///
    /// This function is used to avoid unused template parameter errors in Rust
    /// when generating typedef declarations, and also to know whether we need
    /// to generate a `PhantomData` member for a template parameter.
    ///
    /// For example, in code like the following:
    ///
    /// ```c++
    /// template<typename T, typename U>
    /// struct Foo {
    ///     T bar;
    ///
    ///     struct Baz {
    ///         U bas;
    ///     };
    /// };
    /// ```
    ///
    /// Both `Foo` and `Baz` contain both `T` and `U` template parameters in
    /// their signature:
    ///
    ///  * `Foo<T, U>`
    ///  * `Bar<T, U>`
    ///
    /// But the Rust structure for `Foo` would look like:
    ///
    /// ```rust
    /// struct Foo<T, U> {
    ///     bar: T,
    ///     _phantom0: ::std::marker::PhantomData<U>,
    /// }
    /// ```
    ///
    /// because none of its member fields contained the `U` type in the
    /// signature. Similarly, `Bar` would contain a `PhantomData<T>` type, for
    /// the same reason.
    ///
    /// Note that this is somewhat similar to `applicable_template_args`, but
    /// this also takes into account other kind of types, like arrays,
    /// (`[T; 40]`), pointers: `*mut T`, etc...
    ///
    /// Normally we could do this check just in the `Type` kind, but we also
    /// need to check the `applicable_template_args` more generally, since we
    /// could need a type transitively from our parent, see the test added in
    /// commit 2a3f93074dd2898669dbbce6e97e5cc4405d7cb1.
    ///
    /// It's kind of unfortunate (in the sense that it's a sort of complex
    /// process), but I think it should get all the cases.
    fn signature_contains_named_type(&self,
                                     ctx: &BindgenContext,
                                     ty: &Type)
                                     -> bool {
        debug_assert!(ty.is_named());
        self.expect_type().signature_contains_named_type(ctx, ty) ||
        self.applicable_template_args(ctx).iter().any(|template| {
            ctx.resolve_type(*template).signature_contains_named_type(ctx, ty)
        })
    }

    /// Returns the template arguments that apply to a struct. This is a concept
    /// needed because of type declarations inside templates, for example:
    ///
    /// ```c++
    /// template<typename T>
    /// class Foo {
    ///     typedef T element_type;
    ///     typedef int Bar;
    ///
    ///     template<typename U>
    ///     class Baz {
    ///     };
    /// };
    /// ```
    ///
    /// In this case, the applicable template arguments for the different types
    /// would be:
    ///
    ///  * `Foo`: [`T`]
    ///  * `Foo::element_type`: [`T`]
    ///  * `Foo::Bar`: [`T`]
    ///  * `Foo::Baz`: [`T`, `U`]
    ///
    /// You might notice that we can't generate something like:
    ///
    /// ```rust,ignore
    /// type Foo_Bar<T> = ::std::os::raw::c_int;
    /// ```
    ///
    /// since that would be invalid Rust. Still, conceptually, `Bar` *could* use
    /// the template parameter type `T`, and that's exactly what this method
    /// represents. The unused template parameters get stripped in the
    /// `signature_contains_named_type` check.
    pub fn applicable_template_args(&self,
                                    ctx: &BindgenContext)
                                    -> Vec<ItemId> {
        let ty = match *self.kind() {
            ItemKind::Type(ref ty) => ty,
            _ => return vec![],
        };

        fn parent_contains(ctx: &BindgenContext,
                           parent_template_args: &[ItemId],
                           item: ItemId)
                           -> bool {
            let item_ty = ctx.resolve_type(item);
            parent_template_args.iter().any(|parent_item| {
                let parent_ty = ctx.resolve_type(*parent_item);
                match (parent_ty.kind(), item_ty.kind()) {
                    (&TypeKind::Named(ref n, _),
                     &TypeKind::Named(ref i, _)) => n == i,
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
                let inner = ctx.resolve_item(inner);

                // Avoid unused type parameters, sigh.
                parent_args.iter()
                    .cloned()
                    .filter(|arg| {
                        let arg = ctx.resolve_type(*arg);
                        arg.is_named() &&
                        inner.signature_contains_named_type(ctx, arg)
                    })
                    .collect()
            }
            // XXX Is this completely correct? Partial template specialization
            // is hard anyways, sigh...
            TypeKind::TemplateAlias(_, ref args) |
            TypeKind::TemplateRef(_, ref args) => args.clone(),
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

    /// Get this item's annotations.
    pub fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    /// Whether this item should be hidden.
    ///
    /// This may be due to either annotations or to other kind of configuration.
    pub fn is_hidden(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        self.annotations.hide() ||
        ctx.hidden_by_name(&self.real_canonical_name(ctx, false, true), self.id)
    }

    /// Is this item opaque?
    pub fn is_opaque(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        self.annotations.opaque() ||
        ctx.opaque_by_name(&self.real_canonical_name(ctx, false, true))
    }

    /// Is this a reference to another type?
    pub fn is_type_ref(&self) -> bool {
        self.as_type().map_or(false, |ty| ty.is_type_ref())
    }

    /// Get the canonical name without taking into account the replaces
    /// annotation.
    ///
    /// This is the base logic used to implement hiding and replacing via
    /// annotations, and also to implement proper name mangling.
    ///
    /// The idea is that each generated type in the same "level" (read: module
    /// or namespace) has a unique canonical name.
    ///
    /// This name should be derived from the immutable state contained in the
    /// type and the parent chain, since it should be consistent.
    pub fn real_canonical_name(&self,
                               ctx: &BindgenContext,
                               count_namespaces: bool,
                               for_name_checking: bool)
                               -> String {
        let base_name = match *self.kind() {
            ItemKind::Type(ref ty) => {
                match *ty.kind() {
                    // If we're a template specialization, our name is our
                    // parent's.
                    TypeKind::Comp(ref ci)
                        if ci.is_template_specialization() => {
                        return ci.specialized_template().unwrap()
                                 .canonical_name(ctx);
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
                    // We call codegen on the inner type, but we do not want
                    // this alias's name to appear in the canonical name just
                    // because it is in the inner type's parent chain, so we use
                    // an empty base name.
                    //
                    // Note that this would be incorrect if this type could be
                    // referenced from, let's say, a member variable, but in
                    // that case the referenced type is the inner alias, so
                    // we're good there. If we wouldn't, a more complex solution
                    // would be needed.
                    TypeKind::TemplateAlias(inner, _) => {
                        if for_name_checking {
                            return ctx.resolve_item(inner)
                                      .real_canonical_name(ctx,
                                                           count_namespaces,
                                                           false);
                        }
                        Some("")
                    }
                    // Else use the proper name, or fallback to a name with an
                    // id.
                    _ => {
                        ty.name()
                    }
                }.map(ToOwned::to_owned)
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
                Some(base)
            }
            ItemKind::Var(ref var) => Some(var.name().to_owned()),
            ItemKind::Module(ref module) => {
                module.name().map(ToOwned::to_owned)
            }
        };

        let parent = ctx.resolve_item(self.parent_id());
        let parent_is_namespace = parent.is_module();

        if self.is_toplevel(ctx) || (parent_is_namespace && count_namespaces) {
            let base_name = self.make_exposed_name(None, base_name, ctx);
            return ctx.rust_mangle(&base_name).into_owned();
        }

        // TODO: allow modification of the mangling functions, maybe even per
        // item type?
        let parent_name = parent.canonical_name(ctx);
        self.make_exposed_name(Some(parent_name), base_name, ctx)
    }

    fn exposed_id(&self, ctx: &BindgenContext) -> String {
        // Only use local ids for enums, classes, structs and union types.  All
        // other items use their global id.
        let ty_kind = self.kind().as_type().map(|t| t.kind());
        if let Some(ty_kind) = ty_kind {
            match *ty_kind {
                TypeKind::Comp(..) |
                TypeKind::Enum(..) => return self.local_id(ctx).to_string(),
                _ => {}
            }
        }

        // Note that this `id_` prefix prevents (really unlikely) collisions
        // between the global id and the local id of an item with the same
        // parent.
        format!("id_{}", self.id().0)
    }

    fn make_exposed_name(&self,
                         parent_name: Option<String>,
                         base_name: Option<String>,
                         ctx: &BindgenContext)
                         -> String {
        lazy_static! {
            static ref RE_ENDS_WITH_BINDGEN_TY: Regex =
                Regex::new(r"_bindgen_ty(_\d+)+$").unwrap();

            static ref RE_ENDS_WITH_BINDGEN_MOD: Regex =
                Regex::new(r"_bindgen_mod(_\d+)+$").unwrap();
        }

        let (re, kind) = match *self.kind() {
            ItemKind::Module(..) => (&*RE_ENDS_WITH_BINDGEN_MOD, "mod"),
            _ => (&*RE_ENDS_WITH_BINDGEN_TY, "ty"),
        };

        let parent_name =
            parent_name.and_then(|n| if n.is_empty() { None } else { Some(n) });
        match (parent_name, base_name) {
            (Some(parent), Some(base)) => format!("{}_{}", parent, base),
            (Some(parent), None) => {
                if re.is_match(parent.as_str()) {
                    format!("{}_{}", parent, self.exposed_id(ctx))
                } else {
                    format!("{}__bindgen_{}_{}",
                            parent,
                            kind,
                            self.exposed_id(ctx))
                }
            }
            (None, Some(base)) => base,
            (None, None) => {
                format!("_bindgen_{}_{}", kind, self.exposed_id(ctx))
            }
        }
    }

    /// Get a mutable reference to this item's `Module`, or `None` if this is
    /// not a `Module` item.
    pub fn as_module_mut(&mut self) -> Option<&mut Module> {
        match self.kind {
            ItemKind::Module(ref mut module) => Some(module),
            _ => None,
        }
    }

    /// Can we derive an implementation of the `Copy` trait for this type?
    pub fn can_derive_copy(&self, ctx: &BindgenContext) -> bool {
        self.expect_type().can_derive_copy(ctx, self)
    }

    /// Can we derive an implementation of the `Copy` trait for an array of this
    /// type?
    ///
    /// See `Type::can_derive_copy_in_array` for details.
    pub fn can_derive_copy_in_array(&self, ctx: &BindgenContext) -> bool {
        self.expect_type().can_derive_copy_in_array(ctx, self)
    }
}

impl ClangItemParser for Item {
    fn builtin_type(kind: TypeKind,
                    is_const: bool,
                    ctx: &mut BindgenContext)
                    -> ItemId {
        // Feel free to add more here, I'm just lazy.
        match kind {
            TypeKind::Void |
            TypeKind::Int(..) |
            TypeKind::Pointer(..) |
            TypeKind::Float(..) => {}
            _ => panic!("Unsupported builtin type"),
        }

        let ty = Type::new(None, None, kind, is_const);
        let id = ItemId::next();
        let module = ctx.root_module();
        ctx.add_item(Item::new(id, None, None, module, ItemKind::Type(ty)),
                     None,
                     None);
        id
    }


    fn parse(cursor: clang::Cursor,
             parent_id: Option<ItemId>,
             ctx: &mut BindgenContext)
             -> Result<ItemId, ParseError> {
        use ir::function::Function;
        use ir::module::Module;
        use ir::var::Var;
        use clangll::*;

        if !cursor.is_valid() {
            return Err(ParseError::Continue);
        }

        let comment = cursor.raw_comment();
        let annotations = Annotations::new(&cursor);

        let current_module = ctx.current_module();
        let relevant_parent_id = parent_id.unwrap_or(current_module);

        macro_rules! try_parse {
            ($what:ident) => {
                match $what::parse(cursor, ctx) {
                    Ok(ParseResult::New(item, declaration)) => {
                        let id = ItemId::next();

                        ctx.add_item(Item::new(id, comment, annotations,
                                               relevant_parent_id,
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
            let applicable_cursor = cursor.definition().unwrap_or(cursor);
            match Self::from_ty(&applicable_cursor.cur_type(),
                                Some(applicable_cursor),
                                parent_id,
                                ctx) {
                Ok(ty) => return Ok(ty),
                Err(ParseError::Recurse) => return Err(ParseError::Recurse),
                Err(ParseError::Continue) => {}
            }
        }

        // Guess how does clang treat extern "C" blocks?
        if cursor.kind() == CXCursor_UnexposedDecl {
            Err(ParseError::Recurse)
        } else {
            // We whitelist cursors here known to be unhandled, to prevent being
            // too noisy about this.
            match cursor.kind() {
                CXCursor_MacroDefinition |
                CXCursor_MacroExpansion |
                CXCursor_UsingDeclaration |
                CXCursor_StaticAssert |
                CXCursor_InclusionDirective => {
                    debug!("Unhandled cursor kind {:?}: {:?}",
                           cursor.kind(),
                           cursor);
                }
                _ => {
                    error!("Unhandled cursor kind {:?}: {:?}",
                           cursor.kind(),
                           cursor);
                }
            }

            Err(ParseError::Continue)
        }
    }

    fn from_ty_or_ref(ty: clang::Type,
                      location: Option<clang::Cursor>,
                      parent_id: Option<ItemId>,
                      ctx: &mut BindgenContext)
                      -> ItemId {
        Self::from_ty_or_ref_with_id(ItemId::next(),
                                     ty,
                                     location,
                                     parent_id,
                                     ctx)
    }

    /// Parse a C++ type. If we find a reference to a type that has not been
    /// defined yet, use `UnresolvedTypeRef` as a placeholder.
    ///
    /// This logic is needed to avoid parsing items with the incorrect parent
    /// and it's sort of complex to explain, so I'll just point to
    /// `tests/headers/typeref.hpp` to see the kind of constructs that forced
    /// this.
    ///
    /// Typerefs are resolved once parsing is completely done, see
    /// `BindgenContext::resolve_typerefs`.
    fn from_ty_or_ref_with_id(potential_id: ItemId,
                              ty: clang::Type,
                              location: Option<clang::Cursor>,
                              parent_id: Option<ItemId>,
                              ctx: &mut BindgenContext)
                              -> ItemId {
        debug!("from_ty_or_ref_with_id: {:?} {:?}, {:?}, {:?}",
               potential_id,
               ty,
               location,
               parent_id);

        if ctx.collected_typerefs() {
            debug!("refs already collected, resolving directly");
            return Self::from_ty_with_id(potential_id,
                                         &ty,
                                         location,
                                         parent_id,
                                         ctx)
                .expect("Unable to resolve type");
        }

        if let Some(ty) = ctx.builtin_or_resolved_ty(potential_id,
                                                     parent_id, &ty,
                                                     location) {
            debug!("{:?} already resolved: {:?}", ty, location);
            return ty;
        }

        debug!("New unresolved type reference: {:?}, {:?}", ty, location);

        let is_const = ty.is_const();
        let kind = TypeKind::UnresolvedTypeRef(ty, location, parent_id);
        let current_module = ctx.current_module();
        ctx.add_item(Item::new(potential_id,
                               None,
                               None,
                               parent_id.unwrap_or(current_module),
                               ItemKind::Type(Type::new(None,
                                                        None,
                                                        kind,
                                                        is_const))),
                     Some(clang::Cursor::null()),
                     None);
        potential_id
    }


    fn from_ty(ty: &clang::Type,
               location: Option<clang::Cursor>,
               parent_id: Option<ItemId>,
               ctx: &mut BindgenContext)
               -> Result<ItemId, ParseError> {
        Self::from_ty_with_id(ItemId::next(), ty, location, parent_id, ctx)
    }

    /// This is one of the trickiest methods you'll find (probably along with
    /// some of the ones that handle templates in `BindgenContext`).
    ///
    /// This method parses a type, given the potential id of that type (if
    /// parsing it was correct), an optional location we're scanning, which is
    /// critical some times to obtain information, an optional parent item id,
    /// that will, if it's `None`, become the current module id, and the
    /// context.
    fn from_ty_with_id(id: ItemId,
                       ty: &clang::Type,
                       location: Option<clang::Cursor>,
                       parent_id: Option<ItemId>,
                       ctx: &mut BindgenContext)
                       -> Result<ItemId, ParseError> {
        use clangll::*;

        let decl = {
            let decl = ty.declaration();
            decl.definition().unwrap_or(decl)
        };

        let comment = decl.raw_comment()
            .or_else(|| location.as_ref().and_then(|l| l.raw_comment()));
        let annotations = Annotations::new(&decl)
            .or_else(|| location.as_ref().and_then(|l| Annotations::new(l)));

        if let Some(ref annotations) = annotations {
            if let Some(ref replaced) = annotations.use_instead_of() {
                ctx.replace(replaced, id);
            }
        }

        if let Some(ty) =
               ctx.builtin_or_resolved_ty(id, parent_id, ty, location) {
            return Ok(ty);
        }

        // First, check we're not recursing.
        let mut valid_decl = decl.kind() != CXCursor_NoDeclFound;
        let declaration_to_look_for = if valid_decl {
            decl.canonical()
        } else if location.is_some() &&
                                                location.unwrap().kind() ==
                                                CXCursor_ClassTemplate {
            valid_decl = true;
            location.unwrap()
        } else {
            decl
        };

        if valid_decl {
            if let Some(&(_, item_id)) = ctx.currently_parsed_types
                .iter()
                .find(|&&(d, _)| d == declaration_to_look_for) {
                debug!("Avoiding recursion parsing type: {:?}", ty);
                return Ok(item_id);
            }
        }

        let current_module = ctx.current_module();
        if valid_decl {
            ctx.currently_parsed_types.push((declaration_to_look_for, id));
        }

        let result = Type::from_clang_ty(id, ty, location, parent_id, ctx);
        let relevant_parent_id = parent_id.unwrap_or(current_module);
        let ret = match result {
            Ok(ParseResult::AlreadyResolved(ty)) => Ok(ty),
            Ok(ParseResult::New(item, declaration)) => {
                ctx.add_item(Item::new(id,
                                       comment,
                                       annotations,
                                       relevant_parent_id,
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
                        let (popped_decl, _) =
                            ctx.currently_parsed_types.pop().unwrap();
                        assert_eq!(popped_decl, declaration_to_look_for);
                    }

                    location.visit(|cur| {
                        use clangll::*;
                        result = Item::from_ty_with_id(id,
                                                       ty,
                                                       Some(cur),
                                                       parent_id,
                                                       ctx);
                        match result {
                            Ok(..) => CXChildVisit_Break,
                            Err(ParseError::Recurse) => CXChildVisit_Recurse,
                            Err(ParseError::Continue) => CXChildVisit_Continue,
                        }
                    });

                    if valid_decl {
                        ctx.currently_parsed_types
                            .push((declaration_to_look_for, id));
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
                    Ok(Self::named_type_with_id(id,
                                                ty.spelling(),
                                                None,
                                                relevant_parent_id,
                                                ctx))
                } else {
                    result
                }
            }
        };

        if valid_decl {
            let (popped_decl, _) = ctx.currently_parsed_types.pop().unwrap();
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
                             ctx: &mut BindgenContext)
                             -> ItemId
        where S: Into<String>,
    {
        // see tests/headers/const_tparam.hpp
        // and tests/headers/variadic_tname.hpp
        let name = name.into().replace("const ", "").replace(".", "");

        ctx.add_item(Item::new(id,
                               None,
                               None,
                               parent_id,
                               ItemKind::Type(Type::named(name, default))),
                     None,
                     None);

        id
    }

    fn named_type<S>(name: S,
                     default: Option<ItemId>,
                     parent_id: ItemId,
                     ctx: &mut BindgenContext)
                     -> ItemId
        where S: Into<String>,
    {
        Self::named_type_with_id(ItemId::next(), name, default, parent_id, ctx)
    }
}

impl ItemCanonicalName for Item {
    fn canonical_name(&self, ctx: &BindgenContext) -> String {
        debug_assert!(ctx.in_codegen_phase(),
                      "You're not supposed to call this yet");
        if let Some(other_canon_type) = self.annotations.use_instead_of() {
            return other_canon_type.to_owned();
        }
        if self.canonical_name_cache.borrow().is_none() {
            *self.canonical_name_cache.borrow_mut() =
                Some(self.real_canonical_name(ctx,
                                              ctx.options()
                                                  .enable_cxx_namespaces,
                                              false));
        }
        return self.canonical_name_cache.borrow().as_ref().unwrap().clone();
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
                    return ci.specialized_template()
                        .unwrap()
                        .canonical_path(ctx);
                }
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
        if parent_path.last()
            .map_or(false, |parent_name| parent_name.is_empty()) {
            // This only happens (or should only happen) when we're an alias,
            // and our parent is a templated alias, in which case the last
            // component of the path will be empty.
            let is_alias = match *self.expect_type().kind() {
                TypeKind::Alias(..) => true,
                _ => false,
            };
            debug_assert!(is_alias, "How can this ever happen?");
            parent_path.pop().unwrap();
        }
        parent_path.push(self.real_canonical_name(ctx, true, false));

        parent_path
    }
}
