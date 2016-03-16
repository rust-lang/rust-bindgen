#![allow(non_upper_case_globals, dead_code)]

use std::os::raw::{c_uint, c_char, c_int, c_ulong};
use std::fmt;
use std::str;
use std::ffi::CStr;
use std::hash::Hash;
use std::hash::Hasher;
use std::ffi::CString;
use std::mem;

use clang_sys::*;

// Cursor
#[derive(Copy, Clone)]
pub struct Cursor {
    x: CXCursor
}

pub type CursorVisitor<'s> = for<'a, 'b> FnMut(&'a Cursor, &'b Cursor) -> CXChildVisitResult + 's;

impl Cursor {
    // common
    pub fn spelling(&self) -> String {
        unsafe {
            String_ { x: clang_getCursorSpelling(self.x) }.to_string()
        }
    }

    pub fn kind(&self) -> CXCursorKind {
        unsafe {
            clang_getCursorKind(self.x)
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

    pub fn canonical(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getCanonicalCursor(self.x) }
        }
    }

    pub fn visit<F>(&self, func:F)
        where F: for<'a, 'b> FnMut(&'a Cursor, &'b Cursor) -> CXChildVisitResult
    {
        let mut data: Box<CursorVisitor> = Box::new(func);
        unsafe {
            clang_visitChildren(self.x, visit_children, mem::transmute(&mut data));
        }
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
    pub fn linkage(&self) -> CXLinkageKind {
        unsafe {
            clang_getCursorLinkage(self.x)
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
}

extern fn visit_children(cur: CXCursor, parent: CXCursor,
                         data: CXClientData) -> CXChildVisitResult {
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

impl Type {
    // common
    pub fn kind(&self) -> CXTypeKind {
        self.x.kind
    }

    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor { x: clang_getTypeDeclaration(self.x) }
        }
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

    pub fn align(&self) -> usize {
        unsafe {
            let val = clang_Type_getAlignOf(self.x);
            if val < 0 { 0 } else { val as usize }
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

    pub fn call_conv(&self) -> CXCallingConv {
        unsafe {
            clang_getFunctionTypeCallingConv(self.x)
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
            let mut file = mem::uninitialized();
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

// File
pub struct File {
    x: CXFile
}

impl File {
    pub fn name(&self) -> Option<String> {
        if self.x.0.is_null() {
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
            str::from_utf8(CStr::from_ptr(p).to_bytes()).unwrap().to_owned().fmt(f)
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
        self.x.0.is_null()
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
                 unsaved: &[UnsavedFile], opts: CXTranslationUnit_Flags) -> TranslationUnit {
        let fname = CString::new(file.as_bytes()).unwrap();
        let fname = fname.as_ptr();
        let c_args: Vec<CString> = cmd_args.iter().map(|s| CString::new(s.as_bytes()).unwrap()).collect();
        let c_args: Vec<*const c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
        let mut c_unsaved: Vec<CXUnsavedFile> = unsaved.iter().map(|f| f.x).collect();
        let tu = unsafe {
            clang_parseTranslationUnit(ix.x, fname,
                                       c_args.as_ptr(),
                                       c_args.len() as c_int,
                                       c_unsaved.as_mut_ptr(),
                                       c_unsaved.len() as c_uint,
                                       opts)
        };
        TranslationUnit { x: tu }
    }

    pub fn reparse(&self, unsaved: &[UnsavedFile], opts: CXReparse_Flags) -> bool {
        let mut c_unsaved: Vec<CXUnsavedFile> = unsaved.iter().map(|f| f.x).collect();

        unsafe {
            clang_reparseTranslationUnit(self.x,
                                         c_unsaved.len() as c_uint,
                                         c_unsaved.as_mut_ptr(),
                                         opts) == CXErrorCode::Success
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
        self.x.0.is_null()
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
    pub fn default_opts() -> CXDiagnosticDisplayOptions {
        unsafe {
            clang_defaultDiagnosticDisplayOptions()
        }
    }

    pub fn format(&self, opts: CXDiagnosticDisplayOptions) -> String {
        unsafe {
            String_ { x: clang_formatDiagnostic(self.x, opts) }.to_string()
        }
    }

    pub fn severity(&self) -> CXDiagnosticSeverity {
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
    x: CXUnsavedFile,
    name: CString,
    contents: CString
}

impl UnsavedFile {
    pub fn new(name: &str, contents: &str) -> UnsavedFile {
        let name = CString::new(name.as_bytes()).unwrap();
        let contents = CString::new(contents.as_bytes()).unwrap();
        let x = CXUnsavedFile {
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

// Debug
pub fn ast_dump(c: &Cursor, depth: isize)-> CXChildVisitResult {
    fn print_indent(depth: isize, s: &str) {
        let mut i = 0;
        while i < depth {
            print!("\t");
            i += 1;
        }
        println!("{}", s);
    }
    let ct = c.cur_type().kind();
    print_indent(depth, &format!("({:?} {} {:?}",
        c.kind(),
        c.spelling(),
        ct)[..]
    );
    c.visit(| s, _: &Cursor| {
        ast_dump(s, depth + 1)
    });
    print_indent(depth, ")");
    CXChildVisitResult::Continue
}
