//! A higher level Clang API built on top of the generated bindings in the
//! `clangll` module.

#![allow(non_upper_case_globals, dead_code)]

use std::os::raw::{c_uint, c_char, c_int, c_ulong, c_longlong};
use std::{mem, ptr};
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ffi::{CString, CStr};

use clangll::*;

/// A cursor into the Clang AST, pointing to an AST node.
///
/// We call the AST node pointed to by the cursor the cursor's "referent".
#[derive(Copy, Clone)]
pub struct Cursor {
    x: CXCursor
}

impl fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Cursor({} kind: {}, loc: {}, usr: {:?})",
               self.spelling(), kind_to_str(self.kind()), self.location(), self.usr())
    }
}

/// A cursor visitor function.
///
/// The first argument is the AST node currently being visited. The second
/// argument is the parent of the AST node currently being visited. The return
/// value informs how traversal should proceed.
pub type CursorVisitor<'s> = for<'a, 'b> FnMut(&'a Cursor, &'b Cursor) -> Enum_CXChildVisitResult + 's;

impl Cursor {
    /// Get the Unified Symbol Resolution for this cursor's referent, if
    /// available.
    ///
    /// The USR can be used to compare entities across translation units.
    pub fn usr(&self) -> Option<String> {
        let s = String_ { x: unsafe { clang_getCursorUSR(self.x) } }.to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Is this cursor's referent a declaration?
    pub fn is_declaration(&self) -> bool {
        unsafe { clang_isDeclaration(self.kind()) != 0 }
    }

    /// Get the null cursor, which has no referent.
    pub fn null() -> Self {
        Cursor { x: unsafe { clang_getNullCursor() } }
    }

    /// Get this cursor's referent's spelling.
    pub fn spelling(&self) -> String {
        unsafe {
            String_ { x: clang_getCursorSpelling(self.x) }.to_string()
        }
    }

    /// Get this cursor's referent's display name.
    ///
    /// This is not necessarily a valid identifier. It includes extra
    /// information, such as parameters for a function, etc.
    pub fn display_name(&self) -> String {
        unsafe {
            String_ { x: clang_getCursorDisplayName(self.x) }.to_string()
        }
    }

    /// Get the mangled name of this cursor's referent.
    pub fn mangling(&self) -> String {
        unsafe {
            String_ { x: clang_Cursor_getMangling(self.x) }.to_string()
        }
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
            Cursor { x: clang_getCursorLexicalParent(self.x) }
        }
    }

