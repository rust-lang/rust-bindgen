//! A higher level Clang API built on top of the generated bindings in the
//! `clangll` module.

#![allow(non_upper_case_globals, dead_code)]


use clangll::*;
use std::{mem, ptr};
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

/// A cursor into the Clang AST, pointing to an AST node.
///
/// We call the AST node pointed to by the cursor the cursor's "referent".
#[derive(Copy, Clone)]
pub struct Cursor {
    x: CXCursor,
}

impl fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt,
               "Cursor({} kind: {}, loc: {}, usr: {:?})",
               self.spelling(),
               kind_to_str(self.kind()),
               self.location(),
               self.usr())
    }
}

impl Cursor {
    /// Get the Unified Symbol Resolution for this cursor's referent, if
    /// available.
    ///
    /// The USR can be used to compare entities across translation units.
    pub fn usr(&self) -> Option<String> {
        let s: String = unsafe { clang_getCursorUSR(self.x) }.into();
        if s.is_empty() { None } else { Some(s) }
    }

    /// Is this cursor's referent a declaration?
    pub fn is_declaration(&self) -> bool {
        unsafe { clang_isDeclaration(self.kind()) != 0 }
    }

    /// Get the null cursor, which has no referent.
    pub fn null() -> Self {
        Cursor {
            x: unsafe { clang_getNullCursor() },
        }
    }

    /// Get this cursor's referent's spelling.
    pub fn spelling(&self) -> String {
        unsafe { clang_getCursorSpelling(self.x).into() }
    }

    /// Get this cursor's referent's display name.
    ///
    /// This is not necessarily a valid identifier. It includes extra
    /// information, such as parameters for a function, etc.
    pub fn display_name(&self) -> String {
        unsafe { clang_getCursorDisplayName(self.x).into() }
    }

    /// Get the mangled name of this cursor's referent.
    pub fn mangling(&self) -> String {
        unsafe { clang_Cursor_getMangling(self.x).into() }
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
        unsafe {
            Cursor {
                x: clang_getCursorLexicalParent(self.x),
            }
        }
    }

