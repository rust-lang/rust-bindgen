// The functions in this file are adapted from the LLVM Project, used under the
// Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "llvm/Support/CrashRecoveryContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Comment.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/DeclObjC.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/ExprObjC.h"
#include "clang/AST/Mangle.h"
#include "clang/Basic/TargetInfo.h"
#include "clang/Basic/Version.h"
#include "clang/Frontend/ASTUnit.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/CompilerInvocation.h"
#include "clang/Lex/Lexer.h"
#include "clang/Lex/Preprocessor.h"
#include "clang-c/Documentation.h"
#include "clang-c/Index.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/Mangler.h"

#if CLANG_VERSION_MAJOR <= 8 &&                                                \
    (CLANG_VERSION_MAJOR > 3 ||                                                \
     CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR > 8)
#include "clang/Index/CodegenNameGenerator.h"
#endif // CLANG_VERSION_MAJOR <= 8

#include "clang_interface_impl.hpp"

using namespace clang;

// From CIndex.cpp
const Decl *getDeclFromExpr(const Stmt *E) {
  if (const ImplicitCastExpr *CE = dyn_cast<ImplicitCastExpr>(E))
    return getDeclFromExpr(CE->getSubExpr());

  if (const DeclRefExpr *RefExpr = dyn_cast<DeclRefExpr>(E))
    return RefExpr->getDecl();
  if (const MemberExpr *ME = dyn_cast<MemberExpr>(E))
    return ME->getMemberDecl();
  if (const ObjCIvarRefExpr *RE = dyn_cast<ObjCIvarRefExpr>(E))
    return RE->getDecl();
  if (const ObjCPropertyRefExpr *PRE = dyn_cast<ObjCPropertyRefExpr>(E)) {
    if (PRE->isExplicitProperty())
      return PRE->getExplicitProperty();
    // It could be messaging both getter and setter as in:
    // ++myobj.myprop;
    // in which case prefer to associate the setter since it is less obvious
    // from inspecting the source that the setter is going to get called.
    if (PRE->isMessagingSetter())
      return PRE->getImplicitPropertySetter();
    return PRE->getImplicitPropertyGetter();
  }
  if (const PseudoObjectExpr *POE = dyn_cast<PseudoObjectExpr>(E))
    return getDeclFromExpr(POE->getSyntacticForm());
  if (const OpaqueValueExpr *OVE = dyn_cast<OpaqueValueExpr>(E))
    if (Expr *Src = OVE->getSourceExpr())
      return getDeclFromExpr(Src);
      
  if (const CallExpr *CE = dyn_cast<CallExpr>(E))
    return getDeclFromExpr(CE->getCallee());
  if (const CXXConstructExpr *CE = dyn_cast<CXXConstructExpr>(E))
    if (!CE->isElidable())
      return CE->getConstructor();
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  if (const CXXInheritedCtorInitExpr *CE =
          dyn_cast<CXXInheritedCtorInitExpr>(E))
    return CE->getConstructor();
#endif // CLANG_VERSION > 3.8
  if (const ObjCMessageExpr *OME = dyn_cast<ObjCMessageExpr>(E))
    return OME->getMethodDecl();

  if (const ObjCProtocolExpr *PE = dyn_cast<ObjCProtocolExpr>(E))
    return PE->getProtocol();
  if (const SubstNonTypeTemplateParmPackExpr *NTTP
                              = dyn_cast<SubstNonTypeTemplateParmPackExpr>(E))
    return NTTP->getParameterPack();
  if (const SizeOfPackExpr *SizeOfPack = dyn_cast<SizeOfPackExpr>(E))
    if (isa<NonTypeTemplateParmDecl>(SizeOfPack->getPack()) || 
        isa<ParmVarDecl>(SizeOfPack->getPack()))
      return SizeOfPack->getPack();

  return nullptr;
}

// From CXType.cpp
static bool isTypeIncompleteForLayout(QualType QT) {
  return QT->isIncompleteType() && !QT->isIncompleteArrayType();
}

// Adapted from CXType.cpp
long long visitRecordForValidation(const RecordDecl *RD) {
  if (!RD)
    return -1;
  RD = RD->getDefinition();
  if (!RD || RD->isInvalidDecl() || !RD->isCompleteDefinition())
    return -1;
  for (const auto *I : RD->fields()){
    QualType FQT = I->getType();
    if (isTypeIncompleteForLayout(FQT))
      return CXTypeLayoutError_Incomplete;
    if (FQT->isDependentType())
      return CXTypeLayoutError_Dependent;
    // recurse
    if (const RecordType *ChildType = I->getType()->getAs<RecordType>()) {
      if (const RecordDecl *Child = ChildType->getDecl()) {
        long long ret = visitRecordForValidation(Child);
        if (ret < 0)
          return ret;
      }
    }
    // else try next field
  }
  if (auto *Class = dyn_cast<CXXRecordDecl>(RD)) {
    for (const CXXBaseSpecifier &Base : Class->bases()) {
      auto *baseDecl = Base.getType()->getAsCXXRecordDecl();
      long long ret = visitRecordForValidation(baseDecl);
      if (ret < 0)
        return ret;
    }
  }
  return 0;
}

// From CXType.cpp
static CXTypeKind GetBuiltinTypeKind(const BuiltinType *BT) {
#define BTCASE(K) case BuiltinType::K: return CXType_##K
  switch (BT->getKind()) {
    BTCASE(Void);
    BTCASE(Bool);
    BTCASE(Char_U);
    BTCASE(UChar);
    BTCASE(Char16);
    BTCASE(Char32);
    BTCASE(UShort);
    BTCASE(UInt);
    BTCASE(ULong);
    BTCASE(ULongLong);
    BTCASE(UInt128);
    BTCASE(Char_S);
    BTCASE(SChar);
    case BuiltinType::WChar_S: return CXType_WChar;
    case BuiltinType::WChar_U: return CXType_WChar;
    BTCASE(Short);
    BTCASE(Int);
    BTCASE(Long);
    BTCASE(LongLong);
    BTCASE(Int128);
    BTCASE(Float);
    BTCASE(Double);
    BTCASE(LongDouble);
#if CLANG_VERSION_MAJOR > 6
    BTCASE(ShortAccum);
    BTCASE(Accum);
    BTCASE(LongAccum);
    BTCASE(UShortAccum);
    BTCASE(UAccum);
    BTCASE(ULongAccum);
#endif // CLANG_VERSION_MAJOR > 6
#if CLANG_VERSION_MAJOR > 5
    BTCASE(Float16);
#endif // CLANG_VERSION_MAJOR > 5
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
    BTCASE(Float128);
#endif
    BTCASE(NullPtr);
    BTCASE(Overload);
    BTCASE(Dependent);
    BTCASE(ObjCId);
    BTCASE(ObjCClass);
    BTCASE(ObjCSel);
#if CLANG_VERSION_MAJOR > 7
#define EXT_OPAQUE_TYPE(ExtType, Id, Ext) BTCASE(Id);
#include "clang/Basic/OpenCLExtensionTypes.def"
#endif // CLANG_VERSION_MAJOR > 7
#if CLANG_VERSION_MAJOR > 4
#define IMAGE_TYPE(ImgType, Id, SingletonId, Access, Suffix) BTCASE(Id);
#include "clang/Basic/OpenCLImageTypes.def"
#undef IMAGE_TYPE
    BTCASE(Half);
    BTCASE(OCLSampler);
    BTCASE(OCLEvent);
    BTCASE(OCLQueue);
    BTCASE(OCLReserveID);
#endif // CLANG_VERSION_MAJOR > 4
  default:
    return CXType_Unexposed;
  }
#undef BTCASE
}

// Adapted from GetTypeKind in CXType.cpp
CXTypeKind Type_kind(BindgenQualType T, ASTContext *Context) {
  auto QT = QualType::getFromOpaquePtr(T);
  const Type *TP = QT.getTypePtrOrNull();
  if (!TP)
    return CXType_Invalid;

  // libclang checks for these builtin types specially and matches on things
  // that appear to be correct
#if CLANG_VERSION_MAJOR > 7
  bool isObjc = Context->getLangOpts().ObjC;
#else
  bool isObjc = Context->getLangOpts().ObjC1;
#endif // CLANG_VERSION_MAJOR
  if (isObjc) {
    QualType UT = QT.getUnqualifiedType();
    if (Context->isObjCIdType(UT))
      return CXType_ObjCId;
    if (Context->isObjCClassType(UT))
      return CXType_ObjCClass;
    if (Context->isObjCSelType(UT))
      return CXType_ObjCSel;
  }

#define TKCASE(K) case Type::K: return CXType_##K
  switch (TP->getTypeClass()) {
    case Type::Builtin:
      return GetBuiltinTypeKind(cast<BuiltinType>(TP));
    TKCASE(Complex);
    TKCASE(Pointer);
    TKCASE(BlockPointer);
    TKCASE(LValueReference);
    TKCASE(RValueReference);
    TKCASE(Record);
    TKCASE(Enum);
    TKCASE(Typedef);
    TKCASE(ObjCInterface);
    TKCASE(ObjCObjectPointer);
    TKCASE(FunctionNoProto);
    TKCASE(FunctionProto);
    TKCASE(ConstantArray);
    TKCASE(IncompleteArray);
    TKCASE(VariableArray);
    TKCASE(DependentSizedArray);
    TKCASE(Vector);
    TKCASE(MemberPointer);
    TKCASE(Auto);
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
    TKCASE(Elaborated);
#endif // CLANG_VERSION > 3.8
#if CLANG_VERSION_MAJOR > 4
    TKCASE(Pipe);
#endif // CLANG_VERSION_MAJOR > 4
#if CLANG_VERSION_MAJOR > 7
    TKCASE(Attributed);
    TKCASE(ObjCObject);
    TKCASE(ObjCTypeParam);
#endif // CLANG_VERSION_MAJOR > 7
#if CLANG_VERSION_MAJOR > 8
    TKCASE(ExtVector);
#endif // CLANG_VERSION_MAJOR > 8
    default:
      return CXType_Unexposed;
  }
#undef TKCASE
}

// From CXType.cpp
Optional<ArrayRef<TemplateArgument>>
GetTemplateArguments(QualType Type) {
  assert(!Type.isNull());
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  if (const auto *Specialization = Type->getAs<TemplateSpecializationType>())
    return Specialization->template_arguments();
#endif

  if (const auto *RecordDecl = Type->getAsCXXRecordDecl()) {
    const auto *TemplateDecl =
      dyn_cast<ClassTemplateSpecializationDecl>(RecordDecl);
    if (TemplateDecl)
      return TemplateDecl->getTemplateArgs().asArray();
  }

  return None;
}

