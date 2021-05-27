/** @file relational.h
 *
 *  Interface to relations between expressions. */

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

#ifndef GINAC_RELATIONAL_H
#define GINAC_RELATIONAL_H

#include "basic.h"
#include "ex.h"
#include "archive.h"

namespace GiNaC {

/** This class holds a relation consisting of two expressions and a logical
 *  relation between them. */
class relational : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(relational, basic)

// types
public:
	enum operators {
		equal,
		not_equal,
		less,
		less_or_equal,
		greater,
		greater_or_equal
	};
	
	// other constructors
public:
	relational(const ex & lhs, const ex & rhs, operators oper=equal);
	
	// functions overriding virtual functions from base classes
public:
	unsigned precedence() const override {return 20;}
	bool info(unsigned inf) const override;
	size_t nops() const override;
	ex op(size_t i) const override;
	ex map(map_function & f) const override;
	ex subs(const exmap & m, unsigned options = 0) const override;

	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	ex eval_ncmul(const exvector & v) const override;
	bool match_same_type(const basic & other) const override;
	unsigned return_type() const override;
	return_type_t return_type_tinfo() const override;
	unsigned calchash() const override;

	// new virtual functions which can be overridden by derived classes
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;

public:
	ex lhs() const { return lh; }
	ex rhs() const { return rh; }

	// non-virtual functions in this class
private:
	// For conversions to Boolean, as would be used in an if conditional,
	// implicit conversions from bool to int have a large number of
	// undesirable side effects.  The following safe_bool type enables
	// use of relational objects in conditionals without those side effects
	struct safe_bool_helper {
		void nonnull() {};
	};

	typedef void (safe_bool_helper::*safe_bool)();
	
	safe_bool make_safe_bool(bool) const;

public:
	operator safe_bool() const;
	safe_bool operator!() const;

// member variables
	
protected:
	ex lh;
	ex rh;
	operators o;
};
GINAC_DECLARE_UNARCHIVER(relational); 

// utility functions

// inlined functions for efficiency
inline relational::safe_bool relational::operator!() const
{
	return make_safe_bool(!static_cast<bool>(*this));
}

} // namespace GiNaC

#endif // ndef GINAC_RELATIONAL_H
