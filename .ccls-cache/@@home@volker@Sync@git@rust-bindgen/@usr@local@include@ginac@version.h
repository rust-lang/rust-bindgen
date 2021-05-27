/** @file version.h
 *
 *  GiNaC library version information. */

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

#ifndef GINAC_VERSION_H
#define GINAC_VERSION_H

/* Major version of GiNaC */
#define GINACLIB_MAJOR_VERSION 1

/* Minor version of GiNaC */
#define GINACLIB_MINOR_VERSION 7

/* Micro version of GiNaC */
#define GINACLIB_MICRO_VERSION 10

// GiNaC library version information. It has very little to do with GiNaC
// version number. In particular, library version is OS dependent. 
//
// When making releases, do
// 1. Increment GINAC_LT_REVISION
// 2. If any interfaces have been added, removed, or changed since the last
//    release, increment GINAC_LT_CURRENT and set GINAC_LT_REVISION to 0.
// 3. If any interfaces have been added since the last release, increment
//    GINAC_LT_AGE.
// 4. If any interfaces have been removed since the last release, set 
//    GINAC_LT_AGE to 0.
//
// Please note: the libtool naming scheme cannot guarantee that on all
// systems, the numbering is consecutive. It only guarantees that it is
// increasing. This doesn't matter, though: there is not incurred cost
// for numbers that are omitted, except for shrinking the available space
// of leftover numbers. Not something we need to worry about yet. ;-)
// TODO, when setting GINAC_LT_REVISION to 0:
//  * change matrix inverse to use default argument (twice)
//  * remove interfaces marked as deprecated
#define GINAC_LT_CURRENT  10
#define GINAC_LT_REVISION 4
#define GINAC_LT_AGE      4

/*
 * GiNaC archive file version information.
 *
 * The current archive version is GINACLIB_ARCHIVE_VERSION. This is
 * the version of archives created by the current version of GiNaC.
 * Archives version (GINACLIB_ARCHIVE_VERSION - GINACLIB_ARCHIVE_AGE)
 * thru * GINACLIB_ARCHIVE_VERSION can be read by current version
 * of GiNaC.
 *
 * Backward compatibility notes:
 * If new properties have been added:
 *	GINACLIB_ARCHIVE_VERSION += 1
 *	GINACLIB_ARCHIVE_AGE += 1
 * If backwards compatibility has been broken, i.e. some properties
 * has been removed, or their type and/or meaning changed:
 *	GINACLIB_ARCHIVE_VERSION += 1
 *	GINACLIB_ARCHIVE_AGE = 0
 */
#define GINACLIB_ARCHIVE_VERSION 3
#define GINACLIB_ARCHIVE_AGE 3

#define GINACLIB_STR_HELPER(x) #x
#define GINACLIB_STR(x) GINACLIB_STR_HELPER(x)
#define GINACLIB_VERSION \
	GINACLIB_STR(GINACLIB_MAJOR_VERSION) "." \
	GINACLIB_STR(GINACLIB_MINOR_VERSION) "." \
	GINACLIB_STR(GINACLIB_MICRO_VERSION)

namespace GiNaC {

extern const int version_major;
extern const int version_minor;
extern const int version_micro;

} // namespace GiNaC

#endif // ndef GINAC_VERSION_H
