/** @file indexed.h
 *
 *  Interface to GiNaC's indexed expressions. */

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

#ifndef GINAC_INDEXED_H
#define GINAC_INDEXED_H

#include "exprseq.h"
#include "wildcard.h"

#include <map>

namespace GiNaC {

class scalar_products;
class symmetry;

/** This class holds an indexed expression. It consists of a 'base' expression
 *  (the expression being indexed) which can be accessed as op(0), and n (n >= 0)
 *  indices (all of class idx), accessible as op(1)..op(n). */
class indexed : public exprseq
{
	GINAC_DECLARE_REGISTERED_CLASS(indexed, exprseq)

	friend ex simplify_indexed(const ex & e, exvector & free_indices, exvector & dummy_indices, const scalar_products & sp);
	friend ex simplify_indexed_product(const ex & e, exvector & free_indices, exvector & dummy_indices, const scalar_products & sp);
	friend bool reposition_dummy_indices(ex & e, exvector & variant_dummy_indices, exvector & moved_indices);

	// other constructors
public:
	/** Construct indexed object with no index.
	 *
	 *  @param b Base expression
	 *  @return newly constructed indexed object */
	indexed(const ex & b);

	/** Construct indexed object with one index. The index must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param i1 The index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const ex & i1);

	/** Construct indexed object with two indices. The indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param i1 First index
	 *  @param i2 Second index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const ex & i1, const ex & i2);

	/** Construct indexed object with three indices. The indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param i1 First index
	 *  @param i2 Second index
	 *  @param i3 Third index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const ex & i1, const ex & i2, const ex & i3);

	/** Construct indexed object with four indices. The indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param i1 First index
	 *  @param i2 Second index
	 *  @param i3 Third index
	 *  @param i4 Fourth index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const ex & i1, const ex & i2, const ex & i3, const ex & i4);

	/** Construct indexed object with two indices and a specified symmetry. The
	 *  indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param symm Symmetry of indices
	 *  @param i1 First index
	 *  @param i2 Second index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const symmetry & symm, const ex & i1, const ex & i2);

	/** Construct indexed object with three indices and a specified symmetry.
	 *  The indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param symm Symmetry of indices
	 *  @param i1 First index
	 *  @param i2 Second index
	 *  @param i3 Third index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const symmetry & symm, const ex & i1, const ex & i2, const ex & i3);

	/** Construct indexed object with four indices and a specified symmetry. The
	 *  indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param symm Symmetry of indices
	 *  @param i1 First index
	 *  @param i2 Second index
	 *  @param i3 Third index
	 *  @param i4 Fourth index
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const symmetry & symm, const ex & i1, const ex & i2, const ex & i3, const ex & i4);

	/** Construct indexed object with a specified vector of indices. The indices
	 *  must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param iv Vector of indices
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const exvector & iv);

	/** Construct indexed object with a specified vector of indices and
	 *  symmetry. The indices must be of class idx.
	 *
	 *  @param b Base expression
	 *  @param symm Symmetry of indices
	 *  @param iv Vector of indices
	 *  @return newly constructed indexed object */
	indexed(const ex & b, const symmetry & symm, const exvector & iv);

	// internal constructors
	indexed(const symmetry & symm, const exprseq & es);
	indexed(const symmetry & symm, const exvector & v);
	indexed(const symmetry & symm, exvector && v);

	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 55;}
	bool info(unsigned inf) const override;
	ex eval() const override;
	ex real_part() const override;
	ex imag_part() const override;
	exvector get_free_indices() const override;

	/** Save (a.k.a. serialize) indexed object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) indexed object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	ex derivative(const symbol & s) const override;
	ex thiscontainer(const exvector & v) const override;
	ex thiscontainer(exvector && v) const override;
	unsigned return_type() const override;
	return_type_t return_type_tinfo() const override { return op(0).return_type_tinfo(); }
	ex expand(unsigned options = 0) const override;

	// new virtual functions which can be overridden by derived classes
	// none
	
	// non-virtual functions in this class
public:
	/** Check whether all index values have a certain property.
	 *  @see class info_flags */
	bool all_index_values_are(unsigned inf) const;