// From CXType.cpp
unsigned GetTemplateArgumentArraySize(ArrayRef<TemplateArgument> TA) {
  unsigned size = TA.size();
  for (const auto &Arg : TA)
    if (Arg.getKind() == TemplateArgument::Pack)
      size += Arg.pack_size() - 1;
  return size;
}

// From CXType.cpp
static Optional<QualType> TemplateArgumentToQualType(const TemplateArgument &A) {
  if (A.getKind() == TemplateArgument::Type)
    return A.getAsType();
  return None;
}

// From CXType.cpp
Optional<QualType>
FindTemplateArgumentTypeAt(ArrayRef<TemplateArgument> TA, unsigned index) {
  unsigned current = 0;
  for (const auto &A : TA) {
    if (A.getKind() == TemplateArgument::Pack) {
      if (index < current + A.pack_size())
        return TemplateArgumentToQualType(A.getPackAsArray()[index - current]);
      current += A.pack_size();
      continue;
    }
    if (current == index)
      return TemplateArgumentToQualType(A);
    current++;
  }
  return None;
}

// Adapted from CXSourceLocation.cpp
static void createNullLocation(FileEntry **file, int *line,
                               int *column, int *offset = nullptr) {
  if (file)
    *file = nullptr;
  if (line)
    *line = 0;
  if (column)
    *column = 0;
  if (offset)
    *offset = 0;
}

