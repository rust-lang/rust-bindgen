/** @file pseries.h
 *
 *  Interface to class for extended truncated power series. */

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

#ifndef GINAC_SERIES_H
#define GINAC_SERIES_H

#include "basic.h"
#include "expairseq.h"

namespace GiNaC {

/** This class holds a extended truncated power series (positive and negative
 *  integer powers). It consists of expression coefficients (only non-zero
 *  coefficients are stored), an expansion variable and an expansion point.
 *  Other classes must provide members to convert into this type. */
class pseries : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(pseries, basic)

	// other constructors
public:
	pseries(const ex &rel_, const epvector &ops_);
	pseries(const ex &rel_, epvector &&ops_);

	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 38;} // for clarity just below add::precedence
	size_t nops() const override;
	ex op(size_t i) const override;
	int degree(const ex &s) const override;
	int ldegree(const ex &s) const override;
	ex coeff(const ex &s, int n = 1) const override;
	ex collect(const ex &s, bool distributed = false) const override;
	ex eval() const override;
	ex evalf() const override;
	ex series(const relational & r, int order, unsigned options = 0) const override;
	ex subs(const exmap & m, unsigned options = 0) const override;
	ex normal(exmap & repl, exmap & rev_lookup) const override;
	ex expand(unsigned options = 0) const override;
	ex conjugate() const override;
	ex real_part() const override;
	ex imag_part() const override;
	ex eval_integ() const override;
	ex evalm() const override;
	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	ex derivative(const symbol & s) const override;

	// non-virtual functions in this class
public:
	/** Get the expansion variable. */
	ex get_var() const {return var;}

	/** Get the expansion point. */
	ex get_point() const {return point;}

	/** Convert the pseries object to an ordinary polynomial.
	 *
	 *  @param no_order flag: discard higher order terms */
	ex convert_to_poly(bool no_order = false) const;

	/** Check whether series is compatible to another series (expansion
	 *  variable and point are the same. */
	bool is_compatible_to(const pseries &other) const {return var.is_equal(other.var) && point.is_equal(other.point);}

	/** Check whether series has the value zero. */
	bool is_zero() const {return seq.size() == 0;}

	/** Returns true if there is no order term, i.e. the series terminates and
	 *  false otherwise. */
	bool is_terminating() const;

	/** Get coefficients and exponents. */
	ex coeffop(size_t i) const;
	ex exponop(size_t i) const;

	ex add_series(const pseries &other) const;
	ex mul_const(const numeric &other) const;
	ex mul_series(const pseries &other) const;
	ex power_const(const numeric &p, int deg) const;
	pseries shift_exponents(int deg) const;

protected:
	void print_series(const print_context & c, const char *openbrace, const char *closebrace, const char *mul_sym, const char *pow_sym, unsigned level) const;
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;
	void do_print_python(const print_python & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;

protected:
	/** Vector of {coefficient, power} pairs */
	epvector seq;

	/** Series variable (holds a symbol) */
	ex var;

	/** Expansion point */
	ex point;
};
GINAC_DECLARE_UNARCHIVER(pseries); 


// utility functions

/** Convert the pseries object embedded in an expression to an ordinary
 *  polynomial in the expansion variable. The result is undefined if the
 *  expression does not contain a pseries object at its top level.
 *
 *  @param e expression
 *  @return polynomial expression
 *  @see is_a<>
 *  @see pseries::convert_to_poly */
inline ex series_to_poly(const ex &e)
{
	return (ex_to<pseries>(e).convert_to_poly(true));
}

inline bool is_terminating(const pseries & s)
{
	return s.is_terminating();
}

} // namespace GiNaC

#endif // ndef GINAC_SERIES_H
