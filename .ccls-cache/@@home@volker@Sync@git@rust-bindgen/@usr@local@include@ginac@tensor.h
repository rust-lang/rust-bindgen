/** @file tensor.h
 *
 *  Interface to GiNaC's special tensors. */

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

#ifndef GINAC_TENSOR_H
#define GINAC_TENSOR_H

#include "ex.h"
#include "archive.h"

namespace GiNaC {

/** This class holds one of GiNaC's predefined special tensors such as the
 *  delta and the metric tensors. They are represented without indices.
 *  To attach indices to them, wrap them in an object of class indexed. */
class tensor : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(tensor, basic)

	// functions overriding virtual functions from base classes
protected:
	unsigned return_type() const override { return return_types::noncommutative_composite; }

	// non-virtual functions in this class
public:
	/** Replace dummy index in contracted-with object by the contracting
	 *  object's second index (used internally for delta and metric tensor
	 *  contractions. */
	bool replace_contr_index(exvector::iterator self, exvector::iterator other) const;
};


/** This class represents the delta tensor. If indexed, it must have exactly
 *  two indices of the same type. */
class tensdelta : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(tensdelta, tensor)

	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override;
	ex eval_indexed(const basic & i) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;
protected:
	unsigned return_type() const override { return return_types::commutative; }

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(tensdelta);


/** This class represents a general metric tensor which can be used to
 *  raise/lower indices. If indexed, it must have exactly two indices of the
 *  same type which must be of class varidx or a subclass. */
class tensmetric : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(tensmetric, tensor)

	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override;
	ex eval_indexed(const basic & i) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;
protected:
	unsigned return_type() const override { return return_types::commutative; }

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(tensmetric);


/** This class represents a Minkowski metric tensor. It has all the
 *  properties of a metric tensor and is (as a matrix) equal to
 *  diag(1,-1,-1,...) or diag(-1,1,1,...). */
class minkmetric : public tensmetric
{
	GINAC_DECLARE_REGISTERED_CLASS(minkmetric, tensmetric)

	// other constructors
public:
	/** Construct Lorentz metric tensor with given signature. */
	minkmetric(bool pos_sig);

	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override;
	ex eval_indexed(const basic & i) const override;

	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	unsigned return_type() const override { return return_types::commutative; }

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;

	// member variables
private:
	bool pos_sig; /**< If true, the metric is diag(-1,1,1...). Otherwise it is diag(1,-1,-1,...). */
};
GINAC_DECLARE_UNARCHIVER(minkmetric); 


/** This class represents an antisymmetric spinor metric tensor which
 *  can be used to raise/lower indices of 2-component Weyl spinors. If
 *  indexed, it must have exactly two indices of the same type which
 *  must be of class spinidx or a subclass and have dimension 2. */
class spinmetric : public tensmetric
{
	GINAC_DECLARE_REGISTERED_CLASS(spinmetric, tensmetric)

	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override;
	ex eval_indexed(const basic & i) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;

protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(spinmetric);


/** This class represents the totally antisymmetric epsilon tensor. If
 *  indexed, all indices must be of the same type and their number must
 *  be equal to the dimension of the index space. */
class tensepsilon : public tensor
{
	GINAC_DECLARE_REGISTERED_CLASS(tensepsilon, tensor)

	// other constructors
public:
	tensepsilon(bool minkowski, bool pos_sig);

	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override;
	ex eval_indexed(const basic & i) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;

	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	unsigned return_type() const override { return return_types::commutative; }

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;

	// member variables
private:
	bool minkowski; /**< If true, tensor is in Minkowski-type space. Otherwise it is in a Euclidean space. */
	bool pos_sig;   /**< If true, the metric is assumed to be diag(-1,1,1...). Otherwise it is diag(1,-1,-1,...). This is only relevant if minkowski = true. */
};
GINAC_DECLARE_UNARCHIVER(tensepsilon); 


// utility functions

/** Create a delta tensor with specified indices. The indices must be of class
 *  idx or a subclass. The delta tensor is always symmetric and its trace is
 *  the dimension of the index space.
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @return newly constructed delta tensor */
ex delta_tensor(const ex & i1, const ex & i2);

/** Create a symmetric metric tensor with specified indices. The indices
 *  must be of class varidx or a subclass. A metric tensor with one
 *  covariant and one contravariant index is equivalent to the delta tensor.
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @return newly constructed metric tensor */
ex metric_tensor(const ex & i1, const ex & i2);

/** Create a Minkowski metric tensor with specified indices. The indices
 *  must be of class varidx or a subclass. The Lorentz metric is a symmetric
 *  tensor with a matrix representation of diag(1,-1,-1,...) (negative
 *  signature, the default) or diag(-1,1,1,...) (positive signature).
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @param pos_sig Whether the signature is positive
 *  @return newly constructed Lorentz metric tensor */
ex lorentz_g(const ex & i1, const ex & i2, bool pos_sig = false);

/** Create a spinor metric tensor with specified indices. The indices must be
 *  of class spinidx or a subclass and have a dimension of 2. The spinor
 *  metric is an antisymmetric tensor with a matrix representation of
 *  [[ [[ 0, 1 ]], [[ -1, 0 ]] ]].
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @return newly constructed spinor metric tensor */
ex spinor_metric(const ex & i1, const ex & i2);

/** Create an epsilon tensor in a Euclidean space with two indices. The
 *  indices must be of class idx or a subclass, and have a dimension of 2.
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @return newly constructed epsilon tensor */
ex epsilon_tensor(const ex & i1, const ex & i2);

/** Create an epsilon tensor in a Euclidean space with three indices. The
 *  indices must be of class idx or a subclass, and have a dimension of 3.
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @param i3 Third index
 *  @return newly constructed epsilon tensor */
ex epsilon_tensor(const ex & i1, const ex & i2, const ex & i3);

/** Create an epsilon tensor in a Minkowski space with four indices. The
 *  indices must be of class varidx or a subclass, and have a dimension of 4.
 *
 *  @param i1 First index
 *  @param i2 Second index
 *  @param i3 Third index
 *  @param i4 Fourth index
 *  @param pos_sig Whether the signature of the metric is positive
 *  @return newly constructed epsilon tensor */
ex lorentz_eps(const ex & i1, const ex & i2, const ex & i3, const ex & i4, bool pos_sig = false);

} // namespace GiNaC

#endif // ndef GINAC_TENSOR_H
