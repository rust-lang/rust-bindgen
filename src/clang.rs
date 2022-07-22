//! A higher level Clang API built on top of the generated bindings in the
//! `clang_sys` module.

#![allow(non_upper_case_globals, dead_code)]

use crate::ir::context::BindgenContext;
use cexpr;
use regex;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::Hash;
use std::os::raw::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong};
use std::{mem, ptr, slice};

#[allow(non_camel_case_types, non_snake_case, missing_docs)]
mod clang_interface;
pub use self::clang_interface::CXDiagnosticSeverity::Type as CXDiagnosticSeverity;
pub use self::clang_interface::CXCallingConv::Type as CXCallingConv;
pub use self::clang_interface::CXLinkageKind::Type as CXLinkageKind;
pub use self::clang_interface::CX_CXXAccessSpecifier::Type as CX_CXXAccessSpecifier;
pub use self::clang_interface::CXEvalResultKind::Type as CXEvalResultKind;
pub use self::clang_interface::CXTokenKind::Type as CXTokenKind;
pub use self::clang_interface::CXVisibilityKind::Type as CXVisibilityKind;
pub use self::clang_interface::CXChildVisitResult::Type as CXChildVisitResult;
pub use self::clang_interface::CXCursorKind::Type as CXCursorKind;
pub use self::clang_interface::CXTypeKind::Type as CXTypeKind;
pub use self::clang_interface::CXCommentKind::*;
pub use self::clang_interface::CXDiagnosticSeverity::*;
pub use self::clang_interface::CXCallingConv::*;
pub use self::clang_interface::CX_CXXAccessSpecifier::*;
pub use self::clang_interface::CXChildVisitResult::*;
pub use self::clang_interface::CXCursorKind::*;
pub use self::clang_interface::CXEvalResultKind::*;
pub use self::clang_interface::CXLinkageKind::*;
pub use self::clang_interface::CXTypeKind::*;
pub use self::clang_interface::CXTokenKind::*;
pub use self::clang_interface::CXVisitorResult::*;
pub use self::clang_interface::CXVisibilityKind::*;

impl clang_interface::BindgenSourceRange {
    fn null() -> Self {
        Self { B: ptr::null_mut(), E: ptr::null_mut() }
    }
}

trait ToCString {
    fn to_cstring(&self) -> CString;
}

impl ToCString for clang_interface::BindgenStringRef {
    fn to_cstring(&self) -> CString {
        if !self.s.is_null() {
            unsafe { CStr::from_ptr(self.s).into() }
        } else {
            return CString::new("").unwrap();
        }
    }
}

impl fmt::Display for clang_interface::BindgenStringRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self.to_cstring();
        write!(f, "{}", str.to_str().unwrap())
    }
}

/// A cursor into the Clang AST, pointing to an AST node.
///
/// We call the AST node pointed to by the cursor the cursor's "referent".
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    node: ASTNode,

    /// Kind of this cursor, may differ from the kind of the node, e.g. TypeRef
    /// kind referring to a ClassDecl.
    kind: CXCursorKind,

    /// AST unit that this cursor is part of.
    ///
    /// Some clang interfaces require access to an ASTUnit, so we keep this
    /// available.
    unit: *mut clang_interface::clang_ASTUnit,
}

/// Clang AST nodes.
///
/// Each variant wraps a raw pointer to a type of Clang AST node that we handle.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ASTNode {
    /// Placeholder for an invalid AST node
    Invalid,

    /// Declaration AST node (const Decl*)
    Decl(*const clang_interface::clang_Decl),

    /// Expression AST node (const Expr*)
    Expr(*const clang_interface::clang_Expr),

    /// C++ base specifier AST node (const CXXBaseSpecifier*)
    CXXBaseSpecifier(*const clang_interface::clang_CXXBaseSpecifier),

    /// Attribute AST node (const Attr*)
    Attr(*const clang_interface::clang_Attr),

    /// Preprocessor entity node (const PreprocessedEntity*)
    PreprocessedEntity(*const clang_interface::clang_PreprocessedEntity),
}

impl ASTNode {
    /// Is this node valid?
    fn is_valid(&self) -> bool {
        unsafe { !clang_interface::CursorKind_isInvalid(self.kind()) }
    }

    /// Kind of the AST node. This is NOT the kind of the cursor itself, and may
    /// differ from a cursor holding the node.
    fn kind(&self) -> CXCursorKind {
        unsafe {
            match *self {
                ASTNode::Decl(d) => clang_interface::Decl_getCXCursorKind(d),
                ASTNode::Expr(e) => clang_interface::Expr_getCXCursorKind(e),
                ASTNode::CXXBaseSpecifier(_) => CXCursor_CXXBaseSpecifier,
                ASTNode::Attr(a) => clang_interface::Attr_getCXCursorKind(a),
                _ => CXCursor_InvalidFile,
            }
        }
    }
}

impl fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Cursor({} kind: {}, loc: {}, usr: {:?})",
            self.spelling(),
            kind_to_str(self.kind()),
            self.location(),
            self.usr()
        )
    }
}

impl Cursor {
    /// Create a new Cursor from an ASTNode and a clang ASTUnit
    fn new(node: ASTNode, unit: *mut clang_interface::clang_ASTUnit) -> Self {
        Self {
            node,
            kind: node.kind(),
            unit,
        }
    }

    /// Create a new Cursor with the given ASTNode in the same clang ASTUnit as
    /// self.
    fn with_node(&self, node: ASTNode) -> Self {
        Self::new(node, self.unit)
    }

    /// Get the clang ASTContext for this cursor
    fn context(&self) -> *mut clang_interface::clang_ASTContext {
        unsafe { clang_interface::ASTUnit_getContext(self.unit) }
    }