    /// Get the referent's semantic parent, if one is available.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub fn fallible_semantic_parent(&self) -> Option<Cursor> {
        let sp = self.semantic_parent();
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
        unsafe {
            Cursor { x: clang_getCursorSemanticParent(self.x) }
        }
    }

    /// Return the number of template arguments used by this cursor's referent,
    /// if the referent is either a template specialization or
    /// declaration. Returns -1 otherwise.
    pub fn num_template_args(&self) -> c_int {
        unsafe {
            clang_Cursor_getNumTemplateArguments(self.x)
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
        let mut semantic_parent = self.semantic_parent();

        while semantic_parent.kind() == CXCursor_Namespace ||
              semantic_parent.kind() == CXCursor_NamespaceAlias ||
              semantic_parent.kind() == CXCursor_NamespaceRef
        {
            semantic_parent = semantic_parent.semantic_parent();
        }

        let tu = self.translation_unit();
        // Yes, the second can happen with, e.g., macro definitions.
        semantic_parent == tu || semantic_parent == tu.semantic_parent()
    }

    /// Get the kind of referent this cursor is pointing to.
    pub fn kind(&self) -> Enum_CXCursorKind {
        unsafe {
            clang_getCursorKind(self.x)
        }
    }

    /// Is the referent an anonymous record definition?
    pub fn is_anonymous(&self) -> bool {
        unsafe {
            clang_Cursor_isAnonymous(self.x) != 0
        }
    }

    /// Is the referent a template specialization?
    pub fn is_template(&self) -> bool {
        self.specialized().is_valid()
    }

    /// Is the referent a fully specialized template specialization without any
    /// remaining free template arguments?
    pub fn is_fully_specialized_template(&self) -> bool {
        self.is_template() && self.num_template_args() > 0
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
        unsafe {
            clang_isInvalid(self.kind()) == 0
        }
    }

    /// Get the source location for the referent.
    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation { x: clang_getCursorLocation(self.x) }
        }
    }

    /// Get the source location range for the referent.
    pub fn extent(&self) -> CXSourceRange {
        unsafe {
            clang_getCursorExtent(self.x)
        }
    }

    /// Get the raw declaration comment for this referent, if one exists.
    pub fn raw_comment(&self) -> Option<String> {
        let s = unsafe {
            String_ { x: clang_Cursor_getRawCommentText(self.x) }.to_string()
        };
        if s.is_empty() { None } else { Some(s) }
    }

    /// Get the referent's parsed comment.
    pub fn comment(&self) -> Comment {
        unsafe {
            Comment { x: clang_Cursor_getParsedComment(self.x) }
        }
    }

    /// Get the referent's type.
    pub fn cur_type(&self) -> Type {
        unsafe {
            Type { x: clang_getCursorType(self.x) }
        }
    }

    /// Given that this cursor's referent is a reference to another type, or is
    /// a declaration, get the cursor pointing to the referenced type or type of
    /// the declared thing.
    pub fn definition(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCursorDefinition(self.x) }
        }
    }

    /// Given that this cursor's referent is reference type, get the cursor
    /// pointing to the referenced type.
    pub fn referenced(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCursorReferenced(self.x) }
        }
    }

    /// Get the canonical cursor for this referent.
    ///
    /// Many types can be declared multiple times before finally being properly
    /// defined. This method allows us to get the canonical cursor for the
    /// referent type.
    pub fn canonical(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCanonicalCursor(self.x) }
        }
    }

    /// Given that this cursor points to a template specialization, get a cursor
    /// pointing to the template definition that is being specialized.
    pub fn specialized(&self) -> Cursor {
        unsafe {
           Cursor { x: clang_getSpecializedCursorTemplate(self.x) }
        }
    }

    /// Assuming that this cursor's referent is a template declaration, get the
    /// kind of cursor that would be generated for its specializations.
    pub fn template_kind(&self) -> Enum_CXCursorKind {
        unsafe { clang_getTemplateCursorKind(self.x) }
    }

    /// Traverse this cursor's referent and its children.
    ///
    /// Call the given function on each AST node traversed. See `CursorVisitor`
    /// for details on arguments passed to the function and how its return value
    /// is interpreted.
    pub fn visit<F>(&self, func: F)
        where F: for<'a, 'b> FnMut(&'a Cursor, &'b Cursor) -> Enum_CXChildVisitResult
    {
        let mut data: Box<CursorVisitor> = Box::new(func);
        let opt_visit = Some(visit_children as extern "C" fn(CXCursor, CXCursor, CXClientData) -> Enum_CXChildVisitResult);
        unsafe {
            clang_visitChildren(self.x, opt_visit, mem::transmute(&mut data));
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
            if w == -1 {
                None
            } else {
                Some(w as u32)
            }
        }
    }

    /// Get the integer representation type used to hold this cursor's referent
    /// enum type.
    pub fn enum_type(&self) -> Type {
        unsafe {
            Type { x: clang_getEnumDeclIntegerType(self.x) }
        }
    }

    /// Get the signed constant value for this cursor's enum variant referent.
    ///
    /// Returns `LLONG_MIN` if the cursor's referent is not an enum variant,
    /// which is also a valid enum value, so callers should check the cursor
    /// kind before calling this method (see issue #127).
    pub fn enum_val_signed(&self) -> i64 {
        unsafe {
            clang_getEnumConstantDeclValue(self.x) as i64
        }
    }

    /// Get the unsigned constant value for this cursor's enum variant referent.
    ///
    /// Returns `ULLONG_MAX` if the cursor's referent is not an enum variant,
    /// which is also a valid enum value, so callers should check the cursor
    /// kind before calling this method (see issue #128).
    pub fn enum_val_unsigned(&self) -> u64 {
        unsafe {
            clang_getEnumConstantDeclUnsignedValue(self.x) as u64
        }
    }

    /// Given that this cursor's referent is a `typedef`, get the `Type` that is
    /// being aliased.
    pub fn typedef_type(&self) -> Type {
        unsafe {
            Type { x: clang_getTypedefDeclUnderlyingType(self.x) }
        }
    }

    /// Get the linkage kind for this cursor's referent.
    ///
    /// This only applies to functions and variables.
    pub fn linkage(&self) -> Enum_CXLinkageKind {
        unsafe {
            clang_getCursorLinkage(self.x)
        }
    }

    /// Get the visibility of this cursor's referent.
    pub fn visibility(&self) -> Enum_CXVisibilityKind {
        unsafe {
            clang_getCursorVisibility(self.x)
        }
    }

    /// Given that this cursor's referent is a function, return cursors to its
    /// parameters.
    pub fn args(&self) -> Vec<Cursor> {
        unsafe {
            let num = self.num_args().expect("expected value, got none") as u32;
            let mut args = vec![];
            for i in 0..num {
                args.push(Cursor { x: clang_Cursor_getArgument(self.x, i as c_uint) });
            }
            args
        }
    }

    /// Given that this cursor's referent is a function/method call or
    /// declaration, return a cursor to its return type.
    pub fn ret_type(&self) -> Type {
        unsafe {
            Type { x: clang_getCursorResultType(self.x) }
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
            if w == -1 {
                Err(())
            } else {
                Ok(w as u32)
            }
        }
    }

    /// Get the access specifier for this cursor's referent.
    pub fn access_specifier(&self) -> Enum_CX_CXXAccessSpecifier {
        unsafe {
            clang_getCXXAccessSpecifier(self.x)
        }
    }

    /// Is this cursor's referent a field declaration that is marked as
    /// `mutable`?
    pub fn is_mutable_field(&self) -> bool {
        unsafe {
            clang_CXXField_isMutable(self.x) != 0
        }
    }

    /// Is this cursor's referent a member function that is declared `static`?
    pub fn method_is_static(&self) -> bool {
        unsafe {
            clang_CXXMethod_isStatic(self.x) != 0
        }
    }

    /// Is this cursor's referent a member function that is declared `const`?
    pub fn method_is_const(&self) -> bool {
        unsafe {
            clang_CXXMethod_isConst(self.x) != 0
        }
    }

    /// Is this cursor's referent a member function that is declared `const`?
    pub fn method_is_virtual(&self) -> bool {
        unsafe {
            clang_CXXMethod_isVirtual(self.x) != 0
        }
    }

    /// Is this cursor's referent a struct or class with virtual members?
    pub fn is_virtual_base(&self) -> bool {
        unsafe {
            clang_isVirtualBase(self.x) != 0
        }
    }

    /// Given that this cursor's referent is a template specialization or
    /// declaration, get the `i`th template argument kind.
    ///
    /// If the referent is not a template or `i` is out of bounds, an invalid
    /// kind is returned.
    pub fn template_arg_kind(&self, i: c_int) -> CXTemplateArgumentKind {
        unsafe {
            clang_Cursor_getTemplateArgumentKind(self.x, i as c_uint)
        }
    }

    /// Given that this cursor's referent is a template specialization, and that
    /// the `i`th template argument is an integral, get the `i`th template
    /// argument value.
    pub fn template_arg_value(&self, i: c_int) -> c_longlong {
        unsafe {
            clang_Cursor_getTemplateArgumentValue(self.x, i as c_uint)
        }
    }
}

