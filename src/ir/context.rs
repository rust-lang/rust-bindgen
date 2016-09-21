use super::ty::{Type, TypeKind, FloatKind};
use super::item::{Item, ItemCanonicalName, ItemId};
use super::item_kind::ItemKind;
use super::int::IntKind;
use super::module::Module;
use clang::{self, Cursor};
use std::borrow::{Cow, Borrow};
use std::collections::btree_map::{self, BTreeMap};
use std::collections::{HashSet, HashMap};
use std::fmt;
use syntax::ast::Ident;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::ExtCtxt;
use parse::ClangItemParser;
use BindgenOptions;

// This is just convenience to avoid creating a manual debug impl for the
// context.
struct GenContext<'ctx>(ExtCtxt<'ctx>);

impl<'ctx> fmt::Debug for GenContext <'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "GenContext {{ ... }}")
    }
}

/// A context used during parsing and generation of structs.
#[derive(Debug)]
pub struct BindgenContext<'ctx> {
    /// The map of all the items parsed so far.
    ///
    /// It's a BTreeMap because we want the keys to be sorted to have consistent
    /// output.
    items: BTreeMap<ItemId, Item>,

    /// Clang cursor to type map. This is needed to be able to associate types
    /// with item ids during parsing.
    ///
    /// The cursor used for storage is the definition cursor.
    types: HashMap<Cursor, ItemId>,

    /// A cursor to module map. Similar reason than above.
    modules: HashMap<Cursor, ItemId>,

    /// The root module, this is guaranteed to be an item of kind Module.
    root_module: ItemId,

    /// Current module being traversed.
    current_module: ItemId,

    /// A stack with the current type declarations and types we're parsing. This
    /// is needed to avoid infinite recursion when parsing a type like:
    ///
    /// struct c { struct c* next; };
    ///
    /// This means effectively, that a type has a potential ID before knowing if
    /// it's a correct type. But that's not important in practice.
    ///
    /// We could also use the `types` HashMap, but my intention with it is that
    /// only valid types and declarations end up there, and this could
    /// potentially break that assumption.
    ///
    /// FIXME: Should not be public, though... meh.
    pub currently_parsed_types: Vec<(Cursor, ItemId)>,

    /// A HashSet with all the already parsed macro names. This is done to avoid
    /// hard errors while parsing duplicated macros.
    parsed_macros: HashSet<String>,

    /// The active replacements collected from replaces="xxx" annotations.
    replacements: HashMap<String, ItemId>,

    collected_typerefs: bool,

    /// Dummy structures for code generation.
    gen_ctx: Option<&'ctx GenContext<'ctx>>,
    span: Span,

    /// The clang index for parsing.
    index: clang::Index,

    /// The translation unit for parsing.
    translation_unit: clang::TranslationUnit,

    /// The options given by the user via cli or other medium.
    options: BindgenOptions,
}

impl<'ctx> BindgenContext<'ctx> {
    pub fn new(options: BindgenOptions) -> Self {
        use clangll;

        let index = clang::Index::new(false, true);

        let translation_unit =
            clang::TranslationUnit::parse(&index, "", &options.clang_args, &[],
                                          clangll::CXTranslationUnit_DetailedPreprocessingRecord);

        let root_module = Self::build_root_module();
        let mut me = BindgenContext {
            items: Default::default(),
            types: Default::default(),
            modules: Default::default(),
            root_module: root_module.id(),
            current_module: root_module.id(),
            currently_parsed_types: vec![],
            parsed_macros: Default::default(),
            replacements: Default::default(),
            collected_typerefs: false,
            gen_ctx: None,
            span: DUMMY_SP,
            index: index,
            translation_unit: translation_unit,
            options: options,
        };

        me.add_item(root_module, None, None);

        me
    }

