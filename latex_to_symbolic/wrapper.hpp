#include <ginac/ginac.h>


GiNaC::symbol* wrap_symbol_construct(const char *cptr);
void wrap_symbol_destruct(GiNaC::symbol *ptr);

GiNaC::ex* wrap_ex_construct(const char *cptr);
GiNaC::ex* wrap_ex_construct_copy(const GiNaC::ex *cptr);
GiNaC::ex* wrap_ex_construct_float(const double f);
GiNaC::ex* wrap_ex_construct_integer(const int i);
GiNaC::ex* wrap_ex_construct_power(const GiNaC::ex *base, const GiNaC::ex *expon);
void wrap_ex_destruct(GiNaC::ex *ptr);
void wrap_add_assign(GiNaC::ex *left, const GiNaC::ex *right);
void wrap_mul_assign(GiNaC::ex *left, const GiNaC::ex *right);
// GiNaC::ex* wrap_add(const GiNaC::ex *left, const GiNaC::ex *right);
// GiNaC::ex* wrap_mul(const GiNaC::ex *left, const GiNaC::ex *right);
bool wrap_equal(const GiNaC::ex *left, const GiNaC::ex *right);

void wrap_debugsymbol(GiNaC::symbol *ptr);

void wrap_testfunc();