    /// Get the Unified Symbol Resolution for this cursor's referent, if
    /// available.
    ///
    /// The USR can be used to compare entities across translation units.
    pub fn usr(&self) -> Option<String> {
        let s = unsafe {
            match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getUSR(d),
                _ => return None,
            }
        };
        let s = s.to_string();
        if s.len() == 0 {
            None
        } else {
            Some(s)
        }
    }

    /// Is this cursor's referent a declaration?
    pub fn is_declaration(&self) -> bool {
        match self.node {
            ASTNode::Decl(_) => true,
            _ => false,
        }
    }

    /// Get this cursor's referent's spelling.
    pub fn spelling(&self) -> String {
        unsafe {
            match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getSpelling(d).to_string(),
                ASTNode::Expr(e) => clang_interface::Expr_getSpelling(e).to_string(),
                ASTNode::CXXBaseSpecifier(b) => clang_interface::CXXBaseSpecifier_getSpelling(b)
                    .to_string(),
                ASTNode::PreprocessedEntity(e) => clang_interface::PreprocessedEntity_getSpelling(e).to_string(),
                _ => String::new(),
            }
        }
    }

    /// Get the mangled name of this cursor's referent.
    pub fn mangling(&self) -> String {
        unsafe {
            match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getMangling(d, self.context()).to_string(),
                _ => String::new(),
            }
        }
    }

    /// Gets the C++ manglings for this cursor, or an error if the manglings
    /// are not available.
    pub fn cxx_manglings(&self) -> Result<Vec<String>, ()> {
        unsafe {
            let manglings = match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getCXXManglings(d, self.context()),
                _ => return Err(()),
            };
            let count = manglings.len as usize;

            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let string_ptr = manglings.strings.offset(i as isize);
                result.push((*string_ptr).to_string());
            }
            // clang_disposeStringSet(manglings);
            Ok(result)
        }
    }

    /// Returns whether the cursor refers to a built-in definition.
    pub fn is_builtin(&self) -> bool {
        let (file, _, _, _) = self.location().location();
        file.name().is_none()
    }

    /// Get the `Cursor` for this cursor's referent's lexical parent.
    ///
    /// The lexical parent is the parent of the definition. The semantic parent
    /// is the parent of the declaration. Generally, the lexical parent doesn't
    /// have any effect on semantics, while the semantic parent does.
    ///
    /// In the following snippet, the `Foo` class would be the semantic parent
    /// of the out-of-line `method` definition, while the lexical parent is the
    /// translation unit.
    ///
    /// ```c++
    /// class Foo {
    ///     void method();
    /// };
    ///
    /// void Foo::method() { /* ... */ }
    /// ```
    pub fn lexical_parent(&self) -> Cursor {
        let node = match self.node {
            ASTNode::Decl(d) => unsafe {
                ASTNode::Decl(clang_interface::Decl_getLexicalParent(d))
            },
            _ => ASTNode::Invalid,
        };
        self.with_node(node)
    }

    /// Get the referent's semantic parent, if one is available.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub fn fallible_semantic_parent(&self) -> Option<Cursor> {
        let node = match self.node {
            ASTNode::Decl(d) => unsafe {
                ASTNode::Decl(clang_interface::Decl_getSemanticParent(d))
            },
            ASTNode::Expr(_) => panic!("Unimplemented for Expr"),
            _ => return None,
        };
        if node == self.node || !node.is_valid() {
            return None;
        }
        Some(self.with_node(node))
    }

    /// Get the referent's semantic parent.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub fn semantic_parent(&self) -> Cursor {
        self.fallible_semantic_parent().unwrap()
    }

    /// Return the number of template arguments used by this cursor's referent,
    /// if the referent is either a template instantiation. Returns `None`
    /// otherwise.
    ///
    /// NOTE: This may not return `Some` for partial template specializations,
    /// see #193 and #194.
    pub fn num_template_args(&self) -> Option<u32> {
        // XXX: `clang_Type_getNumTemplateArguments` is sort of reliable, while
        // `clang_Cursor_getNumTemplateArguments` is totally unreliable.
        // Therefore, try former first, and only fallback to the latter if we
        // have to.
        let decl = if let ASTNode::Decl(decl) = self.node {
            decl
        } else {
            return None;
        };
        self.cur_type()
            .num_template_args()
            .or_else(|| {
                let n: c_int = 
                    unsafe { clang_interface::Decl_getNumTemplateArguments(decl) };

                if n >= 0 {
                    Some(n as u32)
                } else {
                    debug_assert_eq!(n, -1);
                    None
                }
            })
            .or_else(|| {
                let canonical = self.canonical();
                if canonical != *self {
                    canonical.num_template_args()
                } else {
                    None
                }
            })
    }

    /// Get a cursor pointing to this referent's containing translation unit.
    ///
    /// Note that we shouldn't create a `TranslationUnit` struct here, because
    /// bindgen assumes there will only be one of them alive at a time, and
    /// disposes it on drop. That can change if this would be required, but I
    /// think we can survive fine without it.
    pub fn translation_unit(&self) -> *mut clang_interface::clang_ASTUnit {
        self.unit
    }

    /// Is the referent a top level construct?
    pub fn is_toplevel(&self) -> bool {
        let mut semantic_parent = self.fallible_semantic_parent();

        while semantic_parent.is_some() &&
            (semantic_parent.unwrap().kind() == CXCursor_Namespace ||
             semantic_parent.unwrap().kind() ==
             CXCursor_NamespaceAlias ||
                semantic_parent.unwrap().kind() == CXCursor_NamespaceRef)
        {
            semantic_parent =
                semantic_parent.unwrap().fallible_semantic_parent();
        }

        let tu = self.with_node(ASTNode::Decl(unsafe {
            clang_interface::getTranslationUnitDecl(self.unit)
        }));
        // Yes, this can happen with, e.g., macro definitions.
        semantic_parent == tu.fallible_semantic_parent()
    }

    /// There are a few kinds of types that we need to treat specially, mainly
    /// not tracking the type declaration but the location of the cursor, given
    /// clang doesn't expose a proper declaration for these types.
    pub fn is_template_like(&self) -> bool {
        matches!(
            self.kind(),
            CXCursor_ClassTemplate |
                CXCursor_ClassTemplatePartialSpecialization |
                CXCursor_TypeAliasTemplateDecl
        )
    }

    /// Is this Cursor pointing to a function-like macro definition?
    pub fn is_macro_function_like(&self) -> bool {
        unsafe { clang_Cursor_isMacroFunctionLike(self.x) != 0 }
    }

    /// Get the kind of referent this cursor is pointing to.
    pub fn kind(&self) -> CXCursorKind {
        self.kind
    }

    /// Returns true if the cursor is a definition
    pub fn is_definition(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_isDefinition(d)
            },
            _ => false,
        }
    }

    /// Is the referent a template specialization?
    pub fn is_template_specialization(&self) -> bool {
        self.specialized().is_some()
    }

    /// Is the referent a fully specialized template specialization without any
    /// remaining free template arguments?
    pub fn is_fully_specialized_template(&self) -> bool {
        self.is_template_specialization() &&
            self.kind() != CXCursor_ClassTemplatePartialSpecialization &&
            self.num_template_args().unwrap_or(0) > 0
    }

    /// Is the referent a template specialization that still has remaining free
    /// template arguments?
    pub fn is_in_non_fully_specialized_template(&self) -> bool {
        if self.is_toplevel() {
            return false;
        }

        let parent = self.semantic_parent();
        if parent.is_fully_specialized_template() {
            return false;
        }

        if !parent.is_template_like() {
            return parent.is_in_non_fully_specialized_template();
        }

        true
    }

    /// Is this cursor pointing a valid referent?
    pub fn is_valid(&self) -> bool {
        unsafe { !clang_interface::CursorKind_isInvalid(self.kind) }
    }

    /// Get the source location for the referent.
    pub fn location(&self) -> SourceLocation {
        unsafe {
            let x = match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getLocation(d),
                ASTNode::Expr(e) => clang_interface::Expr_getLocation(e),
                ASTNode::CXXBaseSpecifier(b) => clang_interface::CXXBaseSpecifier_getLocation(b),
                ASTNode::Attr(b) => clang_interface::Attr_getLocation(b),
                ASTNode::PreprocessedEntity(p) => clang_interface::PreprocessedEntity_getLocation(p),
                ASTNode::Invalid => ptr::null(),
            };
            SourceLocation { x, unit: self.unit }
        }
    }

    /// Get the source location range for the referent.
    ///
    /// Warning: This range goes from the start of the first token to the start
    /// of the last token, unlike the ranges provided by libclang, which are
    /// half-open ranges of characters (end is past the last character in the
    /// last token).
    pub fn extent(&self) -> clang_interface::BindgenSourceRange {
        unsafe {
            match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getSourceRange(d),
                ASTNode::Expr(e) => clang_interface::Expr_getSourceRange(e),
                ASTNode::CXXBaseSpecifier(b) => clang_interface::CXXBaseSpecifier_getSourceRange(b),
                ASTNode::Attr(b) => clang_interface::Attr_getSourceRange(b),
                ASTNode::PreprocessedEntity(b) => clang_interface::PreprocessedEntity_getSourceRange(b),
                _ => clang_interface::BindgenSourceRange::null(),
            }
        }
    }

    /// Get the raw declaration comment for this referent, if one exists.
    pub fn raw_comment(&self) -> Option<String> {
        let s = match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getRawCommentText(d, self.context()).to_string()
            },
            _ => return None,
        };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Get the referent's parsed comment.
    pub fn comment(&self) -> Comment {
        let x = match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getParsedComment(d, self.context())
            },
            _ => ptr::null(),
        };
        Comment { x }
    }

    /// Get the referent's type.
    pub fn cur_type(&self) -> Type {
        unsafe {
            let x = match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getType(d, self.context()),
                ASTNode::Expr(e) => clang_interface::Expr_getType(e),
                ASTNode::CXXBaseSpecifier(base) => clang_interface::CXXBaseSpecifier_getType(base),
                _ => mem::zeroed(),
            };
            Type { x, unit: self.unit }
        }
    }

    /// Given that this cursor's referent is a reference to another type, or is
    /// a declaration, get the cursor pointing to the referenced type or type of
    /// the declared thing.
    pub fn definition(&self) -> Option<Cursor> {
        let is_reference = self.kind >= CXCursor_FirstRef && self.kind <= CXCursor_LastRef;
        let def = match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getDefinition(d, is_reference)
            },
            _ => return None,
        };
        if def.is_null() {
            None
        } else {
            Some(self.with_node(ASTNode::Decl(def)))
        }
    }

    /// Given that this cursor's referent is reference type, get the cursor
    /// pointing to the referenced type.
    pub fn referenced(&self) -> Option<Cursor> {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                let ptr = clang_interface::Decl_getReferenced(d);
                if ptr.is_null() {
                    None
                } else {
                    Some(self.with_node(ASTNode::Decl(ptr)))
                }
            },
            _ => return None,
        }
    }

    /// Get the canonical cursor for this referent.
    ///
    /// Many types can be declared multiple times before finally being properly
    /// defined. This method allows us to get the canonical cursor for the
    /// referent type.
    pub fn canonical(&self) -> Cursor {
        let node = match self.node {
            ASTNode::Decl(d) => unsafe {
                ASTNode::Decl(clang_interface::Decl_getCanonical(d))
            },
            _ => ASTNode::Invalid,
        };
        self.with_node(node)
    }

    /// Given that this cursor points to either a template specialization or a
    /// template instantiation, get a cursor pointing to the template definition
    /// that is being specialized.
    pub fn specialized(&self) -> Option<Cursor> {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                let ptr = clang_interface::Decl_getSpecializedTemplate(d);
                if ptr.is_null() {
                    None
                } else {
                    Some(self.with_node(ASTNode::Decl(ptr)))
                }
            },
            _ => None,
        }
    }

    /// Assuming that this cursor's referent is a template declaration, get the
    /// kind of cursor that would be generated for its specializations.
    pub fn template_kind(&self) -> CXCursorKind {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getTemplateCursorKind(d)
            },
            _ => CXCursor_NoDeclFound,
        }
    }

    /// Traverse this cursor's referent and its children.
    ///
    /// Call the given function on each AST node traversed.
    pub fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(Cursor) -> CXChildVisitResult,
    {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_visitChildren(
                    d,
                    self.kind,
                    Some(visit_children::<Visitor>),
                    self.unit,
                    mem::transmute(&mut visitor),
                );
            }
            ASTNode::Expr(e) => unsafe {
                clang_interface::Expr_visitChildren(
                    e,
                    self.kind,
                    Some(visit_children::<Visitor>),
                    self.unit,
                    mem::transmute(&mut visitor),
                );
            }
            ASTNode::CXXBaseSpecifier(b) => unsafe {
                clang_interface::CXXBaseSpecifier_visitChildren(
                    b,
                    self.kind,
                    Some(visit_children::<Visitor>),
                    self.unit,
                    mem::transmute(&mut visitor),
                );
            }
            _ => panic!("Tried to visit: {:?}", self),
        }
    }

    /// Collect all of this cursor's children into a vec and return them.
    pub fn collect_children(&self) -> Vec<Cursor> {
        let mut children = vec![];
        self.visit(|c| {
            children.push(c);
            CXChildVisit_Continue
        });
        children
    }

    /// Does this cursor have any children?
    pub fn has_children(&self) -> bool {
        let mut has_children = false;
        self.visit(|_| {
            has_children = true;
            CXChildVisit_Break
        });
        has_children
    }

    /// Does this cursor have at least `n` children?
    pub fn has_at_least_num_children(&self, n: usize) -> bool {
        assert!(n > 0);
        let mut num_left = n;
        self.visit(|_| {
            num_left -= 1;
            if num_left == 0 {
                CXChildVisit_Break
            } else {
                CXChildVisit_Continue
            }
        });
        num_left == 0
    }

    /// Returns whether the given location contains a cursor with the given
    /// kind in the first level of nesting underneath (doesn't look
    /// recursively).
    pub fn contains_cursor(&self, kind: CXCursorKind) -> bool {
        let mut found = false;

        self.visit(|c| {
            if c.kind() == kind {
                found = true;
                CXChildVisit_Break
            } else {
                CXChildVisit_Continue
            }
        });

        found
    }

    /// Is the referent an inlined function?
    pub fn is_inlined_function(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_isFunctionInlined(d)
            },
            _ => false,
        }
    }

    /// Is the referent a defaulted function?
    pub fn is_defaulted_function(&self) -> bool {
        unsafe { clang_CXXMethod_isDefaulted(self.x) != 0 }
    }

    /// Is the referent a deleted function?
    pub fn is_deleted_function(&self) -> bool {
        // Unfortunately, libclang doesn't yet have an API for checking if a
        // member function is deleted, but the following should be a good
        // enough approximation.
        // Deleted functions are implicitly inline according to paragraph 4 of
        // [dcl.fct.def.delete] in the C++ standard. Normal inline functions
        // have a definition in the same translation unit, so if this is an
        // inline function without a definition, and it's not a defaulted
        // function, we can reasonably safely conclude that it's a deleted
        // function.
        self.is_inlined_function() &&
            self.definition().is_none() &&
            !self.is_defaulted_function()
    }

    /// Get the width of this cursor's referent bit field, or `None` if the
    /// referent is not a bit field.
    pub fn bit_width(&self) -> Option<u32> {
        let w = match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getFieldDeclBitWidth(d, self.context())
            },
            _ => -1,
        };
        if w == -1 {
            None
        } else {
            Some(w as u32)
        }
    }

    /// Get the integer representation type used to hold this cursor's referent
    /// enum type.
    pub fn enum_type(&self) -> Option<Type> {
        let x = unsafe {
            match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_getEnumDeclIntegerType(d),
                _ => mem::zeroed(),
            }
        };
        if x.ptr.is_null() {
            None
        } else {
            Some(Type { x, unit: self.unit })
        }
    }

    /// Get the boolean constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub fn enum_val_boolean(&self) -> Option<bool> {
        self.enum_val_unsigned().map(|x| x != 0)
    }

    /// Get the signed constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub fn enum_val_signed(&self) -> Option<i64> {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                Some(clang_interface::Decl_getEnumConstantValue(d))
            }
            _ => None,
        }
    }

    /// Get the unsigned constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub fn enum_val_unsigned(&self) -> Option<u64> {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                Some(clang_interface::Decl_getEnumConstantUnsignedValue(d))
            }
            _ => None,
        }
    }

    /// Whether this cursor has the `warn_unused_result` attribute.
    pub fn has_warn_unused_result_attr(&self) -> bool {
        // FIXME(emilio): clang-sys doesn't expose this (from clang 9).
        const CXCursor_WarnUnusedResultAttr: CXCursorKind = 440;
        self.has_attr("warn_unused_result", Some(CXCursor_WarnUnusedResultAttr))
    }

    /// Does this cursor have the given attribute?
    ///
    /// `name` is checked against unexposed attributes.
    fn has_attr(&self, name: &str, clang_kind: Option<CXCursorKind>) -> bool {
        let mut found_attr = false;
        self.visit(|cur| {
            let kind = cur.kind();
            found_attr = clang_kind.map_or(false, |k| k == kind) ||
                (kind == CXCursor_UnexposedAttr &&
                    cur.tokens().iter().any(|t| {
                        t.kind == CXToken_Identifier &&
                            t.spelling() == name.as_bytes()
                    }));

            if found_attr {
                CXChildVisit_Break
            } else {
                CXChildVisit_Continue
            }
        });

        found_attr
    }

    /// Given that this cursor's referent is a `typedef`, get the `Type` that is
    /// being aliased.
    pub fn typedef_type(&self) -> Option<Type> {
        match self.node {
            ASTNode::Decl(d) => Some(Type {
                x: unsafe { clang_interface::Decl_getTypedefDeclUnderlyingType(d) },
                unit: self.unit,
            }),
            _ => None,
        }
    }

    /// Get the linkage kind for this cursor's referent.
    ///
    /// This only applies to functions and variables.
    pub fn linkage(&self) -> CXLinkageKind {
        match self.node {
            ASTNode::Decl(d) => unsafe { clang_interface::Decl_getLinkage(d) },
            _ => CXLinkage_Invalid,
        }
    }

    /// Get the visibility of this cursor's referent.
    pub fn visibility(&self) -> CXVisibilityKind {
        match self.node {
            ASTNode::Decl(d) => unsafe { clang_interface::Decl_getVisibility(d) },
            _ => CXVisibility_Invalid,
        }
    }

    /// Given that this cursor's referent is a function, return cursors to its
    /// parameters.
    ///
    /// Returns None if the cursor's referent is not a function/method call or
    /// declaration.
    pub fn args(&self) -> Option<Vec<Cursor>> {
        // match self.kind() {
        // CXCursor_FunctionDecl |
        // CXCursor_CXXMethod => {
        self.num_args().ok().map(|num| {
            (0..num)
                .map(|i| {
                    let node = match self.node {
                        ASTNode::Decl(d) => unsafe {
                            ASTNode::Decl(clang_interface::Decl_getArgument(d, i as c_uint))
                        },
                        ASTNode::Expr(e) => unsafe {
                            ASTNode::Expr(clang_interface::Expr_getArgument(e, i as c_uint))
                        },
                        _ => ASTNode::Invalid,
                    };
                    self.with_node(node)
                })
                .collect()
        })
    }

    /// Given that this cursor's referent is a function/method call or
    /// declaration, return the number of arguments it takes.
    ///
    /// Returns Err if the cursor's referent is not a function/method call or
    /// declaration.
    pub fn num_args(&self) -> Result<u32, ()> {
        let w = match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getNumArguments(d)
            },
            ASTNode::Expr(e) => unsafe {
                clang_interface::Expr_getNumArguments(e)
            },
            _ => -1,
        };
        if w == -1 {
            Err(())
        } else {
            Ok(w as u32)
        }
    }

    /// Get the access specifier for this cursor's referent.
    pub fn access_specifier(&self) -> CX_CXXAccessSpecifier {
        match self.node {
            ASTNode::Decl(d) => unsafe { clang_interface::Decl_getAccess(d) },
            ASTNode::CXXBaseSpecifier(b) => unsafe {
                clang_interface::CXXBaseSpecifier_getAccess(b)
            },
            _ => CX_CXXInvalidAccessSpecifier,
        }
    }

    /// Is the cursor's referrent publically accessible in C++?
    ///
    /// Returns true if self.access_specifier() is `CX_CXXPublic` or
    /// `CX_CXXInvalidAccessSpecifier`.
    pub fn public_accessible(&self) -> bool {
        let access = self.access_specifier();
        access == CX_CXXPublic || access == CX_CXXInvalidAccessSpecifier
    }

    /// Is this cursor's referent a field declaration that is marked as
    /// `mutable`?
    pub fn is_mutable_field(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::CXXField_isMutable(d)
            },
            _ => false,
        }
    }

    /// Get the offset of the field represented by the Cursor.
    pub fn offset_of_field(&self) -> Result<usize, LayoutError> {
        let offset = match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::Decl_getOffsetOfField(d, self.context())
            },
            _ => -1,
        };
        if offset < 0 {
            Err(LayoutError::from(offset as i32))
        } else {
            Ok(offset as usize)
        }
    }

    /// Is this cursor's referent a member function that is declared `static`?
    pub fn method_is_static(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::CXXMethod_isStatic(d)
            },
            _ => false,
        }
    }

    /// Is this cursor's referent a member function that is declared `const`?
    pub fn method_is_const(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::CXXMethod_isConst(d)
            },
            _ => false,
        }
    }

    /// Is this cursor's referent a member function that is virtual?
    pub fn method_is_virtual(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::CXXMethod_isVirtual(d)
            },
            _ => false,
        }
    }

    /// Is this cursor's referent a member function that is pure virtual?
    pub fn method_is_pure_virtual(&self) -> bool {
        match self.node {
            ASTNode::Decl(d) => unsafe {
                clang_interface::CXXMethod_isPureVirtual(d)
            },
            _ => false,
        }
    }

    /// Is this cursor's referent a struct or class with virtual members?
    pub fn is_virtual_base(&self) -> bool {
        match self.node {
            ASTNode::CXXBaseSpecifier(b) => unsafe {
                clang_interface::CXXBaseSpecifier_isVirtualBase(b)
            },
            _ => false,
        }
    }

    /// Try to evaluate this cursor.
    pub fn evaluate(&self) -> Option<EvalResult> {
        // Work around https://bugs.llvm.org/show_bug.cgi?id=42532, see:
        //  * https://github.com/rust-lang/rust-bindgen/issues/283
        //  * https://github.com/rust-lang/rust-bindgen/issues/1590
        {
            let mut found_cant_eval = false;
            self.visit(|c| {
                if c.kind() == CXCursor_TypeRef &&
                    c.cur_type().canonical_type().kind() == CXType_Unexposed
                {
                    found_cant_eval = true;
                    return CXChildVisit_Break;
                }

                CXChildVisit_Recurse
            });

            if found_cant_eval {
                return None;
            }
        }
        unsafe {
            let x = match self.node {
                ASTNode::Decl(d) => clang_interface::Decl_Evaluate(d, self.context()),
                ASTNode::Expr(e) => clang_interface::Expr_Evaluate(e, self.context()),
                _ => return None,
            };
            Some(EvalResult { x })
        }
    }

    /// Return the result type for this cursor
    pub fn ret_type(&self) -> Option<Type> {
        match self.node {
            ASTNode::Decl(d) => Some(Type {
                x: unsafe { clang_interface::Decl_getResultType(d, self.context()) },
                unit: self.unit,
            }),
            _ => None,
        }
    }

    /// Gets the tokens that correspond to that cursor.
    pub fn tokens(&self) -> RawTokens {
        RawTokens::new(self)
    }

    /// Gets the tokens that correspond to that cursor as  `cexpr` tokens.
    pub fn cexpr_tokens(self) -> Vec<cexpr::token::Token> {
        self.tokens()
            .iter()
            .filter_map(|token| token.as_cexpr_token())
            .collect()
    }

    /// Obtain the real path name of a cursor of InclusionDirective kind.
    ///
    /// Returns None if the cursor does not include a file, otherwise the file's full name
    pub fn get_included_file_name(&self) -> Option<String> {
        // TODO(sjc): implement
        None
        // let file = unsafe { clang_sys::clang_getIncludedFile(self.x) };
        // if file.is_null() {
        //     None
        // } else {
        //     Some(unsafe {
        //         cxstring_into_string(clang_sys::clang_getFileName(file))
        //     })
        // }
    }
}

