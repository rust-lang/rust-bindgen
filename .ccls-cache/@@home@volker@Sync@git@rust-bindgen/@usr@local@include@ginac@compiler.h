/** @file compiler.h
 *
 *  Definition of optimizing macros. */

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

#ifndef GINAC_COMPILER_H
#define GINAC_COMPILER_H

#ifdef __GNUC__
#define unlikely(cond) __builtin_expect((cond), 0)
#define likely(cond) __builtin_expect((cond), 1)
#define attribute_deprecated __attribute__ ((deprecated))
#else
#define unlikely(cond) (cond)
#define likely(cond) (cond)
#define attribute_deprecated
#endif

#endif // ndef GINAC_COMPILER_DEP_H
