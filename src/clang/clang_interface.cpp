#include <string>

#include "clang/AST/Comment.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/DeclObjC.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/ExprObjC.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/TargetInfo.h"
#include "clang/Basic/Version.h"
#include "clang/Frontend/ASTUnit.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Lex/Preprocessor.h"
#include "clang/Lex/PreprocessorOptions.h"
#include "clang-c/Documentation.h"
#include "clang-c/Index.h"

#include "clang_interface_impl.hpp"

using namespace clang;

BindgenStringRef stringref() {
  BindgenStringRef ref;
  ref.s = nullptr;
  ref.len = 0;
  return ref;
}

BindgenStringRef stringref(const char *newStr) {
  BindgenStringRef ref;
  ref.len = strlen(newStr);
  ref.s = new char[ref.len + 1];
  strncpy(ref.s, newStr, ref.len);
  ref.s[ref.len] = '\0';
  return ref;
}

BindgenStringRef stringref(const std::string &s) {
  return stringref(s.c_str());
}

BindgenStringRef stringref(llvm::StringRef S) {
  BindgenStringRef ref;
  ref.len = S.size();
  ref.s = new char[ref.len + 1];
  strncpy(ref.s, S.data(), ref.len);
  ref.s[ref.len] = '\0';
  return ref;
}

BindgenSourceRange make_sourcerange(const SourceRange &range) {
  BindgenSourceRange out;
  out.B = new SourceLocation(range.getBegin());
  out.E = new SourceLocation(range.getEnd());
  return out;
}

BindgenStringRefSet make_stringrefset(std::vector<std::string> &string_vec) {
  BindgenStringRefSet set;
  set.len = string_vec.size();
  set.strings = new BindgenStringRef[set.len];
  for (size_t i = 0; i < set.len; ++i)
    set.strings[i] = stringref(string_vec[i]);
  return set;
}

void deleteString(BindgenStringRef *s) {
  if (s)
    delete[] s->s;
}

char *cString(BindgenStringRef *s) {
  return s ? s->s : nullptr;
}

void deleteStringSet(BindgenStringRefSet *s) {
  if (s) {
    for (size_t i = 0; i < s->len; ++i) {
      delete[] s->strings[i].s;
    }
    delete[] s->strings;
  }
}

void deleteSourceLocation(SourceLocation *s) {
  if (s)
    delete s;
}

void deleteSourceRange(BindgenSourceRange *s) {
  if (s) {
    delete s->B;
    delete s->E;
  }
}

ASTContext *ASTUnit_getContext(ASTUnit *Unit) {
  return &Unit->getASTContext();
}

void disposeASTUnit(ASTUnit *AU) {
  delete AU;
}

unsigned ASTUnit_getNumDiagnostics(const ASTUnit *AU) {
  return AU->stored_diag_size();
}

const StoredDiagnostic *ASTUnit_getDiagnostic(const ASTUnit *AU, unsigned i) {
  return (AU->stored_diag_begin()+i);
}

const TargetInfo *ASTUnit_getTargetInfo(ASTUnit *Unit) {
  return &Unit->getASTContext().getTargetInfo();
}

int TargetInfo_getPointerWidth(const TargetInfo *TI) {
  if (!TI)
    return -1;
  // Address space 0 is the normal address space for all platforms
  return TI->getPointerWidth(0);
}

BindgenStringRef TargetInfo_getTriple(const TargetInfo *TI) {
  if (!TI)
    return stringref();

  std::string Triple = TI->getTriple().normalize();
  return stringref(Triple);
}

struct EvalResult {
  CXEvalResultKind EvalType;
  APValue Val;
  std::string stringVal;

  EvalResult() : EvalType(CXEval_UnExposed) {}
  EvalResult(APValue Val) : EvalType(CXEval_UnExposed), Val(Val) {
    if (Val.isInt())
      EvalType = CXEval_Int;
    else if (Val.isFloat())
      EvalType = CXEval_Float;
  }
  EvalResult(std::string str) : EvalType(CXEval_StrLiteral), stringVal(str) {}
};

void deleteEvalResult(EvalResult *e) {
  delete e;
}

EvalResult *Expr_Evaluate(const Expr *E, ASTContext *Ctx) {
  if (!E || E->isValueDependent())
    return nullptr;

  Expr::EvalResult res;
  if (E->EvaluateAsRValue(res, *Ctx)) {
    if (E->getStmtClass() == Stmt::ImplicitCastExprClass) {
      const ImplicitCastExpr *I = dyn_cast<ImplicitCastExpr>(&*E);
      auto *subExpr = I->getSubExprAsWritten();
      if (subExpr->getStmtClass() == Stmt::StringLiteralClass) {
        auto StrE = cast<StringLiteral>(I->getSubExprAsWritten());
        return new EvalResult(StrE->getString().str());
      }
    } else if (E->getStmtClass() == Stmt::StringLiteralClass) {
      auto StrE = cast<StringLiteral>(&*E);
      return new EvalResult(StrE->getString().str());
    }
    return new EvalResult(res.Val);
  }

  return nullptr;
}

EvalResult *Decl_Evaluate(const Decl *D, ASTContext *Ctx) {
  if (!D)
    return nullptr;

  const Expr *E;
  if (auto *Var = dyn_cast<VarDecl>(&*D))
    E = Var->getInit();
  else if (auto *Field = dyn_cast<FieldDecl>(&*D))
    E = Field->getInClassInitializer();
  else
    return nullptr;

  return Expr_Evaluate(E, Ctx);
}

CXEvalResultKind EvalResult_getKind(EvalResult *ER) {
  if (!ER)
    return CXEval_UnExposed;
  return ER->EvalType;
}

double EvalResult_getAsDouble(EvalResult *ER) {
  if (!ER)
    return 0;
  auto apFloat = ER->Val.getFloat();
  bool ignored;
#if CLANG_VERSION_MAJOR > 3
  apFloat.convert(llvm::APFloat::IEEEdouble(),
                  llvm::APFloat::rmNearestTiesToEven, &ignored);
#else // CLANG_VERSION_MAJOR <= 3.9
  apFloat.convert(llvm::APFloat::IEEEdouble,
                  llvm::APFloat::rmNearestTiesToEven, &ignored);
#endif
  return apFloat.convertToDouble();
}

bool EvalResult_isUnsignedInt(EvalResult *ER) {
  return ER &&
    ER->EvalType == CXEval_Int &&
    ER->Val.getInt().isUnsigned();
}

long long EvalResult_getAsLongLong(EvalResult *ER) {
  if (!ER)
    return 0;

  auto intVal = ER->Val.getInt();
  if (intVal.isUnsigned())
    return intVal.getZExtValue();
  else
    return intVal.getExtValue();
}

unsigned long long EvalResult_getAsUnsigned(EvalResult *ER) {
  return static_cast<unsigned long long>(EvalResult_getAsLongLong(ER));
}

BindgenStringRef EvalResult_getAsStr(EvalResult *ER) {
  if (!ER)
    return stringref();

  return stringref(ER->stringVal);
}

BindgenStringRef Diagnostic_format(const StoredDiagnostic *D) {
  if (!D)
    return stringref();

  return stringref(D->getMessage());
}

const Decl *getTranslationUnitDecl(ASTUnit *Unit) {
  return Unit->getASTContext().getTranslationUnitDecl();
}

bool CursorKind_isInvalid(CXCursorKind kind) {
  return kind >= CXCursor_FirstInvalid && kind <= CXCursor_LastInvalid;
}

