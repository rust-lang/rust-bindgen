/** @file clifford.h
 *
 *  Interface to GiNaC's clifford algebra (Dirac gamma) objects. */

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

#ifndef GINAC_CLIFFORD_H
#define GINAC_CLIFFORD_H

#include "indexed.h"
#include "tensor.h"
#include "symbol.h"
#include "idx.h"

#include <set>

namespace GiNaC {

/** This class holds an object representing an element of the Clifford
 *  algebra (the Dirac gamma matrices). These objects only carry Lorentz
 *  indices. Spinor indices are hidden. A representation label (an unsigned
 *  8-bit integer) is used to distinguish elements from different Clifford
 *  algebras (objects with different labels commutate). */
class clifford : public indexed
{
	GINAC_DECLARE_REGISTERED_CLASS(clifford, indexed)
	// other constructors
public:
	clifford(const ex & b, unsigned char rl = 0);
	clifford(const ex & b, const ex & mu,  const ex & metr, unsigned char rl = 0, int comm_sign = -1);

	// internal constructors
	clifford(unsigned char rl, const ex & metr, int comm_sign, const exvector & v);
	clifford(unsigned char rl, const ex & metr, int comm_sign, exvector && v);

	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override { return 65; }
	void archive(archive_node& n) const override;
	void read_archive(const archive_node& n, lst& sym_lst) override;
protected:
	ex eval_ncmul(const exvector & v) const override;
	bool match_same_type(const basic & other) const override;
	ex thiscontainer(const exvector & v) const override;
	ex thiscontainer(exvector && v) const override;
	unsigned return_type() const override { return return_types::noncommutative; }
	return_type_t return_type_tinfo() const override;
	// non-virtual functions in this class
public:
	unsigned char get_representation_label() const { return representation_label; }
	ex get_metric() const { return metric; }
	virtual ex get_metric(const ex & i, const ex & j, bool symmetrised = false) const;
	bool same_metric(const ex & other) const;
	int get_commutator_sign() const { return commutator_sign; } //**< See the member variable commutator_sign */

	inline size_t nops() const override {return inherited::nops() + 1; }
	ex op(size_t i) const override;
	ex & let_op(size_t i) override;
	ex subs(const exmap & m, unsigned options = 0) const override;

protected:
	void do_print_dflt(const print_dflt & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;

	// member variables
protected:
	unsigned char representation_label; /**< Representation label to distinguish independent spin lines */
	ex metric; /**< Metric of the space, all constructors make it an indexed object */
	int commutator_sign; /**< It is the sign in the definition e~i e~j +/- e~j e~i = B(i, j) + B(j, i)*/
};
GINAC_DECLARE_UNARCHIVER(clifford); 

/** This class represents the Clifford algebra unity element. */
class diracone : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(diracone, tensor)

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(diracone);


/** This class represents the Clifford algebra generators (units). */
class cliffordunit : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(cliffordunit, tensor)

	// functions overriding virtual functions from base classes
public:
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(cliffordunit);


/** This class represents the Dirac gamma Lorentz vector. */
class diracgamma : public cliffordunit
{
	GINAC_DECLARE_REGISTERED_CLASS(diracgamma, cliffordunit)

	// functions overriding virtual functions from base classes
public:
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(diracgamma);


/** This class represents the Dirac gamma5 object which anticommutates with
 *  all other gammas. */
class diracgamma5 : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(diracgamma5, tensor)

	// functions overriding virtual functions from base classes
	ex conjugate() const override;

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(diracgamma5);


/** This class represents the Dirac gammaL object which behaves like
 *  1/2 (1-gamma5). */
class diracgammaL : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(diracgammaL, tensor)

	// functions overriding virtual functions from base classes
	ex conjugate() const override;

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(diracgammaL);


/** This class represents the Dirac gammaL object which behaves like
 *  1/2 (1+gamma5). */
class diracgammaR : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(diracgammaR, tensor)

