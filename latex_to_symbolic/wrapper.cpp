#include "wrapper.hpp"
#include <cassert>

GiNaC::symbol* wrap_symbol_construct(const char *cptr) {
	return new GiNaC::symbol(cptr);
}
void wrap_symbol_destruct(GiNaC::symbol *ptr) {
	assert(ptr != NULL);
	delete ptr;
}


GiNaC::ex* wrap_ex_construct(const char *cptr) {
	return new GiNaC::ex(GiNaC::symbol(cptr));
}
GiNaC::ex* wrap_ex_construct_copy(const GiNaC::ex *cptr) {
	return new GiNaC::ex(*cptr);
}
GiNaC::ex* wrap_ex_construct_float(const double f) {
	return new GiNaC::ex(f);
}
GiNaC::ex* wrap_ex_construct_integer(const int i) {
	return new GiNaC::ex(i);
}
GiNaC::ex* wrap_ex_construct_power(const GiNaC::ex *base, const GiNaC::ex *expon) {
	return new GiNaC::ex(GiNaC::pow(*base, *expon));
}
void wrap_ex_destruct(GiNaC::ex *ptr) {
	assert(ptr != NULL);
	delete ptr;
}
void wrap_add_assign(GiNaC::ex *left, const GiNaC::ex *right) {
	*left += *right;
}
void wrap_mul_assign(GiNaC::ex *left, const GiNaC::ex *right) {
	*left *= *right;
}
// GiNaC::ex* wrap_add(const GiNaC::ex *left, const GiNaC::ex *right) {
// 	return new GiNaC::symbol(*left + *right);
// }
// GiNaC::ex* wrap_mul(const GiNaC::ex *left, const GiNaC::ex *right) {
// 	return new GiNaC::symbol(*left * *right);
// }
bool wrap_equal(const GiNaC::ex *left, const GiNaC::ex *right) {
	return *left == *right;
}
void wrap_debugsymbol(GiNaC::symbol *ptr) {
	ptr->dbgprint();
}


#include <iostream>
#include <ginac/ginac.h>
using namespace std;
using namespace GiNaC;

void wrap_testfunc()
{
    symbol x("x"), y("y");
    ex poly;

	poly = -x;

	poly.dbgprinttree();
}