const Decl *Decl_getLexicalParent(const Decl *D) {
  if (!D)
    return nullptr;

  const DeclContext *DC = D->getLexicalDeclContext();
  if (!DC)
    return nullptr;

  return cast<Decl>(DC);
}

const Decl *Decl_getSemanticParent(const Decl *D) {
  if (!D)
    return nullptr;

  const DeclContext *DC = D->getDeclContext();
  if (!DC)
    return nullptr;

  // We replace CXXRecordDecl's inside ClassTemplateDecls with just the
  // ClassTemplateDecl because we never expose the inner CXXRecordDecl to Rust.
  if (auto *RD = dyn_cast<CXXRecordDecl>(DC)) {
    auto *ClassTemplate = RD->getDescribedClassTemplate();
    if (ClassTemplate)
      return ClassTemplate;
  }

  return cast<Decl>(DC);
}

const Decl *Decl_getReferenced(const Decl *D) {
  if (!D)
    return nullptr;

  // Bindgen doesn't handle UsingDecl references specially, but if we did, this
  // is where that would go.

  return D;
}

const Decl *Decl_getCanonical(const Decl *D) {
  if (!D)
    return nullptr;
  if (const ObjCCategoryImplDecl *CatImplD = dyn_cast<ObjCCategoryImplDecl>(D))
    if (ObjCCategoryDecl *CatD = CatImplD->getCategoryDecl())
      return CatD;

  if (const ObjCImplDecl *ImplD = dyn_cast<ObjCImplDecl>(D))
    if (const ObjCInterfaceDecl *IFD = ImplD->getClassInterface())
      return IFD;

  return D->getCanonicalDecl();
}

const Decl *Decl_getArgument(const Decl *D, unsigned i) {
  if (!D)
    return nullptr;

  if (const ObjCMethodDecl *MD = dyn_cast_or_null<ObjCMethodDecl>(&*D)) {
    if (i < MD->param_size())
      return MD->parameters()[i];
  } else if (const FunctionDecl *FD = dyn_cast_or_null<FunctionDecl>(&*D)) {
    if (i < FD->param_size())
      return FD->parameters()[i];
  }

  return nullptr;
}

int Decl_getNumArguments(const Decl *D) {
  if (const ObjCMethodDecl *MD = dyn_cast_or_null<ObjCMethodDecl>(&*D))
    return MD->param_size();
  if (const FunctionDecl *FD = dyn_cast_or_null<FunctionDecl>(&*D))
    return FD->param_size();

  return -1;
}

BindgenStringRef Decl_getUSR(const Decl *D) {
  SmallString<128> Buf;
  if (index::generateUSRForDecl(&*D, Buf))
    return stringref();
  else
    return stringref(Buf.str());
}

CXCursorKind Decl_getCXCursorKind(const Decl *D) {
  if (!D)
    return CXCursor_NoDeclFound;
  else
    return getCursorKindForDecl(&*D);
}

bool Decl_isDefinition(const Decl *D) {
  if (auto VD = dyn_cast_or_null<VarDecl>(&*D))
    return VD->getDefinition() == &*D;
  if (auto FD = dyn_cast_or_null<FunctionDecl>(&*D))
    return FD->isThisDeclarationADefinition();
  if (auto TD = dyn_cast_or_null<TagDecl>(&*D))
    return TD->getDefinition() == &*D;

  return false;
}

SourceLocation *Decl_getLocation(const Decl *D) {
  if (!D)
    return nullptr;
  return new SourceLocation(D->getLocation());
}

BindgenStringRef Decl_getRawCommentText(const Decl *D, ASTContext *Ctx) {
  if (!D)
    return stringref();

  auto *RC = Ctx->getRawCommentForAnyRedecl(&*D);
  return RC ? stringref(RC->getRawText(Ctx->getSourceManager())) : stringref();
}

comments::Comment *Decl_getParsedComment(const Decl *D, ASTContext *Ctx) {
  if (!D)
    return nullptr;
  return Ctx->getCommentForDecl(&*D, nullptr);
}

BindgenQualType make_type_compatible(QualType QT) {
  if (QT.isNull())
    return nullptr;

  // libclang does not return AttributedTypes if
  // CXTranslationUnit_IncludeAttributedTypes is not set, and bindgen assumes it
  // is not set.
  if (auto *ATT = QT->getAs<AttributedType>())
    return make_type_compatible(ATT->getEquivalentType());

  // libclang does not return ParenTypes
  if (auto *PTT = QT->getAs<ParenType>())
    return make_type_compatible(PTT->getInnerType());

  // Decayed types should be passed as their original type
  if (auto *DT = QT->getAs<DecayedType>())
    return make_type_compatible(DT->getOriginalType());

  return QT.getAsOpaquePtr();
}

BindgenQualType Decl_getType(const Decl *D, ASTContext *Ctx) {
  auto ty = QualType();
  if (!D)
    return nullptr;

  if (auto *TD = dyn_cast<TypeDecl>(&*D))
    ty = Ctx->getTypeDeclType(TD);
  else if (auto *ID = dyn_cast<ObjCInterfaceDecl>(&*D))
    ty = Ctx->getObjCInterfaceType(ID);
  else if (auto *DD = dyn_cast<DeclaratorDecl>(&*D))
    ty = DD->getType();
  else if (auto *VD = dyn_cast<ValueDecl>(&*D))
    ty = VD->getType();
  else if (auto *PD = dyn_cast<ObjCPropertyDecl>(&*D))
    ty = PD->getType();
  else if (auto *FTD = dyn_cast<FunctionTemplateDecl>(&*D))
    ty = FTD->getTemplatedDecl()->getType();
  else
    return nullptr;

  return make_type_compatible(ty);
}

bool Decl_isFunctionInlined(const Decl *D) {
  if (auto *FD = dyn_cast_or_null<FunctionDecl>(&*D))
    return FD->isInlined();
  else
    return false;
}

int Decl_getFieldDeclBitWidth(const Decl *D, ASTContext *Ctx) {
  if (auto *FD = dyn_cast_or_null<FieldDecl>(&*D)) {
    if (FD->isBitField())
      return FD->getBitWidthValue(*Ctx);
  }
  return -1;
}

BindgenQualType Decl_getEnumDeclIntegerType(const Decl *D) {
  if (auto *TD = dyn_cast_or_null<EnumDecl>(&*D))
    return make_type_compatible(TD->getIntegerType());
  else
    return nullptr;
}

int64_t Decl_getEnumConstantValue(const Decl *D) {
  if (auto *TD = dyn_cast_or_null<EnumConstantDecl>(&*D))
    return TD->getInitVal().getSExtValue();
  else
    return LLONG_MIN;
}

uint64_t Decl_getEnumConstantUnsignedValue(const Decl *D) {
  if (auto *TD = dyn_cast_or_null<EnumConstantDecl>(&*D))
    return TD->getInitVal().getZExtValue();
  else
    return ULLONG_MAX;
}

long long Decl_getOffsetOfField(const Decl *D, ASTContext *Ctx) {
  auto *RD = dyn_cast_or_null<RecordDecl>(D->getDeclContext());
  auto Err = visitRecordForValidation(RD);
  if (Err < 0)
    return Err;

  if (auto *FD = dyn_cast_or_null<FieldDecl>(&*D))
    return Ctx->getFieldOffset(FD);
  if (auto *IFD = dyn_cast_or_null<IndirectFieldDecl>(&*D))
    return Ctx->getFieldOffset(IFD);

  return -1;
}

