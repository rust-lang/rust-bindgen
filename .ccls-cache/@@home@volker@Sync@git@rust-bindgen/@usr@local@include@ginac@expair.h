/** @file expair.h
 *
 *  Definition of expression pairs (building blocks of expairseq). */

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

#ifndef GINAC_EXPAIR_H
#define GINAC_EXPAIR_H

#include "ex.h"
#include "numeric.h"
#include "print.h"

namespace GiNaC {

/** A pair of expressions.
 *  This is similar to STL's pair<>.  It is slightly extended since we need to
 *  account for methods like .compare().  Also, since this is meant for use by
 *  class expairseq it must satisfy the invariance that the member coeff must
 *  be of type numeric. */
class expair
{
public:
	expair() : rest(0), coeff(1) { }

	/** Construct an expair from two ex. */
	expair(const ex & r, const ex & c) : rest(r), coeff(c)
	{
		GINAC_ASSERT(is_exactly_a<numeric>(coeff));
	}
	
	/** Member-wise check for canonical ordering equality. */
	bool is_equal(const expair & other) const
	{
		return (rest.is_equal(other.rest) && coeff.is_equal(other.coeff));
	}
	
	/** Member-wise check for canonical ordering lessness. */
	bool is_less(const expair & other) const 
	{
		int restcmp = rest.compare(other.rest);
		return ((restcmp<0) ||
		        (!(restcmp>0) && (coeff.compare(other.coeff)<0)));
	}
	
	/** Member-wise check for canonical ordering. */
	int compare(const expair & other) const
	{
		int restcmp = rest.compare(other.rest);
		if (restcmp!=0)
			return restcmp;
		else
			return coeff.compare(other.coeff);
	}
	
	void print(std::ostream & os) const;
	
	/** True if this is of the form (numeric,ex(1)). */
	bool is_canonical_numeric() const
	{
		GINAC_ASSERT(is_exactly_a<numeric>(coeff));
		return (is_exactly_a<numeric>(rest) && (coeff.is_equal(1)));
	}

	/** Swap contents with other expair. */
	void swap(expair & other)
	{
		rest.swap(other.rest);
		coeff.swap(other.coeff);
	}

	const expair conjugate() const;

	ex rest;    ///< first member of pair, an arbitrary expression
	ex coeff;   ///< second member of pair, must be numeric
};

/** Function object for insertion into third argument of STL's sort() etc. */
struct expair_is_less {
	bool operator()(const expair &lh, const expair &rh) const { return lh.is_less(rh); }
};

/** Function object not caring about the numerical coefficients for insertion
 *  into third argument of STL's sort().  Note that this does not define a
 *  strict weak ordering since for any symbol x we have neither 3*x<2*x or
 *  2*x<3*x.  Handle with care! */
struct expair_rest_is_less {
	bool operator()(const expair &lh, const expair &rh) const { return (lh.rest.compare(rh.rest)<0); }
};

struct expair_swap {
	void operator()(expair &lh, expair &rh) const { lh.swap(rh); }
};

inline void swap(expair & e1, expair & e2)
{ e1.swap(e2); }

} // namespace GiNaC

#endif // ndef GINAC_EXPAIR_H
