import ctypes::*;
import str::sbuf;

type CXIndex = *void;

type CXTranslationUnit = *void;

type CXUnsavedFile = {
    Filename: sbuf,
    Contents: sbuf,
    Length: ulong
};

type CXDiagnostic = *void;

type CXString = {
    data: *void,
    private_flags: unsigned
};

type CXCursor = {
    kind: c_int,
    xdata: c_int,
    data: (*void, *void, *void)
};

type CXType = {
    kind: c_int,
    data: (*void, *void)
};

type CXSourceLocation = {
    ptr_data: (*void, *void),
    int_data: unsigned
};

type CXFile = *void;

type CXClientData = *void;

type CXCursorVisitor = *u8;

const CXDiagnostic_Error: c_int = 3_i32;

const CXChildVisit_Break: c_int = 0_i32;
const CXChildVisit_Continue: c_int = 1_i32;
const CXChildVisit_Recurse: c_int = 2_i32;

const CXCursor_StructDecl: c_int = 2_i32;
const CXCursor_UnionDecl: c_int = 3_i32;
const CXCursor_EnumDecl: c_int = 5_i32;
const CXCursor_FieldDecl: c_int = 6_i32;
const CXCursor_EnumConstantDecl: c_int = 7_i32;
const CXCursor_FunctionDecl: c_int = 8_i32;
const CXCursor_VarDecl: c_int = 9_i32;
const CXCursor_TypedefDecl: c_int = 20_i32;
const CXCursor_InvalidFile: c_int = 70_i32;
const CXCursor_NoDeclFound: c_int = 71_i32;

const CXType_Invalid: c_int = 0_i32;
const CXType_Unexposed: c_int = 1_i32;
const CXType_Void: c_int = 2_i32;
const CXType_Bool: c_int = 3_i32;
const CXType_Char_U: c_int = 4_i32;
const CXType_UChar: c_int = 5_i32;
const CXType_Char16: c_int = 6_i32;
const CXType_Char32: c_int = 7_i32;
const CXType_UShort: c_int = 8_i32;
const CXType_UInt: c_int = 9_i32;
const CXType_ULong: c_int = 10_i32;
const CXType_ULongLong: c_int = 11_i32;
const CXType_UInt128: c_int = 12_i32;
const CXType_Char_S: c_int = 13_i32;
const CXType_SChar: c_int = 14_i32;
const CXType_WChar: c_int = 15_i32;
const CXType_Short: c_int = 16_i32;
const CXType_Int: c_int = 17_i32;
const CXType_Long: c_int = 18_i32;
const CXType_LongLong: c_int = 19_i32;
const CXType_Int128: c_int = 20_i32;
const CXType_Float: c_int = 21_i32;
const CXType_Double: c_int = 22_i32;
const CXType_LongDouble: c_int = 23_i32;
const CXType_NullPtr: c_int = 24_i32;
const CXType_Overload: c_int = 25_i32;
const CXType_Dependent: c_int = 26_i32;
const CXType_ObjCId: c_int = 27_i32;
const CXType_ObjCClass: c_int = 28_i32;
const CXType_ObjCSel: c_int = 29_i32;
const CXType_FirstBuiltin: c_int = 2_i32;
const CXType_LastBuiltin: c_int = 29_i32;
const CXType_Complex: c_int = 100_i32;
const CXType_Pointer: c_int = 101_i32;
const CXType_BlockPointer: c_int = 102_i32;
const CXType_LValueReference: c_int = 103_i32;
const CXType_RValueReference: c_int = 104_i32;
const CXType_Record: c_int = 105_i32;
const CXType_Enum: c_int = 106_i32;
const CXType_Typedef: c_int = 107_i32;
const CXType_ObjCInterface: c_int = 108_i32;
const CXType_ObjCObjectPointer: c_int = 109_i32;
const CXType_FunctionNoProto: c_int = 110_i32;
const CXType_FunctionProto: c_int = 111_i32;
const CXType_ConstantArray: c_int = 112_i32;
const CXType_Vector: c_int = 113_i32;

