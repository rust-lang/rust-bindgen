/** @file matrix.h
 *
 *  Interface to symbolic matrices */

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

#ifndef GINAC_MATRIX_H
#define GINAC_MATRIX_H

#include "basic.h"
#include "ex.h"
#include "archive.h"
#include "compiler.h"

#include <string>
#include <vector>

namespace GiNaC {

/** Helper template to allow initialization of matrices via an overloaded
 *  comma operator (idea stolen from Blitz++). */
template <typename T, typename It>
class matrix_init {
public:
	matrix_init(It i) : iter(i) {}

	matrix_init<T, It> operator,(const T & x)
	{
		*iter = x;
		return matrix_init<T, It>(++iter);
	}

	// The following specializations produce much tighter code than the
	// general case above

	matrix_init<T, It> operator,(int x)
	{
		*iter = T(x);
		return matrix_init<T, It>(++iter);
	}

	matrix_init<T, It> operator,(unsigned int x)
	{
		*iter = T(x);
		return matrix_init<T, It>(++iter);
	}

	matrix_init<T, It> operator,(long x)
	{
		*iter = T(x);
		return matrix_init<T, It>(++iter);
	}

	matrix_init<T, It> operator,(unsigned long x)
	{
		*iter = T(x);
		return matrix_init<T, It>(++iter);
	}

	matrix_init<T, It> operator,(double x)
	{
		*iter = T(x);
		return matrix_init<T, It>(++iter);
	}

	matrix_init<T, It> operator,(const symbol & x)
	{
		*iter = T(x);
		return matrix_init<T, It>(++iter);
	}

private:
	matrix_init();
	It iter;
};


/** Symbolic matrices. */
class matrix : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(matrix, basic)
	
	// other constructors
public:
	matrix(unsigned r, unsigned c);
	matrix(unsigned r, unsigned c, const lst & l);
	matrix(std::initializer_list<std::initializer_list<ex>> l);

	matrix_init<ex, exvector::iterator> operator=(const ex & x) attribute_deprecated;
protected:
	matrix(unsigned r, unsigned c, const exvector & m2);
	matrix(unsigned r, unsigned c, exvector && m2);
	// functions overriding virtual functions from base classes
public:
	size_t nops() const override;
	ex op(size_t i) const override;
	ex & let_op(size_t i) override;
	ex evalm() const override {return *this;}
	ex subs(const exmap & m, unsigned options = 0) const override;
	ex eval_indexed(const basic & i) const override;
	ex add_indexed(const ex & self, const ex & other) const override;
	ex scalar_mul_indexed(const ex & self, const numeric & other) const override;
	bool contract_with(exvector::iterator self, exvector::iterator other, exvector & v) const override;
	ex conjugate() const override;
	ex real_part() const override;
	ex imag_part() const override;

	/** Save (a.k.a. serialize) object into archive. */
	void archive(archive_node& n) const override;
	/** Read (a.k.a. deserialize) object from archive. */
	void read_archive(const archive_node& n, lst& syms) override;
protected:
	bool match_same_type(const basic & other) const override;
	unsigned return_type() const override { return return_types::noncommutative; };
	
	// non-virtual functions in this class
public:
	unsigned rows() const        /// Get number of rows.
		{ return row; }
	unsigned cols() const        /// Get number of columns.
		{ return col; }
	matrix add(const matrix & other) const;
	matrix sub(const matrix & other) const;
	matrix mul(const matrix & other) const;
	matrix mul(const numeric & other) const;
	matrix mul_scalar(const ex & other) const;
	matrix pow(const ex & expn) const;
	const ex & operator() (unsigned ro, unsigned co) const;
	ex & operator() (unsigned ro, unsigned co);
	matrix & set(unsigned ro, unsigned co, const ex & value) { (*this)(ro, co) = value; return *this; }
	matrix transpose() const;
	ex determinant(unsigned algo = determinant_algo::automatic) const;
	ex trace() const;
	ex charpoly(const ex & lambda) const;
	matrix inverse() const;
	matrix inverse(unsigned algo) const;
	matrix solve(const matrix & vars, const matrix & rhs,
	             unsigned algo = solve_algo::automatic) const;
	unsigned rank() const;
	unsigned rank(unsigned solve_algo) const;
	bool is_zero_matrix() const;
protected:
	ex determinant_minor() const;
	std::vector<unsigned> echelon_form(unsigned algo, int n);
	int gauss_elimination(const bool det = false);
	int division_free_elimination(const bool det = false);
	int fraction_free_elimination(const bool det = false);
	std::vector<unsigned> markowitz_elimination(unsigned n);
	int pivot(unsigned ro, unsigned co, bool symbolic = true);