// Adapted from clang_getCursorKindSpelling in CIndex.cpp
BindgenStringRef CursorKind_getSpelling(CXCursorKind Kind) {
  switch (Kind) {
  case CXCursor_FunctionDecl:
      return stringref("FunctionDecl");
  case CXCursor_TypedefDecl:
      return stringref("TypedefDecl");
  case CXCursor_EnumDecl:
      return stringref("EnumDecl");
  case CXCursor_EnumConstantDecl:
      return stringref("EnumConstantDecl");
  case CXCursor_StructDecl:
      return stringref("StructDecl");
  case CXCursor_UnionDecl:
      return stringref("UnionDecl");
  case CXCursor_ClassDecl:
      return stringref("ClassDecl");
  case CXCursor_FieldDecl:
      return stringref("FieldDecl");
  case CXCursor_VarDecl:
      return stringref("VarDecl");
  case CXCursor_ParmDecl:
      return stringref("ParmDecl");
  case CXCursor_ObjCInterfaceDecl:
      return stringref("ObjCInterfaceDecl");
  case CXCursor_ObjCCategoryDecl:
      return stringref("ObjCCategoryDecl");
  case CXCursor_ObjCProtocolDecl:
      return stringref("ObjCProtocolDecl");
  case CXCursor_ObjCPropertyDecl:
      return stringref("ObjCPropertyDecl");
  case CXCursor_ObjCIvarDecl:
      return stringref("ObjCIvarDecl");
  case CXCursor_ObjCInstanceMethodDecl:
      return stringref("ObjCInstanceMethodDecl");
  case CXCursor_ObjCClassMethodDecl:
      return stringref("ObjCClassMethodDecl");
  case CXCursor_ObjCImplementationDecl:
      return stringref("ObjCImplementationDecl");
  case CXCursor_ObjCCategoryImplDecl:
      return stringref("ObjCCategoryImplDecl");
  case CXCursor_CXXMethod:
      return stringref("CXXMethod");
  case CXCursor_UnexposedDecl:
      return stringref("UnexposedDecl");
  case CXCursor_ObjCSuperClassRef:
      return stringref("ObjCSuperClassRef");
  case CXCursor_ObjCProtocolRef:
      return stringref("ObjCProtocolRef");
  case CXCursor_ObjCClassRef:
      return stringref("ObjCClassRef");
  case CXCursor_TypeRef:
      return stringref("TypeRef");
  case CXCursor_TemplateRef:
      return stringref("TemplateRef");
  case CXCursor_NamespaceRef:
    return stringref("NamespaceRef");
  case CXCursor_MemberRef:
    return stringref("MemberRef");
  case CXCursor_LabelRef:
    return stringref("LabelRef");
  case CXCursor_OverloadedDeclRef:
    return stringref("OverloadedDeclRef");
  case CXCursor_VariableRef:
    return stringref("VariableRef");
  case CXCursor_IntegerLiteral:
      return stringref("IntegerLiteral");
#if CLANG_VERSION_MAJOR > 6
  case CXCursor_FixedPointLiteral:
      return stringref("FixedPointLiteral");
#endif // CLANG_VERSION_MAJOR > 6
  case CXCursor_FloatingLiteral:
      return stringref("FloatingLiteral");
  case CXCursor_ImaginaryLiteral:
      return stringref("ImaginaryLiteral");
  case CXCursor_StringLiteral:
      return stringref("StringLiteral");
  case CXCursor_CharacterLiteral:
      return stringref("CharacterLiteral");
  case CXCursor_ParenExpr:
      return stringref("ParenExpr");
  case CXCursor_UnaryOperator:
      return stringref("UnaryOperator");
  case CXCursor_ArraySubscriptExpr:
      return stringref("ArraySubscriptExpr");
  case CXCursor_OMPArraySectionExpr:
      return stringref("OMPArraySectionExpr");
  case CXCursor_BinaryOperator:
      return stringref("BinaryOperator");
  case CXCursor_CompoundAssignOperator:
      return stringref("CompoundAssignOperator");
  case CXCursor_ConditionalOperator:
      return stringref("ConditionalOperator");
  case CXCursor_CStyleCastExpr:
      return stringref("CStyleCastExpr");
  case CXCursor_CompoundLiteralExpr:
      return stringref("CompoundLiteralExpr");
  case CXCursor_InitListExpr:
      return stringref("InitListExpr");
  case CXCursor_AddrLabelExpr:
      return stringref("AddrLabelExpr");
  case CXCursor_StmtExpr:
      return stringref("StmtExpr");
  case CXCursor_GenericSelectionExpr:
      return stringref("GenericSelectionExpr");
  case CXCursor_GNUNullExpr:
      return stringref("GNUNullExpr");
  case CXCursor_CXXStaticCastExpr:
      return stringref("CXXStaticCastExpr");
  case CXCursor_CXXDynamicCastExpr:
      return stringref("CXXDynamicCastExpr");
  case CXCursor_CXXReinterpretCastExpr:
      return stringref("CXXReinterpretCastExpr");
  case CXCursor_CXXConstCastExpr:
      return stringref("CXXConstCastExpr");
  case CXCursor_CXXFunctionalCastExpr:
      return stringref("CXXFunctionalCastExpr");
  case CXCursor_CXXTypeidExpr:
      return stringref("CXXTypeidExpr");
  case CXCursor_CXXBoolLiteralExpr:
      return stringref("CXXBoolLiteralExpr");
  case CXCursor_CXXNullPtrLiteralExpr:
      return stringref("CXXNullPtrLiteralExpr");
  case CXCursor_CXXThisExpr:
      return stringref("CXXThisExpr");
  case CXCursor_CXXThrowExpr:
      return stringref("CXXThrowExpr");
  case CXCursor_CXXNewExpr:
      return stringref("CXXNewExpr");
  case CXCursor_CXXDeleteExpr:
      return stringref("CXXDeleteExpr");
  case CXCursor_UnaryExpr:
      return stringref("UnaryExpr");
  case CXCursor_ObjCStringLiteral:
      return stringref("ObjCStringLiteral");
  case CXCursor_ObjCBoolLiteralExpr:
      return stringref("ObjCBoolLiteralExpr");
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case CXCursor_ObjCAvailabilityCheckExpr:
      return stringref("ObjCAvailabilityCheckExpr");
#endif
  case CXCursor_ObjCSelfExpr:
      return stringref("ObjCSelfExpr");
  case CXCursor_ObjCEncodeExpr:
      return stringref("ObjCEncodeExpr");
  case CXCursor_ObjCSelectorExpr:
      return stringref("ObjCSelectorExpr");
  case CXCursor_ObjCProtocolExpr:
      return stringref("ObjCProtocolExpr");
  case CXCursor_ObjCBridgedCastExpr:
      return stringref("ObjCBridgedCastExpr");
  case CXCursor_BlockExpr:
      return stringref("BlockExpr");
  case CXCursor_PackExpansionExpr:
      return stringref("PackExpansionExpr");
  case CXCursor_SizeOfPackExpr:
      return stringref("SizeOfPackExpr");
  case CXCursor_LambdaExpr:
    return stringref("LambdaExpr");
  case CXCursor_UnexposedExpr:
      return stringref("UnexposedExpr");
  case CXCursor_DeclRefExpr:
      return stringref("DeclRefExpr");
  case CXCursor_MemberRefExpr:
      return stringref("MemberRefExpr");
  case CXCursor_CallExpr:
      return stringref("CallExpr");
  case CXCursor_ObjCMessageExpr:
      return stringref("ObjCMessageExpr");
#if CLANG_VERSION_MAJOR > 8
  case CXCursor_BuiltinBitCastExpr:
    return stringref("BuiltinBitCastExpr");
#endif // CLANG_VERSION_MAJOR > 8
  case CXCursor_UnexposedStmt:
      return stringref("UnexposedStmt");
  case CXCursor_DeclStmt:
      return stringref("DeclStmt");
  case CXCursor_LabelStmt:
      return stringref("LabelStmt");
  case CXCursor_CompoundStmt:
      return stringref("CompoundStmt");
  case CXCursor_CaseStmt:
      return stringref("CaseStmt");
  case CXCursor_DefaultStmt:
      return stringref("DefaultStmt");
  case CXCursor_IfStmt:
      return stringref("IfStmt");
  case CXCursor_SwitchStmt:
      return stringref("SwitchStmt");
  case CXCursor_WhileStmt:
      return stringref("WhileStmt");
  case CXCursor_DoStmt:
      return stringref("DoStmt");
  case CXCursor_ForStmt:
      return stringref("ForStmt");
  case CXCursor_GotoStmt:
      return stringref("GotoStmt");
  case CXCursor_IndirectGotoStmt:
      return stringref("IndirectGotoStmt");
  case CXCursor_ContinueStmt:
      return stringref("ContinueStmt");
  case CXCursor_BreakStmt:
      return stringref("BreakStmt");
  case CXCursor_ReturnStmt:
      return stringref("ReturnStmt");
  case CXCursor_GCCAsmStmt:
      return stringref("GCCAsmStmt");
  case CXCursor_MSAsmStmt:
      return stringref("MSAsmStmt");
  case CXCursor_ObjCAtTryStmt:
      return stringref("ObjCAtTryStmt");
  case CXCursor_ObjCAtCatchStmt:
      return stringref("ObjCAtCatchStmt");
  case CXCursor_ObjCAtFinallyStmt:
      return stringref("ObjCAtFinallyStmt");
  case CXCursor_ObjCAtThrowStmt:
      return stringref("ObjCAtThrowStmt");
  case CXCursor_ObjCAtSynchronizedStmt:
      return stringref("ObjCAtSynchronizedStmt");
  case CXCursor_ObjCAutoreleasePoolStmt:
      return stringref("ObjCAutoreleasePoolStmt");
  case CXCursor_ObjCForCollectionStmt:
      return stringref("ObjCForCollectionStmt");
  case CXCursor_CXXCatchStmt:
      return stringref("CXXCatchStmt");
  case CXCursor_CXXTryStmt:
      return stringref("CXXTryStmt");
  case CXCursor_CXXForRangeStmt:
      return stringref("CXXForRangeStmt");
  case CXCursor_SEHTryStmt:
      return stringref("SEHTryStmt");
  case CXCursor_SEHExceptStmt:
      return stringref("SEHExceptStmt");
  case CXCursor_SEHFinallyStmt:
      return stringref("SEHFinallyStmt");
  case CXCursor_SEHLeaveStmt:
      return stringref("SEHLeaveStmt");
  case CXCursor_NullStmt:
      return stringref("NullStmt");
  case CXCursor_InvalidFile:
      return stringref("InvalidFile");
  case CXCursor_InvalidCode:
    return stringref("InvalidCode");
  case CXCursor_NoDeclFound:
      return stringref("NoDeclFound");
  case CXCursor_NotImplemented:
      return stringref("NotImplemented");
  case CXCursor_TranslationUnit:
      return stringref("TranslationUnit");
  case CXCursor_UnexposedAttr:
      return stringref("UnexposedAttr");
  case CXCursor_IBActionAttr:
      return stringref("attribute(ibaction)");
  case CXCursor_IBOutletAttr:
     return stringref("attribute(iboutlet)");
  case CXCursor_IBOutletCollectionAttr:
      return stringref("attribute(iboutletcollection)");
  case CXCursor_CXXFinalAttr:
      return stringref("attribute(final)");
  case CXCursor_CXXOverrideAttr:
      return stringref("attribute(override)");
  case CXCursor_AnnotateAttr:
    return stringref("attribute(annotate)");
  case CXCursor_AsmLabelAttr:
    return stringref("asm label");
  case CXCursor_PackedAttr:
    return stringref("attribute(packed)");
  case CXCursor_PureAttr:
    return stringref("attribute(pure)");
  case CXCursor_ConstAttr:
    return stringref("attribute(const)");
  case CXCursor_NoDuplicateAttr:
    return stringref("attribute(noduplicate)");
  case CXCursor_CUDAConstantAttr:
    return stringref("attribute(constant)");
  case CXCursor_CUDADeviceAttr:
    return stringref("attribute(device)");
  case CXCursor_CUDAGlobalAttr:
    return stringref("attribute(global)");
  case CXCursor_CUDAHostAttr:
    return stringref("attribute(host)");
  case CXCursor_CUDASharedAttr:
    return stringref("attribute(shared)");
  case CXCursor_VisibilityAttr:
    return stringref("attribute(visibility)");
  case CXCursor_DLLExport:
    return stringref("attribute(dllexport)");
  case CXCursor_DLLImport:
    return stringref("attribute(dllimport)");
#if CLANG_VERSION_MAJOR > 7
  case CXCursor_NSReturnsRetained:
    return stringref("attribute(ns_returns_retained)");
  case CXCursor_NSReturnsNotRetained:
    return stringref("attribute(ns_returns_not_retained)");
  case CXCursor_NSReturnsAutoreleased:
    return stringref("attribute(ns_returns_autoreleased)");
  case CXCursor_NSConsumesSelf:
    return stringref("attribute(ns_consumes_self)");
  case CXCursor_NSConsumed:
    return stringref("attribute(ns_consumed)");
  case CXCursor_ObjCException:
    return stringref("attribute(objc_exception)");
  case CXCursor_ObjCNSObject:
    return stringref("attribute(NSObject)");
  case CXCursor_ObjCIndependentClass:
    return stringref("attribute(objc_independent_class)");
  case CXCursor_ObjCPreciseLifetime:
    return stringref("attribute(objc_precise_lifetime)");
  case CXCursor_ObjCReturnsInnerPointer:
    return stringref("attribute(objc_returns_inner_pointer)");
  case CXCursor_ObjCRequiresSuper:
    return stringref("attribute(objc_requires_super)");
  case CXCursor_ObjCRootClass:
    return stringref("attribute(objc_root_class)");
  case CXCursor_ObjCSubclassingRestricted:
    return stringref("attribute(objc_subclassing_restricted)");
  case CXCursor_ObjCExplicitProtocolImpl:
    return stringref("attribute(objc_protocol_requires_explicit_implementation)");
  case CXCursor_ObjCDesignatedInitializer:
    return stringref("attribute(objc_designated_initializer)");
  case CXCursor_ObjCRuntimeVisible:
    return stringref("attribute(objc_runtime_visible)");
  case CXCursor_ObjCBoxable:
    return stringref("attribute(objc_boxable)");
  case CXCursor_FlagEnum:
    return stringref("attribute(flag_enum)");
#endif // CLANG_VERSION_MAJOR > 7
  case CXCursor_PreprocessingDirective:
    return stringref("preprocessing directive");
  case CXCursor_MacroDefinition:
    return stringref("macro definition");
  case CXCursor_MacroExpansion:
    return stringref("macro expansion");
  case CXCursor_InclusionDirective:
    return stringref("inclusion directive");
  case CXCursor_Namespace:
    return stringref("Namespace");
  case CXCursor_LinkageSpec:
    return stringref("LinkageSpec");
  case CXCursor_CXXBaseSpecifier:
    return stringref("C++ base class specifier");
  case CXCursor_Constructor:
    return stringref("CXXConstructor");
  case CXCursor_Destructor:
    return stringref("CXXDestructor");
  case CXCursor_ConversionFunction:
    return stringref("CXXConversion");
  case CXCursor_TemplateTypeParameter:
    return stringref("TemplateTypeParameter");
  case CXCursor_NonTypeTemplateParameter:
    return stringref("NonTypeTemplateParameter");
  case CXCursor_TemplateTemplateParameter:
    return stringref("TemplateTemplateParameter");
  case CXCursor_FunctionTemplate:
    return stringref("FunctionTemplate");
  case CXCursor_ClassTemplate:
    return stringref("ClassTemplate");
  case CXCursor_ClassTemplatePartialSpecialization:
    return stringref("ClassTemplatePartialSpecialization");
  case CXCursor_NamespaceAlias:
    return stringref("NamespaceAlias");
  case CXCursor_UsingDirective:
    return stringref("UsingDirective");
  case CXCursor_UsingDeclaration:
    return stringref("UsingDeclaration");
  case CXCursor_TypeAliasDecl:
    return stringref("TypeAliasDecl");
  case CXCursor_ObjCSynthesizeDecl:
    return stringref("ObjCSynthesizeDecl");
  case CXCursor_ObjCDynamicDecl:
    return stringref("ObjCDynamicDecl");
  case CXCursor_CXXAccessSpecifier:
    return stringref("CXXAccessSpecifier");
  case CXCursor_ModuleImportDecl:
    return stringref("ModuleImport");
  case CXCursor_OMPParallelDirective:
    return stringref("OMPParallelDirective");
  case CXCursor_OMPSimdDirective:
    return stringref("OMPSimdDirective");
  case CXCursor_OMPForDirective:
    return stringref("OMPForDirective");
  case CXCursor_OMPForSimdDirective:
    return stringref("OMPForSimdDirective");
  case CXCursor_OMPSectionsDirective:
    return stringref("OMPSectionsDirective");
  case CXCursor_OMPSectionDirective:
    return stringref("OMPSectionDirective");
  case CXCursor_OMPSingleDirective:
    return stringref("OMPSingleDirective");
  case CXCursor_OMPMasterDirective:
    return stringref("OMPMasterDirective");
  case CXCursor_OMPCriticalDirective:
    return stringref("OMPCriticalDirective");
  case CXCursor_OMPParallelForDirective:
    return stringref("OMPParallelForDirective");
  case CXCursor_OMPParallelForSimdDirective:
    return stringref("OMPParallelForSimdDirective");
  case CXCursor_OMPParallelSectionsDirective:
    return stringref("OMPParallelSectionsDirective");
  case CXCursor_OMPTaskDirective:
    return stringref("OMPTaskDirective");
  case CXCursor_OMPTaskyieldDirective:
    return stringref("OMPTaskyieldDirective");
  case CXCursor_OMPBarrierDirective:
    return stringref("OMPBarrierDirective");
  case CXCursor_OMPTaskwaitDirective:
    return stringref("OMPTaskwaitDirective");
  case CXCursor_OMPTaskgroupDirective:
    return stringref("OMPTaskgroupDirective");
  case CXCursor_OMPFlushDirective:
    return stringref("OMPFlushDirective");
  case CXCursor_OMPOrderedDirective:
    return stringref("OMPOrderedDirective");
  case CXCursor_OMPAtomicDirective:
    return stringref("OMPAtomicDirective");
  case CXCursor_OMPTargetDirective:
    return stringref("OMPTargetDirective");
  case CXCursor_OMPTargetDataDirective:
    return stringref("OMPTargetDataDirective");
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case CXCursor_OMPTargetEnterDataDirective:
    return stringref("OMPTargetEnterDataDirective");
  case CXCursor_OMPTargetExitDataDirective:
    return stringref("OMPTargetExitDataDirective");
  case CXCursor_OMPTargetParallelDirective:
    return stringref("OMPTargetParallelDirective");
  case CXCursor_OMPTargetParallelForDirective:
    return stringref("OMPTargetParallelForDirective");
  case CXCursor_OMPTargetUpdateDirective:
    return stringref("OMPTargetUpdateDirective");
#endif
  case CXCursor_OMPTeamsDirective:
    return stringref("OMPTeamsDirective");
  case CXCursor_OMPCancellationPointDirective:
    return stringref("OMPCancellationPointDirective");
  case CXCursor_OMPCancelDirective:
    return stringref("OMPCancelDirective");
  case CXCursor_OMPTaskLoopDirective:
    return stringref("OMPTaskLoopDirective");
  case CXCursor_OMPTaskLoopSimdDirective:
    return stringref("OMPTaskLoopSimdDirective");
  case CXCursor_OMPDistributeDirective:
    return stringref("OMPDistributeDirective");
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case CXCursor_OMPDistributeParallelForDirective:
    return stringref("OMPDistributeParallelForDirective");
  case CXCursor_OMPDistributeParallelForSimdDirective:
    return stringref("OMPDistributeParallelForSimdDirective");
  case CXCursor_OMPDistributeSimdDirective:
    return stringref("OMPDistributeSimdDirective");
  case CXCursor_OMPTargetParallelForSimdDirective:
    return stringref("OMPTargetParallelForSimdDirective");
#endif
#if CLANG_VERSION_MAJOR > 3
  case CXCursor_OMPTargetSimdDirective:
    return stringref("OMPTargetSimdDirective");
  case CXCursor_OMPTeamsDistributeDirective:
    return stringref("OMPTeamsDistributeDirective");
  case CXCursor_OMPTeamsDistributeSimdDirective:
    return stringref("OMPTeamsDistributeSimdDirective");
  case CXCursor_OMPTeamsDistributeParallelForSimdDirective:
    return stringref("OMPTeamsDistributeParallelForSimdDirective");
  case CXCursor_OMPTeamsDistributeParallelForDirective:
    return stringref("OMPTeamsDistributeParallelForDirective");
  case CXCursor_OMPTargetTeamsDirective:
    return stringref("OMPTargetTeamsDirective");
  case CXCursor_OMPTargetTeamsDistributeDirective:
    return stringref("OMPTargetTeamsDistributeDirective");
  case CXCursor_OMPTargetTeamsDistributeParallelForDirective:
    return stringref("OMPTargetTeamsDistributeParallelForDirective");
  case CXCursor_OMPTargetTeamsDistributeParallelForSimdDirective:
    return stringref(
        "OMPTargetTeamsDistributeParallelForSimdDirective");
  case CXCursor_OMPTargetTeamsDistributeSimdDirective:
    return stringref("OMPTargetTeamsDistributeSimdDirective");
  case CXCursor_FriendDecl:
      return stringref("FriendDecl");
#endif // CLANG_VERSION_MAJOR > 3
  case CXCursor_OverloadCandidate:
      return stringref("OverloadCandidate");
  case CXCursor_TypeAliasTemplateDecl:
      return stringref("TypeAliasTemplateDecl");
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case CXCursor_StaticAssert:
      return stringref("StaticAssert");
#endif
#if CLANG_VERSION_MAJOR > 8
  case CXCursor_ConvergentAttr:
      return stringref("attribute(convergent)");
  case CXCursor_WarnUnusedAttr:
      return stringref("attribute(warn_unused)");
  case CXCursor_WarnUnusedResultAttr:
      return stringref("attribute(warn_unused_result)");
  case CXCursor_AlignedAttr:
      return stringref("attribute(aligned)");
#endif // CLANG_VERSION_MAJOR > 8
  }

  llvm_unreachable("Unhandled CXCursorKind");
}

