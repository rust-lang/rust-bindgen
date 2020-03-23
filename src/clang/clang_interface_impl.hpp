#ifndef BINDGEN_CLANG_INTERFACE_IMPL_H
#define BINDGEN_CLANG_INTERFACE_IMPL_H

#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/Basic/Version.h"

#define BINDGEN_IMPLEMENTATION
#include "clang_interface.hpp"

using namespace clang;

namespace clang {

#if CLANG_VERSION_MAJOR < 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR <= 8)
// Clang <= 3.8 doesn't include this enum, but we can still expose the same
// functionality
typedef enum {
  CXEval_Int = 1 ,
  CXEval_Float = 2,
  CXEval_ObjCStrLiteral = 3,
  CXEval_StrLiteral = 4,
  CXEval_CFStr = 5,
  CXEval_Other = 6,

  CXEval_UnExposed = 0

} CXEvalResultKind ;
#endif

} // namespace clang

// Utility functions defined in ClangAST.cpp
BindgenStringRef stringref();
BindgenStringRef stringref(const char *newStr);
BindgenStringRef stringref(const std::string &s);
BindgenStringRef stringref(llvm::StringRef S);
BindgenStringRefSet make_stringrefset(std::vector<std::string> &string_vec);
BindgenQualType make_type_compatible(QualType QT);

// Functions defined in libclang_compat.cpp
const Decl *getDeclFromExpr(const Stmt *E);
long long visitRecordForValidation(const RecordDecl *RD);
Optional<ArrayRef<TemplateArgument>> GetTemplateArguments(QualType Type);
unsigned GetTemplateArgumentArraySize(ArrayRef<TemplateArgument> TA);
Optional<QualType> FindTemplateArgumentTypeAt(ArrayRef<TemplateArgument> TA,
                                              unsigned index);
CX_CXXAccessSpecifier TranslateCXXAccessSpecifier(AccessSpecifier spec);

#endif // BINDGEN_CLANG_INTERFACE_IMPL_H
