/** @file numeric.h
 *
 *  Makes the interface to the underlying bignum package available. */

/*
 *  GiNaC Copyright (C) 1999-2020 Johannes Gutenberg University Mainz, Germany
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

#ifndef GINAC_NUMERIC_H
#define GINAC_NUMERIC_H

#include "basic.h"
#include "ex.h"
#include "archive.h"

#include <cln/complex.h>
#include <stdexcept>
#include <vector>

namespace GiNaC {

/** Function pointer to implement callbacks in the case 'Digits' gets changed.
 *  Main purpose of such callbacks is to adjust look-up tables of certain
 *  functions to the new precision. Parameter contains the signed difference
 *  between new Digits and old Digits. */
typedef void (* digits_changed_callback)(long);

/** This class is used to instantiate a global singleton object Digits
 *  which behaves just like Maple's Digits.  We need an object rather 
 *  than a dumber basic type since as a side-effect we let it change
 *  cl_default_float_format when it gets changed.  The only other
 *  meaningful thing to do with it is converting it to an unsigned,
 *  for temporarily storing its value e.g.  The user must not create an
 *  own working object of this class!  Since C++ forces us to make the
 *  class definition visible in order to use an object we put in a
 *  flag which prevents other objects of that class to be created. */
class _numeric_digits
{
// member functions
public:
	_numeric_digits();
	_numeric_digits& operator=(long prec);
	operator long();
	void print(std::ostream& os) const;
	void add_callback(digits_changed_callback callback);
// member variables
private:
	long digits;                        ///< Number of decimal digits
	static bool too_late;               ///< Already one object present
	// Holds a list of functions that get called when digits is changed.
	std::vector<digits_changed_callback> callbacklist;
};


/** Exception class thrown when a singularity is encountered. */
class pole_error : public std::domain_error {
public:
	explicit pole_error(const std::string& what_arg, int degree);
	int degree() const;
private:
	int deg;
};


/** This class is a wrapper around CLN-numbers within the GiNaC class
 *  hierarchy. Objects of this type may directly be created by the user.*/
class numeric : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(numeric, basic)
	
// member functions
	
	// other constructors
public:
	numeric(int i);
	numeric(unsigned int i);
	numeric(long i);
	numeric(unsigned long i);
	numeric(long long i);
	numeric(unsigned long long i);
	numeric(long numer, long denom);
	numeric(double d);
	numeric(const char *);
	
	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 30;}
	bool info(unsigned inf) const override;
	bool is_polynomial(const ex & var) const override;
	int degree(const ex & s) const override;
	int ldegree(const ex & s) const override;
	ex coeff(const ex & s, int n = 1) const override;
	bool has(const ex &other, unsigned options = 0) const override;
	ex eval() const override;
	ex evalf() const override;
	ex subs(const exmap & m, unsigned options = 0) const override { return subs_one_level(m, options); } // overwrites basic::subs() for performance reasons
	ex normal(exmap & repl, exmap & rev_lookup) const override;
	ex to_rational(exmap & repl) const override;
	ex to_polynomial(exmap & repl) const override;
	numeric integer_content() const override;
	ex smod(const numeric &xi) const override;
	numeric max_coefficient() const override;
	ex conjugate() const override;
	ex real_part() const override;
	ex imag_part() const override;
	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	/** Implementation of ex::diff for a numeric always returns 0.
	 *  @see ex::diff */
	ex derivative(const symbol &s) const override { return 0; }
	bool is_equal_same_type(const basic &other) const override;
	unsigned calchash() const override;
	
	// new virtual functions which can be overridden by derived classes
	// (none)
	
	// non-virtual functions in this class
public:
	const numeric add(const numeric &other) const;
	const numeric sub(const numeric &other) const;
	const numeric mul(const numeric &other) const;
	const numeric div(const numeric &other) const;
	const numeric power(const numeric &other) const;
	const numeric & add_dyn(const numeric &other) const;
	const numeric & sub_dyn(const numeric &other) const;
	const numeric & mul_dyn(const numeric &other) const;
	const numeric & div_dyn(const numeric &other) const;
	const numeric & power_dyn(const numeric &other) const;
	const numeric & operator=(int i);
	const numeric & operator=(unsigned int i);
	const numeric & operator=(long i);
	const numeric & operator=(unsigned long i);
	const numeric & operator=(double d);
	const numeric & operator=(const char *s);
	const numeric inverse() const;
	numeric step() const;
	int csgn() const;
	int compare(const numeric &other) const;
	bool is_equal(const numeric &other) const;
	bool is_zero() const;
	bool is_positive() const;
	bool is_negative() const;
	bool is_integer() const;
	bool is_pos_integer() const;
	bool is_nonneg_integer() const;
	bool is_even() const;
	bool is_odd() const;
	bool is_prime() const;
	bool is_rational() const;
	bool is_real() const;
	bool is_cinteger() const;
	bool is_crational() const;
	bool operator==(const numeric &other) const;
	bool operator!=(const numeric &other) const;
	bool operator<(const numeric &other) const;
	bool operator<=(const numeric &other) const;
	bool operator>(const numeric &other) const;
	bool operator>=(const numeric &other) const;
	int to_int() const;
	long to_long() const;
	double to_double() const;
	cln::cl_N to_cl_N() const;
	const numeric real() const;
	const numeric imag() const;
	const numeric numer() const;
	const numeric denom() const;
	int int_length() const;
	// converting routines for interfacing with CLN:
	explicit numeric(const cln::cl_N &z);

