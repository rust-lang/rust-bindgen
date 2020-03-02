#ifndef BINDGEN_CLANG_INTERFACE_IMPL_H
#define BINDGEN_CLANG_INTERFACE_IMPL_H

#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"

#define BINDGEN_IMPLEMENTATION
#include "clang_interface.hpp"

using namespace clang;

// Utility functions defined in ClangAST.cpp
BindgenStringRef stringref();
BindgenStringRef stringref(const char *newStr);
BindgenStringRef stringref(const std::string &s);
BindgenStringRef stringref(llvm::StringRef S);
BindgenStringRefSet make_stringrefset(std::vector<std::string> &string_vec);
QualType make_type_compatible(QualType QT);

// Functions defined in libclang_compat.cpp
const Decl *getDeclFromExpr(const Stmt *E);
long long visitRecordForValidation(const RecordDecl *RD);
Optional<ArrayRef<TemplateArgument>> GetTemplateArguments(QualType Type);
unsigned GetTemplateArgumentArraySize(ArrayRef<TemplateArgument> TA);
Optional<QualType> FindTemplateArgumentTypeAt(ArrayRef<TemplateArgument> TA,
                                              unsigned index);

#endif // BINDGEN_CLANG_INTERFACE_IMPL_H
