#include <string>

#include "clang/AST/Comment.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/DeclObjC.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/ExprObjC.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Version.h"
#include "clang/Frontend/ASTUnit.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Index/USRGeneration.h"
#include "clang-c/Documentation.h"
#include "clang-c/Index.h"

#define BINDGEN_IMPLEMENTATION
#include "ClangAST.hpp"

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

// From libclang/CIndex.cpp
static const Decl *getDeclFromExpr(const Stmt *E) {
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
  if (const CXXInheritedCtorInitExpr *CE =
          dyn_cast<CXXInheritedCtorInitExpr>(E))
    return CE->getConstructor();
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

BindgenSourceRange::BindgenSourceRange(const SourceRange &range) {
  B = new SourceLocation(range.getBegin());
  E = new SourceLocation(range.getEnd());
}

BindgenStringRefSet make_stringrefset(std::vector<std::string> &string_vec) {
  BindgenStringRefSet set;
  set.len = string_vec.size();
  set.strings = new BindgenStringRef[set.len];
  for (size_t i = 0; i < set.len; ++i)
    set.strings[i] = stringref(string_vec[i]);
  return set;
}
  

void freeString(BindgenStringRef s) {
  delete[] s.s;
}

char *cString(BindgenStringRef s) { return s.s; }

ASTContext *ASTUnit_getContext(ASTUnit *Unit) {
  return &Unit->getASTContext();
}

ASTUnit *parseTranslationUnit(const char *source_filename,
                              const char *const *command_line_args,
                              int num_command_line_args,
                              int options) {

  ArrayRef<const char *> Args(command_line_args, num_command_line_args);

  // Configure the diagnostics.
  IntrusiveRefCntPtr<DiagnosticsEngine>
    Diags(CompilerInstance::createDiagnostics(new DiagnosticOptions));

  if (options & CXTranslationUnit_KeepGoing)
    Diags->setFatalsAsError(true);

  CaptureDiagsKind CaptureDiagnostics = CaptureDiagsKind::All;
  if (options & CXTranslationUnit_IgnoreNonErrorsFromIncludedFiles)
    CaptureDiagnostics = CaptureDiagsKind::AllWithoutNonErrorsFromIncludes;

  auto Invoc = createInvocationFromCommandLine(Args, Diags);
  if (!Invoc)
    return nullptr;

  return ASTUnit::LoadFromCompilerInvocationAction(
    std::move(Invoc), std::make_shared<PCHContainerOperations>(), Diags);
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
  return TI->getMaxPointerWidth();
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

EvalResult *Expr_Evaluate(const Expr *E, ASTContext *Ctx) {
  if (!E)
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
    } else {
      return new EvalResult(res.Val);
    }
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
  return ER->EvalType;
}

double EvalResult_getAsDouble(EvalResult *ER) {
  if (!ER)
    return 0;
  auto apFloat = ER->Val.getFloat();
  bool ignored;
  apFloat.convert(llvm::APFloat::IEEEdouble(),
                  llvm::APFloat::rmNearestTiesToEven, &ignored);
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

// ASTUnit *Decl_getTranslationUnit(const Decl *D) {
//   if (!D)
//     return nullptr;

//   return D.unit;
// }

const Decl *Decl_getDefinition(const Decl *D) {
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
  case Decl::Binding:
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
  case Decl::Export:
  case Decl::ObjCPropertyImpl:
  case Decl::FileScopeAsm:
  case Decl::StaticAssert:
  case Decl::Block:
  case Decl::Captured:
  case Decl::OMPCapturedExpr:
  case Decl::Label: // FIXME: Is this right??
  case Decl::ClassScopeFunctionSpecialization:
  case Decl::CXXDeductionGuide:
  case Decl::Import:
  case Decl::OMPThreadPrivate:
  case Decl::OMPAllocate:
  case Decl::OMPDeclareReduction:
  case Decl::OMPDeclareMapper:
  case Decl::OMPRequires:
  case Decl::ObjCTypeParam:
  case Decl::BuiltinTemplate:
  case Decl::PragmaComment:
  case Decl::PragmaDetectMismatch:
  case Decl::UsingPack:
  case Decl::Concept:
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
  case Decl::Decomposition: {
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
  case Decl::ConstructorUsingShadow:
    return Decl_getDefinition(
      cast<UsingShadowDecl>(&*D)->getTargetDecl());

  case Decl::ObjCMethod:
  case Decl::ObjCCategory:
  case Decl::ObjCProtocol:
  case Decl::ObjCInterface:
  case Decl::ObjCProperty:
  case Decl::ObjCCompatibleAlias:
    llvm_unreachable("Objective-C not implemented");
    break;

  case Decl::Friend:
    if (NamedDecl *Friend = cast<FriendDecl>(&*D)->getFriendDecl())
      return Decl_getDefinition(Friend);
    break;

  case Decl::FriendTemplate:
    if (NamedDecl *Friend = cast<FriendTemplateDecl>(&*D)->getFriendDecl())
      return Decl_getDefinition(Friend);
    break;
  }

  return nullptr;
}

const Decl *Decl_getReferenced(const Decl *D) {
  if (!D)
    return nullptr;

  // TODO(sjc): Handle UsingDecl?
  return D;
}

const Decl *Decl_getCanonical(const Decl *D) {
  if (!D)
    return nullptr;
  return D->getCanonicalDecl();
}

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

BindgenStringRef Decl_getSpelling(const Decl *D) {
  const NamedDecl *ND = dyn_cast_or_null<NamedDecl>(&*D);
  if (!ND)
    return stringref();

  SmallString<1024> S;
  llvm::raw_svector_ostream os(S);
  ND->printName(os);
  return stringref(S.str());
}

// BindgenStringRef Decl_getDisplayName(const Decl *D) {
// }

BindgenStringRef Decl_getMangling(const Decl *D, ASTContext *Ctx) {
  if (!D || !(isa<FunctionDecl>(&*D) || isa<VarDecl>(&*D)))
    return stringref();

  ASTNameGenerator ASTNameGen(*Ctx);
  return stringref(ASTNameGen.getName(&*D));
}

BindgenStringRefSet Decl_getCXXManglings(const Decl *D, ASTContext *Ctx) {
  if (!D || !(isa<CXXRecordDecl>(&*D) || isa<CXXMethodDecl>(&*D)))
    return BindgenStringRefSet();

  ASTNameGenerator ASTNameGen(*Ctx);
  std::vector<std::string> Manglings = ASTNameGen.getAllManglings(&*D);
  return make_stringrefset(Manglings);
}

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
    return FD->getDefinition() == &*D;
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

  const RawComment *RC = Ctx->getRawCommentForAnyRedecl(&*D);
  StringRef RawText = RC ? RC->getRawText(Ctx->getSourceManager()) :
                           StringRef();
  return stringref(RawText);
}

comments::Comment *Decl_getParsedComment(const Decl *D, ASTContext *Ctx) {
  if (!D)
    return nullptr;
  return Ctx->getCommentForDecl(&*D, /*PP=*/nullptr);
}

QualType Decl_getType(const Decl *D, ASTContext *Ctx) {
  auto ty = QualType();
  if (!D)
    return ty;

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
    return QualType();

  // libclang does not return AttributedTypes if
  // CXTranslationUnit_IncludeAttributedTypes is not set, and bindgen assumes it
  // is not set.
  if (auto *ATT = ty->getAs<AttributedType>())
    return ATT->getEquivalentType();

  // libclang does not return ParenTypes
  if (auto *PTT = ty->getAs<ParenType>())
    return PTT->getInnerType();

  return ty;
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

QualType Decl_getEnumDeclIntegerType(const Decl *D) {
  if (auto *TD = dyn_cast_or_null<EnumDecl>(&*D))
    return TD->getIntegerType();
  else
    return QualType();
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

static bool isTypeIncompleteForLayout(QualType QT) {
  return QT->isIncompleteType() && !QT->isIncompleteArrayType();
}

static long long visitRecordForValidation(const RecordDecl *RD) {
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
  return 0;
}

long long Decl_getOffsetOfField(const Decl *D, ASTContext *Ctx) {
  auto *RD = dyn_cast_or_null<RecordDecl>(D->getDeclContext());
  if (!RD)
    return -1;
  RD = RD->getDefinition();
  if (!RD || RD->isInvalidDecl() || !RD->isCompleteDefinition())
    return -1;
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
  return BindgenSourceRange(D->getSourceRange());
}

QualType Decl_getTypedefDeclUnderlyingType(const Decl *D) {
  if (auto *TD = dyn_cast_or_null<TypedefNameDecl>(&*D))
    return TD->getUnderlyingType();
  else
    return QualType();
}

CXLinkageKind Decl_getLinkage(const Decl *D) {
  if (auto *ND = dyn_cast_or_null<NamedDecl>(&*D))
    switch (ND->getLinkageInternal()) {
    case NoLinkage:
    case VisibleNoLinkage:
      return CXLinkage_NoLinkage;
    case ModuleInternalLinkage:
    case InternalLinkage:
      return CXLinkage_Internal;
    case UniqueExternalLinkage:
      return CXLinkage_UniqueExternal;
    case ModuleLinkage:
    case ExternalLinkage:
      return CXLinkage_External;
    };

  return CXLinkage_Invalid;
}

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

CX_CXXAccessSpecifier Decl_getAccess(const Decl *D) {
  AccessSpecifier spec = AS_none;
  if (D)
    spec = D->getAccess();

  switch (spec) {
    case AS_public: return CX_CXXPublic;
    case AS_protected: return CX_CXXProtected;
    case AS_private: return CX_CXXPrivate;
    case AS_none: return CX_CXXInvalidAccessSpecifier;
  }

  llvm_unreachable("Invalid AccessSpecifier!");
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
  return Method && Method->getMethodQualifiers().hasConst();
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

QualType Decl_getResultType(const Decl *D, ASTContext *Ctx) {
  if (auto *FT = Decl_getType(D, Ctx)->getAs<FunctionType>())
    return FT->getReturnType();
  else
    return QualType();
}

// const Decl *Expr_getSemanticParent(const Expr *E) {
//   if (!E)
//     return nullptr;
  
//   const DeclContext *DC = E->getParentContext();
//   if (!DC)
//     return nullptr;

//   return cast<Decl>(DC);
// }

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

int Expr_getNumArguments(const Expr *E) {
  if (const CallExpr *CE = dyn_cast_or_null<CallExpr>(&*E))
    return CE->getNumArgs();
  if (const CXXConstructExpr *CE = dyn_cast_or_null<CXXConstructExpr>(&*E))
    return CE->getNumArgs();
  return -1;
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

  case Stmt::FixedPointLiteralClass:
    return CXCursor_FixedPointLiteral;

  case Stmt::FloatingLiteralClass:
    return CXCursor_FloatingLiteral;

  case Stmt::ImaginaryLiteralClass:
    return CXCursor_ImaginaryLiteral;

  case Stmt::StringLiteralClass:
    return CXCursor_StringLiteral;

  case Stmt::CharacterLiteralClass:
    return CXCursor_CharacterLiteral;

  case Stmt::ConstantExprClass:
    return Expr_getCXCursorKind(cast<ConstantExpr>(&*E)->getSubExpr());

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

  case Stmt::ObjCAvailabilityCheckExprClass:
    return CXCursor_ObjCAvailabilityCheckExpr;

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
  case Stmt::CXXInheritedCtorInitExprClass:  
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
  case Stmt::OMPDistributeParallelForDirectiveClass:
    return CXCursor_OMPDistributeParallelForDirective;
  case Stmt::OMPDistributeParallelForSimdDirectiveClass:
    return CXCursor_OMPDistributeParallelForSimdDirective;
  case Stmt::OMPDistributeSimdDirectiveClass:
    return CXCursor_OMPDistributeSimdDirective;
  case Stmt::OMPTargetParallelForSimdDirectiveClass:
    return CXCursor_OMPTargetParallelForSimdDirective;
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
  case Stmt::BuiltinBitCastExprClass:
    return CXCursor_BuiltinBitCastExpr;

  default:
    return CXCursor_UnexposedExpr;
  }
}

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

  return new SourceLocation(E->getBeginLoc());
}

QualType Expr_getType(const Expr *E) { return E->getType(); }

BindgenSourceRange Expr_getSourceRange(const Expr *E) {
  return BindgenSourceRange(E->getSourceRange());
}

const Decl *Type_getDeclaration(QualType T) {
  const Type *TP = T.getTypePtrOrNull();

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
  case Type::DeducedTemplateSpecialization:
    TP = cast<DeducedType>(TP)->getDeducedType().getTypePtrOrNull();
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

class BindgenVisitor : public RecursiveASTVisitor<BindgenVisitor> {
  ASTUnit &AST;
  Visitor VisitFn;
  CXClientData Data;
  Node Parent;

public:
  explicit BindgenVisitor(ASTUnit &AST, Visitor V, CXClientData data) : AST(AST), VisitFn(V), Data(data) {}

  bool shouldVisitImplicitCode() {
    return false;
  }

  bool TraverseDecl(Decl *D) {
    if (!D)
      return false;
    if (D->isImplicit())
      return true;

    bool skip = !Parent;

    // libclang doesn't visit the CXXRecordDecl inside ClassTemplateDecl nodes
    if (Parent.kind == CXCursor_ClassTemplate
        && isa<CXXRecordDecl>(D))
      skip = true;

    if (!skip) {
      switch (VisitFn(Node(D), Parent, &AST, Data)) {
      case CXChildVisit_Break:
        return false;
      case CXChildVisit_Continue:
        return true;
      case CXChildVisit_Recurse:
        break;
      }
    }

    auto OldParent = Parent;
    Parent = Node(D);
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseDecl(D);
    Parent = OldParent;
    return res;
  }

  bool TraverseStmt(Stmt *S) {
    if (!S)
      return false;


    if (Parent) {
      switch (VisitFn(Node(cast<Expr>(S)), Parent, &AST, Data)) {
      case CXChildVisit_Break:
        return false;
      case CXChildVisit_Continue:
        return true;
      case CXChildVisit_Recurse:
        break;
      }
    }

    auto OldParent = Parent;
    Parent = Node(cast<Expr>(S));
    bool res = RecursiveASTVisitor<BindgenVisitor>::TraverseStmt(S);
    Parent = OldParent;
    return res;
  }
};

void Decl_visitChildren(const Decl *Parent, Visitor V, ASTUnit *Unit, CXClientData data) {
  BindgenVisitor visitor(*Unit, V, data);
  visitor.TraverseDecl(const_cast<Decl*>(&*Parent));
}
void Expr_visitChildren(const Expr *Parent, Visitor V, ASTUnit *Unit, CXClientData data) {
  BindgenVisitor visitor(*Unit, V, data);
  visitor.TraverseStmt(const_cast<Expr*>(&*Parent));
}

void tokenize(ASTUnit *TU, BindgenSourceRange Range, CXToken **Tokens,
              unsigned *NumTokens) {
  if (!Tokens || !NumTokens)
    return;

  SmallVector<CXToken, 32> CXTokens;
  SourceManager &SourceMgr = TU->getSourceManager();
  std::pair<FileID, unsigned> BeginLocInfo =
      SourceMgr.getDecomposedSpellingLoc(*Range.B);
  std::pair<FileID, unsigned> EndLocInfo =
      SourceMgr.getDecomposedSpellingLoc(*Range.E);

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

void disposeTokens(const ASTUnit *TU, CXToken *Tokens, unsigned NumTokens) {
  delete[] Tokens;
}

CXTokenKind getTokenKind(CXToken token) {
  return static_cast<CXTokenKind>(token.int_data[0]);
}

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
    BTCASE(Half);
    BTCASE(Float);
    BTCASE(Double);
    BTCASE(LongDouble);
    BTCASE(ShortAccum);
    BTCASE(Accum);
    BTCASE(LongAccum);
    BTCASE(UShortAccum);
    BTCASE(UAccum);
    BTCASE(ULongAccum);
    BTCASE(Float16);
    BTCASE(Float128);
    BTCASE(NullPtr);
    BTCASE(Overload);
    BTCASE(Dependent);
    BTCASE(ObjCId);
    BTCASE(ObjCClass);
    BTCASE(ObjCSel);
#define IMAGE_TYPE(ImgType, Id, SingletonId, Access, Suffix) BTCASE(Id);
#include "clang/Basic/OpenCLImageTypes.def"
#undef IMAGE_TYPE
#define EXT_OPAQUE_TYPE(ExtType, Id, Ext) BTCASE(Id);
#include "clang/Basic/OpenCLExtensionTypes.def"
    BTCASE(OCLSampler);
    BTCASE(OCLEvent);
    BTCASE(OCLQueue);
    BTCASE(OCLReserveID);
  default:
    return CXType_Unexposed;
  }
#undef BTCASE
}

CXTypeKind Type_kind(QualType T) {
  const Type *TP = T.getTypePtrOrNull();
  if (!TP)
    return CXType_Invalid;

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
    TKCASE(ObjCObject);
    TKCASE(ObjCObjectPointer);
    TKCASE(ObjCTypeParam);
    TKCASE(FunctionNoProto);
    TKCASE(FunctionProto);
    TKCASE(ConstantArray);
    TKCASE(IncompleteArray);
    TKCASE(VariableArray);
    TKCASE(DependentSizedArray);
    TKCASE(Vector);
    TKCASE(ExtVector);
    TKCASE(MemberPointer);
    TKCASE(Auto);
    TKCASE(Elaborated);
    TKCASE(Pipe);
    TKCASE(Attributed);
    default:
      return CXType_Unexposed;
  }
#undef TKCASE
}

BindgenStringRef Type_getTypeSpelling(QualType T, ASTContext *Context) {
  SmallString<64> Str;
  llvm::raw_svector_ostream OS(Str);
  PrintingPolicy PP(Context->getLangOpts());

  T.print(OS, PP);
  return stringref(OS.str());
}

bool Type_isConstQualifiedType(QualType T) {
  return T.isLocalConstQualified();
}

long long Type_getSizeOf(QualType QT, ASTContext *Context) {
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
  if (const auto *Deduced = dyn_cast<DeducedType>(QT))
    if (Deduced->getDeducedType().isNull())
      return CXTypeLayoutError_Undeduced;
  // [gcc extension] lib/AST/ExprConstant.cpp:1372
  //                 HandleSizeof : {voidtype,functype} == 1
  // not handled by ASTContext.cpp:1313 getTypeInfoImpl
  if (QT->isVoidType() || QT->isFunctionType())
    return 1;
  return Context->getTypeSizeInChars(QT).getQuantity();
}

long long Type_getAlignOf(QualType QT, ASTContext *Context) {
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
  if (const auto *Deduced = dyn_cast<DeducedType>(QT))
    if (Deduced->getDeducedType().isNull())
      return CXTypeLayoutError_Undeduced;
  // Exceptions by GCC extension - see ASTContext.cpp:1313 getTypeInfoImpl
  // if (QT->isFunctionType()) return 4; // Bug #15511 - should be 1
  // if (QT->isVoidType()) return 1;
  return Context->getTypeAlignInChars(QT).getQuantity();
}

static Optional<ArrayRef<TemplateArgument>>
GetTemplateArguments(QualType Type) {
  assert(!Type.isNull());
  if (const auto *Specialization = Type->getAs<TemplateSpecializationType>())
    return Specialization->template_arguments();

  if (const auto *RecordDecl = Type->getAsCXXRecordDecl()) {
    const auto *TemplateDecl =
      dyn_cast<ClassTemplateSpecializationDecl>(RecordDecl);
    if (TemplateDecl)
      return TemplateDecl->getTemplateArgs().asArray();
  }

  return None;
}

static unsigned GetTemplateArgumentArraySize(ArrayRef<TemplateArgument> TA) {
  unsigned size = TA.size();
  for (const auto &Arg : TA)
    if (Arg.getKind() == TemplateArgument::Pack)
      size += Arg.pack_size() - 1;
  return size;
}

int Type_getNumTemplateArguments(QualType T) {
  if (T.isNull())
    return -1;
  auto TA = GetTemplateArguments(T);
  if (!TA)
    return -1;

  return GetTemplateArgumentArraySize(TA.getValue());
}

QualType Type_getArgType(QualType T, unsigned i) {
  if (T.isNull())
    return QualType();

  if (const FunctionProtoType *FD = T->getAs<FunctionProtoType>()) {
    unsigned numParams = FD->getNumParams();
    if (i >= numParams)
      return QualType();

    return FD->getParamType(i);
  }
  
  return QualType();
}

int Type_getNumArgTypes(QualType T) {
  if (T.isNull())
    return -1;
  
  if (const FunctionProtoType *FD = T->getAs<FunctionProtoType>()) {
    return FD->getNumParams();
  }
  
  if (T->getAs<FunctionNoProtoType>()) {
    return 0;
  }
  
  return -1;
}

QualType Type_getPointeeType(QualType T) {
  const Type *TP = T.getTypePtrOrNull();

  if (!TP)
    return QualType();

try_again:
  switch (TP->getTypeClass()) {
    case Type::Pointer:
      T = cast<PointerType>(TP)->getPointeeType();
      break;
    case Type::BlockPointer:
      T = cast<BlockPointerType>(TP)->getPointeeType();
      break;
    case Type::LValueReference:
    case Type::RValueReference:
      T = cast<ReferenceType>(TP)->getPointeeType();
      break;
    case Type::ObjCObjectPointer:
      T = cast<ObjCObjectPointerType>(TP)->getPointeeType();
      break;
    case Type::MemberPointer:
      T = cast<MemberPointerType>(TP)->getPointeeType();
      break;
    case Type::Auto:
    case Type::DeducedTemplateSpecialization:
      TP = cast<DeducedType>(TP)->getDeducedType().getTypePtrOrNull();
      if (TP)
        goto try_again;
      break;
    default:
      T = QualType();
      break;
  }
  return T;
}

QualType Type_getElementType(QualType T) {
  QualType ET = QualType();
  const Type *TP = T.getTypePtrOrNull();

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
  return ET;
}

int Type_getNumElements(QualType T) {
  long long result = -1;
  const Type *TP = T.getTypePtrOrNull();

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

QualType Type_getCanonicalType(QualType T, ASTContext *Context) {
  if (T.isNull())
    return QualType();

  return Context->getCanonicalType(T);
}

bool Type_isFunctionTypeVariadic(QualType T) {
  if (T.isNull())
    return false;

  if (const FunctionProtoType *FD = T->getAs<FunctionProtoType>())
    return (unsigned)FD->isVariadic();

  if (T->getAs<FunctionNoProtoType>())
    return true;
  
  return false;
}

QualType Type_getResultType(QualType T) {
  if (T.isNull())
    return QualType();
  
  if (const FunctionType *FD = T->getAs<FunctionType>())
    return FD->getReturnType();

  return QualType();
}

CXCallingConv Type_getFunctionTypeCallingConv(QualType T) {
  if (T.isNull())
    return CXCallingConv_Invalid;
  
  if (const FunctionType *FD = T->getAs<FunctionType>()) {
#define TCALLINGCONV(X) case CC_##X: return CXCallingConv_##X
    switch (FD->getCallConv()) {
      TCALLINGCONV(C);
      TCALLINGCONV(X86StdCall);
      TCALLINGCONV(X86FastCall);
      TCALLINGCONV(X86ThisCall);
      TCALLINGCONV(X86Pascal);
      TCALLINGCONV(X86RegCall);
      TCALLINGCONV(X86VectorCall);
      TCALLINGCONV(AArch64VectorCall);
      TCALLINGCONV(Win64);
      TCALLINGCONV(X86_64SysV);
      TCALLINGCONV(AAPCS);
      TCALLINGCONV(AAPCS_VFP);
      TCALLINGCONV(IntelOclBicc);
      TCALLINGCONV(Swift);
      TCALLINGCONV(PreserveMost);
      TCALLINGCONV(PreserveAll);
    case CC_SpirFunction: return CXCallingConv_Unexposed;
    case CC_OpenCLKernel: return CXCallingConv_Unexposed;
      break;
    }
#undef TCALLINGCONV
  }
  
  return CXCallingConv_Invalid;
}

QualType Type_getNamedType(QualType T) {
  const Type *TP = T.getTypePtrOrNull();

  if (TP && TP->getTypeClass() == Type::Elaborated)
    return cast<ElaboratedType>(TP)->getNamedType();

  return QualType();
}

static Optional<QualType> TemplateArgumentToQualType(const TemplateArgument &A) {
  if (A.getKind() == TemplateArgument::Type)
    return A.getAsType();
  return None;
}

static Optional<QualType>
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

QualType Type_getTemplateArgumentAsType(QualType T, unsigned index) {
  if (T.isNull())
    return QualType();

  auto TA = GetTemplateArguments(T);
  if (!TA)
    return QualType();

  Optional<QualType> QT = FindTemplateArgumentTypeAt(TA.getValue(), index);
  return QT.getValueOr(QualType());
}

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

CXCommentKind Comment_getKind(const comments::Comment *) {
  return CXComment_FullComment;
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
  case CXCursor_FixedPointLiteral:
      return stringref("FixedPointLiteral");
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
  case CXCursor_ObjCAvailabilityCheckExpr:
      return stringref("ObjCAvailabilityCheckExpr");
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
  case CXCursor_BuiltinBitCastExpr:
    return stringref("BuiltinBitCastExpr");
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
  case CXCursor_OMPDistributeParallelForDirective:
    return stringref("OMPDistributeParallelForDirective");
  case CXCursor_OMPDistributeParallelForSimdDirective:
    return stringref("OMPDistributeParallelForSimdDirective");
  case CXCursor_OMPDistributeSimdDirective:
    return stringref("OMPDistributeSimdDirective");
  case CXCursor_OMPTargetParallelForSimdDirective:
    return stringref("OMPTargetParallelForSimdDirective");
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
  case CXCursor_OverloadCandidate:
      return stringref("OverloadCandidate");
  case CXCursor_TypeAliasTemplateDecl:
      return stringref("TypeAliasTemplateDecl");
  case CXCursor_StaticAssert:
      return stringref("StaticAssert");
  case CXCursor_FriendDecl:
      return stringref("FriendDecl");
  case CXCursor_ConvergentAttr:
      return stringref("attribute(convergent)");
  case CXCursor_WarnUnusedAttr:
      return stringref("attribute(warn_unused)");
  case CXCursor_WarnUnusedResultAttr:
      return stringref("attribute(warn_unused_result)");
  case CXCursor_AlignedAttr:
      return stringref("attribute(aligned)");
  }

  llvm_unreachable("Unhandled CXCursorKind");
}

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
    TKIND(Half);
    TKIND(Float);
    TKIND(Double);
    TKIND(LongDouble);
    TKIND(ShortAccum);
    TKIND(Accum);
    TKIND(LongAccum);
    TKIND(UShortAccum);
    TKIND(UAccum);
    TKIND(ULongAccum);
    TKIND(Float16);
    TKIND(Float128);
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
    TKIND(ObjCObject);
    TKIND(ObjCObjectPointer);
    TKIND(ObjCTypeParam);
    TKIND(FunctionNoProto);
    TKIND(FunctionProto);
    TKIND(ConstantArray);
    TKIND(IncompleteArray);
    TKIND(VariableArray);
    TKIND(DependentSizedArray);
    TKIND(Vector);
    TKIND(ExtVector);
    TKIND(MemberPointer);
    TKIND(Auto);
    TKIND(Elaborated);
    TKIND(Pipe);
    TKIND(Attributed);
#define IMAGE_TYPE(ImgType, Id, SingletonId, Access, Suffix) TKIND(Id);
#include "clang/Basic/OpenCLImageTypes.def"
#undef IMAGE_TYPE
#define EXT_OPAQUE_TYPE(ExtTYpe, Id, Ext) TKIND(Id);
#include "clang/Basic/OpenCLExtensionTypes.def"
    TKIND(OCLSampler);
    TKIND(OCLEvent);
    TKIND(OCLQueue);
    TKIND(OCLReserveID);
  }
#undef TKIND
  return stringref(s);
}

BindgenStringRef FileEntry_getName(FileEntry *F) {
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