// Adapted from clang_getTypeKindSpelling in CXType.cpp
BindgenStringRef TypeKind_getSpelling(CXTypeKind K) {
  const char *s = nullptr;
#define TKIND(X) case CXType_##X: s = ""  #X  ""; break
  switch (K) {
    TKIND(Invalid);
    TKIND(Unexposed);
    TKIND(Void);
    TKIND(Bool);
    TKIND(Char_U);
    TKIND(UChar);
    TKIND(Char16);
    TKIND(Char32);
    TKIND(UShort);
    TKIND(UInt);
    TKIND(ULong);
    TKIND(ULongLong);
    TKIND(UInt128);
    TKIND(Char_S);
    TKIND(SChar);
    case CXType_WChar: s = "WChar"; break;
    TKIND(Short);
    TKIND(Int);
    TKIND(Long);
    TKIND(LongLong);
    TKIND(Int128);
    TKIND(Float);
    TKIND(Double);
    TKIND(LongDouble);
#if CLANG_VERSION_MAJOR > 6
    TKIND(ShortAccum);
    TKIND(Accum);
    TKIND(LongAccum);
    TKIND(UShortAccum);
    TKIND(UAccum);
    TKIND(ULongAccum);
#endif // CLANG_VERSION_MAJOR > 6
#if CLANG_VERSION_MAJOR > 5
    TKIND(Float16);
#endif // CLANG_VERSION_MAJOR > 5
    TKIND(NullPtr);
    TKIND(Overload);
    TKIND(Dependent);
    TKIND(ObjCId);
    TKIND(ObjCClass);
    TKIND(ObjCSel);
    TKIND(Complex);
    TKIND(Pointer);
    TKIND(BlockPointer);
    TKIND(LValueReference);
    TKIND(RValueReference);
    TKIND(Record);
    TKIND(Enum);
    TKIND(Typedef);
    TKIND(ObjCInterface);
    TKIND(ObjCObjectPointer);
    TKIND(FunctionNoProto);
    TKIND(FunctionProto);
    TKIND(ConstantArray);
    TKIND(IncompleteArray);
    TKIND(VariableArray);
    TKIND(DependentSizedArray);
    TKIND(Vector);
    TKIND(MemberPointer);
    TKIND(Auto);
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
    TKIND(Float128);
    TKIND(Elaborated);
#endif
#if CLANG_VERSION_MAJOR > 7
    TKIND(Attributed);
    TKIND(ObjCObject);
    TKIND(ObjCTypeParam);
#define EXT_OPAQUE_TYPE(ExtTYpe, Id, Ext) TKIND(Id);
#include "clang/Basic/OpenCLExtensionTypes.def"
#endif // CLANG_VERSION_MAJOR > 7
#if CLANG_VERSION_MAJOR > 8
    TKIND(ExtVector);
#endif // CLANG_VERSION_MAJOR > 8
#if CLANG_VERSION_MAJOR > 4
#define IMAGE_TYPE(ImgType, Id, SingletonId, Access, Suffix) TKIND(Id);
#include "clang/Basic/OpenCLImageTypes.def"
#undef IMAGE_TYPE
    TKIND(Half);
    TKIND(Pipe);
    TKIND(OCLSampler);
    TKIND(OCLEvent);
    TKIND(OCLQueue);
    TKIND(OCLReserveID);
#endif // CLANG_VERSION_MAJOR > 4
  }
#undef TKIND
  return stringref(s);
}

// From GetCursorKind in CXCursor.cpp
CXCursorKind Attr_getCXCursorKind(const Attr *A) {
  assert(A && "Invalid arguments!");
  switch (A->getKind()) {
    default: break;
    case attr::IBAction: return CXCursor_IBActionAttr;
    case attr::IBOutlet: return CXCursor_IBOutletAttr;
    case attr::IBOutletCollection: return CXCursor_IBOutletCollectionAttr;
    case attr::Final: return CXCursor_CXXFinalAttr;
    case attr::Override: return CXCursor_CXXOverrideAttr;
    case attr::Annotate: return CXCursor_AnnotateAttr;
    case attr::AsmLabel: return CXCursor_AsmLabelAttr;
    case attr::Packed: return CXCursor_PackedAttr;
    case attr::Pure: return CXCursor_PureAttr;
    case attr::Const: return CXCursor_ConstAttr;
    case attr::NoDuplicate: return CXCursor_NoDuplicateAttr;
    case attr::CUDAConstant: return CXCursor_CUDAConstantAttr;
    case attr::CUDADevice: return CXCursor_CUDADeviceAttr;
    case attr::CUDAGlobal: return CXCursor_CUDAGlobalAttr;
    case attr::CUDAHost: return CXCursor_CUDAHostAttr;
    case attr::CUDAShared: return CXCursor_CUDASharedAttr;
    case attr::Visibility: return CXCursor_VisibilityAttr;
    case attr::DLLExport: return CXCursor_DLLExport;
    case attr::DLLImport: return CXCursor_DLLImport;
#if CLANG_VERSION_MAJOR > 7
    case attr::NSReturnsRetained: return CXCursor_NSReturnsRetained;
    case attr::NSReturnsNotRetained: return CXCursor_NSReturnsNotRetained;
    case attr::NSReturnsAutoreleased: return CXCursor_NSReturnsAutoreleased;
    case attr::NSConsumesSelf: return CXCursor_NSConsumesSelf;
    case attr::NSConsumed: return CXCursor_NSConsumed;
    case attr::ObjCException: return CXCursor_ObjCException;
    case attr::ObjCNSObject: return CXCursor_ObjCNSObject;
    case attr::ObjCIndependentClass: return CXCursor_ObjCIndependentClass;
    case attr::ObjCPreciseLifetime: return CXCursor_ObjCPreciseLifetime;
    case attr::ObjCReturnsInnerPointer: return CXCursor_ObjCReturnsInnerPointer;
    case attr::ObjCRequiresSuper: return CXCursor_ObjCRequiresSuper;
    case attr::ObjCRootClass: return CXCursor_ObjCRootClass;
    case attr::ObjCSubclassingRestricted: return CXCursor_ObjCSubclassingRestricted;
    case attr::ObjCExplicitProtocolImpl: return CXCursor_ObjCExplicitProtocolImpl;
    case attr::ObjCDesignatedInitializer: return CXCursor_ObjCDesignatedInitializer;
    case attr::ObjCRuntimeVisible: return CXCursor_ObjCRuntimeVisible;
    case attr::ObjCBoxable: return CXCursor_ObjCBoxable;
    case attr::FlagEnum: return CXCursor_FlagEnum;
#endif // CLANG_VERSION_MAJOR > 7
#if CLANG_VERSION_MAJOR > 8
    case attr::Convergent: return CXCursor_ConvergentAttr;
    case attr::WarnUnused: return CXCursor_WarnUnusedAttr;
    case attr::WarnUnusedResult: return CXCursor_WarnUnusedResultAttr;
    case attr::Aligned: return CXCursor_AlignedAttr;
#endif // CLANG_VERSION_MAJOR > 8
  }

  return CXCursor_UnexposedAttr;
}

// Adapted from clang_Comment_getKind in CXComment.cpp
CXCommentKind Comment_getKind(const comments::Comment *C) {
  using namespace comments;
  if (!C)
    return CXComment_Null;

  switch (C->getCommentKind()) {
  case Comment::NoCommentKind:
    return CXComment_Null;

  case Comment::TextCommentKind:
    return CXComment_Text;

  case Comment::InlineCommandCommentKind:
    return CXComment_InlineCommand;

  case Comment::HTMLStartTagCommentKind:
    return CXComment_HTMLStartTag;

  case Comment::HTMLEndTagCommentKind:
    return CXComment_HTMLEndTag;

  case Comment::ParagraphCommentKind:
    return CXComment_Paragraph;

  case Comment::BlockCommandCommentKind:
    return CXComment_BlockCommand;

  case Comment::ParamCommandCommentKind:
    return CXComment_ParamCommand;

  case Comment::TParamCommandCommentKind:
    return CXComment_TParamCommand;

  case Comment::VerbatimBlockCommentKind:
    return CXComment_VerbatimBlockCommand;

  case Comment::VerbatimBlockLineCommentKind:
    return CXComment_VerbatimBlockLine;

  case Comment::VerbatimLineCommentKind:
    return CXComment_VerbatimLine;

  case Comment::FullCommentKind:
    return CXComment_FullComment;
  }
  llvm_unreachable("unknown CommentKind");
}

