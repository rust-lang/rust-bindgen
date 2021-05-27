/** @file color.h
 *
 *  Interface to GiNaC's color (SU(3) Lie algebra) objects. */

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

#ifndef GINAC_COLOR_H
#define GINAC_COLOR_H

#include "indexed.h"
#include "tensor.h"

#include <set>

namespace GiNaC {

/** This class holds a generator T_a or the unity element of the Lie algebra
 *  of SU(3), as used for calculations in quantum chromodynamics. A
 *  representation label (an unsigned 8-bit integer) is used to distinguish
 *  elements from different Lie algebras (objects with different labels
 *  commutate). These objects implement an abstract representation of the
 *  group, not a specific matrix representation. The indices used for color
 *  objects should not have a variance. */
class color : public indexed
{
	GINAC_DECLARE_REGISTERED_CLASS(color, indexed)
	// other constructors
public:
	color(const ex & b, unsigned char rl = 0);
	color(const ex & b, const ex & i1, unsigned char rl = 0);

	// internal constructors
	color(unsigned char rl, const exvector & v);
	color(unsigned char rl, exvector && v);
	void archive(archive_node& n) const override;
	void read_archive(const archive_node& n, lst& sym_lst) override;

	// functions overriding virtual functions from base classes
protected:
	ex eval_ncmul(const exvector & v) const override;
	bool match_same_type(const basic & other) const override;
	ex thiscontainer(const exvector & v) const override;
	ex thiscontainer(exvector && v) const override;
	unsigned return_type() const override { return return_types::noncommutative; }
	return_type_t return_type_tinfo() const override;

	// non-virtual functions in this class
public:
	unsigned char get_representation_label() const {return representation_label;}

	// member variables
private:
	unsigned char representation_label; /**< Representation label to distinguish independent color matrices coming from separated fermion lines */
};
GINAC_DECLARE_UNARCHIVER(color); 


/** This class represents the su(3) unity element. */
class su3one : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(su3one, tensor)

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(su3one);

/** This class represents an su(3) generator. */
class su3t : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(su3t, tensor)

	// functions overriding virtual functions from base classes
public:
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(su3t);

/** This class represents the tensor of antisymmetric su(3) structure
 *  constants. */
class su3f : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(su3f, tensor)

	// functions overriding virtual functions from base classes
public:
	ex eval_indexed(const basic & i) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;
protected:
	unsigned return_type() const override { return return_types::commutative; }

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(su3f);

/** This class represents the tensor of symmetric su(3) structure constants. */
class su3d : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(su3d, tensor)

	// functions overriding virtual functions from base classes
public:
	ex eval_indexed(const basic & i) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;
protected:
	unsigned return_type() const override { return return_types::commutative; }

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(su3d);


// global functions

/** Create the su(3) unity element. This is an indexed object, although it
 *  has no indices.
 *
 *  @param rl Representation label
 *  @return newly constructed unity element */
ex color_ONE(unsigned char rl = 0);

/** Create an su(3) generator.
 *
 *  @param a Index
 *  @param rl Representation label
 *  @return newly constructed unity generator */
ex color_T(const ex & a, unsigned char rl = 0);

/** Create an su(3) antisymmetric structure constant.
 *
 *  @param a First index
 *  @param b Second index
 *  @param c Third index
 *  @return newly constructed structure constant */
ex color_f(const ex & a, const ex & b, const ex & c);

/** Create an su(3) symmetric structure constant.
 *
 *  @param a First index
 *  @param b Second index
 *  @param c Third index
 *  @return newly constructed structure constant */
ex color_d(const ex & a, const ex & b, const ex & c);

/** This returns the linear combination d.a.b.c+I*f.a.b.c. */
ex color_h(const ex & a, const ex & b, const ex & c);

/** Calculate color traces over the specified set of representation labels.
 *
 *  @param e Expression to take the trace of
 *  @param rls Set of representation labels */
ex color_trace(const ex & e, const std::set<unsigned char> & rls);

/** Calculate color traces over the specified list of representation labels.
 *
 *  @param e Expression to take the trace of
 *  @param rll List of representation labels */
ex color_trace(const ex & e, const lst & rll);

/** Calculate the trace of an expression containing color objects with a
 *  specified representation label.
 *
 *  @param e Expression to take the trace of
 *  @param rl Representation label */
ex color_trace(const ex & e, unsigned char rl = 0);

} // namespace GiNaC

#endif // ndef GINAC_COLOR_H