extern fn visit_children(cur: CXCursor, parent: CXCursor,
                         data: CXClientData) -> Enum_CXChildVisitResult {
    let func: &mut Box<CursorVisitor> = unsafe { mem::transmute(data) };
    (*func)(&Cursor { x : cur }, &Cursor { x: parent })
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Cursor) -> bool {
        unsafe {
            clang_equalCursors(self.x, other.x) == 1
        }
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
    x: CXType
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            clang_equalTypes(self.x, other.x) != 0
        }
    }
}

impl Eq for Type {}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Type({}, kind: {}, decl: {:?}, canon: {:?})",
               self.spelling(), type_to_str(self.kind()), self.declaration(),
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
            Cursor { x: clang_getTypeDeclaration(self.x) }
        }
    }

    /// Get a raw display name for this type.
    pub fn spelling(&self) -> String {
        unsafe {
            String_ { x: clang_getTypeSpelling(self.x) }.to_string()
        }
    }

    /// Is this type const qualified?
    pub fn is_const(&self) -> bool {
        unsafe {
            clang_isConstQualifiedType(self.x) != 0
        }
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

    /// If this type is a class template specialization, return its number of
    /// template arguments. Otherwise, return -1.
    pub fn num_template_args(&self) -> c_int {
        unsafe {
            clang_Type_getNumTemplateArguments(self.x)
        }
    }

    /// Get the type of the `i`th template argument for this template
    /// specialization.
    pub fn template_arg_type(&self, i: c_int) -> Type {
        unsafe {
            Type { x: clang_Type_getTemplateArgumentAsType(self.x, i) }
        }
    }

    /// Given that this type is a pointer type, return the type that it points
    /// to.
    pub fn pointee_type(&self) -> Type {
        unsafe {
            Type { x: clang_getPointeeType(self.x) }
        }
    }

    /// Given that this type is an array, vector, or complex type, return the
    /// type of its elements.
    pub fn elem_type(&self) -> Type {
        unsafe {
            Type { x: clang_getElementType(self.x) }
        }
    }

    /// Given that this type is an array or vector type, return its number of
    /// elements.
    pub fn num_elements(&self) -> usize {
        unsafe {
            clang_getNumElements(self.x) as usize
        }
    }

    /// Get the canonical version of this type. This sees through `typdef`s and
    /// aliases to get the underlying, canonical type.
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type { x: clang_getCanonicalType(self.x) }
        }
    }

    /// Is this type a variadic function type?
    pub fn is_variadic(&self) -> bool {
        unsafe {
            clang_isFunctionTypeVariadic(self.x) != 0
        }
    }

    /// Given that this type is a function type, get the types of its
    /// parameters.
    pub fn arg_types(&self) -> Vec<Type> {
        unsafe {
            let num = clang_getNumArgTypes(self.x) as usize;
            let mut args = vec!();
            for i in 0..num {
                args.push(Type { x: clang_getArgType(self.x, i as c_uint) });
            }
            args
        }
    }

    /// Given that this type is a function type, get the type of its return
    /// value.
    pub fn ret_type(&self) -> Option<Type> {
        let rt = Type { x: unsafe { clang_getResultType(self.x) } };
        if rt.kind() == CXType_Invalid {
            None
        } else {
            Some(rt)
        }
    }

    /// Given that this type is a function type, get its calling convention. If
    /// this is not a function type, `CXCallingConv_Invalid` is returned.
    pub fn call_conv(&self) -> Enum_CXCallingConv {
        unsafe {
            clang_getFunctionTypeCallingConv(self.x)
        }
    }

    /// For elaborated types (types which use `class`, `struct`, or `union` to
    /// disambiguate types from local bindings), get the underlying type.
    #[cfg(not(feature="llvm_stable"))]
    pub fn named(&self) -> Type {
        unsafe {
            Type { x: clang_Type_getNamedType(self.x) }
        }
    }
}

