/** @file add.h
 *
 *  Interface to GiNaC's sums of expressions. */

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

#ifndef GINAC_ADD_H
#define GINAC_ADD_H

#include "expairseq.h"

namespace GiNaC {

/** Sum of expressions. */
class add : public expairseq
{
	GINAC_DECLARE_REGISTERED_CLASS(add, expairseq)
	
	friend class mul;
	friend class power;
	
	// other constructors
public:
	add(const ex & lh, const ex & rh);
	add(const exvector & v);
	add(const epvector & v);
	add(const epvector & v, const ex & oc);
	add(epvector && v);
	add(epvector && v, const ex & oc);
	
	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 40;}
	bool info(unsigned inf) const override;
	bool is_polynomial(const ex & var) const override;
	int degree(const ex & s) const override;
	int ldegree(const ex & s) const override;
	ex coeff(const ex & s, int n=1) const override;
	ex eval() const override;
	ex evalm() const override;
	ex series(const relational & r, int order, unsigned options = 0) const override;
	ex normal(exmap & repl, exmap & rev_lookup) const override;
	numeric integer_content() const override;
	ex smod(const numeric &xi) const override;
	numeric max_coefficient() const override;
	ex conjugate() const override;
	ex real_part() const override;
	ex imag_part() const override;
	exvector get_free_indices() const override;
	ex eval_ncmul(const exvector & v) const override;
protected:
	ex derivative(const symbol & s) const override;
	unsigned return_type() const override;
	return_type_t return_type_tinfo() const override;
	ex thisexpairseq(const epvector & v, const ex & oc, bool do_index_renaming = false) const override;
	ex thisexpairseq(epvector && vp, const ex & oc, bool do_index_renaming = false) const override;
	expair split_ex_to_pair(const ex & e) const override;
	expair combine_ex_with_coeff_to_pair(const ex & e,
	                                     const ex & c) const override;
	expair combine_pair_with_coeff_to_pair(const expair & p,
	                                       const ex & c) const override;
	ex recombine_pair_to_ex(const expair & p) const override;
	ex expand(unsigned options=0) const override;

	// non-virtual functions in this class
protected:
	void print_add(const print_context & c, const char *openbrace, const char *closebrace, const char *mul_sym, unsigned level) const;
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_csrc(const print_csrc & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(add);

} // namespace GiNaC

#endif // ndef GINAC_ADD_H
