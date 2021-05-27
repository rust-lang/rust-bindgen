/** @file mul.h
 *
 *  Interface to GiNaC's products of expressions. */

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

#ifndef GINAC_MUL_H
#define GINAC_MUL_H

#include "expairseq.h"

namespace GiNaC {

/** Product of expressions. */
class mul : public expairseq
{
	GINAC_DECLARE_REGISTERED_CLASS(mul, expairseq)
	
	friend class add;
	friend class ncmul;
	friend class power;
	
	// other constructors
public:
	mul(const ex & lh, const ex & rh);
	mul(const exvector & v);
	mul(const epvector & v);
	mul(const epvector & v, const ex & oc, bool do_index_renaming = false);
	mul(epvector && vp);
	mul(epvector && vp, const ex & oc, bool do_index_renaming = false);
	mul(const ex & lh, const ex & mh, const ex & rh);
	
	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 50;}
	bool info(unsigned inf) const override;
	bool is_polynomial(const ex & var) const override;
	int degree(const ex & s) const override;
	int ldegree(const ex & s) const override;
	ex coeff(const ex & s, int n = 1) const override;
	bool has(const ex & other, unsigned options = 0) const override;
	ex eval() const override;
	ex evalf() const override;
	ex real_part() const override;
	ex imag_part() const override;
	ex evalm() const override;
	ex series(const relational & s, int order, unsigned options = 0) const override;
	ex normal(exmap & repl, exmap & rev_lookup) const override;
	numeric integer_content() const override;
	ex smod(const numeric &xi) const override;
	numeric max_coefficient() const override;
	exvector get_free_indices() const override;
	ex conjugate() const override;
protected:
	ex derivative(const symbol & s) const override;
	ex eval_ncmul(const exvector & v) const override;
	unsigned return_type() const override;
	return_type_t return_type_tinfo() const override;
	ex thisexpairseq(const epvector & v, const ex & oc, bool do_index_renaming = false) const override;
	ex thisexpairseq(epvector && vp, const ex & oc, bool do_index_renaming = false) const override;
	expair split_ex_to_pair(const ex & e) const override;
	expair combine_ex_with_coeff_to_pair(const ex & e, const ex & c) const override;
	expair combine_pair_with_coeff_to_pair(const expair & p, const ex & c) const override;
	ex recombine_pair_to_ex(const expair & p) const override;
	bool expair_needs_further_processing(epp it) override;
	ex default_overall_coeff() const override;
	void combine_overall_coeff(const ex & c) override;
	void combine_overall_coeff(const ex & c1, const ex & c2) override;
	bool can_make_flat(const expair & p) const override;
	ex expand(unsigned options=0) const override;
	
	// new virtual functions which can be overridden by derived classes
	// none
	
	// non-virtual functions in this class
public:
	ex algebraic_subs_mul(const exmap & m, unsigned options) const;
protected:
	void find_real_imag(ex&, ex&) const;
	void print_overall_coeff(const print_context & c, const char *mul_sym) const;
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_csrc(const print_csrc & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;
	static bool can_be_further_expanded(const ex & e);
	epvector expandchildren(unsigned options) const;
};
GINAC_DECLARE_UNARCHIVER(mul);

} // namespace GiNaC

#endif // ndef GINAC_MUL_H
