#![allow(non_upper_case_globals, dead_code)]

use std::os::raw::{c_uint, c_char, c_int, c_ulong, c_longlong};
use std::{mem, ptr};
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ffi::{CString, CStr};

use clangll::*;

// Cursor
#[derive(Copy, Clone)]
pub struct Cursor {
    x: CXCursor
}

pub type CursorVisitor<'s> = for<'a, 'b> FnMut(&'a Cursor, &'b Cursor) -> Enum_CXChildVisitResult + 's;

impl Cursor {
    // common
    pub fn spelling(&self) -> String {
        unsafe {
            String_ { x: clang_getCursorSpelling(self.x) }.to_string()
        }
    }

    pub fn display_name(&self) -> String {
        unsafe {
            String_ { x: clang_getCursorDisplayName(self.x) }.to_string()
        }
    }

    pub fn mangling(&self) -> String {
        let mut mangling = unsafe {
            String_ { x: clang_Cursor_getMangling(self.x) }.to_string()
        };

        // Try to undo backend mangling
        if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
            mangling.remove(0);
        }
        mangling
    }

    pub fn lexical_parent(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCursorLexicalParent(self.x) }
        }
    }

    pub fn semantic_parent(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCursorSemanticParent(self.x) }
        }
    }

    pub fn kind(&self) -> Enum_CXCursorKind {
        unsafe {
            clang_getCursorKind(self.x)
        }
    }

    pub fn is_template(&self) -> bool {
        self.specialized().is_valid()
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            clang_isInvalid(self.kind()) == 0
        }
    }

    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation { x: clang_getCursorLocation(self.x) }
        }
    }

    pub fn extent(&self) -> CXSourceRange {
        unsafe {
            clang_getCursorExtent(self.x)
        }
    }

    pub fn raw_comment(&self) -> String {
        unsafe {
            String_ { x: clang_Cursor_getRawCommentText(self.x) }.to_string()
        }
    }

    pub fn comment(&self) -> Comment {
        unsafe {
            Comment { x: clang_Cursor_getParsedComment(self.x) }
        }
    }

    pub fn cur_type(&self) -> Type {
        unsafe {
            Type { x: clang_getCursorType(self.x) }
        }
    }

    pub fn definition(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCursorDefinition(self.x) }
        }
    }

    pub fn referenced(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCursorReferenced(self.x) }
        }
    }

    pub fn canonical(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCanonicalCursor(self.x) }
        }
    }

    pub fn specialized(&self) -> Cursor {
        unsafe {
           Cursor { x: clang_getSpecializedCursorTemplate(self.x) }
        }
    }

    pub fn visit<F>(&self, func:F)
        where F: for<'a, 'b> FnMut(&'a Cursor, &'b Cursor) -> Enum_CXChildVisitResult
    {
        let mut data: Box<CursorVisitor> = Box::new(func);
        let opt_visit = Some(visit_children as extern "C" fn(CXCursor, CXCursor, CXClientData) -> Enum_CXChildVisitResult);
        unsafe {
            clang_visitChildren(self.x, opt_visit, mem::transmute(&mut data));
        }
    }

    #[cfg(not(feature="llvm_stable"))]
    pub fn is_inlined_function(&self) -> bool {
        unsafe { clang_Cursor_isFunctionInlined(self.x) != 0 }
    }

    // TODO: Remove this when LLVM 3.9 is released.
    //
    // This is currently used for CI purposes.
    #[cfg(feature="llvm_stable")]
    pub fn is_inlined_function(&self) -> bool {
        false
    }

    // bitfield
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

    // enum
    pub fn enum_type(&self) -> Type {
        unsafe {
            Type { x: clang_getEnumDeclIntegerType(self.x) }
        }
    }

    pub fn enum_val(&self) -> i64 {
        unsafe {
            clang_getEnumConstantDeclValue(self.x) as i64
        }
    }

    // typedef
    pub fn typedef_type(&self) -> Type {
        unsafe {
            Type { x: clang_getTypedefDeclUnderlyingType(self.x) }
        }
    }

    // function, variable
    pub fn linkage(&self) -> Enum_CXLinkageKind {
        unsafe {
            clang_getCursorLinkage(self.x)
        }
    }

    pub fn visibility(&self) -> Enum_CXVisibilityKind {
        unsafe {
            clang_getCursorVisibility(self.x)
        }
    }

    // function
    pub fn args(&self) -> Vec<Cursor> {
        unsafe {
            let num = self.num_args() as usize;
            let mut args = vec!();
            for i in 0..num {
                args.push(Cursor { x: clang_Cursor_getArgument(self.x, i as c_uint) });
            }
            args
        }
    }

    pub fn ret_type(&self) -> Type {
        unsafe {
            Type { x: clang_getCursorResultType(self.x) }
        }
    }

    pub fn num_args(&self) -> i32 {
        unsafe {
            clang_Cursor_getNumArguments(self.x)
        }
    }

    // CXX member
    pub fn access_specifier(&self) -> Enum_CX_CXXAccessSpecifier {
        unsafe {
            clang_getCXXAccessSpecifier(self.x)
        }
    }

    pub fn is_mutable_field(&self) -> bool {
        unsafe {
            clang_CXXField_isMutable(self.x) != 0
        }
    }

    // CXX method
    pub fn method_is_static(&self) -> bool {
        unsafe {
            clang_CXXMethod_isStatic(self.x) != 0
        }
    }

    pub fn method_is_virtual(&self) -> bool {
        unsafe {
            clang_CXXMethod_isVirtual(self.x) != 0
        }
    }

    // CXX base
    pub fn is_virtual_base(&self) -> bool {
        unsafe {
            clang_isVirtualBase(self.x) != 0
        }
    }

    // CXX template
    pub fn template_arg_kind(&self, i: c_int) -> CXTemplateArgumentKind {
        unsafe {
            clang_Cursor_getTemplateArgumentKind(self.x, i as c_uint)
        }
    }

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

    fn ne(&self, other: &Cursor) -> bool {
        !self.eq(other)
    }
}

