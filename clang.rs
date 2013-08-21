use std::libc::*;
use std::{cast, ptr, str, to_bytes, vec};

pub use ll = clangll;
use clangll::*;

// Cursor
pub struct Cursor(CXCursor);

pub type CursorVisitor<'self> = &'self fn(c: &Cursor, p: &Cursor) -> Enum_CXChildVisitResult;

impl Cursor {
    // common
    #[fixed_stack_segment]
    pub fn spelling(&self) -> ~str {
        unsafe {
            String(clang_getCursorSpelling(**self)).to_str()
        }
    }

    #[fixed_stack_segment]
    pub fn kind(&self) -> Enum_CXCursorKind {
        unsafe {
            clang_getCursorKind(**self)
        }
    }

    #[fixed_stack_segment]
    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation(clang_getCursorLocation(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn cur_type(&self) -> Type {
        unsafe {
            Type(clang_getCursorType(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn definition(&self) -> Cursor {
        unsafe {
            Cursor(clang_getCursorDefinition(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn visit(&self, func: CursorVisitor) {
        unsafe {
            let data = cast::transmute::<&CursorVisitor, CXClientData>(&func);
            clang_visitChildren(**self, visit_children, data);
        };
    }

    // enum
    #[fixed_stack_segment]
    pub fn enum_type(&self) -> Type {
        unsafe {
            Type(clang_getEnumDeclIntegerType(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn enum_val(&self) -> int {
        unsafe {
            clang_getEnumConstantDeclValue(**self) as int
        }
    }

    // typedef
    #[fixed_stack_segment]
    pub fn typedef_type(&self) -> Type {
        unsafe {
            Type(clang_getTypedefDeclUnderlyingType(**self))
        }
    }

    // function, variable
    #[fixed_stack_segment]
    pub fn linkage(&self) -> Enum_CXLinkageKind {
        unsafe {
            clang_getCursorLinkage(**self)
        }
    }

    // function
    #[fixed_stack_segment]
    pub fn args(&self) -> ~[Cursor] {
        unsafe {
            let num = clang_Cursor_getNumArguments(**self) as uint;
            let mut args = ~[];
            for i in range(0, num) {
                args.push(Cursor(clang_Cursor_getArgument(**self, i as c_uint)));
            }
            return args;
        }
    }

    #[fixed_stack_segment]
    pub fn ret_type(&self) -> Type {
        unsafe {
            Type(clang_getCursorResultType(**self))
        }
    }
}

extern fn visit_children(cur: CXCursor, parent: ll::CXCursor,
                         data: CXClientData) -> ll::Enum_CXChildVisitResult {
    unsafe {
        let func = cast::transmute::<CXClientData, &CursorVisitor>(data);
        return (*func)(&Cursor(cur), &Cursor(parent));
    }
}

impl Eq for Cursor {
    #[fixed_stack_segment]
    fn eq(&self, other: &Cursor) -> bool {
        unsafe {
            clang_equalCursors(**self, **other) == 1
        }
    }

    fn ne(&self, other: &Cursor) -> bool {
        return !self.eq(other);
    }
}

impl IterBytes for Cursor {
    fn iter_bytes(&self, lsb0: bool, f: to_bytes::Cb) -> bool {
        [(self.kind as int),
         (self.xdata as int),
         (self.data[0] as int),
         (self.data[1] as int),
         (self.data[2] as int)
        ].iter_bytes(lsb0, f)
    }
}

// type
pub struct Type(CXType);

impl Type {
    // common
    pub fn kind(&self) -> Enum_CXTypeKind {
        return (*self).kind;
    }

    #[fixed_stack_segment]
    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor(clang_getTypeDeclaration(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn is_const(&self) -> bool {
        unsafe {
            clang_isConstQualifiedType(**self) == 1
        }
    }

    // pointer
    #[fixed_stack_segment]
    pub fn pointee_type(&self) -> Type {
        unsafe {
            Type(clang_getPointeeType(**self))
        }
    }

    // array
    #[fixed_stack_segment]
    pub fn elem_type(&self) -> Type {
        unsafe {
            Type(clang_getArrayElementType(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn array_size(&self) -> uint {
        unsafe {
            clang_getArraySize(**self) as uint
        }
    }

    // typedef
    #[fixed_stack_segment]
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type(clang_getCanonicalType(**self))
        }
    }

    // function
    #[fixed_stack_segment]
    pub fn is_variadic(&self) -> bool {
        unsafe {
            clang_isFunctionTypeVariadic(**self) == 1
        }
    }

    #[fixed_stack_segment]
    pub fn arg_types(&self) -> ~[Type] {
        unsafe {
            let num = clang_getNumArgTypes(**self) as uint;
            let mut args = ~[];
            for i in range(0, num) {
                args.push(Type(clang_getArgType(**self, i as c_uint)));
            }
            return args;
        }
    }

    #[fixed_stack_segment]
    pub fn ret_type(&self) -> Type {
        unsafe {
            Type(clang_getResultType(**self))
        }
    }
}

// SourceLocation
pub struct SourceLocation(CXSourceLocation);

impl SourceLocation {
    #[fixed_stack_segment]
    pub fn location(&self) -> (File, uint, uint, uint) {
        unsafe {
            let mut file = ptr::mut_null();
            let mut line = 0;
            let mut col = 0;
            let mut off = 0;
            clang_getSpellingLocation(**self, ptr::to_mut_unsafe_ptr(&mut file),
                                          ptr::to_mut_unsafe_ptr(&mut line),
                                          ptr::to_mut_unsafe_ptr(&mut col),
                                          ptr::to_mut_unsafe_ptr(&mut off));
            return (File(file), line as uint, col as uint, off as uint);
        }
    }
}

// File
pub struct File(CXFile);

impl File {
    #[fixed_stack_segment]
    pub fn name(&self) -> ~str {
        unsafe {
            String(clang_getFileName(**self)).to_str()
        }
    }
}

// String
pub struct String(CXString);

impl ToStr for String {
    #[fixed_stack_segment]
    fn to_str(&self) -> ~str {
        unsafe {
            let c_str = clang_getCString(**self) as *c_char;
            str::raw::from_c_str(c_str)
        }
    }
}

// Index
pub struct Index(CXIndex);

impl Index {
    #[fixed_stack_segment]
    pub fn create(pch: bool, diag: bool) -> Index {
        unsafe {
            Index(clang_createIndex(pch as c_int, diag as c_int))
        }
    }

    #[fixed_stack_segment]
    pub fn dispose(&self) {
        unsafe {
            clang_disposeIndex(**self);
        }
    }
}

// TranslationUnit
pub struct TranslationUnit(CXTranslationUnit);

impl TranslationUnit {
    #[fixed_stack_segment]
    pub fn parse(ix: &Index, file: &str, cmd_args: &[~str],
                 unsaved: &[UnsavedFile], opts: uint) -> TranslationUnit {
        let _fname = file.to_c_str();
        let fname = _fname.with_ref(|f| f);
        let _c_args = cmd_args.map(|s| (*s).to_c_str());
        let c_args = _c_args.map(|s| (*s).with_ref(|cs| cs));
        let mut c_unsaved = unsaved.map(|f| **f);
        let tu = unsafe {
            clang_parseTranslationUnit(**ix, fname,
                                       vec::raw::to_ptr(c_args),
                                       c_args.len() as c_int,
                                       vec::raw::to_mut_ptr(c_unsaved),
                                       c_unsaved.len() as c_uint,
                                       opts as c_uint)
        };
        TranslationUnit(tu)
    }

    #[fixed_stack_segment]
    pub fn diags(&self) -> ~[Diagnostic] {
        unsafe {
            let num = clang_getNumDiagnostics(**self) as uint;
            let mut diags = ~[];
            for i in range(0, num) {
                diags.push(Diagnostic(clang_getDiagnostic(**self, i as c_uint)));
            }
            return diags;
        }
    }

    #[fixed_stack_segment]
    pub fn cursor(&self) -> Cursor {
        unsafe {
            Cursor(clang_getTranslationUnitCursor(**self))
        }
    }

    #[fixed_stack_segment]
    pub fn dispose(&self) {
        unsafe {
            clang_disposeTranslationUnit(**self);
        }
    }
}

// Diagnostic
pub struct Diagnostic(CXDiagnostic);

impl Diagnostic {
    #[fixed_stack_segment]
    pub fn default_opts() -> uint {
        unsafe {
            clang_defaultDiagnosticDisplayOptions() as uint
        }
    }

    #[fixed_stack_segment]
    pub fn format(&self, opts: uint) -> ~str {
        unsafe {
            String(clang_formatDiagnostic(**self, opts as c_uint)).to_str()
        }
    }

    #[fixed_stack_segment]
    pub fn severity(&self) -> Enum_CXDiagnosticSeverity {
        unsafe {
            clang_getDiagnosticSeverity(**self)
        }
    }
}

// UnsavedFile
pub struct UnsavedFile(Struct_CXUnsavedFile);