// Adapted from clang_parseTranslationUnit_Impl in CIndex.cpp
ASTUnit *parseTranslationUnit(const char *source_filename,
                              const char *const *command_line_args,
                              int num_command_line_args, unsigned int options,
                              struct CXUnsavedFile *unsaved_files,
                              unsigned num_unsaved_files) {
  SmallVector<const char *, 10> Args;
  Args.push_back("clang");
  Args.append(command_line_args, command_line_args + num_command_line_args);

  std::unique_ptr<std::vector<ASTUnit::RemappedFile>> RemappedFiles(
      new std::vector<ASTUnit::RemappedFile>());
  // Recover resources if we crash before exiting this function.
  llvm::CrashRecoveryContextCleanupRegistrar<
    std::vector<ASTUnit::RemappedFile> > RemappedCleanup(RemappedFiles.get());

  for (auto &UF : llvm::makeArrayRef(unsaved_files, num_unsaved_files)) {
    std::unique_ptr<llvm::MemoryBuffer> MB =
      llvm::MemoryBuffer::getMemBufferCopy(StringRef(UF.Contents, UF.Length), UF.Filename);
    RemappedFiles->push_back(std::make_pair(UF.Filename, MB.release()));
  }

  // Configure the diagnostics.
  IntrusiveRefCntPtr<DiagnosticsEngine>
    Diags(CompilerInstance::createDiagnostics(new DiagnosticOptions));

#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  if (options & CXTranslationUnit_KeepGoing) {
#if CLANG_VERSION_MAJOR > 8
    Diags->setFatalsAsError(true);
#elif CLANG_VERSION_MAJOR > 4
    Diags->setSuppressAfterFatalError(false);
#else // CLANG_VERSION_MAJOR <= 4
    Diags->setFatalsAsError(true);
#endif
  }
#endif // CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)

#if CLANG_VERSION_MAJOR > 8
  CaptureDiagsKind CaptureDiagnostics = CaptureDiagsKind::All;
  if (options & CXTranslationUnit_IgnoreNonErrorsFromIncludedFiles)
    CaptureDiagnostics = CaptureDiagsKind::AllWithoutNonErrorsFromIncludes;
#endif // CLANG_VERSION_MAJOR > 8

  if (options & CXTranslationUnit_DetailedPreprocessingRecord) {
    // Tell the preprocessor to save a detailed preprocessing record
    Args.push_back("-Xclang");
    Args.push_back("-detailed-preprocessing-record");
  }

  return ASTUnit::LoadFromCommandLine(
      Args.data(), Args.data() + Args.size(),
      std::make_shared<PCHContainerOperations>(), Diags,
      /*ResourceFilePath*/ StringRef(),
      /*OnlyLocalDecls*/ false,
#if CLANG_VERSION_MAJOR > 8
      CaptureDiagnostics,
#else
      /*CaptureDiagnostics*/true,
#endif // CLANG_VERSION_MAJOR > 8
      *RemappedFiles.get(),
      /*RemappedFilesKeepOriginalName*/ true);
}

// Adapted from CXStoredDiagnostic::getSeverity in CXStoredDiagnostic.cpp
CXDiagnosticSeverity Diagnostic_getSeverity(const StoredDiagnostic *D) {
  switch (D->getLevel()) {
    case DiagnosticsEngine::Ignored: return CXDiagnostic_Ignored;
    case DiagnosticsEngine::Note:    return CXDiagnostic_Note;
    case DiagnosticsEngine::Remark:
    // The 'Remark' level isn't represented in the stable API.
    case DiagnosticsEngine::Warning: return CXDiagnostic_Warning;
    case DiagnosticsEngine::Error:   return CXDiagnostic_Error;
    case DiagnosticsEngine::Fatal:   return CXDiagnostic_Fatal;
  }
  
  llvm_unreachable("Invalid diagnostic level");
}

// Adapted from clang_getCursorDefinition in CIndex.cpp
const Decl *Decl_getDefinition(const Decl *D, bool isReference) {
  if (!D)
    return nullptr;

  switch (D->getKind()) {
  // Declaration kinds that don't really separate the notions of
  // declaration and definition.
  case Decl::Namespace:
  case Decl::Typedef:
  case Decl::TypeAlias:
  case Decl::TypeAliasTemplate:
  case Decl::TemplateTypeParm:
  case Decl::EnumConstant:
  case Decl::Field:
  case Decl::MSProperty:
  case Decl::IndirectField:
  case Decl::ObjCIvar:
  case Decl::ObjCAtDefsField:
  case Decl::ImplicitParam:
  case Decl::ParmVar:
  case Decl::NonTypeTemplateParm:
  case Decl::TemplateTemplateParm:
  case Decl::ObjCCategoryImpl:
  case Decl::ObjCImplementation:
  case Decl::AccessSpec:
  case Decl::LinkageSpec:
  case Decl::ObjCPropertyImpl:
  case Decl::FileScopeAsm:
  case Decl::StaticAssert:
  case Decl::Block:
  case Decl::Captured:
  case Decl::Label: // FIXME: Is this right??
  case Decl::ClassScopeFunctionSpecialization:
#if CLANG_VERSION_MAJOR > 3
  case Decl::Binding:
  case Decl::Export:
  case Decl::UsingPack:
#endif // CLANG_VERSION_MAJOR > 3
#if CLANG_VERSION_MAJOR > 4
  case Decl::CXXDeductionGuide:
#endif // CLANG_VERSION_MAJOR > 4
  case Decl::Import:
  case Decl::OMPThreadPrivate:
#if CLANG_VERSION_MAJOR > 8
  case Decl::OMPAllocate:
  case Decl::OMPDeclareMapper:
  case Decl::Concept:
#endif // CLANG_VERSION_MAJOR > 8
#if CLANG_VERSION_MAJOR > 7
  case Decl::OMPRequires:
#endif // CLANG_VERSION_MAJOR > 7
  case Decl::ObjCTypeParam:
  case Decl::BuiltinTemplate:
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case Decl::OMPCapturedExpr:
  case Decl::OMPDeclareReduction:
  case Decl::PragmaComment:
  case Decl::PragmaDetectMismatch:
#endif // CLANG_VERSION > 3.8
    return D;

  // Declaration kinds that don't make any sense here, but are
  // nonetheless harmless.
  case Decl::Empty:
  case Decl::TranslationUnit:
  case Decl::ExternCContext:
    break;

  // Declaration kinds for which the definition is not resolvable.
  case Decl::UnresolvedUsingTypename:
  case Decl::UnresolvedUsingValue:
    break;

  case Decl::UsingDirective:
    return cast<UsingDirectiveDecl>(&*D)->getNominatedNamespace();

  case Decl::NamespaceAlias:
    return cast<NamespaceAliasDecl>(&*D)->getNamespace();

  case Decl::Enum:
  case Decl::Record:
  case Decl::CXXRecord:
  case Decl::ClassTemplateSpecialization:
  case Decl::ClassTemplatePartialSpecialization:
    if (TagDecl *Def = cast<TagDecl>(&*D)->getDefinition())
      return Def;
    break;

  case Decl::Function:
  case Decl::CXXMethod:
  case Decl::CXXConstructor:
  case Decl::CXXDestructor:
  case Decl::CXXConversion: {
    const FunctionDecl *Def = nullptr;
    if (cast<FunctionDecl>(&*D)->getBody(Def))
      return Def;
    break;
  }

  case Decl::Var:
  case Decl::VarTemplateSpecialization:
  case Decl::VarTemplatePartialSpecialization:
#if CLANG_VERSION_MAJOR > 3
  case Decl::Decomposition:
#endif // CLANG_VERSION_MAJOR > 3
  {
    // Ask the variable if it has a definition.
    if (const VarDecl *Def = cast<VarDecl>(&*D)->getDefinition())
      return Def;
    break;
  }

  case Decl::FunctionTemplate: {
    const FunctionDecl *Def = nullptr;
    if (cast<FunctionTemplateDecl>(&*D)->getTemplatedDecl()->getBody(Def))
      return Def->getDescribedFunctionTemplate();
    break;
  }

  case Decl::ClassTemplate: {
    if (RecordDecl *Def =
        cast<ClassTemplateDecl>(&*D)->getTemplatedDecl()->getDefinition())
      return cast<CXXRecordDecl>(Def)->getDescribedClassTemplate();
    break;
  }

  case Decl::VarTemplate: {
    if (VarDecl *Def =
        cast<VarTemplateDecl>(&*D)->getTemplatedDecl()->getDefinition())
      return cast<VarDecl>(Def)->getDescribedVarTemplate();
    break;
  }

  case Decl::Using:
    return cast<UsingDecl>(&*D);

  case Decl::UsingShadow:
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case Decl::ConstructorUsingShadow:
#endif // CLANG_VERSION > 3.8
    return Decl_getDefinition(
      cast<UsingShadowDecl>(&*D)->getTargetDecl(), isReference);

  case Decl::ObjCMethod: {
    const ObjCMethodDecl *Method = cast<ObjCMethodDecl>(D);
    if (Method->isThisDeclarationADefinition())
      return D;

    // Dig out the method definition in the associated
    // @implementation, if we have it.
    // FIXME: The ASTs should make finding the definition easier.
    if (const ObjCInterfaceDecl *Class
                       = dyn_cast<ObjCInterfaceDecl>(Method->getDeclContext()))
      if (ObjCImplementationDecl *ClassImpl = Class->getImplementation())
        if (ObjCMethodDecl *Def = ClassImpl->getMethod(Method->getSelector(),
                                                  Method->isInstanceMethod()))
          if (Def->isThisDeclarationADefinition())
            return Def;

    break;
  }

  case Decl::ObjCCategory:
    if (ObjCCategoryImplDecl *Impl = cast<ObjCCategoryDecl>(D)->getImplementation())
      return Impl;
    break;

  case Decl::ObjCProtocol:
    if (const ObjCProtocolDecl *Def = cast<ObjCProtocolDecl>(D)->getDefinition())
      return Def;
    break;

  case Decl::ObjCInterface: {
    // There are two notions of a "definition" for an Objective-C
    // class: the interface and its implementation. When we resolved a
    // reference to an Objective-C class, produce the @interface as
    // the definition; when we were provided with the interface,
    // produce the @implementation as the definition.
    const ObjCInterfaceDecl *IFace = cast<ObjCInterfaceDecl>(D);
    if (isReference) {
      if (const ObjCInterfaceDecl *Def = IFace->getDefinition())
        return Def;
    } else if (ObjCImplementationDecl *Impl = IFace->getImplementation())
      return Impl;
    break;
  }

  case Decl::ObjCProperty:
    // FIXME: We don't really know where to find the
    // ObjCPropertyImplDecls that implement this property.
    break;

  case Decl::ObjCCompatibleAlias:
    if (const ObjCInterfaceDecl *Class
          = cast<ObjCCompatibleAliasDecl>(D)->getClassInterface())
      if (const ObjCInterfaceDecl *Def = Class->getDefinition())
        return Def;

    break;

  case Decl::Friend:
    if (NamedDecl *Friend = cast<FriendDecl>(&*D)->getFriendDecl())
      return Decl_getDefinition(Friend, isReference);
    break;

  case Decl::FriendTemplate:
    if (NamedDecl *Friend = cast<FriendTemplateDecl>(&*D)->getFriendDecl())
      return Decl_getDefinition(Friend, isReference);
    break;
  }

  return nullptr;
}