BindgenSourceRange Decl_getSourceRange(const Decl *D) {
  return make_sourcerange(D->getSourceRange());
}

BindgenQualType Decl_getTypedefDeclUnderlyingType(const Decl *D) {
  if (auto *TD = dyn_cast_or_null<TypedefNameDecl>(&*D))
    return make_type_compatible(TD->getUnderlyingType());
  else
    return nullptr;
}

bool CXXField_isMutable(const Decl *D) {
  if (const auto FD = dyn_cast_or_null<FieldDecl>(&*D))
    return FD->isMutable();
  else
    return false;
}

bool CXXMethod_isStatic(const Decl *D) {
  auto *Method =
      D ? dyn_cast_or_null<CXXMethodDecl>(D->getAsFunction()) : nullptr;
  return Method && Method->isStatic();
}

bool CXXMethod_isConst(const Decl *D) {
  auto *Method =
      D ? dyn_cast_or_null<CXXMethodDecl>(D->getAsFunction()) : nullptr;
  if (!Method)
    return false;
#if CLANG_VERSION_MAJOR > 8
  return Method->getMethodQualifiers().hasConst();
#elif CLANG_VERSION_MAJOR > 7
  return Method->getTypeQualifiers().hasConst();
#else
  return Method->getTypeQualifiers() & Qualifiers::Const;
#endif // CLANG_VERSION_MAJOR
}

bool CXXMethod_isVirtual(const Decl *D) {
  auto *Method =
      D ? dyn_cast_or_null<CXXMethodDecl>(D->getAsFunction()) : nullptr;
  return Method && Method->isVirtual();
}

bool CXXMethod_isPureVirtual(const Decl *D) {
  auto *Method =
      D ? dyn_cast_or_null<CXXMethodDecl>(D->getAsFunction()) : nullptr;
  return Method && Method->isVirtual() && Method->isPure();
}

BindgenQualType Decl_getResultType(const Decl *D, ASTContext *Ctx) {
  if (auto *MD = dyn_cast_or_null<ObjCMethodDecl>(D))
    return make_type_compatible(MD->getReturnType());

  return Type_getResultType(Decl_getType(D, Ctx));
}

BindgenStringRef Expr_getSpelling(const Expr *E) {
  if (auto *SL = dyn_cast_or_null<StringLiteral>(&*E)) {
    SmallString<256> Buf;
    llvm::raw_svector_ostream OS(Buf);
    SL->outputString(OS);
    return stringref(OS.str());
  }
  if (auto *D = getDeclFromExpr(&*E))
    return Decl_getSpelling(D);

  return stringref();
}