protected:
	void print_numeric(const print_context & c, const char *par_open, const char *par_close, const char *imag_sym, const char *mul_sym, unsigned level) const;
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_csrc(const print_csrc & c, unsigned level) const;
	void do_print_csrc_cl_N(const print_csrc_cl_N & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;

// member variables

protected:
	cln::cl_N value;
};
GINAC_DECLARE_UNARCHIVER(numeric); 


// global constants

extern const numeric I;
extern _numeric_digits Digits;

// global functions

const numeric exp(const numeric &x);
const numeric log(const numeric &x);
const numeric sin(const numeric &x);
const numeric cos(const numeric &x);
const numeric tan(const numeric &x);
const numeric asin(const numeric &x);
const numeric acos(const numeric &x);
const numeric atan(const numeric &x);
const numeric atan(const numeric &y, const numeric &x);
const numeric sinh(const numeric &x);
const numeric cosh(const numeric &x);
const numeric tanh(const numeric &x);
const numeric asinh(const numeric &x);
const numeric acosh(const numeric &x);
const numeric atanh(const numeric &x);
const numeric Li2(const numeric &x);
const numeric zeta(const numeric &x);
const numeric lgamma(const numeric &x);
const numeric tgamma(const numeric &x);
const numeric psi(const numeric &x);
const numeric psi(const numeric &n, const numeric &x);
const numeric factorial(const numeric &n);
const numeric doublefactorial(const numeric &n);
const numeric binomial(const numeric &n, const numeric &k);
const numeric bernoulli(const numeric &n);
const numeric fibonacci(const numeric &n);
const numeric isqrt(const numeric &x);
const numeric sqrt(const numeric &x);
const numeric abs(const numeric &x);
const numeric mod(const numeric &a, const numeric &b);
const numeric smod(const numeric &a, const numeric &b);
const numeric irem(const numeric &a, const numeric &b);
const numeric irem(const numeric &a, const numeric &b, numeric &q);
const numeric iquo(const numeric &a, const numeric &b);
const numeric iquo(const numeric &a, const numeric &b, numeric &r);
const numeric gcd(const numeric &a, const numeric &b);
const numeric lcm(const numeric &a, const numeric &b);

// wrapper functions around member functions
inline const numeric pow(const numeric &x, const numeric &y)
{ return x.power(y); }

inline const numeric inverse(const numeric &x)
{ return x.inverse(); }

inline numeric step(const numeric &x)
{ return x.step(); }

inline int csgn(const numeric &x)
{ return x.csgn(); }

inline bool is_zero(const numeric &x)
{ return x.is_zero(); }

inline bool is_positive(const numeric &x)
{ return x.is_positive(); }

inline bool is_negative(const numeric &x)
{ return x.is_negative(); }

inline bool is_integer(const numeric &x)
{ return x.is_integer(); }

inline bool is_pos_integer(const numeric &x)
{ return x.is_pos_integer(); }

inline bool is_nonneg_integer(const numeric &x)
{ return x.is_nonneg_integer(); }

inline bool is_even(const numeric &x)
{ return x.is_even(); }

inline bool is_odd(const numeric &x)
{ return x.is_odd(); }

inline bool is_prime(const numeric &x)
{ return x.is_prime(); }

inline bool is_rational(const numeric &x)
{ return x.is_rational(); }

inline bool is_real(const numeric &x)
{ return x.is_real(); }

inline bool is_cinteger(const numeric &x)
{ return x.is_cinteger(); }

inline bool is_crational(const numeric &x)
{ return x.is_crational(); }

inline int to_int(const numeric &x)
{ return x.to_int(); }

inline long to_long(const numeric &x)
{ return x.to_long(); }

inline double to_double(const numeric &x)
{ return x.to_double(); }

inline const numeric real(const numeric &x)
{ return x.real(); }

inline const numeric imag(const numeric &x)
{ return x.imag(); }

inline const numeric numer(const numeric &x)
{ return x.numer(); }

inline const numeric denom(const numeric &x)
{ return x.denom(); }

// numeric evaluation functions for class constant objects:

ex PiEvalf();
ex EulerEvalf();
ex CatalanEvalf();


} // namespace GiNaC

#endif // ndef GINAC_NUMERIC_H