    /// Get the referent's semantic parent, if one is available.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub fn fallible_semantic_parent(&self) -> Option<Cursor> {
        let sp = unsafe {
            Cursor {
                x: clang_getCursorSemanticParent(self.x),
            }
        };
        if sp == *self || !sp.is_valid() {
            return None;
        }
        Some(sp)
    }

    /// Get the referent's semantic parent.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub fn semantic_parent(&self) -> Cursor {
        self.fallible_semantic_parent().unwrap()
    }

    /// Return the number of template arguments used by this cursor's referent,
    /// if the referent is either a template specialization or
    /// declaration. Returns -1 otherwise.
    ///
    /// NOTE: This may not return `Some` for some non-fully specialized
    /// templates, see #193 and #194.
    pub fn num_template_args(&self) -> Option<u32> {
        let n: c_int = unsafe { clang_Cursor_getNumTemplateArguments(self.x) };

        if n >= 0 {
            Some(n as u32)
        } else {
            debug_assert_eq!(n, -1);
            None
        }
    }

    /// Get a cursor pointing to this referent's containing translation unit.
    ///
    /// Note that we shouldn't create a `TranslationUnit` struct here, because
    /// bindgen assumes there will only be one of them alive at a time, and
    /// disposes it on drop. That can change if this would be required, but I
    /// think we can survive fine without it.
    pub fn translation_unit(&self) -> Cursor {
        assert!(self.is_valid());
        unsafe {
            let tu = clang_Cursor_getTranslationUnit(self.x);
            let cursor = Cursor {
                x: clang_getTranslationUnitCursor(tu),
            };
            assert!(cursor.is_valid());
            cursor
        }
    }

    /// Is the referent a top level construct?
    pub fn is_toplevel(&self) -> bool {
        let mut semantic_parent = self.fallible_semantic_parent();

        while semantic_parent.is_some() &&
              (semantic_parent.unwrap().kind() == CXCursor_Namespace ||
               semantic_parent.unwrap().kind() == CXCursor_NamespaceAlias ||
               semantic_parent.unwrap().kind() == CXCursor_NamespaceRef) {
            semantic_parent = semantic_parent.unwrap()
                .fallible_semantic_parent();
        }

        let tu = self.translation_unit();
        // Yes, this can happen with, e.g., macro definitions.
        semantic_parent == tu.fallible_semantic_parent()
    }

    /// Get the kind of referent this cursor is pointing to.
    pub fn kind(&self) -> Enum_CXCursorKind {
        unsafe { clang_getCursorKind(self.x) }
    }

    /// Is the referent an anonymous record definition?
    pub fn is_anonymous(&self) -> bool {
        unsafe { clang_Cursor_isAnonymous(self.x) != 0 }
    }

    /// Is the referent a template specialization?
    pub fn is_template(&self) -> bool {
        self.specialized().is_some()
    }

    /// Is the referent a fully specialized template specialization without any
    /// remaining free template arguments?
    pub fn is_fully_specialized_template(&self) -> bool {
        self.is_template() && self.num_template_args().unwrap_or(0) > 0
    }

    /// Is the referent a template specialization that still has remaining free
    /// template arguments?
    pub fn is_in_non_fully_specialized_template(&self) -> bool {
        if self.is_toplevel() {
            return false;
        }
        let parent = self.semantic_parent();
        (parent.is_template() && !parent.is_fully_specialized_template()) ||
        parent.is_in_non_fully_specialized_template()
    }

    /// Is this cursor pointing a valid referent?
    pub fn is_valid(&self) -> bool {
        unsafe { clang_isInvalid(self.kind()) == 0 }
    }

    /// Get the source location for the referent.
    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation {
                x: clang_getCursorLocation(self.x),
            }
        }
    }

    /// Get the source location range for the referent.
    pub fn extent(&self) -> CXSourceRange {
        unsafe { clang_getCursorExtent(self.x) }
    }

    /// Get the raw declaration comment for this referent, if one exists.
    pub fn raw_comment(&self) -> Option<String> {
        let s: String =
            unsafe { clang_Cursor_getRawCommentText(self.x).into() };
        if s.is_empty() { None } else { Some(s) }
    }

    /// Get the referent's parsed comment.
    pub fn comment(&self) -> Comment {
        unsafe {
            Comment {
                x: clang_Cursor_getParsedComment(self.x),
            }
        }
    }

    /// Get the referent's type.
    pub fn cur_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_getCursorType(self.x),
            }
        }
    }

    /// Given that this cursor's referent is a reference to another type, or is
    /// a declaration, get the cursor pointing to the referenced type or type of
    /// the declared thing.
    pub fn definition(&self) -> Option<Cursor> {
        unsafe {
            let ret = Cursor {
                x: clang_getCursorDefinition(self.x),
            };

            if ret.is_valid() { Some(ret) } else { None }
        }
    }

    /// Given that this cursor's referent is reference type, get the cursor
    /// pointing to the referenced type.
    pub fn referenced(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getCursorReferenced(self.x),
            }
        }
    }

    /// Get the canonical cursor for this referent.
    ///
    /// Many types can be declared multiple times before finally being properly
    /// defined. This method allows us to get the canonical cursor for the
    /// referent type.
    pub fn canonical(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getCanonicalCursor(self.x),
            }
        }
    }

    /// Given that this cursor points to a template specialization, get a cursor
    /// pointing to the template definition that is being specialized.
    pub fn specialized(&self) -> Option<Cursor> {
        unsafe {
            let ret = Cursor {
                x: clang_getSpecializedCursorTemplate(self.x),
            };
            if ret.is_valid() { Some(ret) } else { None }
        }
    }

    /// Assuming that this cursor's referent is a template declaration, get the
    /// kind of cursor that would be generated for its specializations.
    pub fn template_kind(&self) -> Enum_CXCursorKind {
        unsafe { clang_getTemplateCursorKind(self.x) }
    }

    /// Traverse this cursor's referent and its children.
    ///
    /// Call the given function on each AST node traversed.
    pub fn visit<Visitor>(&self, mut visitor: Visitor)
        where Visitor: FnMut(Cursor) -> Enum_CXChildVisitResult,
    {
        unsafe {
            clang_visitChildren(self.x,
                                Some(visit_children::<Visitor>),
                                mem::transmute(&mut visitor));
        }
    }

    /// Is the referent an inlined function?
    #[cfg(not(feature="llvm_stable"))]
    pub fn is_inlined_function(&self) -> bool {
        unsafe { clang_Cursor_isFunctionInlined(self.x) != 0 }
    }

    // TODO: Remove this when LLVM 3.9 is released.
    //
    // This is currently used for CI purposes.

    /// Is the referent an inlined function?
    #[cfg(feature="llvm_stable")]
    pub fn is_inlined_function(&self) -> bool {
        false
    }

    /// Get the width of this cursor's referent bit field, or `None` if the
    /// referent is not a bit field.
    pub fn bit_width(&self) -> Option<u32> {
        unsafe {
            let w = clang_getFieldDeclBitWidth(self.x);
            if w == -1 { None } else { Some(w as u32) }
        }
    }

    /// Get the integer representation type used to hold this cursor's referent
    /// enum type.
    pub fn enum_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_getEnumDeclIntegerType(self.x),
            }
        }
    }

    /// Get the signed constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub fn enum_val_signed(&self) -> Option<i64> {
        unsafe {
            if self.kind() == CXCursor_EnumConstantDecl {
                Some(clang_getEnumConstantDeclValue(self.x) as i64)
            } else {
                None
            }
        }
    }

    /// Get the unsigned constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub fn enum_val_unsigned(&self) -> Option<u64> {
        unsafe {
            if self.kind() == CXCursor_EnumConstantDecl {
                Some(clang_getEnumConstantDeclUnsignedValue(self.x) as u64)
            } else {
                None
            }
        }
    }

    /// Given that this cursor's referent is a `typedef`, get the `Type` that is
    /// being aliased.
    pub fn typedef_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_getTypedefDeclUnderlyingType(self.x),
            }
        }
    }

    /// Get the linkage kind for this cursor's referent.
    ///
    /// This only applies to functions and variables.
    pub fn linkage(&self) -> Enum_CXLinkageKind {
        unsafe { clang_getCursorLinkage(self.x) }
    }

    /// Get the visibility of this cursor's referent.
    pub fn visibility(&self) -> Enum_CXVisibilityKind {
        unsafe { clang_getCursorVisibility(self.x) }
    }

    /// Given that this cursor's referent is a function, return cursors to its
    /// parameters.
    pub fn args(&self) -> Vec<Cursor> {
        unsafe {
            let num = self.num_args().expect("expected value, got none") as u32;
            let mut args = vec![];
            for i in 0..num {
                args.push(Cursor {
                    x: clang_Cursor_getArgument(self.x, i as c_uint),
                });
            }
            args
        }
    }

    /// Given that this cursor's referent is a function/method call or
    /// declaration, return the number of arguments it takes.
    ///
    /// Returns -1 if the cursor's referent is not a function/method call or
    /// declaration.
    pub fn num_args(&self) -> Result<u32, ()> {
        unsafe {
            let w = clang_Cursor_getNumArguments(self.x);
            if w == -1 { Err(()) } else { Ok(w as u32) }
        }
    }

    /// Get the access specifier for this cursor's referent.
    pub fn access_specifier(&self) -> Enum_CX_CXXAccessSpecifier {
        unsafe { clang_getCXXAccessSpecifier(self.x) }
    }

    /// Is this cursor's referent a field declaration that is marked as
    /// `mutable`?
    pub fn is_mutable_field(&self) -> bool {
        unsafe { clang_CXXField_isMutable(self.x) != 0 }
    }

    /// Is this cursor's referent a member function that is declared `static`?
    pub fn method_is_static(&self) -> bool {
        unsafe { clang_CXXMethod_isStatic(self.x) != 0 }
    }

    /// Is this cursor's referent a member function that is declared `const`?
    pub fn method_is_const(&self) -> bool {
        unsafe { clang_CXXMethod_isConst(self.x) != 0 }
    }

    /// Is this cursor's referent a member function that is declared `const`?
    pub fn method_is_virtual(&self) -> bool {
        unsafe { clang_CXXMethod_isVirtual(self.x) != 0 }
    }

    /// Is this cursor's referent a struct or class with virtual members?
    pub fn is_virtual_base(&self) -> bool {
        unsafe { clang_isVirtualBase(self.x) != 0 }
    }
}