CXCursorKind Expr_getCXCursorKind(const Expr *E) {
  switch (E->getStmtClass()) {
  case Stmt::NoStmtClass:
    return CXCursor_NotImplemented;
  case Stmt::CaseStmtClass:
    return CXCursor_CaseStmt;
  case Stmt::DefaultStmtClass:
    return CXCursor_DefaultStmt;
  case Stmt::IfStmtClass:
    return CXCursor_IfStmt;
  case Stmt::SwitchStmtClass:
    return CXCursor_SwitchStmt;
  case Stmt::WhileStmtClass:
    return CXCursor_WhileStmt;
  case Stmt::DoStmtClass:
    return CXCursor_DoStmt;
  case Stmt::ForStmtClass:
    return CXCursor_ForStmt;
  case Stmt::GotoStmtClass:
    return CXCursor_GotoStmt;
  case Stmt::IndirectGotoStmtClass:
    return CXCursor_IndirectGotoStmt;
  case Stmt::ContinueStmtClass:
    return CXCursor_ContinueStmt;
  case Stmt::BreakStmtClass:
    return CXCursor_BreakStmt;
  case Stmt::ReturnStmtClass:
    return CXCursor_ReturnStmt;
  case Stmt::GCCAsmStmtClass:
    return CXCursor_GCCAsmStmt;
  case Stmt::MSAsmStmtClass:
    return CXCursor_MSAsmStmt;
  case Stmt::ObjCAtTryStmtClass:
    return CXCursor_ObjCAtTryStmt;
  case Stmt::ObjCAtCatchStmtClass:
    return CXCursor_ObjCAtCatchStmt;
  case Stmt::ObjCAtFinallyStmtClass:
    return CXCursor_ObjCAtFinallyStmt;
  case Stmt::ObjCAtThrowStmtClass:
    return CXCursor_ObjCAtThrowStmt;
  case Stmt::ObjCAtSynchronizedStmtClass:
    return CXCursor_ObjCAtSynchronizedStmt;
  case Stmt::ObjCAutoreleasePoolStmtClass:
    return CXCursor_ObjCAutoreleasePoolStmt;
  case Stmt::ObjCForCollectionStmtClass:
    return CXCursor_ObjCForCollectionStmt;
  case Stmt::CXXCatchStmtClass:
    return CXCursor_CXXCatchStmt;
  case Stmt::CXXTryStmtClass:
    return CXCursor_CXXTryStmt;
  case Stmt::CXXForRangeStmtClass:
    return CXCursor_CXXForRangeStmt;
  case Stmt::SEHTryStmtClass:
    return CXCursor_SEHTryStmt;
  case Stmt::SEHExceptStmtClass:
    return CXCursor_SEHExceptStmt;
  case Stmt::SEHFinallyStmtClass:
    return CXCursor_SEHFinallyStmt;
  case Stmt::SEHLeaveStmtClass:
    return CXCursor_SEHLeaveStmt;

  case Stmt::CoroutineBodyStmtClass:
  case Stmt::CoreturnStmtClass:
    return CXCursor_UnexposedStmt;

  case Stmt::OpaqueValueExprClass:
    if (auto *Src = cast<OpaqueValueExpr>(&*E)->getSourceExpr())
      return Expr_getCXCursorKind(Src);
    return CXCursor_UnexposedExpr;

  case Stmt::PseudoObjectExprClass:
    return Expr_getCXCursorKind(
        cast<PseudoObjectExpr>(&*E)->getSyntacticForm());

  case Stmt::CompoundStmtClass:
    return CXCursor_CompoundStmt;

  case Stmt::NullStmtClass:
    return CXCursor_NullStmt;

  case Stmt::LabelStmtClass:
    return CXCursor_LabelStmt;

  case Stmt::AttributedStmtClass:
    return CXCursor_UnexposedStmt;

  case Stmt::DeclStmtClass:
    return CXCursor_DeclStmt;

  case Stmt::CapturedStmtClass:
    return CXCursor_UnexposedStmt;

  case Stmt::IntegerLiteralClass:
    return CXCursor_IntegerLiteral;

#if CLANG_VERSION_MAJOR > 7
  case Stmt::FixedPointLiteralClass:
    return CXCursor_FixedPointLiteral;
#endif // CLANG_VERSION_MAJOR > 7

  case Stmt::FloatingLiteralClass:
    return CXCursor_FloatingLiteral;

  case Stmt::ImaginaryLiteralClass:
    return CXCursor_ImaginaryLiteral;

  case Stmt::StringLiteralClass:
    return CXCursor_StringLiteral;

  case Stmt::CharacterLiteralClass:
    return CXCursor_CharacterLiteral;

#if CLANG_VERSION_MAJOR > 7
  case Stmt::ConstantExprClass:
    return Expr_getCXCursorKind(cast<ConstantExpr>(&*E)->getSubExpr());
#endif // CLANG_VERSION_MAJOR > 7

  case Stmt::ParenExprClass:
    return CXCursor_ParenExpr;

  case Stmt::UnaryOperatorClass:
    return CXCursor_UnaryOperator;

  case Stmt::UnaryExprOrTypeTraitExprClass:
  case Stmt::CXXNoexceptExprClass:
    return CXCursor_UnaryExpr;

  case Stmt::MSPropertySubscriptExprClass:
  case Stmt::ArraySubscriptExprClass:
    return CXCursor_ArraySubscriptExpr;

  case Stmt::OMPArraySectionExprClass:
    return CXCursor_OMPArraySectionExpr;

  case Stmt::BinaryOperatorClass:
    return CXCursor_BinaryOperator;

  case Stmt::CompoundAssignOperatorClass:
    return CXCursor_CompoundAssignOperator;

  case Stmt::ConditionalOperatorClass:
    return CXCursor_ConditionalOperator;

  case Stmt::CStyleCastExprClass:
    return CXCursor_CStyleCastExpr;

  case Stmt::CompoundLiteralExprClass:
    return CXCursor_CompoundLiteralExpr;

  case Stmt::InitListExprClass:
    return CXCursor_InitListExpr;

  case Stmt::AddrLabelExprClass:
    return CXCursor_AddrLabelExpr;

  case Stmt::StmtExprClass:
    return CXCursor_StmtExpr;

  case Stmt::GenericSelectionExprClass:
    return CXCursor_GenericSelectionExpr;

  case Stmt::GNUNullExprClass:
    return CXCursor_GNUNullExpr;

  case Stmt::CXXStaticCastExprClass:
    return CXCursor_CXXStaticCastExpr;

  case Stmt::CXXDynamicCastExprClass:
    return CXCursor_CXXDynamicCastExpr;

  case Stmt::CXXReinterpretCastExprClass:
    return CXCursor_CXXReinterpretCastExpr;

  case Stmt::CXXConstCastExprClass:
    return CXCursor_CXXConstCastExpr;

  case Stmt::CXXFunctionalCastExprClass:
    return CXCursor_CXXFunctionalCastExpr;

  case Stmt::CXXTypeidExprClass:
    return CXCursor_CXXTypeidExpr;

  case Stmt::CXXBoolLiteralExprClass:
    return CXCursor_CXXBoolLiteralExpr;

  case Stmt::CXXNullPtrLiteralExprClass:
    return CXCursor_CXXNullPtrLiteralExpr;

  case Stmt::CXXThisExprClass:
    return CXCursor_CXXThisExpr;

  case Stmt::CXXThrowExprClass:
    return CXCursor_CXXThrowExpr;

  case Stmt::CXXNewExprClass:
    return CXCursor_CXXNewExpr;

  case Stmt::CXXDeleteExprClass:
    return CXCursor_CXXDeleteExpr;

  case Stmt::ObjCStringLiteralClass:
    return CXCursor_ObjCStringLiteral;

  case Stmt::ObjCEncodeExprClass:
    return CXCursor_ObjCEncodeExpr;

  case Stmt::ObjCSelectorExprClass:
    return CXCursor_ObjCSelectorExpr;

  case Stmt::ObjCProtocolExprClass:
    return CXCursor_ObjCProtocolExpr;
      case Stmt::ObjCBoolLiteralExprClass:
    return CXCursor_ObjCBoolLiteralExpr;

#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case Stmt::ObjCAvailabilityCheckExprClass:
    return CXCursor_ObjCAvailabilityCheckExpr;
#endif // CLANG_VERSION > 3.8

  case Stmt::ObjCBridgedCastExprClass:
    return CXCursor_ObjCBridgedCastExpr;

  case Stmt::BlockExprClass:
    return CXCursor_BlockExpr;

  case Stmt::PackExpansionExprClass:
    return CXCursor_PackExpansionExpr;

  case Stmt::SizeOfPackExprClass:
    return CXCursor_SizeOfPackExpr;

  case Stmt::DeclRefExprClass:
    return CXCursor_DeclRefExpr;

  case Stmt::DependentScopeDeclRefExprClass:
  case Stmt::SubstNonTypeTemplateParmExprClass:
  case Stmt::SubstNonTypeTemplateParmPackExprClass:
  case Stmt::FunctionParmPackExprClass:
  case Stmt::UnresolvedLookupExprClass:
  case Stmt::TypoExprClass: // A typo could actually be a DeclRef or a MemberRef
    return CXCursor_DeclRefExpr;

  case Stmt::CXXDependentScopeMemberExprClass:
  case Stmt::CXXPseudoDestructorExprClass:
  case Stmt::MemberExprClass:
  case Stmt::MSPropertyRefExprClass:
  case Stmt::ObjCIsaExprClass:
  case Stmt::ObjCIvarRefExprClass:
  case Stmt::ObjCPropertyRefExprClass:
  case Stmt::UnresolvedMemberExprClass:
    return CXCursor_MemberRefExpr;

  case Stmt::CallExprClass:
  case Stmt::CXXOperatorCallExprClass:
  case Stmt::CXXMemberCallExprClass:
  case Stmt::CUDAKernelCallExprClass:
  case Stmt::CXXConstructExprClass:
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case Stmt::CXXInheritedCtorInitExprClass:
#endif // CLANG_VERSION > 3.8
  case Stmt::CXXTemporaryObjectExprClass:
  case Stmt::CXXUnresolvedConstructExprClass:
  case Stmt::UserDefinedLiteralClass:
    return CXCursor_CallExpr;

  case Stmt::LambdaExprClass:
    return CXCursor_LambdaExpr;

  case Stmt::ObjCMessageExprClass:
    return CXCursor_ObjCMessageExpr;

  case Stmt::MSDependentExistsStmtClass:
    return CXCursor_UnexposedStmt;
  case Stmt::OMPParallelDirectiveClass:
    return CXCursor_OMPParallelDirective;
  case Stmt::OMPSimdDirectiveClass:
    return CXCursor_OMPSimdDirective;
  case Stmt::OMPForDirectiveClass:
    return CXCursor_OMPForDirective;
  case Stmt::OMPForSimdDirectiveClass:
    return CXCursor_OMPForSimdDirective;
  case Stmt::OMPSectionsDirectiveClass:
    return CXCursor_OMPSectionsDirective;
  case Stmt::OMPSectionDirectiveClass:
    return CXCursor_OMPSectionDirective;
  case Stmt::OMPSingleDirectiveClass:
    return CXCursor_OMPSingleDirective;
  case Stmt::OMPMasterDirectiveClass:
    return CXCursor_OMPMasterDirective;
  case Stmt::OMPCriticalDirectiveClass:
    return CXCursor_OMPCriticalDirective;
  case Stmt::OMPParallelForDirectiveClass:
    return CXCursor_OMPParallelForDirective;
  case Stmt::OMPParallelForSimdDirectiveClass:
    return CXCursor_OMPParallelForSimdDirective;
  case Stmt::OMPParallelSectionsDirectiveClass:
    return CXCursor_OMPParallelSectionsDirective;
  case Stmt::OMPTaskDirectiveClass:
    return CXCursor_OMPTaskDirective;
  case Stmt::OMPTaskyieldDirectiveClass:
    return CXCursor_OMPTaskyieldDirective;
  case Stmt::OMPBarrierDirectiveClass:
    return CXCursor_OMPBarrierDirective;
  case Stmt::OMPTaskwaitDirectiveClass:
    return CXCursor_OMPTaskwaitDirective;
  case Stmt::OMPTaskgroupDirectiveClass:
    return CXCursor_OMPTaskgroupDirective;
  case Stmt::OMPFlushDirectiveClass:
    return CXCursor_OMPFlushDirective;
  case Stmt::OMPOrderedDirectiveClass:
    return CXCursor_OMPOrderedDirective;
  case Stmt::OMPAtomicDirectiveClass:
    return CXCursor_OMPAtomicDirective;
  case Stmt::OMPTargetDirectiveClass:
    return CXCursor_OMPTargetDirective;
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case Stmt::OMPTargetDataDirectiveClass:
    return CXCursor_OMPTargetDataDirective;
  case Stmt::OMPTargetEnterDataDirectiveClass:
    return CXCursor_OMPTargetEnterDataDirective;
  case Stmt::OMPTargetExitDataDirectiveClass:
    return CXCursor_OMPTargetExitDataDirective;
  case Stmt::OMPTargetParallelDirectiveClass:
    return CXCursor_OMPTargetParallelDirective;
  case Stmt::OMPTargetParallelForDirectiveClass:
    return CXCursor_OMPTargetParallelForDirective;
  case Stmt::OMPTargetUpdateDirectiveClass:
    return CXCursor_OMPTargetUpdateDirective;
#endif // CLANG_VERSION > 3.8
  case Stmt::OMPTeamsDirectiveClass:
    return CXCursor_OMPTeamsDirective;
  case Stmt::OMPCancellationPointDirectiveClass:
    return CXCursor_OMPCancellationPointDirective;
  case Stmt::OMPCancelDirectiveClass:
    return CXCursor_OMPCancelDirective;
  case Stmt::OMPTaskLoopDirectiveClass:
    return CXCursor_OMPTaskLoopDirective;
  case Stmt::OMPTaskLoopSimdDirectiveClass:
    return CXCursor_OMPTaskLoopSimdDirective;
  case Stmt::OMPDistributeDirectiveClass:
    return CXCursor_OMPDistributeDirective;
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
  case Stmt::OMPDistributeParallelForDirectiveClass:
    return CXCursor_OMPDistributeParallelForDirective;
  case Stmt::OMPDistributeParallelForSimdDirectiveClass:
    return CXCursor_OMPDistributeParallelForSimdDirective;
  case Stmt::OMPDistributeSimdDirectiveClass:
    return CXCursor_OMPDistributeSimdDirective;
  case Stmt::OMPTargetParallelForSimdDirectiveClass:
    return CXCursor_OMPTargetParallelForSimdDirective;
#endif // CLANG_VERSION > 3.8
#if CLANG_VERSION_MAJOR > 3
  case Stmt::OMPTargetSimdDirectiveClass:
    return CXCursor_OMPTargetSimdDirective;
  case Stmt::OMPTeamsDistributeDirectiveClass:
    return CXCursor_OMPTeamsDistributeDirective;
  case Stmt::OMPTeamsDistributeSimdDirectiveClass:
    return CXCursor_OMPTeamsDistributeSimdDirective;
  case Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass:
    return CXCursor_OMPTeamsDistributeParallelForSimdDirective;
  case Stmt::OMPTeamsDistributeParallelForDirectiveClass:
    return CXCursor_OMPTeamsDistributeParallelForDirective;
  case Stmt::OMPTargetTeamsDirectiveClass:
    return CXCursor_OMPTargetTeamsDirective;
  case Stmt::OMPTargetTeamsDistributeDirectiveClass:
    return CXCursor_OMPTargetTeamsDistributeDirective;
  case Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass:
    return CXCursor_OMPTargetTeamsDistributeParallelForDirective;
  case Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass:
    return CXCursor_OMPTargetTeamsDistributeParallelForSimdDirective;
  case Stmt::OMPTargetTeamsDistributeSimdDirectiveClass:
    return CXCursor_OMPTargetTeamsDistributeSimdDirective;
#endif // CLANG_VERSION_MAJOR > 3
#if CLANG_VERSION_MAJOR > 8
  case Stmt::BuiltinBitCastExprClass:
    return CXCursor_BuiltinBitCastExpr;
#endif // CLANG_VERSION_MAJOR > 8

  default:
    return CXCursor_UnexposedExpr;
  }
}

