/** @file fail.h
 *
 *  Interface to class signaling failure of operation. Considered obsolete
 *  somewhat obsolete (most of this can be replaced by exceptions). */

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

#ifndef GINAC_FAIL_H
#define GINAC_FAIL_H

#include "basic.h"
#include "archive.h"

namespace GiNaC {

class fail : public basic
{
	GINAC_DECLARE_REGISTERED_CLASS(fail, basic)
	
	// functions overriding virtual functions from base classes
protected:
	unsigned return_type() const override { return return_types::noncommutative_composite; };

	// non-virtual functions in this class
protected:
	void do_print(const print_context & c, unsigned level) const;
};
GINAC_DECLARE_UNARCHIVER(fail);

} // namespace GiNaC

#endif // ndef GINAC_FAIL_H