impl Eq for Cursor {}

impl Hash for Cursor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.kind.hash(state);
        self.x.xdata.hash(state);
        self.x.data[0].hash(state);
        self.x.data[1].hash(state);
        self.x.data[2].hash(state);
    }
}

// type
pub struct Type {
    x: CXType
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LayoutError {
    Invalid,
    Incomplete,
    Dependent,
    NotConstantSize,
    InvalidFieldName,
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
    // common
    pub fn kind(&self) -> Enum_CXTypeKind {
        self.x.kind
    }

    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getTypeDeclaration(self.x) }
        }
    }

    pub fn spelling(&self) -> String {
        unsafe {
            String_ { x: clang_getTypeSpelling(self.x) }.to_string()
        }
    }


    // XXX make it more consistent
    //
    // This is currently only used to detect typedefs,
    // so it should not be a problem.
    pub fn sanitized_spelling(&self) -> String {
        self.spelling()
            .replace("const ", "")
            .split(' ').next().unwrap_or("").to_owned()
    }

    pub fn sanitized_spelling_in(&self, possible: &[String]) -> bool {
        let type_spelling = self.sanitized_spelling();
        possible.iter()
                .any(|spelling| *spelling == *type_spelling)
    }

    pub fn is_const(&self) -> bool {
        unsafe {
            clang_isConstQualifiedType(self.x) == 1
        }
    }

    pub fn size(&self) -> usize {
        unsafe {
            let val = clang_Type_getSizeOf(self.x);
            if val < 0 { 0 } else { val as usize }
        }
    }

    pub fn fallible_size(&self) -> Result<usize, LayoutError> {
        let val = unsafe { clang_Type_getSizeOf(self.x) };
        if val < 0 {
            Err(LayoutError::from(val as i32))
        } else {
            Ok(val as usize)
        }
    }

    pub fn align(&self) -> usize {
        unsafe {
            let val = clang_Type_getAlignOf(self.x);
            if val < 0 { 0 } else { val as usize }
        }
    }

    pub fn num_template_args(&self) -> c_int {
        unsafe {
            clang_Type_getNumTemplateArguments(self.x)
        }
    }

    pub fn template_arg_type(&self, i: c_int) -> Type {
        unsafe {
            Type { x: clang_Type_getTemplateArgumentAsType(self.x, i) }
        }
    }

    // pointer
    pub fn pointee_type(&self) -> Type {
        unsafe {
            Type { x: clang_getPointeeType(self.x) }
        }
    }

    // array
    pub fn elem_type(&self) -> Type {
        unsafe {
            Type { x: clang_getArrayElementType(self.x) }
        }
    }

    pub fn array_size(&self) -> usize {
        unsafe {
            clang_getArraySize(self.x) as usize
        }
    }

    // typedef
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type { x: clang_getCanonicalType(self.x) }
        }
    }

    // function
    pub fn is_variadic(&self) -> bool {
        unsafe {
            clang_isFunctionTypeVariadic(self.x) == 1
        }
    }

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

    pub fn ret_type(&self) -> Type {
        unsafe {
            Type { x: clang_getResultType(self.x) }
        }
    }

    pub fn call_conv(&self) -> Enum_CXCallingConv {
        unsafe {
            clang_getFunctionTypeCallingConv(self.x)
        }
    }

    #[cfg(not(feature="llvm_stable"))]
    pub fn named(&self) -> Type {
        unsafe {
            Type { x: clang_Type_getNamedType(self.x) }
        }
    }
}

