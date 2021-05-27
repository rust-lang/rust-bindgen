/** @file registrar.h
 *
 *  GiNaC's class registrar (for class basic and all classes derived from it). */

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

#ifndef GINAC_REGISTRAR_H
#define GINAC_REGISTRAR_H

#include "class_info.h"
#include "print.h"

#include <list>
#include <string>
#include <typeinfo>
#include <vector>

namespace GiNaC {

class ex;
class archive_node;

template <template <class T, class = std::allocator<T>> class> class container;
typedef container<std::list> lst;

/** To distinguish between different kinds of non-commutative objects */
struct return_type_t
{
	/// to distinguish between non-commutative objects of different type.
	std::type_info const* tinfo; 
	/// to distinguish between non-commutative objects of the same type.
	/// Think of gamma matrices with different representation labels.
	unsigned rl;

	/// Strict weak ordering (so one can put return_type_t's into
	/// a STL container).
	inline bool operator<(const return_type_t& other) const
	{
		if (tinfo->before(*other.tinfo))
			return true;
		return rl < other.rl;
	}
	inline bool operator==(const return_type_t& other) const
	{
		if (*tinfo != *(other.tinfo))
			return false;
		return rl == other.rl;
	}
	inline bool operator!=(const return_type_t& other) const
	{
		return ! (operator==(other));
	}
};

template<typename T> inline return_type_t make_return_type_t(const unsigned rl = 0)
{
	return_type_t ret;
	ret.rl = rl;
	ret.tinfo = &typeid(T);
	return ret;
}

/** This class stores information about a registered GiNaC class. */
class registered_class_options {
public:
	registered_class_options(const char *n, const char *p, 
		                 const std::type_info& ti)
	 : name(n), parent_name(p), tinfo_key(&ti) { }

	const char *get_name() const { return name; }
	const char *get_parent_name() const { return parent_name; }
	std::type_info const* get_id() const { return tinfo_key; }
	const std::vector<print_functor> &get_print_dispatch_table() const { return print_dispatch_table; }

	template <class Ctx, class T, class C>
	registered_class_options & print_func(void f(const T &, const C & c, unsigned))
	{
		set_print_func(Ctx::get_class_info_static().options.get_id(), f);
		return *this;
	}

	template <class Ctx, class T, class C>
	registered_class_options & print_func(void (T::*f)(const C &, unsigned))
	{
		set_print_func(Ctx::get_class_info_static().options.get_id(), f);
		return *this;
	}

	template <class Ctx>
	registered_class_options & print_func(const print_functor & f)
	{
		set_print_func(Ctx::get_class_info_static().options.get_id(), f);
		return *this;
	}

	void set_print_func(unsigned id, const print_functor & f)
	{
		if (id >= print_dispatch_table.size())
			print_dispatch_table.resize(id + 1);
		print_dispatch_table[id] = f;
	}

private:
	const char *name;         /**< Class name. */
	const char *parent_name;  /**< Name of superclass. */
	std::type_info const* tinfo_key;        /**< Type information key. */
	std::vector<print_functor> print_dispatch_table; /**< Method table for print() dispatch */
};

typedef class_info<registered_class_options> registered_class_info;

/** Common part of GINAC_DECLARE_REGISTERED_CLASS_NO_CTORS and GINAC_DECLARE_REGISTERED_CLASS. */
#define GINAC_DECLARE_REGISTERED_CLASS_COMMON(classname)	\
private: \
	static GiNaC::registered_class_info reg_info; \
public: \
	static GiNaC::registered_class_info &get_class_info_static() { return reg_info; } \
	class visitor { \
	public: \
		virtual void visit(const classname &) = 0; \
		virtual ~visitor() {}; \
	};

/** Primary macro for inclusion in the declaration of each registered class. */
#define GINAC_DECLARE_REGISTERED_CLASS_NO_CTORS(classname, supername) \
	GINAC_DECLARE_REGISTERED_CLASS_COMMON(classname) \
	typedef supername inherited; \
	virtual const GiNaC::registered_class_info &get_class_info() const { return classname::get_class_info_static(); } \
	virtual GiNaC::registered_class_info &get_class_info() { return classname::get_class_info_static(); } \
	virtual const char *class_name() const { return classname::get_class_info_static().options.get_name(); } \
private:

/** Macro for inclusion in the declaration of each registered class.
 *  It declares some functions that are common to all classes derived
 *  from 'basic' as well as all required stuff for the GiNaC class
 *  registry (mainly needed for archiving). */
#define GINAC_DECLARE_REGISTERED_CLASS(classname, supername) \
	GINAC_DECLARE_REGISTERED_CLASS_COMMON(classname) \
	template<class B, typename... Args> friend B & dynallocate(Args &&... args); \
	typedef supername inherited; \
	classname(); \
	classname * duplicate() const override { \
		classname * bp = new classname(*this); \
		bp->setflag(GiNaC::status_flags::dynallocated);	\
		return bp; \
	} \
	\
	void accept(GiNaC::visitor & v) const override \
	{ \
		if (visitor *p = dynamic_cast<visitor *>(&v)) \
			p->visit(*this); \
		else \
			inherited::accept(v); \
	} \
	const GiNaC::registered_class_info &get_class_info() const override { return classname::get_class_info_static(); } \
	GiNaC::registered_class_info &get_class_info() override { return classname::get_class_info_static(); } \
	const char *class_name() const override { return classname::get_class_info_static().options.get_name(); } \
protected: \
	int compare_same_type(const GiNaC::basic & other) const override; \
private:


/** Macro for inclusion in the implementation of each registered class. */
#define GINAC_IMPLEMENT_REGISTERED_CLASS(classname, supername) \
	GiNaC::registered_class_info classname::reg_info = GiNaC::registered_class_info(GiNaC::registered_class_options(#classname, #supername, typeid(classname))); 

/** Macro for inclusion in the implementation of each registered class.
 *  Additional options can be specified. */
#define GINAC_IMPLEMENT_REGISTERED_CLASS_OPT(classname, supername, options) \
	GiNaC::registered_class_info classname::reg_info = GiNaC::registered_class_info(GiNaC::registered_class_options(#classname, #supername, typeid(classname)).options);

/** Macro for inclusion in the implementation of each registered class.
 *  Additional options can be specified. */
#define GINAC_IMPLEMENT_REGISTERED_CLASS_OPT_T(classname, supername, options) \
	GiNaC::registered_class_info classname::reg_info = GiNaC::registered_class_info(GiNaC::registered_class_options(#classname, #supername, typeid(classname)).options);


/** Add or replace a print method. */
template <class Alg, class Ctx, class T, class C>
extern void set_print_func(void f(const T &, const C & c, unsigned))
{
	Alg::get_class_info_static().options.set_print_func(Ctx::get_class_info_static().options.get_id(), f);
}

/** Add or replace a print method. */
template <class Alg, class Ctx, class T, class C>
extern void set_print_func(void (T::*f)(const C &, unsigned))
{
	Alg::get_class_info_static().options.set_print_func(Ctx::get_class_info_static().options.get_id(), f);
}


} // namespace GiNaC

#endif // ndef GINAC_REGISTRAR_H