BindgenQualType Expr_getType(const Expr *E) { return make_type_compatible(E->getType()); }

BindgenSourceRange Expr_getSourceRange(const Expr *E) {
  return make_sourcerange(E->getSourceRange());
}

class BindgenVisitor : public RecursiveASTVisitor<BindgenVisitor> {
  ASTUnit &AST;
  Visitor VisitFn;
  CXClientData Data;
  Node Parent;

public:
  explicit BindgenVisitor(ASTUnit &AST, Visitor V, CXClientData data) : AST(AST), VisitFn(V), Data(data) {}

  bool shouldVisitImplicitCode() {
    return true;
  }

  void setParent(Node n) {
    Parent = n;
  }

  bool TraverseDeclTyped(Decl *D, CXCursorKind kind) {
    if (!D)
      return false;

    bool skip = !Parent;

    // D->dump();
    // libclang doesn't visit the CXXRecordDecl inside ClassTemplateDecl nodes
    if (Parent.kind == CXCursor_ClassTemplate
        && isa<CXXRecordDecl>(D))
      skip = true;

    // libclang exposes forward class and protocol declarations as references
    if (kind == CXCursor_ObjCInterfaceDecl) {
      auto *ID = cast<ObjCInterfaceDecl>(D);
      if (!ID->isThisDeclarationADefinition())
        kind = CXCursor_ObjCClassRef;
    } else if (kind == CXCursor_ObjCProtocolDecl) {
      auto *PD = cast<ObjCProtocolDecl>(D);
      if (!PD->isThisDeclarationADefinition())
        kind = CXCursor_ObjCProtocolRef;
    }

    // D->dump();
    Node node(D, kind);
    if (!skip) {
      // We don't want to visit implicit decls or their children
      if (D->isImplicit() && !isa<ObjCMethodDecl>(D))
        return true;

      switch (VisitFn(node, Parent, &AST, Data)) {
      case CXChildVisit_Break:
        return false;
      case CXChildVisit_Continue:
        return true;
      case CXChildVisit_Recurse:
        break;
      }
    }

    // Do not recurse through references
    if (kind >= CXCursor_FirstRef && kind <= CXCursor_LastRef
        && kind != CXCursor_CXXBaseSpecifier)
      return true;

    auto OldParent = Parent;
    Parent = node;
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseDecl(D);
    Parent = OldParent;
    return res;
  }

  bool TraverseDecl(Decl *D) {
    return TraverseDeclTyped(D, Decl_getCXCursorKind(D));
  }

  bool TraverseExprTyped(Expr *E, CXCursorKind kind) {
    if (!E)
      return true;

    Node node(E, kind);
    if (Parent) {
      // E->dump();
      switch (VisitFn(node, Parent, &AST, Data)) {
      case CXChildVisit_Break:
        return false;
      case CXChildVisit_Continue:
        return true;
      case CXChildVisit_Recurse:
        break;
      }
    }

    auto OldParent = Parent;
    Parent = node;
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseStmt(E);
    Parent = OldParent;
    return res;
  }

  bool TraverseStmt(Stmt *S) {
    if (!S)
      return true;
    if (auto *E = dyn_cast<Expr>(S))
      return TraverseExprTyped(E, Expr_getCXCursorKind(E));
    return RecursiveASTVisitor<BindgenVisitor>::TraverseStmt(S);
  }

