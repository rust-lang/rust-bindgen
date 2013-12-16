#[allow(non_uppercase_pattern_statics)];

use std::libc::*;
use std::{cast, ptr, str, to_bytes};

pub use ll = clangll;
use clangll::*;

// Cursor
pub struct Cursor(CXCursor);

pub type CursorVisitor<'s> = 's |c: &Cursor, p: &Cursor| -> Enum_CXChildVisitResult;

impl Cursor {
    // common
    pub fn spelling(&self) -> ~str {
        unsafe {
            String(clang_getCursorSpelling(**self)).to_str()
        }
    }

    pub fn kind(&self) -> Enum_CXCursorKind {
        unsafe {
            clang_getCursorKind(**self)
        }
    }

    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation(clang_getCursorLocation(**self))
        }
    }

    pub fn cur_type(&self) -> Type {
        unsafe {
            Type(clang_getCursorType(**self))
        }
    }

    pub fn definition(&self) -> Cursor {
        unsafe {
            Cursor(clang_getCursorDefinition(**self))
        }
    }

    pub fn visit(&self, func: CursorVisitor) {
        unsafe {
            let data = cast::transmute::<&CursorVisitor, CXClientData>(&func);
            clang_visitChildren(**self, visit_children, data);
        };
    }

    // bitfield
    pub fn bit_width(&self) -> Option<uint> {
        unsafe {
            let w = clang_getFieldDeclBitWidth(**self);
            if (w == -1) {
                None
            } else {
                Some(w as uint)
            }
        }
    }

    // enum
    pub fn enum_type(&self) -> Type {
        unsafe {
            Type(clang_getEnumDeclIntegerType(**self))
        }
    }

    pub fn enum_val(&self) -> int {
        unsafe {
            clang_getEnumConstantDeclValue(**self) as int
        }
    }

    // typedef
    pub fn typedef_type(&self) -> Type {
        unsafe {
            Type(clang_getTypedefDeclUnderlyingType(**self))
        }
    }

    // function, variable
    pub fn linkage(&self) -> Enum_CXLinkageKind {
        unsafe {
            clang_getCursorLinkage(**self)
        }
    }

    // function
    pub fn args(&self) -> ~[Cursor] {
        unsafe {
            let num = self.num_args() as uint;
            let mut args = ~[];
            for i in range(0, num) {
                args.push(Cursor(clang_Cursor_getArgument(**self, i as c_uint)));
            }
            return args;
        }
    }

    pub fn ret_type(&self) -> Type {
        unsafe {
            Type(clang_getCursorResultType(**self))
        }
    }

    pub fn num_args(&self) -> i32 {
        unsafe {
            clang_Cursor_getNumArguments(**self)
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

    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor(clang_getTypeDeclaration(**self))
        }
    }

    pub fn is_const(&self) -> bool {
        unsafe {
            clang_isConstQualifiedType(**self) == 1
        }
    }

    pub fn size(&self) -> uint {
        unsafe {
            let val = clang_Type_getSizeOf(**self);
            if val < 0 { 0 } else { val as uint }
        }
    }

    pub fn align(&self) -> uint {
        unsafe {
            let val = clang_Type_getAlignOf(**self);
            if val < 0 { 0 } else { val as uint }
        }
    }

    // pointer
    pub fn pointee_type(&self) -> Type {
        unsafe {
            Type(clang_getPointeeType(**self))
        }
    }

    // array
    pub fn elem_type(&self) -> Type {
        unsafe {
            Type(clang_getArrayElementType(**self))
        }
    }

    pub fn array_size(&self) -> uint {
        unsafe {
            clang_getArraySize(**self) as uint
        }
    }

    // typedef
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type(clang_getCanonicalType(**self))
        }
    }

    // function
    pub fn is_variadic(&self) -> bool {
        unsafe {
            clang_isFunctionTypeVariadic(**self) == 1
        }
    }

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

    pub fn ret_type(&self) -> Type {
        unsafe {
            Type(clang_getResultType(**self))
        }
    }
}

