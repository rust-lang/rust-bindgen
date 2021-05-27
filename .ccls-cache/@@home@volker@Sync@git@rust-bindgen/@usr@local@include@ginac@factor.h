/** @file factor.h
 *
 *  Polynomial factorization. */

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

#ifndef GINAC_FACTOR_H
#define GINAC_FACTOR_H

namespace GiNaC {

class ex;

/** Factorizes univariate and multivariate polynomials.
 *  
 *  The default option is factor_options::polynomial, which means that factor()
 *  will only factorize an expression if it is a proper polynomial (i.e. the
 *  flag info_flags::polynomial is set). Given the option factor_options::all,
 *  factor() will factorize all subexpressions, e.g. polynomials containing
 *  functions or polynomials inside function arguments.
 *
 *  @param[in] poly    expression to factorize
 *  @param[in] option  options to influence the factorization
 *  @return            factorized expression
 */
extern ex factor(const ex& poly, unsigned options = 0);

} // namespace GiNaC

#endif // ndef GINAC_FACTOR_H