#[link_name="clang"]
native mod bindgen {
    fn clang_createIndex(excludeDeclarationsFromPCH: c_int,
                         displayDiagnostics: c_int) -> CXIndex;

    fn clang_disposeIndex(index: CXIndex);

    fn clang_parseTranslationUnit(CIdx: CXIndex,
                                  source_filename: sbuf,
                                  command_line_args: *sbuf,
                                  num_command_line_args: c_int,
                                  unsaved_files: *CXUnsavedFile,
                                  num_unsaved_files: unsigned,
                                  options: unsigned) -> CXTranslationUnit;

    fn clang_disposeTranslationUnit(arg0: CXTranslationUnit);

    fn clang_getNumDiagnostics(Unit: CXTranslationUnit) -> unsigned;

    fn clang_getDiagnostic(Unit: CXTranslationUnit,
                           Index: unsigned) -> CXDiagnostic;

    fn clang_formatDiagnostic(Diagnostic: CXDiagnostic,
                              Options: unsigned) -> CXString;

    fn clang_defaultDiagnosticDisplayOptions() -> unsigned;

    fn clang_getDiagnosticSeverity(arg0: CXDiagnostic) -> c_int;

    fn clang_getTranslationUnitCursor(u: CXTranslationUnit) -> CXCursor;

    fn clang_visitChildren(++parent: CXCursor,
                           visitor: CXCursorVisitor,
                           client_data: CXClientData) -> unsigned;

    fn clang_getCString(++string: CXString) -> sbuf;

    fn clang_getCursorSpelling(++arg0: CXCursor) -> CXString;

    fn clang_getResultType(++T: CXType) -> CXType;

    fn clang_getTypeDeclaration(++T: CXType) -> CXCursor;

    fn clang_isFunctionTypeVariadic(++T: CXType) -> unsigned;

    fn clang_getFunctionTypeCallingConv(++T: CXType) -> c_int;

    fn clang_getNumArgTypes(++T: CXType) -> c_int;

    fn clang_getArgType(++T: CXType, i: unsigned) -> c_int;

    fn clang_getTypeKindSpelling(K: c_int) -> CXString;

    fn clang_getCursorKindSpelling(K: c_int) -> CXString;

    fn clang_getTypedefDeclUnderlyingType(++C: CXCursor) -> CXType;

    fn clang_getElementType(++T: CXType) -> CXType;

    fn clang_getPointeeType(++T: CXType) -> CXType;

    fn clang_getArrayElementType(++T: CXType) -> CXType;

    fn clang_getArraySize(++T: CXType) -> longlong;

    fn clang_getNumElements(++T: CXType) -> longlong;

    fn clang_getCursorKind(++arg0: CXCursor) -> c_int;

    fn clang_getCursorType(++C: CXCursor) -> CXType;

    fn clang_getEnumDeclIntegerType(++C: CXCursor) -> CXType;

    fn clang_getEnumConstantDeclValue(++C: CXCursor) -> longlong;

    fn clang_getCursorDefinition(++arg0: CXCursor) -> CXCursor;

    fn clang_getSpellingLocation(++location: CXSourceLocation,
                                 file: *CXFile,
                                 line: *unsigned,
                                 column: *unsigned,
                                 offset: *unsigned);

    fn clang_getCursorLocation(++arg0: CXCursor) -> CXSourceLocation;

    fn clang_getFileName(SFile: CXFile) -> CXString;

    fn clang_getCursorResultType(++C: CXCursor) -> CXType;

    fn clang_getCanonicalType(++T: CXType) -> CXType;

    fn clang_hashCursor(++arg0: CXCursor) -> unsigned;

    fn clang_equalCursors(++arg0: CXCursor, ++arg1: CXCursor) -> unsigned;
}