	/** Return a vector containing the object's indices. */
	exvector get_indices() const;

	/** Return a vector containing the dummy indices of the object, if any. */
	exvector get_dummy_indices() const;

	/** Return a vector containing the dummy indices in the contraction with
	 *  another indexed object. This is symmetric: a.get_dummy_indices(b)
	 *  == b.get_dummy_indices(a) */
	exvector get_dummy_indices(const indexed & other) const;

	/** Check whether the object has an index that forms a dummy index pair
	 *  with a given index. */
	bool has_dummy_index_for(const ex & i) const;

	/** Return symmetry properties. */
	ex get_symmetry() const {return symtree;}

protected:
	void printindices(const print_context & c, unsigned level) const;
	void print_indexed(const print_context & c, const char *openbrace, const char *closebrace, unsigned level) const;
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;
	void validate() const;

	// member variables
protected:
	ex symtree; /**< Index symmetry (tree of symmetry objects) */
};
GINAC_DECLARE_UNARCHIVER(indexed);


class spmapkey {
public:
	spmapkey() : dim(wild()) {}
	spmapkey(const ex & v1, const ex & v2, const ex & dim = wild());

	bool operator==(const spmapkey &other) const;
	bool operator<(const spmapkey &other) const;

	void debugprint() const;

protected:
	ex v1, v2, dim;
};

typedef std::map<spmapkey, ex> spmap;

/** Helper class for storing information about known scalar products which
 *  are to be automatically replaced by simplify_indexed().
 *
 *  @see simplify_indexed */
class scalar_products {
public:
	/** Register scalar product pair and its value. */
	void add(const ex & v1, const ex & v2, const ex & sp);

	/** Register scalar product pair and its value for a specific space dimension. */
	void add(const ex & v1, const ex & v2, const ex & dim, const ex & sp);

	/** Register list of vectors. This adds all possible pairs of products
	 *  a.i * b.i with the value a*b (note that this is not a scalar vector
	 *  product but an ordinary product of scalars). */
	void add_vectors(const lst & l, const ex & dim = wild());

	/** Clear all registered scalar products. */
	void clear();

	bool is_defined(const ex & v1, const ex & v2, const ex & dim) const;
	ex evaluate(const ex & v1, const ex & v2, const ex & dim) const;
	void debugprint() const;

protected:
	spmap spm; /*< Map from defined scalar product pairs to their values */
};


// utility functions

/** Returns all dummy indices from the expression */
exvector get_all_dummy_indices(const ex & e);

/** More reliable version of the form. The former assumes that e is an
  * expanded expression. */
exvector get_all_dummy_indices_safely(const ex & e);

/** Returns b with all dummy indices, which are listed in va, renamed 
 *  if modify_va is set to TRUE all dummy indices of b will be appended to va */
ex rename_dummy_indices_uniquely(exvector & va, const ex & b, bool modify_va = false);

/** Returns b with all dummy indices, which are common with a, renamed */
ex rename_dummy_indices_uniquely(const ex & a, const ex & b);

/** Same as above, where va and vb contain the indices of a and b and are sorted */
ex rename_dummy_indices_uniquely(const exvector & va, const exvector & vb, const ex & b);

/** Similar to above, where va and vb are the same and the return value is a list of two lists 
 *  for substitution in b */
lst rename_dummy_indices_uniquely(const exvector & va, const exvector & vb);

/** This function returns the given expression with expanded sums
 *  for all dummy index summations, where the dimensionality of 
 *  the dummy index is a nonnegative integer.
 *  Optionally all indices with a variance will be substituted by 
 *  indices with the corresponding numeric values without variance.
 *
 *  @param e the given expression
 *  @param subs_idx indicates if variance of dummy indices should be neglected
 */
ex expand_dummy_sum(const ex & e, bool subs_idx = false);

} // namespace GiNaC

#endif // ndef GINAC_INDEXED_H