// Adapted from clang_getSpecializedCursorTemplate in CIndexCXX.cpp
const Decl *Decl_getSpecializedTemplate(const Decl *D) {
  if (!D)
    return nullptr;

  Decl *Template = nullptr;
  if (const CXXRecordDecl *CXXRecord = dyn_cast<CXXRecordDecl>(&*D)) {
    if (const ClassTemplatePartialSpecializationDecl *PartialSpec
          = dyn_cast<ClassTemplatePartialSpecializationDecl>(CXXRecord))
      Template = PartialSpec->getSpecializedTemplate();
    else if (const ClassTemplateSpecializationDecl *ClassSpec 
               = dyn_cast<ClassTemplateSpecializationDecl>(CXXRecord)) {
      llvm::PointerUnion<ClassTemplateDecl *,
                         ClassTemplatePartialSpecializationDecl *> Result
        = ClassSpec->getSpecializedTemplateOrPartial();
      if (Result.is<ClassTemplateDecl *>())
        Template = Result.get<ClassTemplateDecl *>();
      else
        Template = Result.get<ClassTemplatePartialSpecializationDecl *>();
      
    } else 
      Template = CXXRecord->getInstantiatedFromMemberClass();
  } else if (const FunctionDecl *Function = dyn_cast<FunctionDecl>(&*D)) {
    Template = Function->getPrimaryTemplate();
    if (!Template)
      Template = Function->getInstantiatedFromMemberFunction();
  } else if (const VarDecl *Var = dyn_cast<VarDecl>(&*D)) {
    if (Var->isStaticDataMember())
      Template = Var->getInstantiatedFromStaticDataMember();
  } else if (const RedeclarableTemplateDecl *Tmpl
                                        = dyn_cast<RedeclarableTemplateDecl>(&*D))
    Template = Tmpl->getInstantiatedFromMemberTemplate();

  return Template;
}

// Adapted from clang_getTemplateCursorKind in CIndexCXX.cpp
CXCursorKind Decl_getTemplateCursorKind(const Decl *D) {
  if (!D)
    return CXCursor_NoDeclFound;

  switch (D->getKind()) {
  case Decl::ClassTemplate:
  case Decl::FunctionTemplate:
    if (const TemplateDecl *Template = dyn_cast_or_null<TemplateDecl>(&*D))
      return getCursorKindForDecl(Template->getTemplatedDecl());
    break;

  case Decl::ClassTemplatePartialSpecialization:
    if (const ClassTemplateSpecializationDecl *PartialSpec
        = dyn_cast_or_null<ClassTemplatePartialSpecializationDecl>(&*D)) {
      switch (PartialSpec->getTagKind()) {
      case TTK_Interface:
      case TTK_Struct: return CXCursor_StructDecl;
      case TTK_Class: return CXCursor_ClassDecl;
      case TTK_Union: return CXCursor_UnionDecl;
      case TTK_Enum: return CXCursor_NoDeclFound;
      }
    }
    break;

  default:
    break;
  }

  return CXCursor_NoDeclFound;
}

// Adapted from getDeclSpelling in CIndex.cpp
BindgenStringRef Decl_getSpelling(const Decl *D) {
  if (!D)
    return stringref();

  const NamedDecl *ND = dyn_cast_or_null<NamedDecl>(&*D);
  if (!ND) {
    if (const ObjCPropertyImplDecl *PropImpl =
            dyn_cast<ObjCPropertyImplDecl>(D))
      if (ObjCPropertyDecl *Property = PropImpl->getPropertyDecl())
        return stringref(Property->getIdentifier()->getName());

    if (const ImportDecl *ImportD = dyn_cast<ImportDecl>(D))
      if (Module *Mod = ImportD->getImportedModule())
        return stringref(Mod->getFullModuleName());

    return stringref();
  }

  if (const ObjCMethodDecl *OMD = dyn_cast<ObjCMethodDecl>(ND))
    return stringref(OMD->getSelector().getAsString());

  if (const ObjCCategoryImplDecl *CIMP = dyn_cast<ObjCCategoryImplDecl>(ND))
    // No, this isn't the same as the code below. getIdentifier() is non-virtual
    // and returns different names. NamedDecl returns the class name and
    // ObjCCategoryImplDecl returns the category name.
    return stringref(CIMP->getIdentifier()->getNameStart());

  if (isa<UsingDirectiveDecl>(D))
    return stringref();

  SmallString<1024> S;
  llvm::raw_svector_ostream os(S);
  ND->printName(os);
  return stringref(S.str());
}

// Adapted from clang_Cursor_getNumTemplateArguments in CXCursor.cpp
int Decl_getNumTemplateArguments(const Decl *D) {
  const FunctionDecl *FD = llvm::dyn_cast_or_null<FunctionDecl>(&*D);
  if (!FD)
    return -1;

  const FunctionTemplateSpecializationInfo* SpecInfo =
    FD->getTemplateSpecializationInfo();
  if (!SpecInfo) {
    return -1;
  }

  return SpecInfo->TemplateArguments->size();
}

// Adapted from clang_getCursorLinkage in CIndex.cpp
CXLinkageKind Decl_getLinkage(const Decl *D) {
  if (auto *ND = dyn_cast_or_null<NamedDecl>(&*D))
    switch (ND->getLinkageInternal()) {
    case NoLinkage:
    case VisibleNoLinkage:
      return CXLinkage_NoLinkage;
#if CLANG_VERSION_MAJOR > 4
    case ModuleInternalLinkage:
#endif // CLANG_VERSION_MAJOR > 4
    case InternalLinkage:
      return CXLinkage_Internal;
    case UniqueExternalLinkage:
      return CXLinkage_UniqueExternal;
#if CLANG_VERSION_MAJOR > 4
    case ModuleLinkage:
#endif // CLANG_VERSION_MAJOR > 4
    case ExternalLinkage:
      return CXLinkage_External;
    };

  return CXLinkage_Invalid;
}

// Adapted from clang_getCursorVisibility in CIndex.cpp
CXVisibilityKind Decl_getVisibility(const Decl *D) {
  if (auto *ND = dyn_cast_or_null<NamedDecl>(&*D))
    switch (ND->getVisibility()) {
    case HiddenVisibility:
      return CXVisibility_Hidden;
    case ProtectedVisibility:
      return CXVisibility_Protected;
    case DefaultVisibility:
      return CXVisibility_Default;
    };

  return CXVisibility_Invalid;
}

// Adapted from clang_getCXXAccessSpecifier in CIndexCXX.cpp
CX_CXXAccessSpecifier TranslateCXXAccessSpecifier(AccessSpecifier spec) {
  switch (spec) {
    case AS_public: return CX_CXXPublic;
    case AS_protected: return CX_CXXProtected;
    case AS_private: return CX_CXXPrivate;
    case AS_none: return CX_CXXInvalidAccessSpecifier;
  }

  llvm_unreachable("Invalid AccessSpecifier!");
}

// Adapted from clang_Cursor_getArgument in CXCursor.cpp
const Expr *Expr_getArgument(const Expr *E, unsigned i) {
  if (const CallExpr *CE = dyn_cast_or_null<CallExpr>(&*E)) {
    if (i < CE->getNumArgs()) {
      return CE->getArg(i);
    }
  }
  if (const CXXConstructExpr *CE = dyn_cast_or_null<CXXConstructExpr>(&*E)) {
    if (i < CE->getNumArgs()) {
      return CE->getArg(i);
    }
  }
  return nullptr;
}

// Adapted from clang_Cursor_getNumArguments in CXCursor.cpp
int Expr_getNumArguments(const Expr *E) {
  if (const CallExpr *CE = dyn_cast_or_null<CallExpr>(&*E))
    return CE->getNumArgs();
  if (const CXXConstructExpr *CE = dyn_cast_or_null<CXXConstructExpr>(&*E))
    return CE->getNumArgs();
  return -1;
}

// Adapted from getLocationFromExpr in CIndex.cpp
SourceLocation *Expr_getLocation(const Expr *E) {
  if (!E)
    return nullptr;
  if (const ImplicitCastExpr *CE = dyn_cast<ImplicitCastExpr>(&*E))
    return Expr_getLocation(CE->getSubExpr());

  if (const ObjCMessageExpr *Msg = dyn_cast<ObjCMessageExpr>(&*E))
    return new SourceLocation(/*FIXME:*/Msg->getLeftLoc());
  if (const DeclRefExpr *DRE = dyn_cast<DeclRefExpr>(&*E))
    return new SourceLocation(DRE->getLocation());
  if (const MemberExpr *Member = dyn_cast<MemberExpr>(&*E))
    return new SourceLocation(Member->getMemberLoc());
  if (const ObjCIvarRefExpr *Ivar = dyn_cast<ObjCIvarRefExpr>(&*E))
    return new SourceLocation(Ivar->getLocation());
  if (const SizeOfPackExpr *SizeOfPack = dyn_cast<SizeOfPackExpr>(&*E))
    return new SourceLocation(SizeOfPack->getPackLoc());
  if (const ObjCPropertyRefExpr *PropRef = dyn_cast<ObjCPropertyRefExpr>(&*E))
    return new SourceLocation(PropRef->getLocation());

#if CLANG_VERSION_MAJOR > 6
  return new SourceLocation(E->getBeginLoc());
#else
  return new SourceLocation(E->getLocStart());
#endif
}