	// functions overriding virtual functions from base classes
	ex conjugate() const override;

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(diracgammaR);


// global functions

/** Check whether a given return_type_t object (as returned by return_type_tinfo()
  * is that of a clifford object (with an arbitrary representation label).
  *
  * @param ti tinfo key */
inline bool is_clifford_tinfo(const return_type_t& ti)
{
	return *(ti.tinfo) == typeid(clifford);
}

/** Create a Clifford unity object.
 *
 *  @param rl Representation label
 *  @return newly constructed object */
ex dirac_ONE(unsigned char rl = 0);

/** Create a Clifford unit object.
 *
 *  @param mu Index (must be of class varidx or a derived class)
 *  @param metr Metric (should be indexed, tensmetric or a derived class, or a matrix)
 *  @param rl Representation label
 *  @return newly constructed Clifford unit object */
ex clifford_unit(const ex & mu, const ex & metr, unsigned char rl = 0);

/** Create a Dirac gamma object.
 *
 *  @param mu Index (must be of class varidx or a derived class)
 *  @param rl Representation label
 *  @return newly constructed gamma object */
ex dirac_gamma(const ex & mu, unsigned char rl = 0);

/** Create a Dirac gamma5 object.
 *
 *  @param rl Representation label
 *  @return newly constructed object */
ex dirac_gamma5(unsigned char rl = 0);

/** Create a Dirac gammaL object.
 *
 *  @param rl Representation label
 *  @return newly constructed object */
ex dirac_gammaL(unsigned char rl = 0);

/** Create a Dirac gammaR object.
 *
 *  @param rl Representation label
 *  @return newly constructed object */
ex dirac_gammaR(unsigned char rl = 0);

/** Create a term of the form e_mu * gamma~mu with a unique index mu.
 *
 *  @param e Original expression
 *  @param dim Dimension of index
 *  @param rl Representation label */
ex dirac_slash(const ex & e, const ex & dim, unsigned char rl = 0);

/** Calculate dirac traces over the specified set of representation labels.
 *  The computed trace is a linear functional that is equal to the usual
 *  trace only in D = 4 dimensions. In particular, the functional is not
 *  always cyclic in D != 4 dimensions when gamma5 is involved.
 *
 *  @param e Expression to take the trace of
 *  @param rls Set of representation labels
 *  @param trONE Expression to be returned as the trace of the unit matrix */
ex dirac_trace(const ex & e, const std::set<unsigned char> & rls, const ex & trONE = 4);

/** Calculate dirac traces over the specified list of representation labels.
 *  The computed trace is a linear functional that is equal to the usual
 *  trace only in D = 4 dimensions. In particular, the functional is not
 *  always cyclic in D != 4 dimensions when gamma5 is involved.
 *
 *  @param e Expression to take the trace of
 *  @param rll List of representation labels
 *  @param trONE Expression to be returned as the trace of the unit matrix */
ex dirac_trace(const ex & e, const lst & rll, const ex & trONE = 4);

/** Calculate the trace of an expression containing gamma objects with
 *  a specified representation label. The computed trace is a linear
 *  functional that is equal to the usual trace only in D = 4 dimensions.
 *  In particular, the functional is not always cyclic in D != 4 dimensions
 *  when gamma5 is involved.
 *
 *  @param e Expression to take the trace of
 *  @param rl Representation label
 *  @param trONE Expression to be returned as the trace of the unit matrix */
ex dirac_trace(const ex & e, unsigned char rl = 0, const ex & trONE = 4);

/** Bring all products of clifford objects in an expression into a canonical
 *  order. This is not necessarily the most simple form but it will allow
 *  to check two expressions for equality. */
ex canonicalize_clifford(const ex & e);

/** Automorphism of the Clifford algebra, simply changes signs of all
 *  clifford units. */
ex clifford_prime(const ex & e);

/** An auxillary function performing clifford_star() and clifford_bar().*/
ex clifford_star_bar(const ex & e, bool do_bar, unsigned options);

/** Main anti-automorphism of the Clifford algebra: makes reversion
 *  and changes signs of all clifford units. */
inline ex clifford_bar(const ex & e) { return clifford_star_bar(e, true, 0); }

/** Reversion of the Clifford algebra, reverse the order of all clifford units
 *  in ncmul. */
inline ex clifford_star(const ex & e) { return clifford_star_bar(e, false, 0); }

/** Replaces dirac_ONE's (with a representation_label no less than rl) in e with 1.
 *  For the default value rl = 0 remove all of them. Aborts if e contains any 
 *  clifford_unit with representation_label to be removed.
 *
 *  @param e Expression to be processed
 *  @param rl Value of representation label 
 *  @param options Defines some internal use */
ex remove_dirac_ONE(const ex & e, unsigned char rl = 0, unsigned options = 0);

/** Returns the maximal representation label of a clifford object 
 *  if e contains at least one, otherwise returns -1 
 *
 *  @param e Expression to be processed
 *  @ignore_ONE defines if clifford_ONE should be ignored in the search*/
int clifford_max_label(const ex & e, bool ignore_ONE = false);

/** Calculation of the norm in the Clifford algebra. */
ex clifford_norm(const ex & e);

/** Calculation of the inverse in the Clifford algebra. */
ex clifford_inverse(const ex & e);

/** List or vector conversion into the Clifford vector.
 *
 *  @param v List or vector of coordinates
 *  @param mu Index (must be of class varidx or a derived class)
 *  @param metr Metric (should be indexed, tensmetric or a derived class, or a matrix)
 *  @param rl Representation label
 *  @param e Clifford unit object
 *  @return Clifford vector with given components */
ex lst_to_clifford(const ex & v, const ex & mu,  const ex & metr, unsigned char rl = 0);
ex lst_to_clifford(const ex & v, const ex & e);

/** An inverse function to lst_to_clifford(). For given Clifford vector extracts
 *  its components with respect to given Clifford unit. Obtained components may 
 *  contain Clifford units with a different metric. Extraction is based on 
 *  the algebraic formula (e * c.i + c.i * e)/ pow(e.i, 2) for non-degenerate cases
 *  (i.e. neither pow(e.i, 2) = 0).
 *  
 *  @param e Clifford expression to be decomposed into components
 *  @param c Clifford unit defining the metric for splitting (should have numeric dimension of indices)
 *  @param algebraic Use algebraic or symbolic algorithm for extractions 
 *  @return List of components of a Clifford vector*/
lst clifford_to_lst(const ex & e, const ex & c, bool algebraic=true);

/** Calculations of Moebius transformations (conformal map) defined by a 2x2 Clifford matrix
 *  (a b\\c d) in linear spaces with arbitrary signature. The expression is 
 *  (a * x + b)/(c * x + d), where x is a vector build from list v with metric G.
 *  (see Jan Cnops. An introduction to {D}irac operators on manifolds, v.24 of
 *  Progress in Mathematical Physics. Birkhauser Boston Inc., Boston, MA, 2002.)
 * 
 *  @param a (1,1) entry of the defining matrix
 *  @param b (1,2) entry of the defining matrix
 *  @param c (2,1) entry of the defining matrix
 *  @param d (2,2) entry of the defining matrix
 *  @param v Vector to be transformed
 *  @param G Metric of the surrounding space, may be a Clifford unit then the next parameter is ignored
 *  @param rl Representation label 
 *  @return List of components of the transformed vector*/
ex clifford_moebius_map(const ex & a, const ex & b, const ex & c, const ex & d, const ex & v, const ex & G, unsigned char rl = 0);

/** The second form of Moebius transformations defined by a 2x2 Clifford matrix M
 *  This function takes the transformation matrix M as a single entity.
 * 
 *  @param M the defining matrix
 *  @param v Vector to be transformed
 *  @param G Metric of the surrounding space, may be a Clifford unit then the next parameter is ignored
 *  @param rl Representation label 
 *  @return List of components of the transformed vector*/
ex clifford_moebius_map(const ex & M, const ex & v, const ex & G, unsigned char rl = 0);

} // namespace GiNaC

#endif // ndef GINAC_CLIFFORD_H