/// A struct that owns the tokenizer result from a given cursor.
pub struct RawTokens<'a> {
    cursor: &'a Cursor,
    tu: *mut clang_interface::clang_ASTUnit,
    tokens: *mut clang_interface::CXToken,
    token_count: c_uint,
}

impl<'a> RawTokens<'a> {
    fn new(cursor: &'a Cursor) -> Self {
        let mut tokens = ptr::null_mut();
        let mut token_count = 0;
        let range = cursor.extent();
        let tu = cursor.translation_unit();
        unsafe { clang_interface::tokenize(tu, range, &mut tokens, &mut token_count) };
        Self {
            cursor,
            tu,
            tokens,
            token_count,
        }
    }

    fn as_slice(&self) -> &[clang_interface::CXToken] {
        if self.tokens.is_null() {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.tokens, self.token_count as usize) }
    }

    /// Get an iterator over these tokens.
    pub fn iter(&self) -> ClangTokenIterator {
        ClangTokenIterator {
            tu: self.tu,
            raw: self.as_slice().iter(),
        }
    }
}

impl<'a> Drop for RawTokens<'a> {
    fn drop(&mut self) {
        if !self.tokens.is_null() {
            unsafe {
                clang_interface::disposeTokens(
                    self.tu,
                    self.tokens,
                    self.token_count as c_uint,
                );
            }
        }
    }
}