/// A `SourceLocation` is a file, line, column, and byte offset location for
/// some source text.
pub struct SourceLocation {
    x: CXSourceLocation
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
            clang_getSpellingLocation(self.x, &mut file, &mut line, &mut col, &mut off);
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

/// A comment in the source text.
///
/// Comments are sort of parsed by Clang, and have a tree structure.
pub struct Comment {
    x: CXComment
}

impl Comment {
    /// What kind of comment is this?
    pub fn kind(&self) -> Enum_CXCommentKind {
        unsafe {
            clang_Comment_getKind(self.x)
        }
    }

    /// Get the number of children this comment node has.
    pub fn num_children(&self) -> c_uint {
        unsafe {
            clang_Comment_getNumChildren(self.x)
        }
    }

    /// Get this comment's `idx`th child comment
    pub fn get_child(&self, idx: c_uint) -> Comment {
        unsafe {
            Comment { x: clang_Comment_getChild(self.x, idx) }
        }
    }

    /// Given that this comment is the start or end of an HTML tag, get its tag
    /// name.
    pub fn get_tag_name(&self) -> String {
        unsafe {
            String_ { x: clang_HTMLTagComment_getTagName(self.x) }.to_string()
        }
    }

    /// Given that this comment is an HTML start tag, get the number of HTML
    /// attributes it has.
    pub fn get_num_tag_attrs(&self) -> c_uint {
        unsafe {
            clang_HTMLStartTag_getNumAttrs(self.x)
        }
    }

    /// Given that this comment is an HTML start tag, get the `idx`th
    /// attribute's name.
    pub fn get_tag_attr_name(&self, idx: c_uint) -> String {
        unsafe {
            String_ { x: clang_HTMLStartTag_getAttrName(self.x, idx) }.to_string()
        }
    }

    /// Given that this comment is an HTML start tag, get the `idx`th
    /// attribute's value.
    pub fn get_tag_attr_value(&self, idx: c_uint) -> String {
        unsafe {
            String_ { x: clang_HTMLStartTag_getAttrValue(self.x, idx) }.to_string()
        }
    }
}

/// A source file.
pub struct File {
    x: CXFile
}

impl File {
    /// Get the name of this source file.
    pub fn name(&self) -> Option<String> {
        if self.x.is_null() {
            return None;
        }
        unsafe {
            Some(String_ { x: clang_getFileName(self.x) }.to_string())
        }
    }
}

/// A Clang string.
pub struct String_ {
    x: CXString
}

impl fmt::Display for String_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x.data.is_null() {
            return "".fmt(f);
        }
        unsafe {
            let c_str = clang_getCString(self.x) as *const c_char;
            let p = c_str as *const _;
            f.write_str(&String::from_utf8_lossy(CStr::from_ptr(p).to_bytes()))
        }
    }
}