    pub fn add_item(&mut self,
                    item: Item,
                    declaration: Option<Cursor>,
                    location: Option<Cursor>) {
        use clangll::{CXCursor_ClassTemplate, CXCursor_ClassTemplatePartialSpecialization};
        debug!("BindgenContext::add_item({:?}, declaration: {:?}, loc: {:?}", item, declaration, location);
        debug_assert!(declaration.is_some() || !item.kind().is_type() ||
                      item.kind().expect_type().is_builtin_or_named(),
                      "Adding a type without declaration?");

        let id = item.id();
        let is_type = item.kind().is_type();
        let old_item = self.items.insert(id, item);
        assert!(old_item.is_none(), "Inserted type twice?");

        if is_type && declaration.is_some() {
            let declaration = declaration.unwrap();
            debug_assert_eq!(declaration, declaration.canonical());
            if declaration.is_valid() {
                let old = self.types.insert(declaration, id);
                debug_assert_eq!(old, None);
            } else if location.is_some() &&
                      (location.unwrap().kind() == CXCursor_ClassTemplate ||
                       location.unwrap().kind() == CXCursor_ClassTemplatePartialSpecialization) {
                let old = self.types.insert(location.unwrap().canonical(), id);
                debug_assert_eq!(old, None);
            } else {
                // This could happen, for example, with types like `int*` or
                // similar.
                //
                // Fortunately, we don't care about those types being
                // duplicated, so we can just ignore them.
                debug!("Invalid declaration {:?} found for type {:?}",
                       declaration, self.items.get(&id).unwrap().kind().expect_type());
            }
        }
    }