  bool VisitSizeOfPackExpr(SizeOfPackExpr *E) {
    NamedDecl *Pack = E->getPack();
    if (isa<TemplateTypeParmDecl>(Pack)) {
      Node node(Pack, CXCursor_TypeRef);
      Node parent(E, Expr_getCXCursorKind(E));
      return VisitFn(node, parent, &AST, Data) != CXChildVisit_Break;
    }
    if (isa<TemplateTemplateParmDecl>(Pack)) {
      Node node(Pack, CXCursor_TemplateRef);
      Node parent(E, Expr_getCXCursorKind(E));
      return VisitFn(node, parent, &AST, Data) != CXChildVisit_Break;
    }
    return true;
  }

  bool VisitTypeLoc(TypeLoc TL) {
    // if (TL) TL.getTypePtr()->dump();
    return true;
  }

  bool VisitDecltypeTypeLoc(DecltypeTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseStmt(TL.getUnderlyingExpr());
  }

  bool VisitTypeOfExprTypeLoc(TypeOfExprTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseStmt(TL.getUnderlyingExpr());
  }

  bool VisitTypedefTypeLoc(TypedefTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseDeclTyped(TL.getTypedefNameDecl(), CXCursor_TypeRef);
  }

  bool VisitInjectedClassNameTypeLoc(InjectedClassNameTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseDeclTyped(TL.getDecl(), CXCursor_TypeRef);
  }

  bool VisitUnresolvedUsingTypeLoc(UnresolvedUsingTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseDeclTyped(TL.getDecl(), CXCursor_TypeRef);
  }

  bool VisitTagTypeLoc(TagTypeLoc TL) {
    if (!TL)
      return true;
    if (TL.isDefinition())
      return TraverseDecl(TL.getDecl());
    else
      return TraverseDeclTyped(TL.getDecl(), CXCursor_TypeRef);
  }

  bool VisitRecordTypeLoc(RecordTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseDeclTyped(TL.getDecl(), CXCursor_TypeRef);
  }

  bool VisitEnumTypeLoc(EnumTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseDeclTyped(TL.getDecl(), CXCursor_TypeRef);
  }

  bool VisitTemplateTypeParmTypeLoc(TemplateTypeParmTypeLoc TL) {
    if (!TL)
      return true;
    return TraverseDeclTyped(TL.getDecl(), CXCursor_TypeRef);
  }

  bool VisitBuiltinTypeLoc(BuiltinTypeLoc TL) {
    if (!TL)
      return true;

    ASTContext &Context = AST.getASTContext();
    QualType Ty;
    switch (TL.getTypePtr()->getKind()) {
    case BuiltinType::ObjCId:
      Ty = Context.getObjCIdType();
      break;
    case BuiltinType::ObjCClass:
      Ty = Context.getObjCClassType();
      break;
    case BuiltinType::ObjCSel:
      Ty = Context.getObjCSelType();
      break;
    default:
      break;
    }

    if (!Ty.isNull()) {
      if (auto *TD = Ty->getAs<TypedefType>()) {
        Node node(TD->getDecl(), CXCursor_TypeRef);
        return VisitFn(node, Parent, &AST, Data) != CXChildVisit_Break;
      }
    }

    return true;
  }

  bool VisitObjCInterfaceTypeLoc(ObjCInterfaceTypeLoc TL) {
    if (!TL)
      return true;

    return TraverseDeclTyped(TL.getIFaceDecl(), CXCursor_ObjCClassRef);
  }

#if CLANG_VERSION_MAJOR > 3
  bool VisitObjCTypeParamTypeLoc(ObjCTypeParamTypeLoc TL) {
    if (!TL)
      return true;
    Node node(TL.getDecl(), CXCursor_TypeRef);
    if (VisitFn(node, Parent, &AST, Data) == CXChildVisit_Break)
      return false;
    for (unsigned I = 0, N = TL.getNumProtocols(); I != N; ++I) {
      Node node(TL.getProtocol(I), CXCursor_ObjCProtocolRef);
      if (VisitFn(node, Parent, &AST, Data) == CXChildVisit_Break)
        return false;
    }
    return true;
  }
#endif // CLANG_VERSION_MAJOR > 3

  bool VisitObjCObjectTypeLoc(ObjCObjectTypeLoc TL) {
    if (!TL)
      return true;

    for (unsigned I = 0, N = TL.getNumProtocols(); I != N; ++I) {
      Node node(TL.getProtocol(I), CXCursor_ObjCProtocolRef);
      if (VisitFn(node, Parent, &AST, Data) == CXChildVisit_Break)
        return false;
    }

    return true;
  }

  bool TraverseTemplateName(TemplateName Name) {
    Node node;
    switch (Name.getKind()) {
    case TemplateName::Template:
      node = Node(Name.getAsTemplateDecl(), CXCursor_TemplateRef);
      break;

    case TemplateName::OverloadedTemplate:
      // libclang visits this, but we don't need it for bindgen
      return true;

#if CLANG_VERSION_MAJOR > 8
    case TemplateName::AssumedTemplate:
      return true;
#endif // CLANG_VERSION_MAJOR > 8

    case TemplateName::DependentTemplate:
      return true;

    case TemplateName::QualifiedTemplate:
      node = Node(Name.getAsQualifiedTemplateName()->getDecl(), CXCursor_TemplateRef);
      break;

    case TemplateName::SubstTemplateTemplateParm:
      node = Node(Name.getAsSubstTemplateTemplateParm()->getParameter(), CXCursor_TemplateRef);
      break;

    case TemplateName::SubstTemplateTemplateParmPack:
      node = Node(Name.getAsSubstTemplateTemplateParmPack()->getParameterPack(), CXCursor_TemplateRef);
      break;
    }

    switch (VisitFn(node, Parent, &AST, Data)) {
    case CXChildVisit_Break:
      return false;
    case CXChildVisit_Continue:
      return true;
    case CXChildVisit_Recurse:
      break;
    }

    auto OldParent = Parent;
    Parent = node;
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseTemplateName(Name);
    Parent = OldParent;
    return res;
  }

#if CLANG_VERSION_MAJOR > 5

  bool TraverseCXXBaseSpecifier(const CXXBaseSpecifier &Base) {
    if (Parent) {
      switch (VisitFn(Node(&Base), Parent, &AST, Data)) {
      case CXChildVisit_Break:
        return false;
      case CXChildVisit_Continue:
        return true;
      case CXChildVisit_Recurse:
        break;
      }
    }

    auto OldParent = Parent;
    Parent = Node(&Base);
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseCXXBaseSpecifier(Base);
    Parent = OldParent;
    return res;
  }

#else // CLANG_VERSION_MAJOR <= 5

  // We need to visit the record decl instead of the base specifier here because
  // clang <= 5 doesn't traverse CXXBaseSpecifiers
  bool TraverseCXXRecordDecl(CXXRecordDecl *D) {
    // D->dump();
    if (D && D->isCompleteDefinition()) {
      bool recurse = false;
      for (const auto &I : D->bases()) {
        switch (VisitFn(Node(&I), Node(D, CXCursor_ClassDecl), &AST, Data)) {
        case CXChildVisit_Break:
          return false;
        case CXChildVisit_Continue:
          break;
        case CXChildVisit_Recurse:
          // Skip siblings and move on to other children of D
          recurse = true;
          break;
        }
        if (recurse)
          break;
        if (!TraverseTypeLoc(I.getTypeSourceInfo()->getTypeLoc()))
          return false;
      }
    }

    auto OldParent = Parent;
    Parent = Node(D, CXCursor_ClassDecl);
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseCXXRecordDecl(D);
    Parent = OldParent;
    return res;
  }

#endif // CLANG_VERSION_MAJOR > 5

#if CLANG_VERSION_MAJOR < 6
  bool TraverseClassTemplateDecl(ClassTemplateDecl *D) {
    auto params = D->getTemplateParameters();
    if (params) {
      for (const auto &P : *params) {
        if (!TraverseDecl(P))
          return false;
      }
    }

    return TraverseCXXRecordDecl(D->getTemplatedDecl());
  }