/// A raw clang token, that exposes only the kind and spelling. This is a
/// slightly more convenient version of `CXToken` which owns the spelling
/// string.
#[derive(Debug)]
pub struct ClangToken {
    spelling: CString,
    /// The kind of token, this is the same as the relevant member from
    /// `CXToken`.
    pub kind: CXTokenKind,
}

impl ClangToken {
    /// Get the token spelling, without being converted to utf-8.
    pub fn spelling(&self) -> &[u8] {
        self.spelling.to_bytes()
    }

    /// Converts a ClangToken to a `cexpr` token if possible.
    pub fn as_cexpr_token(&self) -> Option<cexpr::token::Token> {
        use cexpr::token;

        let kind = match self.kind {
            CXToken_Punctuation => token::Kind::Punctuation,
            CXToken_Literal => token::Kind::Literal,
            CXToken_Identifier => token::Kind::Identifier,
            CXToken_Keyword => token::Kind::Keyword,
            // NB: cexpr is not too happy about comments inside
            // expressions, so we strip them down here.
            CXToken_Comment => return None,
            _ => {
                warn!("Found unexpected token kind: {:?}", self);
                return None;
            }
        };

        Some(token::Token {
            kind,
            raw: self.spelling().to_vec().into_boxed_slice(),
        })
    }
}

/// An iterator over a set of Tokens.
pub struct ClangTokenIterator<'a> {
    tu: *mut clang_interface::clang_ASTUnit,
    raw: slice::Iter<'a, clang_interface::CXToken>,
}

impl<'a> Iterator for ClangTokenIterator<'a> {
    type Item = ClangToken;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.raw.next()?;
        unsafe {
            let kind = clang_interface::getTokenKind(*raw);
            let spelling = clang_interface::getTokenSpelling(self.tu, *raw).to_cstring();
            Some(ClangToken {
                kind,
                spelling,
            })
        }
    }
}

