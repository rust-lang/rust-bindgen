/** @file function.h
 *
 *  Interface to class of symbolic functions. */

/*
 *  This file was generated automatically from function.hppy.
 *  Please do not modify it directly, edit function.hppy instead!
 *
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

#ifndef GINAC_FUNCTION_H
#define GINAC_FUNCTION_H

#include "exprseq.h"

#include <string>
#include <vector>

#define DECLARE_FUNCTION_1P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 1; \
template< typename T1 > const GiNaC::function NAME( const T1 & p1 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1) ); \
}
#define DECLARE_FUNCTION_2P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 2; \
template< typename T1, typename T2 > const GiNaC::function NAME( const T1 & p1, const T2 & p2 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2) ); \
}
#define DECLARE_FUNCTION_3P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 3; \
template< typename T1, typename T2, typename T3 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3) ); \
}
#define DECLARE_FUNCTION_4P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 4; \
template< typename T1, typename T2, typename T3, typename T4 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4) ); \
}
#define DECLARE_FUNCTION_5P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 5; \
template< typename T1, typename T2, typename T3, typename T4, typename T5 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5) ); \
}
#define DECLARE_FUNCTION_6P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 6; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6) ); \
}
#define DECLARE_FUNCTION_7P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 7; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7) ); \
}
#define DECLARE_FUNCTION_8P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 8; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8) ); \
}
#define DECLARE_FUNCTION_9P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 9; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8, typename T9 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8, const T9 & p9 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8), GiNaC::ex(p9) ); \
}
#define DECLARE_FUNCTION_10P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 10; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8, typename T9, typename T10 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8, const T9 & p9, const T10 & p10 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8), GiNaC::ex(p9), GiNaC::ex(p10) ); \
}
#define DECLARE_FUNCTION_11P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 11; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8, typename T9, typename T10, typename T11 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8, const T9 & p9, const T10 & p10, const T11 & p11 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8), GiNaC::ex(p9), GiNaC::ex(p10), GiNaC::ex(p11) ); \
}
#define DECLARE_FUNCTION_12P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 12; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8, typename T9, typename T10, typename T11, typename T12 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8, const T9 & p9, const T10 & p10, const T11 & p11, const T12 & p12 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8), GiNaC::ex(p9), GiNaC::ex(p10), GiNaC::ex(p11), GiNaC::ex(p12) ); \
}
#define DECLARE_FUNCTION_13P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 13; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8, typename T9, typename T10, typename T11, typename T12, typename T13 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8, const T9 & p9, const T10 & p10, const T11 & p11, const T12 & p12, const T13 & p13 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8), GiNaC::ex(p9), GiNaC::ex(p10), GiNaC::ex(p11), GiNaC::ex(p12), GiNaC::ex(p13) ); \
}
#define DECLARE_FUNCTION_14P(NAME) \
class NAME##_SERIAL { public: static unsigned serial; }; \
const unsigned NAME##_NPARAMS = 14; \
template< typename T1, typename T2, typename T3, typename T4, typename T5, typename T6, typename T7, typename T8, typename T9, typename T10, typename T11, typename T12, typename T13, typename T14 > const GiNaC::function NAME( const T1 & p1, const T2 & p2, const T3 & p3, const T4 & p4, const T5 & p5, const T6 & p6, const T7 & p7, const T8 & p8, const T9 & p9, const T10 & p10, const T11 & p11, const T12 & p12, const T13 & p13, const T14 & p14 ) { \
	return GiNaC::function(NAME##_SERIAL::serial, GiNaC::ex(p1), GiNaC::ex(p2), GiNaC::ex(p3), GiNaC::ex(p4), GiNaC::ex(p5), GiNaC::ex(p6), GiNaC::ex(p7), GiNaC::ex(p8), GiNaC::ex(p9), GiNaC::ex(p10), GiNaC::ex(p11), GiNaC::ex(p12), GiNaC::ex(p13), GiNaC::ex(p14) ); \
}
// end of generated lines

#define REGISTER_FUNCTION(NAME,OPT) \
unsigned NAME##_SERIAL::serial = \
	GiNaC::function::register_new(GiNaC::function_options(#NAME, NAME##_NPARAMS).OPT);

namespace GiNaC {

class function;
class symmetry;

typedef ex (* eval_funcp)();
typedef ex (* evalf_funcp)();
typedef ex (* conjugate_funcp)();
typedef ex (* real_part_funcp)();
typedef ex (* imag_part_funcp)();
typedef ex (* expand_funcp)();
typedef ex (* derivative_funcp)();
typedef ex (* expl_derivative_funcp)();
typedef ex (* power_funcp)();
typedef ex (* series_funcp)();
typedef void (* print_funcp)();
typedef bool (* info_funcp)();

// the following lines have been generated for max. 14 parameters
typedef ex (* eval_funcp_1)( const ex & );
typedef ex (* evalf_funcp_1)( const ex & );
typedef ex (* conjugate_funcp_1)( const ex & );
typedef ex (* real_part_funcp_1)( const ex & );
typedef ex (* imag_part_funcp_1)( const ex & );
typedef ex (* expand_funcp_1)( const ex &, unsigned );
typedef ex (* derivative_funcp_1)( const ex &, unsigned );
typedef ex (* expl_derivative_funcp_1)( const ex &, const symbol & );
typedef ex (* power_funcp_1)( const ex &, const ex & );
typedef ex (* series_funcp_1)( const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_1)( const ex &, const print_context & );
typedef bool (* info_funcp_1)( const ex &, unsigned );
typedef ex (* eval_funcp_2)( const ex &, const ex & );
typedef ex (* evalf_funcp_2)( const ex &, const ex & );
typedef ex (* conjugate_funcp_2)( const ex &, const ex & );
typedef ex (* real_part_funcp_2)( const ex &, const ex & );
typedef ex (* imag_part_funcp_2)( const ex &, const ex & );
typedef ex (* expand_funcp_2)( const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_2)( const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_2)( const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_2)( const ex &, const ex &, const ex & );
typedef ex (* series_funcp_2)( const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_2)( const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_2)( const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_3)( const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_3)( const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_3)( const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_3)( const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_3)( const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_3)( const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_3)( const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_3)( const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_3)( const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_3)( const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_3)( const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_3)( const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_4)( const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_4)( const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_4)( const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_4)( const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_4)( const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_4)( const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_4)( const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_4)( const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_4)( const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_4)( const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_4)( const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_4)( const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_5)( const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_6)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_7)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_8)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_9)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_10)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_11)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_12)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_13)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* eval_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* evalf_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* conjugate_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* real_part_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* imag_part_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* expand_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* derivative_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
typedef ex (* expl_derivative_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const symbol & );
typedef ex (* power_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex & );
typedef ex (* series_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const relational &, int, unsigned );
typedef void (* print_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const print_context & );
typedef bool (* info_funcp_14)( const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, const ex &, unsigned );
// end of generated lines

// Alternatively, an exvector may be passed into the static function, instead
// of individual ex objects.  Then, the number of arguments is not limited.
typedef ex (* eval_funcp_exvector)(const exvector &);
typedef ex (* evalf_funcp_exvector)(const exvector &);
typedef ex (* conjugate_funcp_exvector)(const exvector &);
typedef ex (* real_part_funcp_exvector)(const exvector &);
typedef ex (* imag_part_funcp_exvector)(const exvector &);
typedef ex (* expand_funcp_exvector)(const exvector &, unsigned);
typedef ex (* derivative_funcp_exvector)(const exvector &, unsigned);
typedef ex (* expl_derivative_funcp_exvector)(const exvector &, const symbol &);
typedef ex (* power_funcp_exvector)(const exvector &, const ex &);
typedef ex (* series_funcp_exvector)(const exvector &, const relational &, int, unsigned);
typedef void (* print_funcp_exvector)(const exvector &, const print_context &);
typedef bool (* info_funcp_exvector)(const exvector &, unsigned);


class function_options
{
	friend class function;
	friend class fderivative;
public:
	function_options();
	function_options(std::string const & n, std::string const & tn=std::string());
	function_options(std::string const & n, unsigned np);
	~function_options();
	void initialize();

	function_options & dummy() { return *this; }
	function_options & set_name(std::string const & n, std::string const & tn=std::string());
	function_options & latex_name(std::string const & tn);
	// following lines have been generated for max. 14 parameters
	function_options & eval_func(eval_funcp_1 e);
	function_options & eval_func(eval_funcp_2 e);
	function_options & eval_func(eval_funcp_3 e);
	function_options & eval_func(eval_funcp_4 e);
	function_options & eval_func(eval_funcp_5 e);
	function_options & eval_func(eval_funcp_6 e);
	function_options & eval_func(eval_funcp_7 e);
	function_options & eval_func(eval_funcp_8 e);
	function_options & eval_func(eval_funcp_9 e);
	function_options & eval_func(eval_funcp_10 e);
	function_options & eval_func(eval_funcp_11 e);
	function_options & eval_func(eval_funcp_12 e);
	function_options & eval_func(eval_funcp_13 e);
	function_options & eval_func(eval_funcp_14 e);
	function_options & evalf_func(evalf_funcp_1 e);
	function_options & evalf_func(evalf_funcp_2 e);
	function_options & evalf_func(evalf_funcp_3 e);
	function_options & evalf_func(evalf_funcp_4 e);
	function_options & evalf_func(evalf_funcp_5 e);
	function_options & evalf_func(evalf_funcp_6 e);
	function_options & evalf_func(evalf_funcp_7 e);
	function_options & evalf_func(evalf_funcp_8 e);
	function_options & evalf_func(evalf_funcp_9 e);
	function_options & evalf_func(evalf_funcp_10 e);
	function_options & evalf_func(evalf_funcp_11 e);
	function_options & evalf_func(evalf_funcp_12 e);
	function_options & evalf_func(evalf_funcp_13 e);
	function_options & evalf_func(evalf_funcp_14 e);
	function_options & conjugate_func(conjugate_funcp_1 e);
	function_options & conjugate_func(conjugate_funcp_2 e);
	function_options & conjugate_func(conjugate_funcp_3 e);
	function_options & conjugate_func(conjugate_funcp_4 e);
	function_options & conjugate_func(conjugate_funcp_5 e);
	function_options & conjugate_func(conjugate_funcp_6 e);
	function_options & conjugate_func(conjugate_funcp_7 e);
	function_options & conjugate_func(conjugate_funcp_8 e);
	function_options & conjugate_func(conjugate_funcp_9 e);
	function_options & conjugate_func(conjugate_funcp_10 e);
	function_options & conjugate_func(conjugate_funcp_11 e);
	function_options & conjugate_func(conjugate_funcp_12 e);
	function_options & conjugate_func(conjugate_funcp_13 e);
	function_options & conjugate_func(conjugate_funcp_14 e);
	function_options & real_part_func(real_part_funcp_1 e);
	function_options & real_part_func(real_part_funcp_2 e);
	function_options & real_part_func(real_part_funcp_3 e);
	function_options & real_part_func(real_part_funcp_4 e);
	function_options & real_part_func(real_part_funcp_5 e);
	function_options & real_part_func(real_part_funcp_6 e);
	function_options & real_part_func(real_part_funcp_7 e);
	function_options & real_part_func(real_part_funcp_8 e);
	function_options & real_part_func(real_part_funcp_9 e);
	function_options & real_part_func(real_part_funcp_10 e);
	function_options & real_part_func(real_part_funcp_11 e);
	function_options & real_part_func(real_part_funcp_12 e);
	function_options & real_part_func(real_part_funcp_13 e);
	function_options & real_part_func(real_part_funcp_14 e);
	function_options & imag_part_func(imag_part_funcp_1 e);
	function_options & imag_part_func(imag_part_funcp_2 e);
	function_options & imag_part_func(imag_part_funcp_3 e);
	function_options & imag_part_func(imag_part_funcp_4 e);
	function_options & imag_part_func(imag_part_funcp_5 e);
	function_options & imag_part_func(imag_part_funcp_6 e);
	function_options & imag_part_func(imag_part_funcp_7 e);
	function_options & imag_part_func(imag_part_funcp_8 e);
	function_options & imag_part_func(imag_part_funcp_9 e);
	function_options & imag_part_func(imag_part_funcp_10 e);
	function_options & imag_part_func(imag_part_funcp_11 e);
	function_options & imag_part_func(imag_part_funcp_12 e);
	function_options & imag_part_func(imag_part_funcp_13 e);
	function_options & imag_part_func(imag_part_funcp_14 e);
	function_options & expand_func(expand_funcp_1 e);
	function_options & expand_func(expand_funcp_2 e);
	function_options & expand_func(expand_funcp_3 e);
	function_options & expand_func(expand_funcp_4 e);
	function_options & expand_func(expand_funcp_5 e);
	function_options & expand_func(expand_funcp_6 e);
	function_options & expand_func(expand_funcp_7 e);
	function_options & expand_func(expand_funcp_8 e);
	function_options & expand_func(expand_funcp_9 e);
	function_options & expand_func(expand_funcp_10 e);
	function_options & expand_func(expand_funcp_11 e);
	function_options & expand_func(expand_funcp_12 e);
	function_options & expand_func(expand_funcp_13 e);
	function_options & expand_func(expand_funcp_14 e);
	function_options & derivative_func(derivative_funcp_1 e);
	function_options & derivative_func(derivative_funcp_2 e);
	function_options & derivative_func(derivative_funcp_3 e);
	function_options & derivative_func(derivative_funcp_4 e);
	function_options & derivative_func(derivative_funcp_5 e);
	function_options & derivative_func(derivative_funcp_6 e);
	function_options & derivative_func(derivative_funcp_7 e);
	function_options & derivative_func(derivative_funcp_8 e);
	function_options & derivative_func(derivative_funcp_9 e);
	function_options & derivative_func(derivative_funcp_10 e);
	function_options & derivative_func(derivative_funcp_11 e);
	function_options & derivative_func(derivative_funcp_12 e);
	function_options & derivative_func(derivative_funcp_13 e);
	function_options & derivative_func(derivative_funcp_14 e);
	function_options & expl_derivative_func(expl_derivative_funcp_1 e);
	function_options & expl_derivative_func(expl_derivative_funcp_2 e);
	function_options & expl_derivative_func(expl_derivative_funcp_3 e);
	function_options & expl_derivative_func(expl_derivative_funcp_4 e);
	function_options & expl_derivative_func(expl_derivative_funcp_5 e);
	function_options & expl_derivative_func(expl_derivative_funcp_6 e);
	function_options & expl_derivative_func(expl_derivative_funcp_7 e);
	function_options & expl_derivative_func(expl_derivative_funcp_8 e);
	function_options & expl_derivative_func(expl_derivative_funcp_9 e);
	function_options & expl_derivative_func(expl_derivative_funcp_10 e);
	function_options & expl_derivative_func(expl_derivative_funcp_11 e);
	function_options & expl_derivative_func(expl_derivative_funcp_12 e);
	function_options & expl_derivative_func(expl_derivative_funcp_13 e);
	function_options & expl_derivative_func(expl_derivative_funcp_14 e);
	function_options & power_func(power_funcp_1 e);
	function_options & power_func(power_funcp_2 e);
	function_options & power_func(power_funcp_3 e);
	function_options & power_func(power_funcp_4 e);
	function_options & power_func(power_funcp_5 e);
	function_options & power_func(power_funcp_6 e);
	function_options & power_func(power_funcp_7 e);
	function_options & power_func(power_funcp_8 e);
	function_options & power_func(power_funcp_9 e);
	function_options & power_func(power_funcp_10 e);
	function_options & power_func(power_funcp_11 e);
	function_options & power_func(power_funcp_12 e);
	function_options & power_func(power_funcp_13 e);
	function_options & power_func(power_funcp_14 e);
	function_options & series_func(series_funcp_1 e);
	function_options & series_func(series_funcp_2 e);
	function_options & series_func(series_funcp_3 e);
	function_options & series_func(series_funcp_4 e);
	function_options & series_func(series_funcp_5 e);
	function_options & series_func(series_funcp_6 e);
	function_options & series_func(series_funcp_7 e);
	function_options & series_func(series_funcp_8 e);
	function_options & series_func(series_funcp_9 e);
	function_options & series_func(series_funcp_10 e);
	function_options & series_func(series_funcp_11 e);
	function_options & series_func(series_funcp_12 e);
	function_options & series_func(series_funcp_13 e);
	function_options & series_func(series_funcp_14 e);
	function_options & info_func(info_funcp_1 e);
	function_options & info_func(info_funcp_2 e);
	function_options & info_func(info_funcp_3 e);
	function_options & info_func(info_funcp_4 e);
	function_options & info_func(info_funcp_5 e);
	function_options & info_func(info_funcp_6 e);
	function_options & info_func(info_funcp_7 e);
	function_options & info_func(info_funcp_8 e);
	function_options & info_func(info_funcp_9 e);
	function_options & info_func(info_funcp_10 e);
	function_options & info_func(info_funcp_11 e);
	function_options & info_func(info_funcp_12 e);
	function_options & info_func(info_funcp_13 e);
	function_options & info_func(info_funcp_14 e);
	function_options & eval_func(eval_funcp_exvector e);
	function_options & evalf_func(evalf_funcp_exvector e);
	function_options & conjugate_func(conjugate_funcp_exvector e);
	function_options & real_part_func(real_part_funcp_exvector e);
	function_options & imag_part_func(imag_part_funcp_exvector e);
	function_options & expand_func(expand_funcp_exvector e);
	function_options & derivative_func(derivative_funcp_exvector e);
	function_options & expl_derivative_func(expl_derivative_funcp_exvector e);
	function_options & power_func(power_funcp_exvector e);
	function_options & series_func(series_funcp_exvector e);
	function_options & info_func(info_funcp_exvector e);
	template <class Ctx> function_options & print_func(print_funcp_1 p)
	{
		test_and_set_nparams(1);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_2 p)
	{
		test_and_set_nparams(2);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_3 p)
	{
		test_and_set_nparams(3);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_4 p)
	{
		test_and_set_nparams(4);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_5 p)
	{
		test_and_set_nparams(5);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_6 p)
	{
		test_and_set_nparams(6);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_7 p)
	{
		test_and_set_nparams(7);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_8 p)
	{
		test_and_set_nparams(8);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_9 p)
	{
		test_and_set_nparams(9);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_10 p)
	{
		test_and_set_nparams(10);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_11 p)
	{
		test_and_set_nparams(11);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_12 p)
	{
		test_and_set_nparams(12);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_13 p)
	{
		test_and_set_nparams(13);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	template <class Ctx> function_options & print_func(print_funcp_14 p)
	{
		test_and_set_nparams(14);
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}
	// end of generated lines

	template <class Ctx> function_options & print_func(print_funcp_exvector p)
	{
		print_use_exvector_args = true;
		set_print_func(Ctx::get_class_info_static().options.get_id(), print_funcp(p));
		return *this;
	}

	function_options & set_return_type(unsigned rt, const return_type_t* rtt = nullptr);
	function_options & do_not_evalf_params();
	function_options & remember(unsigned size, unsigned assoc_size=0,
	                            unsigned strategy=remember_strategies::delete_never);
	function_options & overloaded(unsigned o);
	function_options & set_symmetry(const symmetry & s);

	std::string get_name() const { return name; }
	unsigned get_nparams() const { return nparams; }

protected:
	bool has_derivative() const { return derivative_f != nullptr; }
	bool has_power() const { return power_f != nullptr; }
	void test_and_set_nparams(unsigned n);
	void set_print_func(unsigned id, print_funcp f);

	std::string name;
	std::string TeX_name;

	unsigned nparams;

	eval_funcp eval_f;
	evalf_funcp evalf_f;
	conjugate_funcp conjugate_f;
	real_part_funcp real_part_f;
	imag_part_funcp imag_part_f;
	expand_funcp expand_f;
	derivative_funcp derivative_f;
	expl_derivative_funcp expl_derivative_f;
	power_funcp power_f;
	series_funcp series_f;
	std::vector<print_funcp> print_dispatch_table;
	info_funcp info_f;

	bool evalf_params_first;

	bool use_return_type;
	unsigned return_type;
	return_type_t return_type_tinfo;

	bool use_remember;
	unsigned remember_size;
	unsigned remember_assoc_size;
	unsigned remember_strategy;

	bool eval_use_exvector_args;
	bool evalf_use_exvector_args;
	bool conjugate_use_exvector_args;
	bool real_part_use_exvector_args;
	bool imag_part_use_exvector_args;
	bool expand_use_exvector_args;
	bool derivative_use_exvector_args;
	bool expl_derivative_use_exvector_args;
	bool power_use_exvector_args;
	bool series_use_exvector_args;
	bool print_use_exvector_args;
	bool info_use_exvector_args;

	unsigned functions_with_same_name;

	ex symtree;
};


/** Exception class thrown by classes which provide their own series expansion
 *  to signal that ordinary Taylor expansion is safe. */