/// An `Index` is an environment for a set of translation units that will
/// typically end up linked together in one final binary.
pub struct Index {
    x: CXIndex
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
            Index { x: clang_createIndex(pch as c_int, diag as c_int) }
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
    x: CXTranslationUnit
}

impl fmt::Debug for TranslationUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "TranslationUnit {{ }}")
    }
}

impl TranslationUnit {
    /// Parse a source file into a translation unit.
    pub fn parse(ix: &Index, file: &str, cmd_args: &[String],
                 unsaved: &[UnsavedFile], opts: ::libc::c_uint) -> Option<TranslationUnit> {
        let fname = CString::new(file).unwrap();
        let _c_args: Vec<CString> = cmd_args.iter().map(|s| CString::new(s.clone()).unwrap()).collect();
        let c_args: Vec<*const c_char> = _c_args.iter().map(|s| s.as_ptr()).collect();
        let mut c_unsaved: Vec<Struct_CXUnsavedFile> = unsaved.iter().map(|f| f.x).collect();
        let tu = unsafe {
            clang_parseTranslationUnit(ix.x, fname.as_ptr(),
                                       c_args.as_ptr(),
                                       c_args.len() as c_int,
                                       c_unsaved.as_mut_ptr(),
                                       c_unsaved.len() as c_uint,
                                       opts)
        };
        if tu.is_null() {
            None
        } else {
            Some(TranslationUnit { x: tu })
        }
    }

    /// Reparse this translation unit, maybe because the file changed on disk or
    /// something like that.
    pub fn reparse(&self, unsaved: &[UnsavedFile], opts: usize) -> bool {
        let mut c_unsaved: Vec<Struct_CXUnsavedFile> = unsaved.iter().map(|f| f.x).collect();

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
            let mut diags = vec!();
            for i in 0..num {
                diags.push(Diagnostic { x: clang_getDiagnostic(self.x, i as c_uint) });
            }
            diags
        }
    }

    /// Get a cursor pointing to the root of this translation unit's AST.
    pub fn cursor(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getTranslationUnitCursor(self.x) }
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
            let mut num_tokens : c_uint = 0;
            clang_tokenize(self.x, range, &mut token_ptr, &mut num_tokens);
            if token_ptr.is_null() {
                return None;
            }
            let token_array = ::std::slice::from_raw_parts(token_ptr, num_tokens as usize);
            for &token in token_array.iter() {
                let kind = clang_getTokenKind(token);
                let spelling = String_ { x: clang_getTokenSpelling(self.x, token) }.to_string();
                tokens.push(Token { kind: kind, spelling: spelling });
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
    x: CXDiagnostic
}

impl Diagnostic {
    /// Get the default diagnostic display option bit flags.
    pub fn default_opts() -> usize {
        unsafe {
            clang_defaultDiagnosticDisplayOptions() as usize
        }
    }

    /// Format this diagnostic message as a string, using the given option bit
    /// flags.
    pub fn format(&self, opts: usize) -> String {
        unsafe {
            String_ { x: clang_formatDiagnostic(self.x, opts as c_uint) }.to_string()
        }
    }

    /// What is the severity of this diagnostic message?
    pub fn severity(&self) -> Enum_CXDiagnosticSeverity {
        unsafe {
            clang_getDiagnosticSeverity(self.x)
        }
    }

    /// Destroy this diagnostic message.
    pub fn dispose(&self) {
        unsafe {
            clang_disposeDiagnostic(self.x);
        }
    }
}

/// A file which has not been saved to disk.
pub struct UnsavedFile {
    x: Struct_CXUnsavedFile,
    name: CString,
    contents: CString
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
            contents: contents
        }
    }
}