/// Checks whether the name looks like an identifier, i.e. is alphanumeric
/// (including '_') and does not start with a digit.
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let first_valid = chars
        .next()
        .map(|c| c.is_alphabetic() || c == '_')
        .unwrap_or(false);

    first_valid && chars.all(|c| c.is_alphanumeric() || c == '_')
}

unsafe extern "C" fn visit_children<Visitor>(
    raw_node: clang_interface::Node,
    _parent: clang_interface::Node,
    unit: *mut clang_interface::clang_ASTUnit,
    data: clang_interface::CXClientData,
) -> CXChildVisitResult
where
    Visitor: FnMut(Cursor) -> CXChildVisitResult,
{
    let func: &mut Visitor = mem::transmute(data);
    let node = {
        // CXCursor_CXXBaseSpecifier must come before decls, because it is in
        // the range [FirstRef, LastRef]
        if raw_node.kind == CXCursor_CXXBaseSpecifier {
            ASTNode::CXXBaseSpecifier(raw_node.ptr.base)
        } else if (raw_node.kind >= CXCursor_FirstDecl && raw_node.kind <= CXCursor_LastDecl)
            || (raw_node.kind >= CXCursor_FirstExtraDecl && raw_node.kind <= CXCursor_LastExtraDecl)
            || (raw_node.kind >= CXCursor_FirstRef && raw_node.kind <= CXCursor_LastRef)
        {
            ASTNode::Decl(raw_node.ptr.decl)
        } else if raw_node.kind >= CXCursor_FirstExpr && raw_node.kind <= CXCursor_LastExpr {
            ASTNode::Expr(raw_node.ptr.expr)
        } else if raw_node.kind >= CXCursor_FirstAttr && raw_node.kind <= CXCursor_LastAttr {
            ASTNode::Attr(raw_node.ptr.attr)
        } else if raw_node.kind >= CXCursor_FirstPreprocessing && raw_node.kind <= CXCursor_LastPreprocessing {
            ASTNode::PreprocessedEntity(raw_node.ptr.ppe)
        } else {
            return CXChildVisit_Recurse;
        }
    };
    let child = Cursor {
        node,
        kind: raw_node.kind,
        unit,
    };

    (*func)(child)
}

/// The type of a node in clang's AST.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Type {
    x: clang_interface::clang_QualType,
    unit: *mut clang_interface::clang_ASTUnit,
}

impl PartialEq for clang_interface::clang_QualType {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.ptr, other.ptr)
    }
}

impl Eq for clang_interface::clang_QualType {}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Type({}, kind: {}, cconv: {}, decl: {:?}, canon: {:?})",
            self.spelling(),
            type_to_str(self.kind()),
            self.call_conv(),
            self.declaration(),
            self.declaration().canonical()
        )
    }
}

/// An error about the layout of a struct, class, or type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LayoutError {
    /// Asked for the layout of an invalid type.
    Invalid,
    /// Asked for the layout of an incomplete type.
    Incomplete,
    /// Asked for the layout of a dependent type.
    Dependent,
    /// Asked for the layout of a type that does not have constant size.
    NotConstantSize,
    /// Asked for the layout of a field in a type that does not have such a
    /// field.
    InvalidFieldName,
    /// An unknown layout error.
    Unknown,
}

impl ::std::convert::From<i32> for LayoutError {
    fn from(val: i32) -> Self {
        use self::LayoutError::*;
        use self::clang_interface::CXTypeLayoutError::*;

        match val {
            CXTypeLayoutError_Invalid => Invalid,
            CXTypeLayoutError_Incomplete => Incomplete,
            CXTypeLayoutError_Dependent => Dependent,
            CXTypeLayoutError_NotConstantSize => NotConstantSize,
            CXTypeLayoutError_InvalidFieldName => InvalidFieldName,
            _ => Unknown,
        }
    }
}

impl Type {
    fn context(&self) -> *mut clang_interface::clang_ASTContext {
        unsafe { clang_interface::ASTUnit_getContext(self.unit) }
    }

    /// Get this type's kind.
    pub fn kind(&self) -> CXTypeKind {
        unsafe { clang_interface::Type_kind(self.x, self.context()) }
    }