class do_taylor {};


/** The class function is used to implement builtin functions like sin, cos...
	and user defined functions */
class function : public exprseq
{
	GINAC_DECLARE_REGISTERED_CLASS(function, exprseq)

	friend class remember_table_entry;

// member functions

	// other constructors
public:
	function(unsigned ser);
	// the following lines have been generated for max. 14 parameters
	function(unsigned ser, const ex & param1);
	function(unsigned ser, const ex & param1, const ex & param2);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8, const ex & param9);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8, const ex & param9, const ex & param10);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8, const ex & param9, const ex & param10, const ex & param11);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8, const ex & param9, const ex & param10, const ex & param11, const ex & param12);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8, const ex & param9, const ex & param10, const ex & param11, const ex & param12, const ex & param13);
	function(unsigned ser, const ex & param1, const ex & param2, const ex & param3, const ex & param4, const ex & param5, const ex & param6, const ex & param7, const ex & param8, const ex & param9, const ex & param10, const ex & param11, const ex & param12, const ex & param13, const ex & param14);
	// end of generated lines
	function(unsigned ser, const exprseq & es);
	function(unsigned ser, const exvector & v);
	function(unsigned ser, exvector && v);
	
	// functions overriding virtual functions from base classes
public:
	void print(const print_context & c, unsigned level = 0) const override;
	unsigned precedence() const override {return 70;}
	ex expand(unsigned options=0) const override;
	ex eval() const override;
	ex evalf() const override;
	ex eval_ncmul(const exvector & v) const override;
	unsigned calchash() const override;
	ex series(const relational & r, int order, unsigned options = 0) const override;
	ex thiscontainer(const exvector & v) const override;
	ex thiscontainer(exvector && v) const override;
	ex conjugate() const override;
	ex real_part() const override;
	ex imag_part() const override;
	void archive(archive_node& n) const override;
	void read_archive(const archive_node& n, lst& syms) override;
	bool info(unsigned inf) const override;