	void print_elements(const print_context & c, const char *row_start, const char *row_end, const char *row_sep, const char *col_sep) const;
	void do_print(const print_context & c, unsigned level) const;
	void do_print_latex(const print_latex & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;
	
// member variables
protected:
	unsigned row;             ///< number of rows
	unsigned col;             ///< number of columns
	exvector m;               ///< representation (cols indexed first)
};
GINAC_DECLARE_UNARCHIVER(matrix); 

// First step of initialization of matrix with a comma-separated sequence
// of expressions. Subsequent steps are handled by matrix_init<>::operator,().
inline matrix_init<ex, exvector::iterator> matrix::operator=(const ex & x)
{
	m[0] = x;
	return matrix_init<ex, exvector::iterator>(++m.begin());
}

// wrapper functions around member functions

inline size_t nops(const matrix & m)
{ return m.nops(); }

inline ex expand(const matrix & m, unsigned options = 0)
{ return m.expand(options); }

inline ex evalf(const matrix & m)
{ return m.evalf(); }

inline unsigned rows(const matrix & m)
{ return m.rows(); }

inline unsigned cols(const matrix & m)
{ return m.cols(); }

inline matrix transpose(const matrix & m)
{ return m.transpose(); }

inline ex determinant(const matrix & m, unsigned options = determinant_algo::automatic)
{ return m.determinant(options); }

inline ex trace(const matrix & m)
{ return m.trace(); }

inline ex charpoly(const matrix & m, const ex & lambda)
{ return m.charpoly(lambda); }

inline matrix inverse(const matrix & m)
{ return m.inverse(solve_algo::automatic); }
inline matrix inverse(const matrix & m, unsigned algo)
{ return m.inverse(algo); }

inline unsigned rank(const matrix & m)
{ return m.rank(); }
inline unsigned rank(const matrix & m, unsigned solve_algo)
{ return m.rank(solve_algo); }

// utility functions

/** Convert list of lists to matrix. */
extern ex lst_to_matrix(const lst & l);

/** Convert list of diagonal elements to matrix. */
extern ex diag_matrix(const lst & l);
extern ex diag_matrix(std::initializer_list<ex> l);

/** Create an r times c unit matrix. */
extern ex unit_matrix(unsigned r, unsigned c);

/** Create a x times x unit matrix. */
inline ex unit_matrix(unsigned x)
{ return unit_matrix(x, x); }

/** Create an r times c matrix of newly generated symbols consisting of the
 *  given base name plus the numeric row/column position of each element.
 *  The base name for LaTeX output is specified separately. */
extern ex symbolic_matrix(unsigned r, unsigned c, const std::string & base_name, const std::string & tex_base_name);

/** Return the reduced matrix that is formed by deleting the rth row and cth
 *  column of matrix m. The determinant of the result is the Minor r, c. */
extern ex reduced_matrix(const matrix& m, unsigned r, unsigned c);

/** Return the nr times nc submatrix starting at position r, c of matrix m. */
extern ex sub_matrix(const matrix&m, unsigned r, unsigned nr, unsigned c, unsigned nc);

/** Create an r times c matrix of newly generated symbols consisting of the
 *  given base name plus the numeric row/column position of each element. */
inline ex symbolic_matrix(unsigned r, unsigned c, const std::string & base_name)
{ return symbolic_matrix(r, c, base_name, base_name); }

} // namespace GiNaC

#endif // ndef GINAC_MATRIX_H