    /// Get a cursor pointing to this type's declaration.
    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor::new(
                ASTNode::Decl(clang_interface::Type_getDeclaration(self.x)),
                self.unit,
            )
        }
    }

    /// Get the canonical declaration of this type, if it is available.
    pub fn canonical_declaration(
        &self,
        location: Option<&Cursor>,
    ) -> Option<CanonicalTypeDeclaration> {
        let mut declaration = self.declaration();
        if !declaration.is_valid() {
            if let Some(location) = location {
                let mut location = *location;
                if let Some(referenced) = location.referenced() {
                    location = referenced;
                }
                if location.is_template_like() {
                    declaration = location;
                }
            }
        }

        let canonical = declaration.canonical();
        if canonical.is_valid() && canonical.kind() != CXCursor_NoDeclFound {
            Some(CanonicalTypeDeclaration(*self, canonical))
        } else {
            None
        }
    }

    /// Get a raw display name for this type.
    pub fn spelling(&self) -> String {
        let s = unsafe { clang_interface::Type_getTypeSpelling(self.x, self.context()).to_string() };
        // Clang 5.0 introduced changes in the spelling API so it returned the
        // full qualified name. Let's undo that here.
        if s.split("::").all(is_valid_identifier) {
            if let Some(s) = s.split("::").last() {
                return s.to_owned();
            }
        }

        s
    }

    /// Is this type const qualified?
    pub fn is_const(&self) -> bool {
        unsafe { clang_interface::Type_isConstQualifiedType(self.x) }
    }

    #[inline]
    fn is_non_deductible_auto_type(&self) -> bool {
        debug_assert_eq!(self.kind(), CXType_Auto);
        self.canonical_type() == *self
    }

    #[inline]
    fn clang_size_of(&self, ctx: &BindgenContext) -> c_longlong {
        match self.kind() {
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40975
            CXType_RValueReference | CXType_LValueReference => {
                ctx.target_pointer_size() as c_longlong
            }
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40813
            CXType_Auto if self.is_non_deductible_auto_type() => return -6,
            _ => unsafe { clang_interface::Type_getSizeOf(self.x, self.context()) },
        }
    }

    #[inline]
    fn clang_align_of(&self, ctx: &BindgenContext) -> c_longlong {
        match self.kind() {
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40975
            CXType_RValueReference | CXType_LValueReference => {
                ctx.target_pointer_size() as c_longlong
            }
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40813
            CXType_Auto if self.is_non_deductible_auto_type() => return -6,
            _ => unsafe { clang_interface::Type_getAlignOf(self.x, self.context()) },
        }
    }

    /// What is the size of this type? Paper over invalid types by returning `0`
    /// for them.
    pub fn size(&self, ctx: &BindgenContext) -> usize {
        let val = self.clang_size_of(ctx);
        if val < 0 {
            0
        } else {
            val as usize
        }
    }

    /// What is the size of this type?
    pub fn fallible_size(
        &self,
        ctx: &BindgenContext,
    ) -> Result<usize, LayoutError> {
        let val = self.clang_size_of(ctx);
        if val < 0 {
            Err(LayoutError::from(val as i32))
        } else {
            Ok(val as usize)
        }
    }

    /// What is the alignment of this type? Paper over invalid types by
    /// returning `0`.
    pub fn align(&self, ctx: &BindgenContext) -> usize {
        let val = self.clang_align_of(ctx);
        if val < 0 {
            0
        } else {
            val as usize
        }
    }

    /// What is the alignment of this type?
    pub fn fallible_align(
        &self,
        ctx: &BindgenContext,
    ) -> Result<usize, LayoutError> {
        let val = self.clang_align_of(ctx);
        if val < 0 {
            Err(LayoutError::from(val as i32))
        } else {
            Ok(val as usize)
        }
    }

    /// Get the layout for this type, or an error describing why it does not
    /// have a valid layout.
    pub fn fallible_layout(
        &self,
        ctx: &BindgenContext,
    ) -> Result<crate::ir::layout::Layout, LayoutError> {
        use crate::ir::layout::Layout;
        let size = self.fallible_size(ctx)?;
        let align = self.fallible_align(ctx)?;
        Ok(Layout::new(size, align))
    }

    /// Get the number of template arguments this type has, or `None` if it is
    /// not some kind of template.
    pub fn num_template_args(&self) -> Option<u32> {
        // If an old libclang is loaded, we have no hope of answering this
        // question correctly. However, that's no reason to panic when
        // generating bindings for simple C headers with an old libclang.
        // if !clang_interface::Type_getNumTemplateArguments::is_loaded() {
        //     return None;
        // }

        let n = unsafe { clang_interface::Type_getNumTemplateArguments(self.x) };
        if n >= 0 {
            Some(n as u32)
        } else {
            debug_assert_eq!(n, -1);
            None
        }
    }

    /// If this type is a class template specialization, return its
    /// template arguments. Otherwise, return None.
    pub fn template_args(&self) -> Option<TypeTemplateArgIterator> {
        self.num_template_args().map(|n| TypeTemplateArgIterator {
            x: self.x,
            unit: self.unit,
            length: n,
            index: 0,
        })
    }

    /// Given that this type is a function prototype, return the types of its parameters.
    ///
    /// Returns None if the type is not a function prototype.
    pub fn args(&self) -> Option<Vec<Type>> {
        self.num_args().ok().map(|num| {
            (0..num)
                .map(|i| Type {
                    x: unsafe { clang_interface::Type_getArgType(self.x, i as c_uint) },
                    unit: self.unit,
                })
                .collect()
        })
    }

    /// Given that this type is a function prototype, return the number of arguments it takes.
    ///
    /// Returns Err if the type is not a function prototype.
    pub fn num_args(&self) -> Result<u32, ()> {
        unsafe {
            let w = clang_interface::Type_getNumArgTypes(self.x);
            if w == -1 {
                Err(())
            } else {
                Ok(w as u32)
            }
        }
    }

    /// Given that this type is a pointer type, return the type that it points
    /// to.
    pub fn pointee_type(&self) -> Option<Type> {
        match self.kind() {
            CXType_Pointer |
            CXType_RValueReference |
            CXType_LValueReference |
            CXType_MemberPointer |
            CXType_BlockPointer |
            CXType_ObjCObjectPointer => {
                let ret = Type {
                    x: unsafe { clang_interface::Type_getPointeeType(self.x) },
                    unit: self.unit,
                };
                debug_assert!(ret.is_valid());
                Some(ret)
            }
            _ => None,
        }
    }

    /// Given that this type is an array, vector, or complex type, return the
    /// type of its elements.
    pub fn elem_type(&self) -> Option<Type> {
        let current_type = Type {
            x: unsafe { clang_interface::Type_getElementType(self.x) },
            unit: self.unit,
        };
        if current_type.is_valid() {
            Some(current_type)
        } else {
            None
        }
    }

    /// Given that this type is an array or vector type, return its number of
    /// elements.
    pub fn num_elements(&self) -> Option<usize> {
        let num_elements_returned = unsafe { clang_interface::Type_getNumElements(self.x) };
        if num_elements_returned != -1 {
            Some(num_elements_returned as usize)
        } else {
            None
        }
    }

    /// Get the canonical version of this type. This sees through `typedef`s and
    /// aliases to get the underlying, canonical type.
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_interface::Type_getCanonicalType(self.x, self.context()),
                unit: self.unit,
            }
        }
    }

    /// Is this type a variadic function type?
    pub fn is_variadic(&self) -> bool {
        unsafe { clang_interface::Type_isFunctionTypeVariadic(self.x) }
    }

    /// Given that this type is a function type, get the type of its return
    /// value.
    pub fn ret_type(&self) -> Option<Type> {
        let rt = Type {
            x: unsafe { clang_interface::Type_getResultType(self.x) },
            unit: self.unit,
        };
        if rt.is_valid() {
            Some(rt)
        } else {
            None
        }
    }

    /// Given that this type is a function type, get its calling convention. If
    /// this is not a function type, `CXCallingConv_Invalid` is returned.
    pub fn call_conv(&self) -> clang_interface::CXCallingConv::Type {
        unsafe { clang_interface::Type_getFunctionTypeCallingConv(self.x) }
    }

    /// For elaborated types (types which use `class`, `struct`, or `union` to
    /// disambiguate types from local bindings), get the underlying type.
    pub fn named(&self) -> Type {
        unsafe {
            Type {
                x: clang_interface::Type_getNamedType(self.x),
                unit: self.unit,
            }
        }
    }

    /// Is this a valid type?
    pub fn is_valid(&self) -> bool {
        self.kind() != CXType_Invalid
    }

    /// Is this a valid and exposed type?
    pub fn is_valid_and_exposed(&self) -> bool {
        self.is_valid() && self.kind() != CXType_Unexposed
    }

    /// Is this type a fully instantiated template?
    pub fn is_fully_instantiated_template(&self) -> bool {
        // Yep, the spelling of this containing type-parameter is extremely
        // nasty... But can happen in <type_traits>. Unfortunately I couldn't
        // reduce it enough :(
        self.template_args().map_or(false, |args| args.len() > 0) &&
            !matches!(
                self.declaration().kind(),
                CXCursor_ClassTemplatePartialSpecialization |
                    CXCursor_TypeAliasTemplateDecl |
                    CXCursor_TemplateTemplateParameter
            )
    }

    /// Is this type an associated template type? Eg `T::Associated` in
    /// this example:
    ///
    /// ```c++
    /// template <typename T>
    /// class Foo {
    ///     typename T::Associated member;
    /// };
    /// ```
    pub fn is_associated_type(&self) -> bool {
        // This is terrible :(
        fn hacky_parse_associated_type<S: AsRef<str>>(spelling: S) -> bool {
            lazy_static! {
                static ref ASSOC_TYPE_RE: regex::Regex = regex::Regex::new(
                    r"typename type\-parameter\-\d+\-\d+::.+"
                )
                .unwrap();
            }
            ASSOC_TYPE_RE.is_match(spelling.as_ref())
        }

        self.kind() == CXType_Unexposed &&
            (hacky_parse_associated_type(self.spelling()) ||
                hacky_parse_associated_type(
                    self.canonical_type().spelling(),
                ))
    }
}

/// The `CanonicalTypeDeclaration` type exists as proof-by-construction that its
/// cursor is the canonical declaration for its type. If you have a
/// `CanonicalTypeDeclaration` instance, you know for sure that the type and
/// cursor match up in a canonical declaration relationship, and it simply
/// cannot be otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalTypeDeclaration(Type, Cursor);

impl CanonicalTypeDeclaration {
    /// Get the type.
    pub fn ty(&self) -> &Type {
        &self.0
    }

    /// Get the type's canonical declaration cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.1
    }
}

/// An iterator for a type's template arguments.
pub struct TypeTemplateArgIterator {
    x: clang_interface::clang_QualType,
    unit: *mut clang_interface::clang_ASTUnit,
    length: u32,
    index: u32,
}

impl Iterator for TypeTemplateArgIterator {
    type Item = Type;
    fn next(&mut self) -> Option<Type> {
        if self.index < self.length {
            let idx = self.index as c_uint;
            self.index += 1;
            Some(Type {
                x: unsafe { clang_interface::Type_getTemplateArgumentAsType(self.x, idx) },
                unit: self.unit,
            })
        } else {
            None
        }
    }
}

impl ExactSizeIterator for TypeTemplateArgIterator {
    fn len(&self) -> usize {
        assert!(self.index <= self.length);
        (self.length - self.index) as usize
    }
}

/// A `SourceLocation` is a file, line, column, and byte offset location for
/// some source text.
pub struct SourceLocation {
    x: *const clang_interface::clang_SourceLocation,
    unit: *mut clang_interface::clang_ASTUnit,
}

impl SourceLocation {
    /// Get the (file, line, column, byte offset) tuple for this source
    /// location.
    pub fn location(&self) -> (File, usize, usize, usize) {
        unsafe {
            let mut file = mem::zeroed();
            let mut line = 0;
            let mut col = 0;
            let mut off = 0;
            clang_interface::getSpellingLocation(
                self.unit, self.x, &mut file, &mut line, &mut col, &mut off,
            );
            (File { x: file }, line as usize, col as usize, off as usize)
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (file, line, col, _) = self.location();
        if let Some(name) = file.name() {
            write!(f, "{}:{}:{}", name, line, col)
        } else {
            "builtin definitions".fmt(f)
        }
    }
}

impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// A comment in the source text.
///
/// Comments are sort of parsed by Clang, and have a tree structure.
pub struct Comment {
    x: *const clang_interface::clang_comments_Comment,
}

impl Comment {
    /// What kind of comment is this?
    pub fn kind(&self) -> clang_interface::CXCommentKind::Type {
        unsafe { clang_interface::Comment_getKind(self.x) }
    }

