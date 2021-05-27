/** @file symmetry.h
 *
 *  Interface to GiNaC's symmetry definitions. */

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

#ifndef GINAC_SYMMETRY_H
#define GINAC_SYMMETRY_H

#include "ex.h"
#include "archive.h"

#include <set>

namespace GiNaC {

class sy_is_less;
class sy_swap;

/** This class describes the symmetry of a group of indices. These objects
 *  can be grouped into a tree to form complex mixed symmetries. */
class symmetry : public basic
{
	friend class sy_is_less;
	friend class sy_swap;
	friend int canonicalize(exvector::iterator v, const symmetry &symm);

	GINAC_DECLARE_REGISTERED_CLASS(symmetry, basic)

	// types
public:
	/** Type of symmetry */
	typedef enum {
		none,          /**< no symmetry properties */
		symmetric,     /**< totally symmetric */
		antisymmetric, /**< totally antisymmetric */
		cyclic         /**< cyclic symmetry */
	} symmetry_type;

	// other constructors
public:
	/** Create leaf node that represents one index. */
	symmetry(unsigned i);

	/** Create node with two children. */
	symmetry(symmetry_type t, const symmetry &c1, const symmetry &c2);

	// functions overriding virtual functions from base classes
public:
	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	unsigned calchash() const override;

	// non-virtual functions in this class
public:
	/** Get symmetry type. */
	symmetry_type get_type() const {return type;}

	/** Set symmetry type. */
	void set_type(symmetry_type t) {type = t;}

	/** Add child node, check index sets for consistency. */
	symmetry &add(const symmetry &c);

	/** Verify that all indices of this node are in the range [0..n-1].
	 *  This function throws an exception if the verification fails.
	 *  If the top node has a type != none and no children, add all indices
	 *  in the range [0..n-1] as children. */
	void validate(unsigned n);

	/** Check whether this node actually represents any kind of symmetry. */
	bool has_symmetry() const {return type != none || !children.empty(); }
	/** Check whether this node involves anything non symmetric. */
	bool has_nonsymmetric() const;
	/** Check whether this node involves a cyclic symmetry. */
	bool has_cyclic() const;
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;

	// member variables
private:
	/** Type of symmetry described by this node. */
	symmetry_type type;

	/** Sorted union set of all indices handled by this node. */
	std::set<unsigned> indices;

	/** Vector of child nodes. */
	exvector children;
};
GINAC_DECLARE_UNARCHIVER(symmetry); 


// global functions

inline symmetry sy_none() { return symmetry(); }
inline symmetry sy_none(const symmetry &c1, const symmetry &c2) { return symmetry(symmetry::none, c1, c2); }
inline symmetry sy_none(const symmetry &c1, const symmetry &c2, const symmetry &c3) { return symmetry(symmetry::none, c1, c2).add(c3); }
inline symmetry sy_none(const symmetry &c1, const symmetry &c2, const symmetry &c3, const symmetry &c4) { return symmetry(symmetry::none, c1, c2).add(c3).add(c4); }

inline symmetry sy_symm() { symmetry s; s.set_type(symmetry::symmetric); return s; }
inline symmetry sy_symm(const symmetry &c1, const symmetry &c2) { return symmetry(symmetry::symmetric, c1, c2); }
inline symmetry sy_symm(const symmetry &c1, const symmetry &c2, const symmetry &c3) { return symmetry(symmetry::symmetric, c1, c2).add(c3); }
inline symmetry sy_symm(const symmetry &c1, const symmetry &c2, const symmetry &c3, const symmetry &c4) { return symmetry(symmetry::symmetric, c1, c2).add(c3).add(c4); }

inline symmetry sy_anti() { symmetry s; s.set_type(symmetry::antisymmetric); return s; }
inline symmetry sy_anti(const symmetry &c1, const symmetry &c2) { return symmetry(symmetry::antisymmetric, c1, c2); }
inline symmetry sy_anti(const symmetry &c1, const symmetry &c2, const symmetry &c3) { return symmetry(symmetry::antisymmetric, c1, c2).add(c3); }
inline symmetry sy_anti(const symmetry &c1, const symmetry &c2, const symmetry &c3, const symmetry &c4) { return symmetry(symmetry::antisymmetric, c1, c2).add(c3).add(c4); }

inline symmetry sy_cycl() { symmetry s; s.set_type(symmetry::cyclic); return s; }
inline symmetry sy_cycl(const symmetry &c1, const symmetry &c2) { return symmetry(symmetry::cyclic, c1, c2); }
inline symmetry sy_cycl(const symmetry &c1, const symmetry &c2, const symmetry &c3) { return symmetry(symmetry::cyclic, c1, c2).add(c3); }
inline symmetry sy_cycl(const symmetry &c1, const symmetry &c2, const symmetry &c3, const symmetry &c4) { return symmetry(symmetry::cyclic, c1, c2).add(c3).add(c4); }

// These return references to preallocated common symmetries (similar to
// the numeric flyweights).
const symmetry & not_symmetric();
const symmetry & symmetric2();
const symmetry & symmetric3();
const symmetry & symmetric4();
const symmetry & antisymmetric2();
const symmetry & antisymmetric3();
const symmetry & antisymmetric4();

/** Canonicalize the order of elements of an expression vector, according to
 *  the symmetry properties defined in a symmetry tree.
 *
 *  @param v Start of expression vector
 *  @param symm Root node of symmetry tree
 *  @return the overall sign introduced by the reordering (+1, -1 or 0)
 *          or numeric_limits<int>::max() if nothing changed */
extern int canonicalize(exvector::iterator v, const symmetry &symm);

/** Symmetrize expression over a set of objects (symbols, indices). */
ex symmetrize(const ex & e, exvector::const_iterator first, exvector::const_iterator last);

/** Symmetrize expression over a set of objects (symbols, indices). */
inline ex symmetrize(const ex & e, const exvector & v)
{
	return symmetrize(e, v.begin(), v.end());
}

/** Antisymmetrize expression over a set of objects (symbols, indices). */
ex antisymmetrize(const ex & e, exvector::const_iterator first, exvector::const_iterator last);

/** Antisymmetrize expression over a set of objects (symbols, indices). */
inline ex antisymmetrize(const ex & e, const exvector & v)
{
	return antisymmetrize(e, v.begin(), v.end());
}

/** Symmetrize expression by cyclic permutation over a set of objects
 *  (symbols, indices). */
ex symmetrize_cyclic(const ex & e, exvector::const_iterator first, exvector::const_iterator last);

/** Symmetrize expression by cyclic permutation over a set of objects
 *  (symbols, indices). */
inline ex symmetrize_cyclic(const ex & e, const exvector & v)
{
	return symmetrize(e, v.begin(), v.end());
}

} // namespace GiNaC

#endif // ndef GINAC_SYMMETRY_H
