/** @file symbol.h
 *
 *  Interface to GiNaC's symbolic objects. */

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

#ifndef GINAC_SYMBOL_H
#define GINAC_SYMBOL_H

#include "basic.h"
#include "ex.h"
#include "ptr.h"
#include "archive.h"

#include <string>
#include <typeinfo>

namespace GiNaC {

/** Basic CAS symbol.  It has a name because it must know how to output itself.
 */
class symbol : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(symbol, basic)
	// other constructors
public:
	explicit symbol(const std::string & initname);
	symbol(const std::string & initname, const std::string & texname);
	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override;
	ex eval() const override { return *this; } // for performance reasons
	ex evalf() const override { return *this; } // overwrites basic::evalf() for performance reasons
	ex series(const relational & s, int order, unsigned options = 0) const override;
	ex subs(const exmap & m, unsigned options = 0) const override { return subs_one_level(m, options); } // overwrites basic::subs() for performance reasons
	ex normal(exmap & repl, exmap & rev_lookup) const override;
	ex to_rational(exmap & repl) const override;
	ex to_polynomial(exmap & repl) const override;
	ex conjugate() const override;
	ex real_part() const override;
	ex imag_part() const override;
	bool is_polynomial(const ex & var) const override;
	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	ex derivative(const symbol & s) const override;
	bool is_equal_same_type(const basic & other) const override;
	unsigned calchash() const override;

	// new virtual functions which can be overridden by derived classes
public:
	virtual unsigned get_domain() const { return domain::complex; }
	
	// non-virtual functions in this class
public:
	void set_name(const std::string & n) { name = n; }
	void set_TeX_name(const std::string & n) { TeX_name = n; }
	std::string get_name() const;
	std::string get_TeX_name() const;
protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;

// member variables

protected:
	unsigned serial;                 ///< unique serial number for comparison
	mutable std::string name;        ///< printname of this symbol
	std::string TeX_name;            ///< LaTeX name of this symbol
private:
	static unsigned next_serial;
};
GINAC_DECLARE_UNARCHIVER(symbol);


/** Specialization of symbol to real domain */
class realsymbol : public symbol
{
public:
	realsymbol();
	explicit realsymbol(const std::string & initname);
	realsymbol(const std::string & initname, const std::string & texname);

	unsigned get_domain() const override { return domain::real; }

	ex conjugate() const override { return *this; }
	ex real_part() const override { return *this; }
	ex imag_part() const override { return 0; }

	realsymbol* duplicate() const override
	{
		realsymbol * bp = new realsymbol(*this);
		bp->setflag(status_flags::dynallocated);
		return bp;
	}
};
GINAC_DECLARE_UNARCHIVER(realsymbol);


/** Specialization of symbol to real positive domain */
class possymbol : public realsymbol
{
public:
	possymbol();
	explicit possymbol(const std::string & initname);
	possymbol(const std::string & initname, const std::string & texname);

	unsigned get_domain() const override { return domain::positive; }

	possymbol* duplicate() const override
	{
		possymbol * bp = new possymbol(*this);
		bp->setflag(status_flags::dynallocated);
		return bp;
	}
};
GINAC_DECLARE_UNARCHIVER(possymbol);

} // namespace GiNaC

#endif // ndef GINAC_SYMBOL_H