    /// Get this comment's children comment
    pub fn get_children(&self) -> CommentChildrenIterator {
        CommentChildrenIterator {
            parent: self.x,
            length: unsafe { clang_interface::Comment_getNumChildren(self.x) },
            index: 0,
        }
    }

    /// Given that this comment is the start or end of an HTML tag, get its tag
    /// name.
    pub fn get_tag_name(&self) -> String {
        unsafe { clang_interface::HTMLTagComment_getTagName(self.x).to_string() }
    }

    /// Given that this comment is an HTML start tag, get its attributes.
    pub fn get_tag_attrs(&self) -> CommentAttributesIterator {
        CommentAttributesIterator {
            x: self.x,
            length: unsafe { clang_interface::HTMLStartTag_getNumAttrs(self.x) },
            index: 0,
        }
    }
}

/// An iterator for a comment's children
pub struct CommentChildrenIterator {
    parent: *const clang_interface::clang_comments_Comment,
    length: c_uint,
    index: c_uint,
}

impl Iterator for CommentChildrenIterator {
    type Item = Comment;
    fn next(&mut self) -> Option<Comment> {
        if self.index < self.length {
            let idx = self.index;
            self.index += 1;
            Some(Comment {
                x: unsafe { clang_interface::Comment_getChild(self.parent, idx) },
            })
        } else {
            None
        }
    }
}

/// An HTML start tag comment attribute
pub struct CommentAttribute {
    /// HTML start tag attribute name
    pub name: String,
    /// HTML start tag attribute value
    pub value: String,
}

/// An iterator for a comment's attributes
pub struct CommentAttributesIterator {
    x: *const clang_interface::clang_comments_Comment,
    length: c_uint,
    index: c_uint,
}

impl Iterator for CommentAttributesIterator {
    type Item = CommentAttribute;
    fn next(&mut self) -> Option<CommentAttribute> {
        if self.index < self.length {
            let idx = self.index;
            self.index += 1;
            Some(CommentAttribute {
                name: unsafe {
                    clang_interface::HTMLStartTag_getAttrName(
                        self.x, idx,
                    ).to_string()
                },
                value: unsafe {
                    clang_interface::HTMLStartTag_getAttrValue(
                        self.x, idx,
                    ).to_string()
                },
            })
        } else {
            None
        }
    }
}

/// A source file.
pub struct File {
    x: *mut clang_interface::clang_FileEntry,
}

impl File {
    /// Get the name of this source file.
    pub fn name(&self) -> Option<String> {
        if self.x.is_null() {
            return None;
        }
        Some(unsafe { clang_interface::FileEntry_getName(self.x).to_string() })
    }
}

// fn cxstring_to_string_leaky(s: CXString) -> String {
//     if s.data.is_null() {
//         return "".to_owned();
//     }
//     let c_str = unsafe { CStr::from_ptr(clang_getCString(s) as *const _) };
//     c_str.to_string_lossy().into_owned()
// }

// fn cxstring_into_string(s: CXString) -> String {
//     let ret = cxstring_to_string_leaky(s);
//     unsafe { clang_disposeString(s) };
//     ret
// }

// /// An `Index` is an environment for a set of translation units that will
// /// typically end up linked together in one final binary.
// pub struct Index {
//     x: CXIndex,
// }

// impl Index {
//     /// Construct a new `Index`.
//     ///
//     /// The `pch` parameter controls whether declarations in pre-compiled
//     /// headers are included when enumerating a translation unit's "locals".
//     ///
//     /// The `diag` parameter controls whether debugging diagnostics are enabled.
//     pub fn new(pch: bool, diag: bool) -> Index {
//         unsafe {
//             Index {
//                 x: clang_createIndex(pch as c_int, diag as c_int),
//             }
//         }
//     }
// }

// impl fmt::Debug for Index {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         write!(fmt, "Index {{ }}")
//     }
// }

// impl Drop for Index {
//     fn drop(&mut self) {
//         unsafe {
//             clang_disposeIndex(self.x);
//         }
//     }
// }

/// A translation unit (or "compilation unit").
pub struct TranslationUnit {
    x: *mut clang_interface::clang_ASTUnit,
}

impl fmt::Debug for TranslationUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "TranslationUnit {{ }}")
    }
}

impl TranslationUnit {
    /// Parse a source file into a translation unit.
    pub fn parse(
        // ix: &Index,
        file: &str,
        cmd_args: &[String],
        unsaved: &[UnsavedFile],
        opts: clang_sys::CXTranslationUnit_Flags,
    ) -> Option<TranslationUnit> {
        let fname = CString::new(file).unwrap();
        let _c_args: Vec<CString> = cmd_args
            .iter()
            .map(|s| CString::new(s.clone()).unwrap())
            .collect();
        let c_args: Vec<*const c_char> =
            _c_args.iter().map(|s| s.as_ptr()).collect();
        let mut c_unsaved: Vec<clang_interface::CXUnsavedFile> =
            unsaved.iter().map(|f| f.x).collect();
        let tu = unsafe {
            clang_interface::parseTranslationUnit(
                fname.as_ptr(),
                c_args.as_ptr(),
                c_args.len() as c_int,
                opts,
                c_unsaved.as_mut_ptr(),
                c_unsaved.len() as c_uint,
            )
        };
        if tu.is_null() {
            None
        } else {
            Some(TranslationUnit { x: tu })
        }
    }

    /// Get the Clang diagnostic information associated with this translation
    /// unit.
    pub fn diags(&self) -> Vec<Diagnostic> {
        unsafe {
            let num = clang_interface::ASTUnit_getNumDiagnostics(self.x) as usize;
            let mut diags = vec![];
            for i in 0..num {
                diags.push(Diagnostic {
                    x: clang_interface::ASTUnit_getDiagnostic(self.x, i as c_uint),
                });
            }
            diags
        }
    }

    /// Get a cursor pointing to the root of this translation unit's AST.
    pub fn cursor(&self) -> Cursor {
        unsafe {
            Cursor::new(
                ASTNode::Decl(clang_interface::getTranslationUnitDecl(self.x)),
                self.x,
            )
        }
    }

    /// Is this the null translation unit?
    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe {
            clang_interface::disposeASTUnit(self.x);
        }
    }
}

/// A diagnostic message generated while parsing a translation unit.
pub struct Diagnostic {
    x: *const clang_interface::clang_StoredDiagnostic,
}

impl Diagnostic {
    /// Format this diagnostic message as a string, using the given option bit
    /// flags.
    pub fn format(&self) -> String {
        unsafe {
            clang_interface::Diagnostic_format(self.x).to_string()
        }
    }

    /// What is the severity of this diagnostic message?
    pub fn severity(&self) -> clang_interface::CXDiagnosticSeverity::Type {
        unsafe { clang_interface::Diagnostic_getSeverity(self.x) }
    }
}

// impl Drop for Diagnostic {
//     /// Destroy this diagnostic message.
//     fn drop(&mut self) {
//         unsafe {
//             clang_interface::Diagnostic_dispose(self.x);
//         }
//     }
// }

/// A file which has not been saved to disk.
pub struct UnsavedFile {
    x: clang_interface::CXUnsavedFile,
    /// The name of the unsaved file. Kept here to avoid leaving dangling pointers in
    /// `CXUnsavedFile`.
    pub name: CString,
    contents: CString,
}

impl UnsavedFile {
    /// Construct a new unsaved file with the given `name` and `contents`.
    pub fn new(name: &str, contents: &str) -> UnsavedFile {
        let name = CString::new(name).unwrap();
        let contents = CString::new(contents).unwrap();
        let x = clang_interface::CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: contents.as_ptr(),
            Length: contents.as_bytes().len() as c_ulong,
        };
        UnsavedFile { x, name, contents }
    }
}

impl fmt::Debug for UnsavedFile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "UnsavedFile(name: {:?}, contents: {:?})",
            self.name, self.contents
        )
    }
}

/// Convert a cursor kind into a static string.
pub fn kind_to_str(x: CXCursorKind) -> String {
    unsafe { clang_interface::CursorKind_getSpelling(x).to_string() }
}

/// Convert a type kind to a static string.
pub fn type_to_str(x: CXTypeKind) -> String {
    unsafe { clang_interface::TypeKind_getSpelling(x).to_string() }
}