extern "C" fn visit_children<Visitor>(cur: CXCursor,
                                      _parent: CXCursor,
                                      data: CXClientData)
                                      -> Enum_CXChildVisitResult
    where Visitor: FnMut(Cursor) -> Enum_CXChildVisitResult,
{
    let func: &mut Visitor = unsafe { mem::transmute(data) };
    let child = Cursor {
        x: cur,
    };

    (*func)(child)
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Cursor) -> bool {
        unsafe { clang_equalCursors(self.x, other.x) == 1 }
    }
}

impl Eq for Cursor {}

impl Hash for Cursor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { clang_hashCursor(self.x) }.hash(state)
    }
}

/// The type of a node in clang's AST.
#[derive(Clone, Hash)]
pub struct Type {
    x: CXType,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        unsafe { clang_equalTypes(self.x, other.x) != 0 }
    }
}

impl Eq for Type {}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt,
               "Type({}, kind: {}, decl: {:?}, canon: {:?})",
               self.spelling(),
               type_to_str(self.kind()),
               self.declaration(),
               self.declaration().canonical())
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
    /// Get this type's kind.
    pub fn kind(&self) -> Enum_CXTypeKind {
        self.x.kind
    }

    /// Get a cursor pointing to this type's declaration.
    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getTypeDeclaration(self.x),
            }
        }
    }

    /// Get a raw display name for this type.
    pub fn spelling(&self) -> String {
        unsafe { clang_getTypeSpelling(self.x).into() }
    }

    /// Is this type const qualified?
    pub fn is_const(&self) -> bool {
        unsafe { clang_isConstQualifiedType(self.x) != 0 }
    }

    /// What is the size of this type? Paper over invalid types by returning `0`
    /// for them.
    pub fn size(&self) -> usize {
        unsafe {
            let val = clang_Type_getSizeOf(self.x);
            if val < 0 { 0 } else { val as usize }
        }
    }

    /// What is the size of this type?
    pub fn fallible_size(&self) -> Result<usize, LayoutError> {
        let val = unsafe { clang_Type_getSizeOf(self.x) };
        if val < 0 {
            Err(LayoutError::from(val as i32))
        } else {
            Ok(val as usize)
        }
    }

    /// What is the alignment of this type? Paper over invalid types by
    /// returning `0`.
    pub fn align(&self) -> usize {
        unsafe {
            let val = clang_Type_getAlignOf(self.x);
            if val < 0 { 0 } else { val as usize }
        }
    }

    /// What is the alignment of this type?
    pub fn fallible_align(&self) -> Result<usize, LayoutError> {
        unsafe {
            let val = clang_Type_getAlignOf(self.x);
            if val < 0 {
                Err(LayoutError::from(val as i32))
            } else {
                Ok(val as usize)
            }
        }
    }

    /// Get the layout for this type, or an error describing why it does not
    /// have a valid layout.
    pub fn fallible_layout(&self) -> Result<::ir::layout::Layout, LayoutError> {
        use ir::layout::Layout;
        let size = try!(self.fallible_size());
        let align = try!(self.fallible_align());
        Ok(Layout::new(size, align))
    }

    /// If this type is a class template specialization, return its
    /// template arguments. Otherwise, return None.
    pub fn template_args(&self) -> Option<TypeTemplateArgIterator> {
        let n = unsafe { clang_Type_getNumTemplateArguments(self.x) };
        if n >= 0 {
            Some(TypeTemplateArgIterator {
                x: self.x,
                length: n as u32,
                index: 0,
            })
        } else {
            debug_assert_eq!(n, -1);
            None
        }
    }

    /// Given that this type is a pointer type, return the type that it points
    /// to.
    pub fn pointee_type(&self) -> Option<Type> {
        match self.kind() {
            CXType_Pointer |
            CXType_RValueReference |
            CXType_LValueReference |
            CXType_MemberPointer => {
                let ret = Type {
                    x: unsafe { clang_getPointeeType(self.x) },
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
            x: unsafe { clang_getElementType(self.x) },
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
        let num_elements_returned = unsafe { clang_getNumElements(self.x) };
        if num_elements_returned != -1 {
            Some(num_elements_returned as usize)
        } else {
            None
        }
    }

    /// Get the canonical version of this type. This sees through `typdef`s and
    /// aliases to get the underlying, canonical type.
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_getCanonicalType(self.x),
            }
        }
    }

    /// Is this type a variadic function type?
    pub fn is_variadic(&self) -> bool {
        unsafe { clang_isFunctionTypeVariadic(self.x) != 0 }
    }

    /// Given that this type is a function type, get the type of its return
    /// value.
    pub fn ret_type(&self) -> Option<Type> {
        let rt = Type {
            x: unsafe { clang_getResultType(self.x) },
        };
        if rt.is_valid() { Some(rt) } else { None }
    }

    /// Given that this type is a function type, get its calling convention. If
    /// this is not a function type, `CXCallingConv_Invalid` is returned.
    pub fn call_conv(&self) -> Enum_CXCallingConv {
        unsafe { clang_getFunctionTypeCallingConv(self.x) }
    }

    /// For elaborated types (types which use `class`, `struct`, or `union` to
    /// disambiguate types from local bindings), get the underlying type.
    #[cfg(not(feature="llvm_stable"))]
    pub fn named(&self) -> Type {
        unsafe {
            Type {
                x: clang_Type_getNamedType(self.x),
            }
        }
    }

    /// Is this a valid type?
    pub fn is_valid(&self) -> bool {
        self.kind() != CXType_Invalid
    }
}