// Adapted from clang_getTypeDeclaration in CXType.cpp
const Decl *Type_getDeclaration(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  const Type *TP = QT.getTypePtrOrNull();

  if (!TP)
    return nullptr;

  const Decl *D = nullptr;

try_again:
  switch (TP->getTypeClass()) {
  case Type::Typedef:
    D = cast<TypedefType>(TP)->getDecl();
    break;
  case Type::ObjCObject:
    D = cast<ObjCObjectType>(TP)->getInterface();
    break;
  case Type::ObjCInterface:
    D = cast<ObjCInterfaceType>(TP)->getDecl();
    break;
  case Type::Record:
  case Type::Enum:
    D = cast<TagType>(TP)->getDecl();
    break;
  case Type::TemplateSpecialization:
    if (const RecordType *Record = TP->getAs<RecordType>())
      D = Record->getDecl();
    else
      D = cast<TemplateSpecializationType>(TP)->getTemplateName()
                                                         .getAsTemplateDecl();
    break;

  case Type::Auto:
#if CLANG_VERSION_MAJOR > 4
  case Type::DeducedTemplateSpecialization:
    TP = cast<DeducedType>(TP)->getDeducedType().getTypePtrOrNull();
#else // CLANG_VERSION_MAJOR <= 4
    TP = cast<AutoType>(TP)->getDeducedType().getTypePtrOrNull();
#endif // CLANG_VERSION_MAJOR > 4
    if (TP)
      goto try_again;
    break;

  case Type::InjectedClassName:
    D = cast<InjectedClassNameType>(TP)->getDecl();
    break;

  // FIXME: Template type parameters!      

  case Type::Elaborated:
    TP = cast<ElaboratedType>(TP)->getNamedType().getTypePtrOrNull();
    goto try_again;

  default:
    break;
  }

  if (!D)
    return nullptr;

  return D;
}

// Adapted from getTokens and others in CIndex.cpp
void tokenize(ASTUnit *TU, BindgenSourceRange Range, CXToken **Tokens,
              unsigned *NumTokens) {
  if (!Tokens || !NumTokens || !Range.B || !Range.E)
    return;

  // Translate the Range end location to after the last token, instead of
  // the beginning of the last token.
  SourceManager &SourceMgr = TU->getSourceManager();
  SourceLocation EndLoc = *Range.E;
  bool IsTokenRange = true;
  if (EndLoc.isValid() && EndLoc.isMacroID() && !SourceMgr.isMacroArgExpansion(EndLoc)) {
#if CLANG_VERSION_MAJOR > 6
    CharSourceRange Expansion = SourceMgr.getExpansionRange(EndLoc);
    EndLoc = Expansion.getEnd();
    IsTokenRange = Expansion.isTokenRange();
#else
    EndLoc = SourceMgr.getExpansionRange(EndLoc).second;
#endif
  }
  if (IsTokenRange && EndLoc.isValid()) {
    unsigned Length = Lexer::MeasureTokenLength(SourceMgr.getSpellingLoc(EndLoc),
                                                SourceMgr, TU->getLangOpts());
    EndLoc = EndLoc.getLocWithOffset(Length);
  }

  SmallVector<CXToken, 32> CXTokens;
  std::pair<FileID, unsigned> BeginLocInfo =
      SourceMgr.getDecomposedSpellingLoc(*Range.B);
  std::pair<FileID, unsigned> EndLocInfo =
      SourceMgr.getDecomposedSpellingLoc(EndLoc);

  // BindgenSourceRange elements need to be manually destructed because it is
  // C-style struct shared with Rust.
  delete Range.B;
  delete Range.E;

  // Cannot tokenize across files.
  if (BeginLocInfo.first != EndLocInfo.first)
    return;

  // Create a lexer
  bool Invalid = false;
  StringRef Buffer
    = SourceMgr.getBufferData(BeginLocInfo.first, &Invalid);
  if (Invalid)
    return;
  
  Lexer Lex(SourceMgr.getLocForStartOfFile(BeginLocInfo.first),
            TU->getASTContext().getLangOpts(),
            Buffer.begin(), Buffer.data() + BeginLocInfo.second, Buffer.end());
  Lex.SetCommentRetentionState(true);

  // Lex tokens until we hit the end of the range.
  const char *EffectiveBufferEnd = Buffer.data() + EndLocInfo.second;
  Token Tok;
  bool previousWasAt = false;
  do {
    // Lex the next token
    Lex.LexFromRawLexer(Tok);
    if (Tok.is(tok::eof))
      break;

    // Initialize the CXToken.
    CXToken CXTok;

    //   - Common fields
    CXTok.int_data[1] = Tok.getLocation().getRawEncoding();
    CXTok.int_data[2] = Tok.getLength();
    CXTok.int_data[3] = 0;

    //   - Kind-specific fields
    if (Tok.isLiteral()) {
      CXTok.int_data[0] = CXToken_Literal;
      CXTok.ptr_data = const_cast<char *>(Tok.getLiteralData());
    } else if (Tok.is(tok::raw_identifier)) {
      // Lookup the identifier to determine whether we have a keyword.
      IdentifierInfo *II
        = TU->getPreprocessor().LookUpIdentifierInfo(Tok);

      if ((II->getObjCKeywordID() != tok::objc_not_keyword) && previousWasAt) {
        CXTok.int_data[0] = CXToken_Keyword;
      }
      else {
        CXTok.int_data[0] = Tok.is(tok::identifier)
          ? CXToken_Identifier
          : CXToken_Keyword;
      }
      CXTok.ptr_data = II;
    } else if (Tok.is(tok::comment)) {
      CXTok.int_data[0] = CXToken_Comment;
      CXTok.ptr_data = nullptr;
    } else {
      CXTok.int_data[0] = CXToken_Punctuation;
      CXTok.ptr_data = nullptr;
    }
    CXTokens.push_back(CXTok);
    previousWasAt = Tok.is(tok::at);
  } while (Lex.getBufferLocation() < EffectiveBufferEnd);

  *Tokens = new CXToken[CXTokens.size()];
  memmove(*Tokens, CXTokens.data(), sizeof(CXToken) * CXTokens.size());
  *NumTokens = CXTokens.size();
}

// Originally clang_getTokenKind in CIndex.cpp
CXTokenKind getTokenKind(CXToken token) {
  return static_cast<CXTokenKind>(token.int_data[0]);
}

// Adapted from clang_getTokenSpelling in CIndex.cpp
BindgenStringRef getTokenSpelling(ASTUnit *CXXUnit, CXToken CXTok) {
  if (!CXXUnit)
    return stringref();

  switch (getTokenKind(CXTok)) {
  case CXToken_Identifier:
  case CXToken_Keyword:
    // We know we have an IdentifierInfo*, so use that.
    return stringref(static_cast<IdentifierInfo *>(CXTok.ptr_data)
                            ->getNameStart());

  case CXToken_Literal: {
    // We have stashed the starting pointer in the ptr_data field. Use it.
    const char *Text = static_cast<const char *>(CXTok.ptr_data);
    return stringref(StringRef(Text, CXTok.int_data[2]));
  }

  case CXToken_Punctuation:
  case CXToken_Comment:
    break;
  }

  // We have to find the starting buffer pointer the hard way, by
  // deconstructing the source location.
  SourceLocation Loc = SourceLocation::getFromRawEncoding(CXTok.int_data[1]);
  std::pair<FileID, unsigned> LocInfo
    = CXXUnit->getSourceManager().getDecomposedSpellingLoc(Loc);
  bool Invalid = false;
  StringRef Buffer
    = CXXUnit->getSourceManager().getBufferData(LocInfo.first, &Invalid);
  if (Invalid)
    return stringref();

  return stringref(Buffer.substr(LocInfo.second, CXTok.int_data[2]));
}

// Adapted from clang_Type_getSizeOf in CXType.cpp
long long Type_getSizeOf(BindgenQualType T, ASTContext *Context) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return CXTypeLayoutError_Invalid;
  // [expr.sizeof] p2: if reference type, return size of referenced type
  if (QT->isReferenceType())
    QT = QT.getNonReferenceType();
  // [expr.sizeof] p1: return -1 on: func, incomplete, bitfield, incomplete
  //                   enumeration
  // Note: We get the cxtype, not the cxcursor, so we can't call
  //       FieldDecl->isBitField()
  // [expr.sizeof] p3: pointer ok, function not ok.
  // [gcc extension] lib/AST/ExprConstant.cpp:1372 HandleSizeof : vla == error
  if (QT->isIncompleteType())
    return CXTypeLayoutError_Incomplete;
  if (QT->isDependentType())
    return CXTypeLayoutError_Dependent;
  if (!QT->isConstantSizeType())
    return CXTypeLayoutError_NotConstantSize;
#if CLANG_VERSION_MAJOR > 8
  if (const auto *Deduced = dyn_cast<DeducedType>(QT))
    if (Deduced->getDeducedType().isNull())
      return CXTypeLayoutError_Undeduced;
#endif // CLANG_VERSION_MAJOR > 8
  // [gcc extension] lib/AST/ExprConstant.cpp:1372
  //                 HandleSizeof : {voidtype,functype} == 1
  // not handled by ASTContext.cpp:1313 getTypeInfoImpl
  if (QT->isVoidType() || QT->isFunctionType())
    return 1;
  return Context->getTypeSizeInChars(QT).getQuantity();
}

// Adapted from clang_Type_getAlignOf in CXType.cpp
long long Type_getAlignOf(BindgenQualType T, ASTContext *Context) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return CXTypeLayoutError_Invalid;
  // [expr.alignof] p1: return size_t value for complete object type, reference
  //                    or array.
  // [expr.alignof] p3: if reference type, return size of referenced type
  if (QT->isReferenceType())
    QT = QT.getNonReferenceType();
  if (!(QT->isIncompleteArrayType() || !QT->isIncompleteType()))
    return CXTypeLayoutError_Incomplete;
  if (QT->isDependentType())
    return CXTypeLayoutError_Dependent;
#if CLANG_VERSION_MAJOR > 8
  if (const auto *Deduced = dyn_cast<DeducedType>(QT))
    if (Deduced->getDeducedType().isNull())
      return CXTypeLayoutError_Undeduced;
#endif // CLANG_VERSION_MAJOR > 8
  // Exceptions by GCC extension - see ASTContext.cpp:1313 getTypeInfoImpl
  // if (QT->isFunctionType()) return 4; // Bug #15511 - should be 1
  // if (QT->isVoidType()) return 1;
  return Context->getTypeAlignInChars(QT).getQuantity();
}