    // TODO: Move all this syntax crap to other part of the code.
    pub fn ext_cx(&self) -> &ExtCtxt<'ctx> {
        &self.gen_ctx.expect("Not in gen phase").0
    }

    pub fn span(&self) -> Span {
        self.span
    }

    /// Mangles a name so it doesn't conflict with any keyword.
    pub fn rust_mangle<'a>(&self, name: &'a str) -> Cow<'a, str> {
        use syntax::parse::token;
        let ident = self.rust_ident_raw(&name);
        let token = token::Ident(ident);
        if token.is_any_keyword() ||
            name.contains("@") ||
            name.contains("?") ||
            name.contains("$") ||
            "bool" == name
        {
            let mut s = name.to_owned();
            s = s.replace("@", "_");
            s = s.replace("?", "_");
            s = s.replace("$", "_");
            s.push_str("_");
            return Cow::Owned(s)
        }
        Cow::Borrowed(name)
    }

    /// Returns a mangled name as a rust identifier.
    pub fn rust_ident(&self, name: &str) -> Ident {
        self.rust_ident_raw(&self.rust_mangle(name))
    }

    pub fn rust_ident_raw<S>(&self, name: &S) -> Ident
        where S: Borrow<str>,
    {
        self.ext_cx().ident_of(name.borrow())
    }

    pub fn items<'a>(&'a self) -> btree_map::Iter<'a, ItemId, Item> {
        self.items.iter()
    }

    pub fn collected_typerefs(&self) -> bool {
        self.collected_typerefs
    }

    fn collect_typerefs(&mut self) -> Vec<(ItemId, clang::Type, Option<clang::Cursor>)> {
        debug_assert!(!self.collected_typerefs);
        self.collected_typerefs = true;
        let mut typerefs = vec![];
        for (id, ref mut item) in &mut self.items {
            let kind = item.kind();
            let ty = match kind.as_type() {
                Some(ty) => ty,
                None => continue,
            };

            match *ty.kind() {
                TypeKind::UnresolvedTypeRef(ref ty, loc) => {
                    typerefs.push((*id, ty.clone(), loc));
                }
                _ => {},
            };
        }
        typerefs
    }

    fn resolve_typerefs(&mut self) {
        let typerefs = self.collect_typerefs();

        for (id, ty, loc) in typerefs {
            let _resolved = {
                let resolved = Item::from_ty(&ty, loc, None, self)
                                     .expect("What happened?");
                let mut item = self.items.get_mut(&id).unwrap();

                *item.kind_mut().as_type_mut().unwrap().kind_mut() =
                    TypeKind::ResolvedTypeRef(resolved);
                resolved
            };

            // Something in the STL is trolling me. I don't need this assertion
            // right now, but worth investigating properly once this lands.
            //
            // debug_assert!(self.items.get(&resolved).is_some(), "How?");
        }
    }

    fn process_replacements(&mut self) {
        if self.replacements.is_empty() {
            return;
        }

        // FIXME: This is linear, but the replaces="xxx" annotation was already
        // there, and for better or worse it's useful, sigh...
        //
        // We leverage the ResolvedTypeRef thing, though, which is cool :P.

        let mut replacements = vec![];

        for (id, item) in self.items.iter() {
            let ty = match item.kind().as_type() {
                Some(ty) => ty,
                None => continue,
            };

            // canonical_name calls are expensive.
            let ci = match ty.as_comp() {
                Some(ci) => ci,
                None => continue,
            };

            if ci.is_template_specialization() {
                continue;
            }

            if let Some(replacement) = self.replacements.get(&item.canonical_name(self)) {
                if replacement != id {
                    // We set this just after parsing the annotation. It's
                    // very unlikely, but this can happen.
                    if self.items.get(replacement).is_some() {
                        replacements.push((*id, *replacement));
                    }
                }
            }
        }

        for (id, replacement) in replacements {
            let mut item = self.items.get_mut(&id).unwrap();
            *item.kind_mut().as_type_mut().unwrap().kind_mut() =
                TypeKind::ResolvedTypeRef(replacement);
        }
    }

    // Enters in the generation phase.
    pub fn gen<F, Out>(&mut self, cb: F) -> Out
        where F: FnOnce(&Self) -> Out
    {
        use syntax::ext::expand::ExpansionConfig;
        use syntax::codemap::{ExpnInfo, MacroBang, NameAndSpan};
        use syntax::ext::base;
        use syntax::parse;
        use std::mem;

        let cfg = ExpansionConfig::default("xxx".to_owned());
        let sess = parse::ParseSess::new();
        let mut loader = base::DummyMacroLoader;
        let mut ctx =
            GenContext(base::ExtCtxt::new(&sess, vec![], cfg, &mut loader));

        ctx.0.bt_push(ExpnInfo {
            call_site: self.span,
            callee: NameAndSpan {
                format: MacroBang(parse::token::intern("")),
                allow_internal_unstable: false,
                span: None
            }
        });

        // FIXME: This is evil, we should move code generation to use a wrapper
        // of BindgenContext instead, I guess. Even though we know it's fine
        // because we remove it before the end of this function.
        self.gen_ctx = Some(unsafe { mem::transmute(&ctx) });

        self.resolve_typerefs();
        self.process_replacements();

        let ret = cb(self);
        self.gen_ctx = None;
        ret
    }

    // This deserves a comment. Builtin types don't get a valid declaration, so
    // we can't add it to the cursor->type map.
    //
    // That being said, they're not generated anyway, and are few, so the
    // duplication and special-casing is fine.
    //
    // If at some point we care about the memory here, probably a map TypeKind
    // -> builtin type ItemId would be the best to improve that.
    fn add_builtin_item(&mut self, item: Item) {
        debug_assert!(item.kind().is_type());
        let id = item.id();
        let old_item = self.items.insert(id, item);
        assert!(old_item.is_none(), "Inserted type twice?");
    }

    fn build_root_module() -> Item {
        let module = Module::new(Some("root".into()));
        let id = ItemId::next();
        Item::new(id, None, None, id, ItemKind::Module(module))
    }

    pub fn root_module(&self) -> ItemId {
        self.root_module
    }

    pub fn resolve_type(&self, type_id: ItemId) -> &Type {
        self.items.get(&type_id).unwrap().kind().expect_type()
    }

    pub fn safe_resolve_type(&self, type_id: ItemId) -> Option<&Type> {
        self.items.get(&type_id).map(|t| t.kind().expect_type())
    }

    pub fn resolve_item_fallible(&self, item_id: ItemId) -> Option<&Item> {
        self.items.get(&item_id)
    }

    pub fn resolve_item(&self, item_id: ItemId) -> &Item {
        match self.items.get(&item_id) {
            Some(item) => item,
            None => panic!("Not an item: {:?}", item_id),
        }
    }

    pub fn current_module(&self) -> ItemId {
        self.current_module
    }

    /// This is one of the hackiest methods in all the parsing code. This method
    /// is used to allow having templates with another argument names instead of
    /// the canonical ones.
    ///
    /// This is surprisingly difficult to do with libclang, due to the fact that
    /// partial template specializations don't provide explicit template
    /// argument information.
    ///
    /// The only way to do this as far as I know, is inspecting manually the
    /// AST, looking for TypeRefs inside. This, unfortunately, doesn't work for
    /// more complex cases, see the comment on the assertion below.
    ///
    /// To see an example of what this handles:
    ///
    /// ```
    ///     template<typename T>
    ///     class Incomplete {
    ///       T p;
    ///     };
    ///
    ///     template<typename U>
    ///     class Foo {
    ///       Incomplete<U> bar;
    ///     };
    /// ```
    fn build_template_wrapper(&mut self,
                              wrapping: ItemId,
                              parent_id: ItemId,
                              ty: &clang::Type,
                              location: clang::Cursor) -> ItemId {
        use clangll::*;
        let mut args = vec![];
        let mut found_invalid_template_ref = false;
        let self_id = ItemId::next();
        location.visit(|c, _| {
            if c.kind() == CXCursor_TemplateRef &&
               c.cur_type().kind() == CXType_Invalid {
                found_invalid_template_ref = true;
            }
            if c.kind() == CXCursor_TypeRef {
                let new_ty =
                    Item::from_ty_or_ref(c.cur_type(), Some(*c), Some(self_id), self);
                args.push(new_ty);
            }
            CXChildVisit_Continue
        });

        let item = {
            let wrapping_type = self.resolve_type(wrapping);
            let old_args = match *wrapping_type.kind() {
                TypeKind::Comp(ref ci) => ci.template_args(),
                _ => panic!("how?"),
            };
            // The following assertion actually fails with partial template
            // specialization. But as far as I know there's no way at all to
            // grab the specialized types from neither the AST or libclang.
            //
            // This flaw was already on the old parser, but I now think it has
            // no clear solution.
            //
            // For an easy example in which there's no way at all of getting the
            // `int` type, except manually parsing the spelling:
            //
            //     template<typename T, typename U>
            //     class Incomplete {
            //       T d;
            //       U p;
            //     };
            //
            //     template<typename U>
            //     class Foo {
            //       Incomplete<U, int> bar;
            //     };
            //
            // debug_assert_eq!(old_args.len(), args.len());
            //
            // That being said, this is not so common, so just error! and hope
            // for the best, returning the previous type, who knows.
            if old_args.len() != args.len() {
                error!("Found partial template specialization, expect dragons!");
                return wrapping;
            }

            let type_kind = TypeKind::TemplateRef(wrapping, args);
            let name = ty.spelling();
            let name = if name.is_empty() { None } else { Some(name) };
            let ty = Type::new(name, ty.fallible_layout().ok(), type_kind, ty.is_const());
            Item::new(self_id, None, None, parent_id, ItemKind::Type(ty))
        };

        // Bypass all the validations in add_item explicitly.
        self.items.insert(self_id, item);
        self_id
    }

    /// Looks up for an already resolved type, either because it's builtin, or
    /// because we already have it in the map.
    pub fn builtin_or_resolved_ty(&mut self,
                                  parent_id: Option<ItemId>,
                                  ty: &clang::Type,
                                  location: Option<clang::Cursor>) -> Option<ItemId> {
        use clangll::{CXCursor_ClassTemplate, CXCursor_ClassTemplatePartialSpecialization};
        debug!("builtin_or_resolved_ty: {:?}, {:?}, {:?}", ty, location, parent_id);
        let mut declaration = ty.declaration();
        if !declaration.is_valid() {
            if let Some(location) = location {
                if location.kind() == CXCursor_ClassTemplate ||
                   location.kind() == CXCursor_ClassTemplatePartialSpecialization {
                    declaration = location;
                }
            }
        }
        let canonical_declaration = declaration.canonical();
        if canonical_declaration.is_valid() {
            // First lookup to see if we already have it resolved.
            let id = self.types.get(&canonical_declaration).map(|id| *id);
            if let Some(id) = id {
                debug!("Already resolved ty {:?}, {:?}, {:?} {:?}",
                       id, declaration, ty, location);
                // If the declaration existed, we *might* be done, but it's not
                // the case for class templates, where the template arguments
                // may vary.
                //
                // In this case, we create a TemplateRef with the new template
                // arguments, pointing to the canonical template.
                //
                // Note that we only do it if parent_id is some, and we have a
                // location for building the new arguments, the template
                // argument names don't matter in the global context.
                if (declaration.kind() == CXCursor_ClassTemplate ||
                    declaration.kind() == CXCursor_ClassTemplatePartialSpecialization) &&
                   *ty != canonical_declaration.cur_type() &&
                   location.is_some() && parent_id.is_some() {
                    return Some(
                        self.build_template_wrapper(id, parent_id.unwrap(), ty,
                                                    location.unwrap()));
                }

                return Some(self.build_ty_wrapper(id, parent_id, ty));
            }
        }

        debug!("Not resolved, maybe builtin?");

        // Else, build it.
        self.build_builtin_ty(ty, declaration)
    }

    // This is unfortunately a lot of bloat, but is needed to properly track
    // constness et. al.
    //
    // We should probably make the constness tracking separate, so it doesn't
    // bloat that much, but hey, we already bloat the heck out of builtin types.
    fn build_ty_wrapper(&mut self,
                        wrapped_id: ItemId,
                        parent_id: Option<ItemId>,
                        ty: &clang::Type) -> ItemId {
        let id = ItemId::next();
        let spelling = ty.spelling();
        let is_const = ty.is_const();
        let layout = ty.fallible_layout().ok();
        let type_kind = TypeKind::ResolvedTypeRef(wrapped_id);
        let ty = Type::new(Some(spelling), layout, type_kind, is_const);
        let item = Item::new(id, None, None,
                             parent_id.unwrap_or(self.current_module), ItemKind::Type(ty));
        self.add_builtin_item(item);
        id
    }

    fn build_builtin_ty(&mut self,
                        ty: &clang::Type,
                        _declaration: Cursor) -> Option<ItemId> {
        use clangll::*;
        let type_kind = match ty.kind() {
            CXType_NullPtr => TypeKind::NullPtr,
            CXType_Void => TypeKind::Void,
            CXType_Bool => TypeKind::Int(IntKind::Bool),
            CXType_Int => TypeKind::Int(IntKind::Int),
            CXType_UInt => TypeKind::Int(IntKind::UInt),
            CXType_SChar |
            CXType_Char_S => TypeKind::Int(IntKind::Char),
            CXType_UChar |
            CXType_Char_U => TypeKind::Int(IntKind::UChar),
            CXType_Short => TypeKind::Int(IntKind::Short),
            CXType_UShort => TypeKind::Int(IntKind::UShort),
            CXType_WChar |
            CXType_Char16 => TypeKind::Int(IntKind::U16),
            CXType_Char32 => TypeKind::Int(IntKind::U32),
            CXType_Long => TypeKind::Int(IntKind::Long),
            CXType_ULong => TypeKind::Int(IntKind::ULong),
            CXType_LongLong => TypeKind::Int(IntKind::LongLong),
            CXType_ULongLong => TypeKind::Int(IntKind::ULongLong),
            CXType_Float => TypeKind::Float(FloatKind::Float),
            CXType_Double => TypeKind::Float(FloatKind::Double),
            CXType_LongDouble => TypeKind::Float(FloatKind::LongDouble),
            _ => return None,
        };

        let spelling = ty.spelling();
        let is_const = ty.is_const();
        let layout = ty.fallible_layout().ok();
        let ty = Type::new(Some(spelling), layout, type_kind, is_const);
        let id = ItemId::next();
        let item = Item::new(id, None, None, self.root_module, ItemKind::Type(ty));
        self.add_builtin_item(item);
        Some(id)
    }

    pub fn translation_unit(&self) -> &clang::TranslationUnit {
        &self.translation_unit
    }

    pub fn parsed_macro(&self, macro_name: &str) -> bool {
        self.parsed_macros.contains(macro_name)
    }

    pub fn note_parsed_macro(&mut self, macro_name: String) {
        debug_assert!(!self.parsed_macros.contains(&macro_name));
        self.parsed_macros.insert(macro_name);
    }

    pub fn in_codegen_phase(&self) -> bool {
        self.gen_ctx.is_some()
    }

    /// This is a bit of a hack, but it's done so using the replaces="xxx"
    /// annotation implies hide in the other type.
    pub fn replace(&mut self, name: &str, potential_ty: ItemId) {
        self.replacements.insert(name.into(), potential_ty);
    }

    pub fn hidden_by_name(&self, name: &str) -> bool {
        debug_assert!(self.in_codegen_phase(),
                      "You're not supposed to call this yet");
        self.options.hidden_types.contains(name)
    }

    pub fn opaque_by_name(&self, name: &str) -> bool {
        debug_assert!(self.in_codegen_phase(),
                      "You're not supposed to call this yet");
        self.options.opaque_types.contains(name)
    }

    pub fn options(&self) -> &BindgenOptions {
        &self.options
    }

    /// Given a CXCursor_Namespace cursor, return the item id of the
    /// corresponding module, or create one on the fly.
    pub fn module(&mut self, cursor: clang::Cursor) -> ItemId {
        use clangll::*;
        assert!(cursor.kind() == CXCursor_Namespace, "Be a nice person");
        let cursor = cursor.canonical();
        let module_id = match self.modules.get(&cursor) {
            Some(id) => return *id,
            None => ItemId::next(),
        };

        let module_name = self.translation_unit
                              .tokens(&cursor).and_then(|tokens| {
            if tokens.len() <= 1 {
                None
            } else {
                match &*tokens[1].spelling {
                    "{" => None,
                    s => Some(s.to_owned()),
                }
            }
        });

        let module = Module::new(module_name);
        let module = Item::new(module_id, None, None, self.current_module,
                               ItemKind::Module(module));

        self.add_item(module, None, None);

        module_id
    }

    pub fn with_module<F>(&mut self, module_id: ItemId, cb: F)
        where F: FnOnce(&mut Self, &mut Vec<ItemId>)
    {
        debug_assert!(self.resolve_item(module_id).kind().is_module(), "Wat");

        let previous_id = self.current_module;
        self.current_module = module_id;

        let mut children = vec![];
        cb(self, &mut children);

        self.items.get_mut(&module_id).unwrap()
                  .as_module_mut().expect("Not a module?")
                  .children_mut().extend(children.into_iter());

        self.current_module = previous_id;
    }
}

/// This was originally a type that only exposes the resolve_type operation to
/// its consumers.
///
/// Later a made resolve_type public, so... meh. It should go away soon.
pub type TypeResolver<'ctx> = BindgenContext<'ctx>;