/// An iterator for a type's template arguments.
pub struct TypeTemplateArgIterator {
    x: CXType,
    length: u32,
    index: u32,
}

impl Iterator for TypeTemplateArgIterator {
    type Item = Type;
    fn next(&mut self) -> Option<Type> {
        if self.index < self.length {
            let idx = self.index as c_int;
            self.index += 1;
            Some(Type {
                x: unsafe { clang_Type_getTemplateArgumentAsType(self.x, idx) },
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
    x: CXSourceLocation,
}

impl SourceLocation {
    /// Get the (file, line, column, byte offset) tuple for this source
    /// location.
    pub fn location(&self) -> (File, usize, usize, usize) {
        unsafe {
            let mut file = ptr::null_mut();
            let mut line = 0;
            let mut col = 0;
            let mut off = 0;
            clang_getSpellingLocation(self.x,
                                      &mut file,
                                      &mut line,
                                      &mut col,
                                      &mut off);
            (File {
                x: file,
            },
             line as usize,
             col as usize,
             off as usize)
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

/// A comment in the source text.
///
/// Comments are sort of parsed by Clang, and have a tree structure.
pub struct Comment {
    x: CXComment,
}

impl Comment {
    /// What kind of comment is this?
    pub fn kind(&self) -> Enum_CXCommentKind {
        unsafe { clang_Comment_getKind(self.x) }
    }

    /// Get this comment's children comment
    pub fn get_children(&self) -> CommentChildrenIterator {
        CommentChildrenIterator {
            parent: self.x,
            length: unsafe { clang_Comment_getNumChildren(self.x) },
            index: 0,
        }
    }

    /// Given that this comment is the start or end of an HTML tag, get its tag
    /// name.
    pub fn get_tag_name(&self) -> String {
        unsafe { clang_HTMLTagComment_getTagName(self.x).into() }
    }

    /// Given that this comment is an HTML start tag, get its attributes.
    pub fn get_tag_attrs(&self) -> CommentAttributesIterator {
        CommentAttributesIterator {
            x: self.x,
            length: unsafe { clang_HTMLStartTag_getNumAttrs(self.x) },
            index: 0,
        }
    }
}

/// An iterator for a comment's children
pub struct CommentChildrenIterator {
    parent: CXComment,
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
                x: unsafe { clang_Comment_getChild(self.parent, idx) },
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
    x: CXComment,
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
                    clang_HTMLStartTag_getAttrName(self.x, idx).into()
                },
                value: unsafe {
                    clang_HTMLStartTag_getAttrValue(self.x, idx).into()
                },
            })
        } else {
            None
        }
    }
}

/// A source file.
pub struct File {
    x: CXFile,
}

impl File {
    /// Get the name of this source file.
    pub fn name(&self) -> Option<String> {
        if self.x.is_null() {
            return None;
        }
        unsafe { Some(clang_getFileName(self.x).into()) }
    }
}

impl Into<String> for CXString {
    fn into(self) -> String {
        if self.data.is_null() {
            return "".to_owned();
        }
        unsafe {
            let c_str = CStr::from_ptr(clang_getCString(self) as *const _);
            c_str.to_string_lossy().into_owned()
        }
    }
}

/// An `Index` is an environment for a set of translation units that will
/// typically end up linked together in one final binary.
pub struct Index {
    x: CXIndex,
}

impl Index {
    /// Construct a new `Index`.
    ///
    /// The `pch` parameter controls whether declarations in pre-compiled
    /// headers are included when enumerating a translation unit's "locals".
    ///
    /// The `diag` parameter controls whether debugging diagnostics are enabled.
    pub fn new(pch: bool, diag: bool) -> Index {
        unsafe {
            Index {
                x: clang_createIndex(pch as c_int, diag as c_int),
            }
        }
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Index {{ }}")
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            clang_disposeIndex(self.x);
        }
    }
}

/// A token emitted by clang's lexer.
#[derive(Debug)]
pub struct Token {
    /// The kind of token this is.
    pub kind: CXTokenKind,
    /// A display name for this token.
    pub spelling: String,
}

/// A translation unit (or "compilation unit").
pub struct TranslationUnit {
    x: CXTranslationUnit,
}

impl fmt::Debug for TranslationUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "TranslationUnit {{ }}")
    }
}

