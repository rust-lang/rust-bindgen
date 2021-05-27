/** @file integral.h
 *
 *  Interface to GiNaC's symbolic  integral. */

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

#ifndef GINAC_INTEGRAL_H
#define GINAC_INTEGRAL_H

#include "basic.h"
#include "ex.h"
#include "archive.h"

namespace GiNaC {

/** Symbolic integral. */
class integral : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(integral, basic)
	
	// other constructors
public:
	integral(const ex & x_, const ex & a_, const ex & b_, const ex & f_);
	
	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 45;}
	ex eval() const override;
	ex evalf() const override;
	int degree(const ex & s) const override;
	int ldegree(const ex & s) const override;
	ex eval_ncmul(const exvector & v) const override;
	size_t nops() const override;
	ex op(size_t i) const override;
	ex & let_op(size_t i) override;
	ex expand(unsigned options = 0) const override;
	exvector get_free_indices() const override;
	unsigned return_type() const override;
	return_type_t return_type_tinfo() const override;
	ex conjugate() const override;
	ex eval_integ() const override;
	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	ex derivative(const symbol & s) const override;
	ex series(const relational & r, int order, unsigned options = 0) const override;

	// new virtual functions which can be overridden by derived classes
	// none
	
	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
public:
	static int max_integration_level;
	static ex relative_integration_error;
private:
	ex x;
	ex a;
	ex b;
	ex f;
};
GINAC_DECLARE_UNARCHIVER(integral); 

// utility functions

GiNaC::ex adaptivesimpson(
	const GiNaC::ex &x,
	const GiNaC::ex &a,
	const GiNaC::ex &b,
	const GiNaC::ex &f,
	const GiNaC::ex &error = integral::relative_integration_error
);

} // namespace GiNaC

#endif // ndef GINAC_INTEGRAL_H