// SourceLocation
pub struct SourceLocation(CXSourceLocation);

impl SourceLocation {
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
    pub fn name(&self) -> ~str {
        unsafe {
            String(clang_getFileName(**self)).to_str()
        }
    }
}

// String
pub struct String(CXString);

impl ToStr for String {
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
    pub fn create(pch: bool, diag: bool) -> Index {
        unsafe {
            Index(clang_createIndex(pch as c_int, diag as c_int))
        }
    }

    pub fn dispose(&self) {
        unsafe {
            clang_disposeIndex(**self);
        }
    }
}

// TranslationUnit
pub struct TranslationUnit(CXTranslationUnit);

impl TranslationUnit {
    pub fn parse(ix: &Index, file: &str, cmd_args: &[~str],
                 unsaved: &[UnsavedFile], opts: uint) -> TranslationUnit {
        let _fname = file.to_c_str();
        let fname = _fname.with_ref(|f| f);
        let _c_args = cmd_args.map(|s| (*s).to_c_str());
        let c_args = _c_args.map(|s| (*s).with_ref(|cs| cs));
        let mut c_unsaved = unsaved.map(|f| **f);
        let tu = unsafe {
            clang_parseTranslationUnit(**ix, fname,
                                       c_args.as_ptr(),
                                       c_args.len() as c_int,
                                       c_unsaved.as_mut_ptr(),
                                       c_unsaved.len() as c_uint,
                                       opts as c_uint)
        };
        TranslationUnit(tu)
    }

    pub fn reparse(&self, unsaved: &[UnsavedFile], opts: uint) -> bool {
        let mut c_unsaved = unsaved.map(|f| **f);

        unsafe {
            clang_reparseTranslationUnit(**self,
                                         c_unsaved.len() as c_uint,
                                         c_unsaved.as_mut_ptr(),
                                         opts as c_uint) == 0
        }
    }

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

    pub fn cursor(&self) -> Cursor {
        unsafe {
            Cursor(clang_getTranslationUnitCursor(**self))
        }
    }

    pub fn dispose(&self) {
        unsafe {
            clang_disposeTranslationUnit(**self);
        }
    }
}

// Diagnostic
pub struct Diagnostic(CXDiagnostic);

impl Diagnostic {
    pub fn default_opts() -> uint {
        unsafe {
            clang_defaultDiagnosticDisplayOptions() as uint
        }
    }

    pub fn format(&self, opts: uint) -> ~str {
        unsafe {
            String(clang_formatDiagnostic(**self, opts as c_uint)).to_str()
        }
    }

    pub fn severity(&self) -> Enum_CXDiagnosticSeverity {
        unsafe {
            clang_getDiagnosticSeverity(**self)
        }
    }

    pub fn dispose(&self) {
        unsafe {
            clang_disposeDiagnostic(**self);
        }
    }
}

// UnsavedFile
pub struct UnsavedFile(Struct_CXUnsavedFile);

pub fn kind_to_str(x: Enum_CXCursorKind) -> &str {
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

      _ => "?",
    }
}

pub fn type_to_str(x: Enum_CXTypeKind) -> &str {
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
      _ => "?"
    }
}

// Debug
pub fn ast_dump(c: &Cursor, depth: int)-> Enum_CXVisitorResult {
    fn print_indent(depth: int, s: &str) {
        let mut i = 0;
        while i < depth {
            print("\t");
            i += 1;
        }
        println(s);
    }
    let ct = c.cur_type().kind();
    print_indent(depth, format!("({} {} {}", kind_to_str(c.kind()), c.spelling(), type_to_str(ct)));
    c.visit(|s, _| {
        ast_dump(s, depth + 1)
    });
    print_indent(depth, ")");
    return CXChildVisit_Continue;
}