impl TranslationUnit {
    /// Parse a source file into a translation unit.
    pub fn parse(ix: &Index,
                 file: &str,
                 cmd_args: &[String],
                 unsaved: &[UnsavedFile],
                 opts: ::libc::c_uint)
                 -> Option<TranslationUnit> {
        let fname = CString::new(file).unwrap();
        let _c_args: Vec<CString> =
            cmd_args.iter().map(|s| CString::new(s.clone()).unwrap()).collect();
        let c_args: Vec<*const c_char> =
            _c_args.iter().map(|s| s.as_ptr()).collect();
        let mut c_unsaved: Vec<Struct_CXUnsavedFile> =
            unsaved.iter().map(|f| f.x).collect();
        let tu = unsafe {
            clang_parseTranslationUnit(ix.x,
                                       fname.as_ptr(),
                                       c_args.as_ptr(),
                                       c_args.len() as c_int,
                                       c_unsaved.as_mut_ptr(),
                                       c_unsaved.len() as c_uint,
                                       opts)
        };
        if tu.is_null() {
            None
        } else {
            Some(TranslationUnit {
                x: tu,
            })
        }
    }

    /// Reparse this translation unit, maybe because the file changed on disk or
    /// something like that.
    pub fn reparse(&self, unsaved: &[UnsavedFile], opts: usize) -> bool {
        let mut c_unsaved: Vec<Struct_CXUnsavedFile> =
            unsaved.iter().map(|f| f.x).collect();

        unsafe {
            clang_reparseTranslationUnit(self.x,
                                         c_unsaved.len() as c_uint,
                                         c_unsaved.as_mut_ptr(),
                                         opts as c_uint) == 0
        }
    }

