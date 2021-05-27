/** @file operators.h
 *
 *  Interface to GiNaC's overloaded operators. */

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

#ifndef GINAC_OPERATORS_H
#define GINAC_OPERATORS_H

#include <iosfwd>

namespace GiNaC {

class ex;
class numeric;
class relational;

// binary arithmetic operators ex with ex
const ex operator+(const ex & lh, const ex & rh);
const ex operator-(const ex & lh, const ex & rh);
const ex operator*(const ex & lh, const ex & rh);
const ex operator/(const ex & lh, const ex & rh);

// binary arithmetic operators numeric with numeric
const numeric operator+(const numeric & lh, const numeric & rh);
const numeric operator-(const numeric & lh, const numeric & rh);
const numeric operator*(const numeric & lh, const numeric & rh);
const numeric operator/(const numeric & lh, const numeric & rh);

// binary arithmetic assignment operators with ex
ex & operator+=(ex & lh, const ex & rh);
ex & operator-=(ex & lh, const ex & rh);
ex & operator*=(ex & lh, const ex & rh);
ex & operator/=(ex & lh, const ex & rh);

// binary arithmetic assignment operators with numeric
numeric & operator+=(numeric & lh, const numeric & rh);
numeric & operator-=(numeric & lh, const numeric & rh);
numeric & operator*=(numeric & lh, const numeric & rh);
numeric & operator/=(numeric & lh, const numeric & rh);

// unary operators
const ex operator+(const ex & lh);
const ex operator-(const ex & lh);

const numeric operator+(const numeric & lh);
const numeric operator-(const numeric & lh);

// increment / decrement operators
ex & operator++(ex & rh);
ex & operator--(ex & rh);
const ex operator++(ex & lh, int);
const ex operator--(ex & lh, int);

numeric& operator++(numeric & rh);
numeric& operator--(numeric & rh);
const numeric operator++(numeric & lh, int);
const numeric operator--(numeric & lh, int);

// binary relational operators ex with ex
const relational operator==(const ex & lh, const ex & rh);
const relational operator!=(const ex & lh, const ex & rh);
const relational operator<(const ex & lh, const ex & rh);
const relational operator<=(const ex & lh, const ex & rh);
const relational operator>(const ex & lh, const ex & rh);
const relational operator>=(const ex & lh, const ex & rh);

// input/output stream operators
std::ostream & operator<<(std::ostream & os, const ex & e);
std::istream & operator>>(std::istream & is, ex & e);

// input/output stream manipulators
std::ostream & dflt(std::ostream & os);
std::ostream & latex(std::ostream & os);
std::ostream & python(std::ostream & os);
std::ostream & python_repr(std::ostream & os);
std::ostream & tree(std::ostream & os);
std::ostream & csrc(std::ostream & os); // same as csrc_double
std::ostream & csrc_float(std::ostream & os);
std::ostream & csrc_double(std::ostream & os);
std::ostream & csrc_cl_N(std::ostream & os);

std::ostream & index_dimensions(std::ostream & os);
std::ostream & no_index_dimensions(std::ostream & os);

} // namespace GiNaC

#endif // ndef GINAC_OPERATORS_H