/// Dump the Clang AST to stdout for debugging purposes.
pub fn ast_dump(c: &Cursor, depth: isize) -> CXChildVisitResult {
    fn print_indent<S: AsRef<str>>(depth: isize, s: S) {
        for _ in 0..depth {
            print!("    ");
        }
        println!("{}", s.as_ref());
    }

    fn print_cursor<S: AsRef<str>>(depth: isize, prefix: S, c: &Cursor) {
        let prefix = prefix.as_ref();
        print_indent(
            depth,
            format!(" {}kind = {}", prefix, kind_to_str(c.kind())),
        );
        print_indent(
            depth,
            format!(" {}spelling = \"{}\"", prefix, c.spelling()),
        );
        print_indent(depth, format!(" {}location = {}", prefix, c.location()));
        print_indent(
            depth,
            format!(" {}is-definition? {}", prefix, c.is_definition()),
        );
        print_indent(
            depth,
            format!(" {}is-declaration? {}", prefix, c.is_declaration()),
        );
        print_indent(
            depth,
            format!(
                " {}is-inlined-function? {}",
                prefix,
                c.is_inlined_function()
            ),
        );

        let templ_kind = c.template_kind();
        if templ_kind != CXCursor_NoDeclFound {
            print_indent(
                depth,
                format!(
                    " {}template-kind = {}",
                    prefix,
                    kind_to_str(templ_kind)
                ),
            );
        }
        if let Some(usr) = c.usr() {
            print_indent(depth, format!(" {}usr = \"{}\"", prefix, usr));
        }
        if let Ok(num) = c.num_args() {
            print_indent(depth, format!(" {}number-of-args = {}", prefix, num));
        }
        if let Some(num) = c.num_template_args() {
            print_indent(
                depth,
                format!(" {}number-of-template-args = {}", prefix, num),
            );
        }
        if let Some(width) = c.bit_width() {
            print_indent(depth, format!(" {}bit-width = {}", prefix, width));
        }
        if let Some(ty) = c.enum_type() {
            print_indent(
                depth,
                format!(" {}enum-type = {}", prefix, type_to_str(ty.kind())),
            );
        }
        if let Some(val) = c.enum_val_signed() {
            print_indent(depth, format!(" {}enum-val = {}", prefix, val));
        }
        if let Some(ty) = c.typedef_type() {
            print_indent(
                depth,
                format!(" {}typedef-type = {}", prefix, type_to_str(ty.kind())),
            );
        }
        if let Some(ty) = c.ret_type() {
            print_indent(
                depth,
                format!(" {}ret-type = {}", prefix, type_to_str(ty.kind())),
            );
        }

        if let Some(refd) = c.referenced() {
            if refd != *c {
                println!();
                print_cursor(
                    depth,
                    String::from(prefix) + "referenced.",
                    &refd,
                );
            }
        }

        let canonical = c.canonical();
        if canonical != *c {
            println!();
            print_cursor(
                depth,
                String::from(prefix) + "canonical.",
                &canonical,
            );
        }

        if let Some(specialized) = c.specialized() {
            if specialized != *c {
                println!();
                print_cursor(
                    depth,
                    String::from(prefix) + "specialized.",
                    &specialized,
                );
            }
        }

        if let Some(parent) = c.fallible_semantic_parent() {
            println!();
            print_cursor(
                depth,
                String::from(prefix) + "semantic-parent.",
                &parent,
            );
        }
    }

    fn print_type<S: AsRef<str>>(depth: isize, prefix: S, ty: &Type) {
        let prefix = prefix.as_ref();

        let kind = ty.kind();
        print_indent(depth, format!(" {}kind = {}", prefix, type_to_str(kind)));
        if kind == CXType_Invalid {
            return;
        }

        print_indent(depth, format!(" {}cconv = {}", prefix, ty.call_conv()));

        print_indent(
            depth,
            format!(" {}spelling = \"{}\"", prefix, ty.spelling()),
        );
        let num_template_args =
            unsafe { clang_interface::Type_getNumTemplateArguments(ty.x) };
        if num_template_args >= 0 {
            print_indent(
                depth,
                format!(
                    " {}number-of-template-args = {}",
                    prefix, num_template_args
                ),
            );
        }
        if let Some(num) = ty.num_elements() {
            print_indent(
                depth,
                format!(" {}number-of-elements = {}", prefix, num),
            );
        }
        print_indent(
            depth,
            format!(" {}is-variadic? {}", prefix, ty.is_variadic()),
        );

        let canonical = ty.canonical_type();
        if canonical != *ty {
            println!();
            print_type(depth, String::from(prefix) + "canonical.", &canonical);
        }

        if let Some(pointee) = ty.pointee_type() {
            if pointee != *ty {
                println!();
                print_type(depth, String::from(prefix) + "pointee.", &pointee);
            }
        }

        if let Some(elem) = ty.elem_type() {
            if elem != *ty {
                println!();
                print_type(depth, String::from(prefix) + "elements.", &elem);
            }
        }

        if let Some(ret) = ty.ret_type() {
            if ret != *ty {
                println!();
                print_type(depth, String::from(prefix) + "return.", &ret);
            }
        }

        let named = ty.named();
        if named != *ty && named.is_valid() {
            println!();
            print_type(depth, String::from(prefix) + "named.", &named);
        }
    }

    print_indent(depth, "(");
    print_cursor(depth, "", c);

    println!();
    let ty = c.cur_type();
    print_type(depth, "type.", &ty);

    let declaration = ty.declaration();
    if declaration != *c && declaration.kind() != CXCursor_NoDeclFound {
        println!();
        print_cursor(depth, "type.declaration.", &declaration);
    }

    // Recurse.
    let mut found_children = false;
    c.visit(|s| {
        if !found_children {
            println!();
            found_children = true;
        }
        ast_dump(&s, depth + 1)
    });

    print_indent(depth, ")");

    CXChildVisit_Continue
}

/// Try to extract the clang version to a string
pub fn extract_clang_version() -> String {
    let version = unsafe { clang_interface::getClangVersion() };
    version.to_string()
}

/// A wrapper for the result of evaluating an expression.
#[derive(Debug)]
pub struct EvalResult {
    x: *mut clang_interface::EvalResult,
}

impl EvalResult {
    fn kind(&self) -> CXEvalResultKind {
        unsafe { clang_interface::EvalResult_getKind(self.x) }
    }

    /// Try to get back the result as a double.
    pub fn as_double(&self) -> Option<f64> {
        match self.kind() {
            CXEval_Float => {
                Some(unsafe { clang_interface::EvalResult_getAsDouble(self.x) } as f64)
            }
            _ => None,
        }
    }

    /// Try to get back the result as an integer.
    pub fn as_int(&self) -> Option<i64> {
        if self.kind() != CXEval_Int {
            return None;
        }

        if unsafe { clang_interface::EvalResult_isUnsignedInt(self.x) } {
            let value = unsafe { clang_interface::EvalResult_getAsUnsigned(self.x) };
            if value > i64::max_value() as c_ulonglong {
                return None;
            }

            return Some(value as i64);
        }

        let value = unsafe { clang_interface::EvalResult_getAsLongLong(self.x) };
        if value > i64::max_value() as c_longlong {
            return None;
        }
        if value < i64::min_value() as c_longlong {
            return None;
        }
        Some(value as i64)
    }

    /// Evaluates the expression as a literal string, that may or may not be
    /// valid utf-8.
    pub fn as_literal_string(&self) -> Option<Vec<u8>> {
        match self.kind() {
            CXEval_StrLiteral => {
                let ret = unsafe {
                    CStr::from_ptr(clang_interface::cString(clang_interface::EvalResult_getAsStr(self.x)))
                };
                Some(ret.to_bytes().to_vec())
            }
            _ => None,
        }
    }
}

// impl Drop for EvalResult {
//     fn drop(&mut self) {
//         unsafe { clang_interface::EvalResult_dispose(self.x) };
//     }
// }

/// Target information obtained from libclang.
#[derive(Debug)]
pub struct TargetInfo {
    /// The target triple.
    pub triple: String,
    /// The width of the pointer _in bits_.
    pub pointer_width: usize,
}

impl TargetInfo {
    /// Tries to obtain target information from libclang.
    pub fn new(tu: &TranslationUnit) -> Self {
        let triple;
        let pointer_width;
        unsafe {
            let ti = clang_interface::ASTUnit_getTargetInfo(tu.x);
            triple = clang_interface::TargetInfo_getTriple(ti).to_string();
            pointer_width = clang_interface::TargetInfo_getPointerWidth(ti);
        }
        assert!(pointer_width > 0);
        assert_eq!(pointer_width % 8, 0);
        TargetInfo {
            triple,
            pointer_width: pointer_width as usize,
        }
    }
}