    /// Get the Clang diagnostic information associated with this translation
    /// unit.
    pub fn diags(&self) -> Vec<Diagnostic> {
        unsafe {
            let num = clang_getNumDiagnostics(self.x) as usize;
            let mut diags = vec![];
            for i in 0..num {
                diags.push(Diagnostic {
                    x: clang_getDiagnostic(self.x, i as c_uint),
                });
            }
            diags
        }
    }

    /// Get a cursor pointing to the root of this translation unit's AST.
    pub fn cursor(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getTranslationUnitCursor(self.x),
            }
        }
    }

    /// Is this the null translation unit?
    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }

    /// Invoke Clang's lexer on this translation unit and get the stream of
    /// tokens that come out.
    pub fn tokens(&self, cursor: &Cursor) -> Option<Vec<Token>> {
        let range = cursor.extent();
        let mut tokens = vec![];
        unsafe {
            let mut token_ptr = ::std::ptr::null_mut();
            let mut num_tokens: c_uint = 0;
            clang_tokenize(self.x, range, &mut token_ptr, &mut num_tokens);
            if token_ptr.is_null() {
                return None;
            }
            let token_array = ::std::slice::from_raw_parts(token_ptr,
                                                           num_tokens as usize);
            for &token in token_array.iter() {
                let kind = clang_getTokenKind(token);
                let spelling: String = clang_getTokenSpelling(self.x, token)
                    .into();

                tokens.push(Token {
                    kind: kind,
                    spelling: spelling,
                });
            }
            clang_disposeTokens(self.x, token_ptr, num_tokens);
        }
        Some(tokens)
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTranslationUnit(self.x);
        }
    }
}