  bool TraverseVarTemplateDecl(VarTemplateDecl *D) {
    auto params = D->getTemplateParameters();
    if (params) {
      for (const auto &P : *params) {
        if (!TraverseDecl(P))
          return false;
      }
    }

    return TraverseVarDecl(D->getTemplatedDecl());
  }

  bool TraverseFunctionTemplateDecl(FunctionTemplateDecl *D) {
    auto params = D->getTemplateParameters();
    if (params) {
      for (const auto &P : *params) {
        if (!TraverseDecl(P))
          return false;
      }
    }

    return TraverseFunctionDecl(D->getTemplatedDecl());
  }
#endif // CLANG_VERSION_MAJOR < 6

  bool VisitAttr(Attr *A) {
    if (Parent)
      VisitFn(Node(A), Parent, &AST, Data);
    return true;
  }

  bool VisitTranslationUnitDecl(TranslationUnitDecl *TU) {
    if (!AST.getPreprocessor().getPreprocessingRecord())
      return true;

    PreprocessingRecord &PPRec =
        *AST.getPreprocessor().getPreprocessingRecord();
    SourceManager &SM = AST.getSourceManager();

    bool OnlyLocalDecls = !AST.isMainFileAST() && AST.getOnlyLocalDecls();

    if (OnlyLocalDecls)
      return visitPreprocessedEntities(PPRec.local_begin(), PPRec.local_end(),
                                       PPRec);

    return visitPreprocessedEntities(PPRec.begin(), PPRec.end(), PPRec);
  }

  bool VisitObjCInterfaceDecl(ObjCInterfaceDecl *D) {
    // We handle forward declarations in TraverseDecl
    if (!D || !D->isThisDeclarationADefinition())
      return true;

    if (D->getSuperClass()
        && VisitFn(Node(D->getSuperClass(), CXCursor_ObjCSuperClassRef), Parent, &AST, Data) == CXChildVisit_Break)
      return false;

    for (auto I : D->protocols())
      if (VisitFn(Node(&*I, CXCursor_ObjCProtocolRef), Parent, &AST, Data) == CXChildVisit_Break)
        return false;

    return true;
  }

  bool VisitObjCCategoryDecl(ObjCCategoryDecl *ND) {
    Node interfaceNode(ND->getClassInterface(), CXCursor_ObjCClassRef);
    if (VisitFn(interfaceNode, Parent, &AST, Data) == CXChildVisit_Break)
      return false;

    // TypeParamList is visited in RecursiveASTvisitor

    for (auto I : ND->protocols())
      if (VisitFn(Node(&*I, CXCursor_ObjCProtocolRef), Parent, &AST, Data) == CXChildVisit_Break)
        return false;

    // We may need to do the weird hacky thing that the libclang visitor does in
    // VisitObjCContainerDecl, but I hope not...
    return true;
  }

  bool VisitObjCProtocolDecl(ObjCProtocolDecl *PD) {
    for (auto I : PD->protocols())
      if (VisitFn(Node(&*I, CXCursor_ObjCProtocolRef), Parent, &AST, Data) == CXChildVisit_Break)
        return false;

    // We may need to do the weird hacky thing that the libclang visitor does in
    // VisitObjCContainerDecl, but I hope not...
    return true;
  }

  bool VisitObjCPropertyDecl(ObjCPropertyDecl *PD) {
    if (!PD)
      return true;

    // FIXME: This implements a workaround with @property declarations also
    // being
    // installed in the DeclContext for the @interface.  Eventually this code
    // should be removed.
    ObjCCategoryDecl *CDecl = dyn_cast<ObjCCategoryDecl>(PD->getDeclContext());
    if (!CDecl || !CDecl->IsClassExtension())
      return true;

    ObjCInterfaceDecl *ID = CDecl->getClassInterface();
    if (!ID)
      return true;

    IdentifierInfo *PropertyId = PD->getIdentifier();
    ObjCPropertyDecl *prevDecl =
#if CLANG_VERSION_MAJOR > 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR == 9)
      ObjCPropertyDecl::findPropertyDecl(cast<DeclContext>(ID), PropertyId, PD->getQueryKind());
#else // CLANG_VERSION <= 3.8
      ObjCPropertyDecl::findPropertyDecl(cast<DeclContext>(ID), PropertyId);
#endif

    if (!prevDecl)
      return true;

    // Visit synthesized methods since they will be skipped when visiting
    // the @interface.
    if (ObjCMethodDecl *MD = prevDecl->getGetterMethodDecl())
      if (MD->isPropertyAccessor() && MD->getLexicalDeclContext() == CDecl)
        if (!TraverseDecl(MD))
          return false;

    if (ObjCMethodDecl *MD = prevDecl->getSetterMethodDecl())
      if (MD->isPropertyAccessor() && MD->getLexicalDeclContext() == CDecl)
        if (!TraverseDecl(MD))
          return false;

    return true;
  }

  bool VisitObjCTypeParamDecl(ObjCTypeParamDecl *D) {
    if (!D)
      return true;
    return TraverseDecl(D) != CXChildVisit_Break;
  }

private:
  template <typename InputIterator>
  bool visitPreprocessedEntities(InputIterator First, InputIterator Last,
                                 PreprocessingRecord &PPRec) {
    for (; First != Last; ++First) {
      PreprocessedEntity *PPE = *First;
      if (!PPE)
        continue;

      Node node;
      if (isa<MacroExpansion>(PPE)) {
        node = Node(PPE, CXCursor_MacroExpansion);
      } else if (isa<MacroDefinitionRecord>(PPE)) {
        node = Node(PPE, CXCursor_MacroDefinition);
      } else if (isa<InclusionDirective>(PPE)) {
        node = Node(PPE, CXCursor_InclusionDirective);
      }
      if (node) {
        if (VisitFn(node, Parent, &AST, Data) == CXChildVisit_Break)
          return false;
      }
    }

    return true;
  }
};

void Decl_visitChildren(const Decl *Parent, CXCursorKind kind, Visitor V,
                        ASTUnit *Unit, CXClientData data) {
  BindgenVisitor visitor(*Unit, V, data);
  visitor.TraverseDeclTyped(const_cast<Decl *>(&*Parent), kind);
}
void Expr_visitChildren(const Expr *Parent, CXCursorKind kind, Visitor V,
                        ASTUnit *Unit, CXClientData data) {
  BindgenVisitor visitor(*Unit, V, data);
  visitor.TraverseExprTyped(const_cast<Expr *>(&*Parent), kind);
}
void CXXBaseSpecifier_visitChildren(const CXXBaseSpecifier *Parent,
                                    CXCursorKind kind, Visitor V, ASTUnit *Unit,
                                    CXClientData data) {
  BindgenVisitor visitor(*Unit, V, data);
#if CLANG_VERSION_MAJOR > 5
  visitor.TraverseCXXBaseSpecifier(*Parent);
#else
  // Clang <= 5 doesn't have RecursiveASTvisitor::TraverseCXXBaseSpecifier
  visitor.setParent(Node(Parent));
  visitor.TraverseTypeLoc(Parent->getTypeSourceInfo()->getTypeLoc());
#endif
}