/// Convert a cursor kind into a static string.
pub fn kind_to_str(x: Enum_CXCursorKind) -> &'static str {
    match x {
        CXCursor_UnexposedDecl => "UnexposedDecl",
        CXCursor_StructDecl => "StructDecl",
        CXCursor_UnionDecl => "UnionDecl",
        CXCursor_ClassDecl => "ClassDecl",
        CXCursor_EnumDecl => "EnumDecl",
        CXCursor_FieldDecl => "FieldDecl",
        CXCursor_EnumConstantDecl => "EnumConstantDecl",
        CXCursor_FunctionDecl => "FunctionDecl",
        CXCursor_VarDecl => "VarDecl",
        CXCursor_ParmDecl => "ParmDecl",
        CXCursor_ObjCInterfaceDecl => "ObjCInterfaceDecl",
        CXCursor_ObjCCategoryDecl => "ObjCCategoryDecl",
        CXCursor_ObjCProtocolDecl => "ObjCProtocolDecl",
        CXCursor_ObjCPropertyDecl => "ObjCPropertyDecl",
        CXCursor_ObjCIvarDecl => "ObjCIvarDecl",
        CXCursor_ObjCInstanceMethodDecl => "ObjCInstanceMethodDecl",
        CXCursor_ObjCClassMethodDecl => "ObjCClassMethodDecl",
        CXCursor_ObjCImplementationDecl => "ObjCImplementationDecl",
        CXCursor_ObjCCategoryImplDecl => "ObjCCategoryImplDecl",
        CXCursor_TypedefDecl => "TypedefDecl",
        CXCursor_CXXMethod => "CXXMethod",
        CXCursor_Namespace => "Namespace",
        CXCursor_LinkageSpec => "LinkageSpec",
        CXCursor_Constructor => "Constructor",
        CXCursor_Destructor => "Destructor",
        CXCursor_ConversionFunction => "ConversionFunction",
        CXCursor_TemplateTypeParameter => "TemplateTypeParameter",
        CXCursor_NonTypeTemplateParameter => "NonTypeTemplateParameter",
        CXCursor_TemplateTemplateParameter => "TemplateTemplateParameter",
        CXCursor_FunctionTemplate => "FunctionTemplate",
        CXCursor_ClassTemplate => "ClassTemplate",
        CXCursor_ClassTemplatePartialSpecialization => "ClassTemplatePartialSpecialization",
        CXCursor_NamespaceAlias => "NamespaceAlias",
        CXCursor_UsingDirective => "UsingDirective",
        CXCursor_UsingDeclaration => "UsingDeclaration",
        CXCursor_TypeAliasDecl => "TypeAliasDecl",
        CXCursor_ObjCSynthesizeDecl => "ObjCSynthesizeDecl",
        CXCursor_ObjCDynamicDecl => "ObjCDynamicDecl",
        CXCursor_CXXAccessSpecifier => "CXXAccessSpecifier",
        // CXCursor_FirstDecl => "FirstDecl",
        // CXCursor_LastDecl => "LastDecl",
        CXCursor_FirstRef => "FirstRef",
        // CXCursor_ObjCSuperClassRef => "ObjCSuperClassRef",
        CXCursor_ObjCProtocolRef => "ObjCProtocolRef",
        CXCursor_ObjCClassRef => "ObjCClassRef",
        CXCursor_TypeRef => "TypeRef",
        CXCursor_CXXBaseSpecifier => "CXXBaseSpecifier",
        CXCursor_TemplateRef => "TemplateRef",
        CXCursor_NamespaceRef => "NamespaceRef",
        CXCursor_MemberRef => "MemberRef",
        // CXCursor_LabelRef => "LabelRef",
        CXCursor_OverloadedDeclRef => "OverloadedDeclRef",
        CXCursor_VariableRef => "VariableRef",
        // CXCursor_LastRef => "LastRef",
        CXCursor_FirstInvalid => "FirstInvalid",
        // CXCursor_InvalidFile => "InvalidFile",
        CXCursor_NoDeclFound => "NoDeclFound",
        CXCursor_NotImplemented => "NotImplemented",
        CXCursor_InvalidCode => "InvalidCode",
        // CXCursor_LastInvalid => "LastInvalid",
        CXCursor_FirstExpr => "FirstExpr",
        // CXCursor_UnexposedExpr => "UnexposedExpr",
        CXCursor_DeclRefExpr => "DeclRefExpr",
        CXCursor_MemberRefExpr => "MemberRefExpr",
        CXCursor_CallExpr => "CallExpr",
        CXCursor_ObjCMessageExpr => "ObjCMessageExpr",
        CXCursor_BlockExpr => "BlockExpr",
        CXCursor_IntegerLiteral => "IntegerLiteral",
        CXCursor_FloatingLiteral => "FloatingLiteral",
        CXCursor_ImaginaryLiteral => "ImaginaryLiteral",
        CXCursor_StringLiteral => "StringLiteral",
        CXCursor_CharacterLiteral => "CharacterLiteral",
        CXCursor_ParenExpr => "ParenExpr",
        CXCursor_UnaryOperator => "UnaryOperator",
        CXCursor_ArraySubscriptExpr => "ArraySubscriptExpr",
        CXCursor_BinaryOperator => "BinaryOperator",
        CXCursor_CompoundAssignOperator => "CompoundAssignOperator",
        CXCursor_ConditionalOperator => "ConditionalOperator",
        CXCursor_CStyleCastExpr => "CStyleCastExpr",
        CXCursor_CompoundLiteralExpr => "CompoundLiteralExpr",
        CXCursor_InitListExpr => "InitListExpr",
        CXCursor_AddrLabelExpr => "AddrLabelExpr",
        CXCursor_StmtExpr => "StmtExpr",
        CXCursor_GenericSelectionExpr => "GenericSelectionExpr",
        CXCursor_GNUNullExpr => "GNUNullExpr",
        CXCursor_CXXStaticCastExpr => "CXXStaticCastExpr",
        CXCursor_CXXDynamicCastExpr => "CXXDynamicCastExpr",
        CXCursor_CXXReinterpretCastExpr => "CXXReinterpretCastExpr",
        CXCursor_CXXConstCastExpr => "CXXConstCastExpr",
        CXCursor_CXXFunctionalCastExpr => "CXXFunctionalCastExpr",
        CXCursor_CXXTypeidExpr => "CXXTypeidExpr",
        CXCursor_CXXBoolLiteralExpr => "CXXBoolLiteralExpr",
        CXCursor_CXXNullPtrLiteralExpr => "CXXNullPtrLiteralExpr",
        CXCursor_CXXThisExpr => "CXXThisExpr",
        CXCursor_CXXThrowExpr => "CXXThrowExpr",
        CXCursor_CXXNewExpr => "CXXNewExpr",
        CXCursor_CXXDeleteExpr => "CXXDeleteExpr",
        CXCursor_UnaryExpr => "UnaryExpr",
        CXCursor_ObjCStringLiteral => "ObjCStringLiteral",
        CXCursor_ObjCEncodeExpr => "ObjCEncodeExpr",
        CXCursor_ObjCSelectorExpr => "ObjCSelectorExpr",
        CXCursor_ObjCProtocolExpr => "ObjCProtocolExpr",
        CXCursor_ObjCBridgedCastExpr => "ObjCBridgedCastExpr",
        CXCursor_PackExpansionExpr => "PackExpansionExpr",
        CXCursor_SizeOfPackExpr => "SizeOfPackExpr",
        CXCursor_LambdaExpr => "LambdaExpr",
        CXCursor_ObjCBoolLiteralExpr => "ObjCBoolLiteralExpr",
        // CXCursor_LastExpr => "LastExpr",
        CXCursor_FirstStmt => "FirstStmt",
        // CXCursor_UnexposedStmt => "UnexposedStmt",
        CXCursor_LabelStmt => "LabelStmt",
        CXCursor_CompoundStmt => "CompoundStmt",
        CXCursor_CaseStmt => "CaseStmt",
        CXCursor_DefaultStmt => "DefaultStmt",
        CXCursor_IfStmt => "IfStmt",
        CXCursor_SwitchStmt => "SwitchStmt",
        CXCursor_WhileStmt => "WhileStmt",
        CXCursor_DoStmt => "DoStmt",
        CXCursor_ForStmt => "ForStmt",
        CXCursor_GotoStmt => "GotoStmt",
        CXCursor_IndirectGotoStmt => "IndirectGotoStmt",
        CXCursor_ContinueStmt => "ContinueStmt",
        CXCursor_BreakStmt => "BreakStmt",
        CXCursor_ReturnStmt => "ReturnStmt",
        CXCursor_AsmStmt => "AsmStmt",
        CXCursor_ObjCAtTryStmt => "ObjCAtTryStmt",
        CXCursor_ObjCAtCatchStmt => "ObjCAtCatchStmt",
        CXCursor_ObjCAtFinallyStmt => "ObjCAtFinallyStmt",
        CXCursor_ObjCAtThrowStmt => "ObjCAtThrowStmt",
        CXCursor_ObjCAtSynchronizedStmt => "ObjCAtSynchronizedStmt",
        CXCursor_ObjCAutoreleasePoolStmt => "ObjCAutoreleasePoolStmt",
        CXCursor_ObjCForCollectionStmt => "ObjCForCollectionStmt",
        CXCursor_CXXCatchStmt => "CXXCatchStmt",
        CXCursor_CXXTryStmt => "CXXTryStmt",
        CXCursor_CXXForRangeStmt => "CXXForRangeStmt",
        CXCursor_SEHTryStmt => "SEHTryStmt",
        CXCursor_SEHExceptStmt => "SEHExceptStmt",
        CXCursor_SEHFinallyStmt => "SEHFinallyStmt",
        CXCursor_NullStmt => "NullStmt",
        CXCursor_DeclStmt => "DeclStmt",
        // CXCursor_LastStmt => "LastStmt",
        CXCursor_TranslationUnit => "TranslationUnit",
        CXCursor_FirstAttr => "FirstAttr",
        // CXCursor_UnexposedAttr => "UnexposedAttr",
        CXCursor_IBActionAttr => "IBActionAttr",
        CXCursor_IBOutletAttr => "IBOutletAttr",
        CXCursor_IBOutletCollectionAttr => "IBOutletCollectionAttr",
        CXCursor_CXXFinalAttr => "CXXFinalAttr",
        CXCursor_CXXOverrideAttr => "CXXOverrideAttr",
        CXCursor_AnnotateAttr => "AnnotateAttr",
        CXCursor_AsmLabelAttr => "AsmLabelAttr",
        // CXCursor_LastAttr => "LastAttr",
        CXCursor_PreprocessingDirective => "PreprocessingDirective",
        CXCursor_MacroDefinition => "MacroDefinition",
        CXCursor_MacroExpansion => "MacroExpansion",
        // CXCursor_MacroInstantiation => "MacroInstantiation",
        CXCursor_InclusionDirective => "InclusionDirective",
        //CXCursor_FirstPreprocessing => "FirstPreprocessing",
        //CXCursor_LastPreprocessing => "LastPreprocessing",
        CXCursor_PackedAttr => "PackedAttr",
        CXCursor_ModuleImportDecl => "ModuleImportDecl",
        CXCursor_TypeAliasTemplateDecl => "TypeAliasTemplateDecl",
        CXCursor_StaticAssert => "StaticAssert",
        _ => "?",
    }
}