/// A diagnostic message generated while parsing a translation unit.
pub struct Diagnostic {
    x: CXDiagnostic,
}

impl Diagnostic {
    /// Get the default diagnostic display option bit flags.
    pub fn default_opts() -> usize {
        unsafe { clang_defaultDiagnosticDisplayOptions() as usize }
    }

    /// Format this diagnostic message as a string, using the given option bit
    /// flags.
    pub fn format(&self, opts: usize) -> String {
        unsafe { clang_formatDiagnostic(self.x, opts as c_uint).into() }
    }

    /// What is the severity of this diagnostic message?
    pub fn severity(&self) -> Enum_CXDiagnosticSeverity {
        unsafe { clang_getDiagnosticSeverity(self.x) }
    }
}

impl Drop for Diagnostic {
    /// Destroy this diagnostic message.
    fn drop(&mut self) {
        unsafe {
            clang_disposeDiagnostic(self.x);
        }
    }
}

/// A file which has not been saved to disk.
pub struct UnsavedFile {
    x: Struct_CXUnsavedFile,
    name: CString,
    contents: CString,
}

impl UnsavedFile {
    /// Construct a new unsaved file with the given `name` and `contents`.
    pub fn new(name: &str, contents: &str) -> UnsavedFile {
        let name = CString::new(name).unwrap();
        let contents = CString::new(contents).unwrap();
        let x = Struct_CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: contents.as_ptr(),
            Length: contents.as_bytes().len() as c_ulong,
        };
        UnsavedFile {
            x: x,
            name: name,
            contents: contents,
        }
    }
}

/// Convert a cursor kind into a static string.
pub fn kind_to_str(x: Enum_CXCursorKind) -> String {
    unsafe { clang_getCursorKindSpelling(x) }.into()
}

/// Convert a type kind to a static string.
pub fn type_to_str(x: Enum_CXTypeKind) -> String {
    unsafe { clang_getTypeKindSpelling(x).into() }
}

/// Dump the Clang AST to stdout for debugging purposes.
pub fn ast_dump(c: &Cursor, depth: isize) -> Enum_CXVisitorResult {
    fn print_indent(depth: isize, s: &str) {
        let mut i = 0;
        while i < depth {
            print!("\t");
            i += 1;
        }
        println!("{}", s);
    }
    let ct = c.cur_type().kind();
    print_indent(depth,
                 &format!("({} {} {}",
                          kind_to_str(c.kind()),
                          c.spelling(),
                          type_to_str(ct)));
    c.visit(|s| ast_dump(&s, depth + 1));
    print_indent(depth, ")");
    CXChildVisit_Continue
}

/// Try to extract the clang version to a string
pub fn extract_clang_version() -> String {
    unsafe { clang_getClangVersion().into() }
}