void disposeTokens(const ASTUnit *TU, CXToken *Tokens, unsigned NumTokens) {
  delete[] Tokens;
}

BindgenStringRef Type_getTypeSpelling(BindgenQualType T, ASTContext *Context) {
  auto QT = QualType::getFromOpaquePtr(T);
  SmallString<64> Str;
  llvm::raw_svector_ostream OS(Str);
  PrintingPolicy PP(Context->getLangOpts());

  QT.print(OS, PP);
  return stringref(OS.str());
}

bool Type_isConstQualifiedType(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  return QT.isLocalConstQualified();
}

int Type_getNumTemplateArguments(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return -1;
  auto TA = GetTemplateArguments(QT);
  if (!TA)
    return -1;

  return GetTemplateArgumentArraySize(TA.getValue());
}

BindgenQualType Type_getArgType(BindgenQualType T, unsigned i) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return nullptr;;

  if (const FunctionProtoType *FD = QT->getAs<FunctionProtoType>()) {
    unsigned numParams = FD->getNumParams();
    if (i >= numParams)
      return nullptr;
    return make_type_compatible(FD->getParamType(i));
  }

  return nullptr;
}

int Type_getNumArgTypes(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return -1;

  if (const FunctionProtoType *FD = QT->getAs<FunctionProtoType>()) {
    return FD->getNumParams();
  }

  if (QT->getAs<FunctionNoProtoType>()) {
    return 0;
  }

  return -1;
}

BindgenQualType Type_getCanonicalType(BindgenQualType T, ASTContext *Context) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return nullptr;

  return make_type_compatible(Context->getCanonicalType(QT));
}

bool Type_isFunctionTypeVariadic(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return false;

  if (const FunctionProtoType *FD = QT->getAs<FunctionProtoType>())
    return (unsigned)FD->isVariadic();

  if (QT->getAs<FunctionNoProtoType>())
    return true;

  return false;
}

BindgenQualType Type_getResultType(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return nullptr;

  if (const FunctionType *FD = QT->getAs<FunctionType>())
    return make_type_compatible(FD->getReturnType());

  return nullptr;
}

BindgenQualType Type_getNamedType(BindgenQualType T) {
  auto QT = QualType::getFromOpaquePtr(T);
  const Type *TP = QT.getTypePtrOrNull();

  if (TP && TP->getTypeClass() == Type::Elaborated)
    return make_type_compatible(cast<ElaboratedType>(TP)->getNamedType());

  return nullptr;
}

BindgenQualType Type_getTemplateArgumentAsType(BindgenQualType T, unsigned index) {
  auto QT = QualType::getFromOpaquePtr(T);
  if (QT.isNull())
    return nullptr;

  auto TA = GetTemplateArguments(QT);
  if (!TA)
    return nullptr;

  Optional<QualType> ArgQT = FindTemplateArgumentTypeAt(TA.getValue(), index);
  return make_type_compatible(ArgQT.getValueOr(QualType()));
}

unsigned Comment_getNumChildren(const comments::Comment *C) {
  if (!C)
    return 0;

  return C->child_count();
}

comments::Comment *Comment_getChild(const comments::Comment *C, unsigned index) {
  if (!C || index >= C->child_count())
    return nullptr;

  return *(C->child_begin() + index);
}

BindgenStringRef HTMLTagComment_getTagName(const comments::Comment *C) {
  if (auto *HTML = dyn_cast_or_null<comments::HTMLTagComment>(C)) {
    return stringref(HTML->getTagName());
  } else {
    return stringref();
  }
}

unsigned HTMLStartTag_getNumAttrs(const comments::Comment *C) {
  if (auto *HTML = dyn_cast_or_null<comments::HTMLStartTagComment>(C)) {
    return HTML->getNumAttrs();
  } else {
    return 0;
  }
}

BindgenStringRef HTMLStartTag_getAttrName(const comments::Comment *C, unsigned i) {
  if (auto *HTML = dyn_cast_or_null<comments::HTMLStartTagComment>(C)) {
    return stringref(HTML->getAttr(i).Name);
  } else {
    return stringref();
  }
}

BindgenStringRef HTMLStartTag_getAttrValue(const comments::Comment *C, unsigned i) {
  if (auto *HTML = dyn_cast_or_null<comments::HTMLStartTagComment>(C)) {
    return stringref(HTML->getAttr(i).Value);
  } else {
    return stringref();
  }
}

BindgenStringRef FileEntry_getName(const FileEntry *F) {
  if (!F)
    return stringref();
  return stringref(F->getName());
}

BindgenStringRef getClangVersion() {
  return stringref(getClangFullVersion());
}

bool CXXBaseSpecifier_isVirtualBase(const CXXBaseSpecifier *B) {
  return B && B->isVirtual();
}

BindgenQualType CXXBaseSpecifier_getType(const CXXBaseSpecifier *B) {
  if (!B)
    return nullptr;
  return make_type_compatible(B->getType());
}

BindgenStringRef CXXBaseSpecifier_getSpelling(const CXXBaseSpecifier *B) {
  return stringref(B->getType().getAsString());
}

SourceLocation *CXXBaseSpecifier_getLocation(const CXXBaseSpecifier *B) {
#if CLANG_VERSION_MAJOR > 4
  return new SourceLocation(B->getBaseTypeLoc());
#else // CLANG_VERSION_MAJOR <= 4
  return new SourceLocation(B->getLocStart());
#endif
}

SourceLocation *Attr_getLocation(const Attr *A) {
  return new SourceLocation(A->getLocation());
}

SourceLocation *PreprocessedEntity_getLocation(const PreprocessedEntity *PPE) {
  return new SourceLocation(PPE->getSourceRange().getBegin());
}

BindgenSourceRange CXXBaseSpecifier_getSourceRange(const CXXBaseSpecifier *B) {
  return make_sourcerange(B->getSourceRange());
}

BindgenSourceRange Attr_getSourceRange(const Attr *A) {
  return make_sourcerange(A->getRange());
}

BindgenSourceRange PreprocessedEntity_getSourceRange(const PreprocessedEntity *PPE) {
  return make_sourcerange(PPE->getSourceRange());
}

BindgenStringRef PreprocessedEntity_getSpelling(const PreprocessedEntity *PPE) {
  if (!PPE)
    return stringref();
  if (const auto *MDR = dyn_cast_or_null<MacroDefinitionRecord>(PPE))
      return stringref(MDR->getName()->getName());
  if (const auto *ME = dyn_cast<MacroExpansion>(PPE))
      return stringref(ME->getName()->getName());
  return stringref();
}

const FileEntry *PreprocessedEntity_getIncludedFile(const PreprocessedEntity *PPE) {
  if (const auto *ID = dyn_cast_or_null<InclusionDirective>(PPE))
    return ID->getFile();
  return nullptr;
}

CX_CXXAccessSpecifier Decl_getAccess(const Decl *D) {
  auto spec = AS_none;
  if (D)
    spec = D->getAccess();
  return TranslateCXXAccessSpecifier(spec);
}

CX_CXXAccessSpecifier CXXBaseSpecifier_getAccess(const CXXBaseSpecifier *B) {
  auto spec = AS_none;
  if (B)
    spec = B->getAccessSpecifier();
  return TranslateCXXAccessSpecifier(spec);
}