protected:
	ex derivative(const symbol & s) const override;
	bool is_equal_same_type(const basic & other) const override;
	bool match_same_type(const basic & other) const override;
	unsigned return_type() const override;
	return_type_t return_type_tinfo() const override;
	
	// new virtual functions which can be overridden by derived classes
	// none
	
	// non-virtual functions in this class
protected:
	ex pderivative(unsigned diff_param) const; // partial differentiation
	ex expl_derivative(const symbol & s) const; // partial differentiation
	static std::vector<function_options> & registered_functions();
	bool lookup_remember_table(ex & result) const;
	void store_remember_table(ex const & result) const;
public:
	ex power(const ex & exp) const;
	static unsigned register_new(function_options const & opt);
	static unsigned current_serial;
	static unsigned find_function(const std::string &name, unsigned nparams);
	static std::vector<function_options> get_registered_functions() { return registered_functions(); };
	unsigned get_serial() const {return serial;}
	std::string get_name() const;

// member variables

protected:
	unsigned serial;
};
GINAC_DECLARE_UNARCHIVER(function);

// utility functions/macros

template <typename T>
inline bool is_the_function(const ex & x)
{
	return is_exactly_a<function>(x)
	    && ex_to<function>(x).get_serial() == T::serial;
}

// Check whether OBJ is the specified symbolic function.
#define is_ex_the_function(OBJ, FUNCNAME) (GiNaC::is_the_function<FUNCNAME##_SERIAL>(OBJ))

} // namespace GiNaC

#endif // ndef GINAC_FUNCTION_H