// SourceLocation
pub struct SourceLocation {
    x: CXSourceLocation
}

impl SourceLocation {
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

// Comment
pub struct Comment {
    x: CXComment
}

impl Comment {
    pub fn kind(&self) -> Enum_CXCommentKind {
        unsafe {
            clang_Comment_getKind(self.x)
        }
    }

    pub fn num_children(&self) -> c_uint {
        unsafe {
            clang_Comment_getNumChildren(self.x)
        }
    }

    pub fn get_child(&self, idx: c_uint) -> Comment {
        unsafe {
            Comment { x: clang_Comment_getChild(self.x, idx) }
        }
    }

    // HTML
    pub fn get_tag_name(&self) -> String {
        unsafe {
            String_ { x: clang_HTMLTagComment_getTagName(self.x) }.to_string()
        }
    }

    pub fn get_num_tag_attrs(&self) -> c_uint {
        unsafe {
            clang_HTMLStartTag_getNumAttrs(self.x)
        }
    }

    pub fn get_tag_attr_name(&self, idx: c_uint) -> String {
        unsafe {
            String_ { x: clang_HTMLStartTag_getAttrName(self.x, idx) }.to_string()
        }
    }

    pub fn get_tag_attr_value(&self, idx: c_uint) -> String {
        unsafe {
            String_ { x: clang_HTMLStartTag_getAttrValue(self.x, idx) }.to_string()
        }
    }
}

// File
pub struct File {
    x: CXFile
}

impl File {
    pub fn name(&self) -> Option<String> {
        if self.x.is_null() {
            return None;
        }
        unsafe {
            Some(String_ { x: clang_getFileName(self.x) }.to_string())
        }
    }
}

// String
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

// Index
pub struct Index {
    x: CXIndex
}

impl Index {
    pub fn create(pch: bool, diag: bool) -> Index {
        unsafe {
            Index { x: clang_createIndex(pch as c_int, diag as c_int) }
        }
    }

    pub fn dispose(&self) {
        unsafe {
            clang_disposeIndex(self.x);
        }
    }

    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }
}

// Token
pub struct Token {
    pub kind: CXTokenKind,
    pub spelling: String,
}

// TranslationUnit
pub struct TranslationUnit {
    x: CXTranslationUnit
}

impl TranslationUnit {
    pub fn parse(ix: &Index, file: &str, cmd_args: &[String],
                 unsaved: &[UnsavedFile], opts: ::libc::c_uint) -> TranslationUnit {
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
        TranslationUnit { x: tu }
    }

    pub fn reparse(&self, unsaved: &[UnsavedFile], opts: usize) -> bool {
        let mut c_unsaved: Vec<Struct_CXUnsavedFile> = unsaved.iter().map(|f| f.x).collect();

        unsafe {
            clang_reparseTranslationUnit(self.x,
                                         c_unsaved.len() as c_uint,
                                         c_unsaved.as_mut_ptr(),
                                         opts as c_uint) == 0
        }
    }

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

    pub fn cursor(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getTranslationUnitCursor(self.x) }
        }
    }

    pub fn dispose(&self) {
        unsafe {
            clang_disposeTranslationUnit(self.x);
        }
    }

    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }

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

// Diagnostic
pub struct Diagnostic {
    x: CXDiagnostic
}

impl Diagnostic {
    pub fn default_opts() -> usize {
        unsafe {
            clang_defaultDiagnosticDisplayOptions() as usize
        }
    }

    pub fn format(&self, opts: usize) -> String {
        unsafe {
            String_ { x: clang_formatDiagnostic(self.x, opts as c_uint) }.to_string()
        }
    }

    pub fn severity(&self) -> Enum_CXDiagnosticSeverity {
        unsafe {
            clang_getDiagnosticSeverity(self.x)
        }
    }

    pub fn dispose(&self) {
        unsafe {
            clang_disposeDiagnostic(self.x);
        }
    }
}

// UnsavedFile
pub struct UnsavedFile {
    x: Struct_CXUnsavedFile,
    name: CString,
    contents: CString
}

impl UnsavedFile {
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

        _ => "?",
    }
}

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

// Debug
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