// Adapted from clang_getPointeeType in CXType.cpp
BindgenQualType Type_getPointeeType(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  const Type *TP = QT.getTypePtrOrNull();

  if (!TP)
    return nullptr;

try_again:
  switch (TP->getTypeClass()) {
    case Type::Pointer:
      QT = cast<PointerType>(TP)->getPointeeType();
      break;
    case Type::BlockPointer:
      QT = cast<BlockPointerType>(TP)->getPointeeType();
      break;
    case Type::LValueReference:
    case Type::RValueReference:
      QT = cast<ReferenceType>(TP)->getPointeeType();
      break;
    case Type::ObjCObjectPointer:
      QT = cast<ObjCObjectPointerType>(TP)->getPointeeType();
      break;
    case Type::MemberPointer:
      QT = cast<MemberPointerType>(TP)->getPointeeType();
      break;
    case Type::Auto:
#if CLANG_VERSION_MAJOR > 4
    case Type::DeducedTemplateSpecialization:
      TP = cast<DeducedType>(TP)->getDeducedType().getTypePtrOrNull();
#else // CLANG_VERSION_MAJOR <= 4
      TP = cast<AutoType>(TP)->getDeducedType().getTypePtrOrNull();
#endif // CLANG_VERSION_MAJOR > 4
      if (TP)
        goto try_again;
      break;
    default:
      QT = QualType();
      break;
  }
  return make_type_compatible(QT);
}

// Adapted from clang_getElementType in CXType.cpp
BindgenQualType Type_getElementType(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  QualType ET = QualType();
  const Type *TP = QT.getTypePtrOrNull();

  if (TP) {
    switch (TP->getTypeClass()) {
    case Type::ConstantArray:
      ET = cast<ConstantArrayType> (TP)->getElementType();
      break;
    case Type::IncompleteArray:
      ET = cast<IncompleteArrayType> (TP)->getElementType();
      break;
    case Type::VariableArray:
      ET = cast<VariableArrayType> (TP)->getElementType();
      break;
    case Type::DependentSizedArray:
      ET = cast<DependentSizedArrayType> (TP)->getElementType();
      break;
    case Type::Vector:
      ET = cast<VectorType> (TP)->getElementType();
      break;
    case Type::ExtVector:
      ET = cast<ExtVectorType>(TP)->getElementType();
      break;
    case Type::Complex:
      ET = cast<ComplexType> (TP)->getElementType();
      break;
    default:
      break;
    }
  }
  return make_type_compatible(ET);
}

// Adapted from clang_getNumElements in CXType.cpp
int Type_getNumElements(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  long long result = -1;
  const Type *TP = QT.getTypePtrOrNull();

  if (TP) {
    switch (TP->getTypeClass()) {
    case Type::ConstantArray:
      result = cast<ConstantArrayType> (TP)->getSize().getSExtValue();
      break;
    case Type::Vector:
      result = cast<VectorType> (TP)->getNumElements();
      break;
    case Type::ExtVector:
      result = cast<ExtVectorType>(TP)->getNumElements();
      break;
    default:
      break;
    }
  }
  return result;
}

// Adapted from clang_getFunctionTypeCallingConv in CXType.cpp
CXCallingConv Type_getFunctionTypeCallingConv(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return CXCallingConv_Invalid;
  
  if (const FunctionType *FD = QT->getAs<FunctionType>()) {
#define TCALLINGCONV(X) case CC_##X: return CXCallingConv_##X
    switch (FD->getCallConv()) {
      TCALLINGCONV(C);
      TCALLINGCONV(X86StdCall);
      TCALLINGCONV(X86FastCall);
      TCALLINGCONV(X86ThisCall);
      TCALLINGCONV(X86Pascal);
#if CLANG_VERSION_MAJOR > 3
      TCALLINGCONV(X86RegCall);
#endif // CLANG_VERSION_MAJOR > 3
      TCALLINGCONV(X86VectorCall);
#if CLANG_VERSION_MAJOR > 7
      TCALLINGCONV(AArch64VectorCall);
#endif // CLANG_VERSION_MAJOR > 7
#if CLANG_VERSION_MAJOR > 4
      TCALLINGCONV(Win64);
#else
      TCALLINGCONV(X86_64Win64);
#endif // CLANG_VERSION_MAJOR > 4
      TCALLINGCONV(X86_64SysV);
      TCALLINGCONV(AAPCS);
      TCALLINGCONV(AAPCS_VFP);
      TCALLINGCONV(IntelOclBicc);
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
      TCALLINGCONV(Swift);
      TCALLINGCONV(PreserveMost);
      TCALLINGCONV(PreserveAll);
    case CC_OpenCLKernel: return CXCallingConv_Unexposed;
#else // CLANG_VERSION <= 3.8
    case CC_SpirKernel: return CXCallingConv_Unexposed;
#endif // CLANG_VERSION > 3.8
    case CC_SpirFunction: return CXCallingConv_Unexposed;
      break;
    }
#undef TCALLINGCONV
  }
  
  return CXCallingConv_Invalid;
}

// Adapted from clang_getSpellingLocation in CXSourceLocation.cpp
void getSpellingLocation(ASTUnit *AST, const SourceLocation *location, FileEntry **file, int *line, int *column, int *offset) {
  if (!location)
    return createNullLocation(file, line, column, offset);

  const SourceManager &SM = AST->getSourceManager();
  // FIXME: This should call SourceManager::getSpellingLoc().
  SourceLocation SpellLoc = SM.getFileLoc(*location);
  std::pair<FileID, unsigned> LocInfo = SM.getDecomposedLoc(SpellLoc);
  FileID FID = LocInfo.first;
  unsigned FileOffset = LocInfo.second;
  
  if (FID.isInvalid())
    return createNullLocation(file, line, column, offset);
  
  if (file)
    *file = const_cast<FileEntry *>(SM.getFileEntryForID(FID));
  if (line)
    *line = SM.getLineNumber(FID, FileOffset);
  if (column)
    *column = SM.getColumnNumber(FID, FileOffset);
  if (offset)
    *offset = FileOffset;
}

// From CIndex.cpp
static std::string getMangledStructor(std::unique_ptr<MangleContext> &M,
                                      std::unique_ptr<llvm::DataLayout> &DL,
                                      const NamedDecl *ND,
                                      unsigned StructorType) {
  std::string FrontendBuf;
  llvm::raw_string_ostream FOS(FrontendBuf);

  if (const auto *CD = dyn_cast_or_null<CXXConstructorDecl>(ND))
    M->mangleCXXCtor(CD, static_cast<CXXCtorType>(StructorType), FOS);
  else if (const auto *DD = dyn_cast_or_null<CXXDestructorDecl>(ND))
    M->mangleCXXDtor(DD, static_cast<CXXDtorType>(StructorType), FOS);

  std::string BackendBuf;
  llvm::raw_string_ostream BOS(BackendBuf);

  llvm::Mangler::getNameWithPrefix(BOS, llvm::Twine(FOS.str()), *DL);

  return BOS.str();
}

// Adapted from clang_Cursor_getMangling in CIndex.cpp
BindgenStringRef Decl_getMangling(const Decl *D, ASTContext *Ctx) {
  if (!D || !(isa<FunctionDecl>(&*D) || isa<VarDecl>(&*D)))
    return stringref();

#if CLANG_VERSION_MAJOR > 8
  ASTNameGenerator NameGen(*Ctx);
  return stringref(NameGen.getName(&*D));
#elif CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  index::CodegenNameGenerator NameGen(*Ctx);
  return stringref(NameGen.getName(&*D));
#else // CLANG_VERSION <= 3.8
  // First apply frontend mangling.
  const NamedDecl *ND = cast<NamedDecl>(D);
  std::unique_ptr<MangleContext> MC(Ctx->createMangleContext());

  std::string FrontendBuf;
  llvm::raw_string_ostream FrontendBufOS(FrontendBuf);
  if (MC->shouldMangleDeclName(ND)) {
    MC->mangleName(ND, FrontendBufOS);
  } else {
    ND->printName(FrontendBufOS);
  }

  // Now apply backend mangling.
  std::unique_ptr<llvm::DataLayout> DL(
    new llvm::DataLayout(Ctx->getTargetInfo().getDataLayoutString()));

  std::string FinalBuf;
  llvm::raw_string_ostream FinalBufOS(FinalBuf);
  llvm::Mangler::getNameWithPrefix(FinalBufOS, llvm::Twine(FrontendBufOS.str()),
                                   *DL);
  return stringref(FinalBuf);
#endif // CLANG_VERSION_MAJOR > 8
}

// Adapted from clang_Cursor_getCXXManglings in CIndex.cpp
BindgenStringRefSet Decl_getCXXManglings(const Decl *D, ASTContext *Ctx) {
  if (!D || !(isa<CXXRecordDecl>(&*D) || isa<CXXMethodDecl>(&*D)))
    return BindgenStringRefSet();

  std::vector<std::string> Manglings;

#if CLANG_VERSION_MAJOR > 8
  ASTNameGenerator NameGen(*Ctx);
  Manglings = NameGen.getAllManglings(&*D);
#elif CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  index::CodegenNameGenerator NameGen(*Ctx);
  Manglings = NameGen.getAllManglings(&*D);
#else // CLANG_VERSION <= 3.8
  const NamedDecl *ND = cast<NamedDecl>(D);

  std::unique_ptr<MangleContext> M(Ctx->createMangleContext());
  std::unique_ptr<llvm::DataLayout> DL(
      new llvm::DataLayout(Ctx->getTargetInfo().getDataLayoutString()));

  auto hasDefaultCXXMethodCC = [](ASTContext &C, const CXXMethodDecl *MD) {
    auto DefaultCC = C.getDefaultCallingConvention(/*IsVariadic=*/false,
                                                   /*IsCSSMethod=*/true);
    auto CC = MD->getType()->getAs<FunctionProtoType>()->getCallConv();
    return CC == DefaultCC;
  };

  if (const auto *CD = dyn_cast_or_null<CXXConstructorDecl>(ND)) {
    Manglings.emplace_back(getMangledStructor(M, DL, CD, Ctor_Base));

    if (Ctx->getTargetInfo().getCXXABI().isItaniumFamily())
      if (!CD->getParent()->isAbstract())
        Manglings.emplace_back(getMangledStructor(M, DL, CD, Ctor_Complete));

    if (Ctx->getTargetInfo().getCXXABI().isMicrosoft())
      if (CD->hasAttr<DLLExportAttr>() && CD->isDefaultConstructor())
        if (!(hasDefaultCXXMethodCC(*Ctx, CD) && CD->getNumParams() == 0))
          Manglings.emplace_back(getMangledStructor(M, DL, CD,
                                                    Ctor_DefaultClosure));
  } else if (const auto *DD = dyn_cast_or_null<CXXDestructorDecl>(ND)) {
    Manglings.emplace_back(getMangledStructor(M, DL, DD, Dtor_Base));
    if (Ctx->getTargetInfo().getCXXABI().isItaniumFamily()) {
      Manglings.emplace_back(getMangledStructor(M, DL, DD, Dtor_Complete));
      if (DD->isVirtual())
        Manglings.emplace_back(getMangledStructor(M, DL, DD, Dtor_Deleting));
    }
  }
#endif // CLANG_VERSION_MAJOR > 8

  return make_stringrefset(Manglings);
}