/// Convert a type kind to a static string.
pub fn type_to_str(x: Enum_CXTypeKind) -> &'static str {
    match x {
        CXType_Invalid => "Invalid",
        CXType_Unexposed => "Unexposed",
        CXType_Void => "Void",
        CXType_Bool => "Bool",
        CXType_Char_U =>  "Char_U",
        CXType_UChar => "UChar",
        CXType_Char16=> "Char16",
        CXType_Char32=> "Char32",
        CXType_UShort => "UShort",
        CXType_UInt => "UInt",
        CXType_ULong => "ULong",
        CXType_ULongLong => "ULongLong",
        CXType_UInt128=>"UInt128",
        CXType_Char_S => "Char_S",
        CXType_SChar => "SChar",
        CXType_WChar => "WChar",
        CXType_Short => "Short",
        CXType_Int => "Int",
        CXType_Long => "Long",
        CXType_LongLong => "LongLong",
        CXType_Int128=>"Int128",
        CXType_Float => "Float",
        CXType_Double => "Double",
        CXType_LongDouble => "LongDouble",
        CXType_NullPtr => "NullPtr",
        CXType_Overload => "Overload",
        CXType_Dependent => "Dependent",
        CXType_ObjCId => "ObjCId",
        CXType_ObjCClass => "ObjCClass",
        CXType_ObjCSel => "ObjCSel",
        // CXType_FirstBuiltin => "FirstBuiltin",
        // CXType_LastBuiltin => "LastBuiltin",
        CXType_Complex => "Complex",
        CXType_Pointer => "Pointer",
        CXType_BlockPointer => "BlockPointer",
        CXType_LValueReference => "LValueReference",
        CXType_RValueReference => "RValueReference",
        CXType_Record => "Record",
        CXType_Enum => "Enum",
        CXType_Typedef => "Typedef",
        CXType_ObjCInterface => "ObjCInterface",
        CXType_ObjCObjectPointer => "ObjCObjectPointer",
        CXType_FunctionNoProto => "FunctionNoProto",
        CXType_FunctionProto => "FunctionProto",
        CXType_ConstantArray => "ConstantArray",
        CXType_Vector => "Vector",
        CXType_IncompleteArray => "IncompleteArray",
        CXType_VariableArray => "VariableArray",
        CXType_DependentSizedArray => "DependentSizedArray",
        CXType_MemberPointer => "MemberPointer",
        #[cfg(not(feature="llvm_stable"))]
        CXType_Auto => "Auto",
        #[cfg(not(feature="llvm_stable"))]
        CXType_Elaborated => "Elaborated",
        _ => "?"
    }
}

/// Dump the Clang AST to stdout for debugging purposes.
pub fn ast_dump(c: &Cursor, depth: isize)-> Enum_CXVisitorResult {
    fn print_indent(depth: isize, s: &str) {
        let mut i = 0;
        while i < depth {
            print!("\t");
            i += 1;
        }
        println!("{}", s);
    }
    let ct = c.cur_type().kind();
    print_indent(depth, &format!("({} {} {}",
        kind_to_str(c.kind()),
        c.spelling(),
        type_to_str(ct))
    );
    c.visit(| s, _: &Cursor| {
        ast_dump(s, depth + 1)
    });
    print_indent(depth, ")");
    CXChildVisit_Continue
}
