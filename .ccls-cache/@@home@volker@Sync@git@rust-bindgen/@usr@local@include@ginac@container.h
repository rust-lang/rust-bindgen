/** @file container.h
 *
 *  Wrapper template for making GiNaC classes out of STL containers. */

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

#ifndef GINAC_CONTAINER_H
#define GINAC_CONTAINER_H

#include "ex.h"
#include "print.h"
#include "archive.h"
#include "assertion.h"
#include "compiler.h"

#include <algorithm>
#include <iterator>
#include <list>
#include <memory>
#include <stdexcept>
#include <vector>

namespace GiNaC {

/** Helper template for encapsulating the reserve() mechanics of STL containers. */
template <template <class T, class = std::allocator<T>> class C>
class container_storage {
protected:
	typedef C<ex> STLT;

	container_storage() {}
	container_storage(size_t n, const ex & e) : seq(n, e) {}
	container_storage(std::initializer_list<ex> il) : seq(il) {}

	template <class In>
	container_storage(In b, In e) : seq(b, e) {}

	void reserve(size_t) {}
	static void reserve(STLT &, size_t) {}

	STLT seq;

	// disallow destruction of container through a container_storage*
protected:
	~container_storage() {}
};

template <>
inline void container_storage<std::vector>::reserve(size_t n) { seq.reserve(n); }

template <>
inline void container_storage<std::vector>::reserve(std::vector<ex> & v, size_t n) { v.reserve(n); }


/** Helper template to allow initialization of containers via an overloaded
 *  comma operator (idea stolen from Blitz++). */
template <typename T, typename STLT>
class container_init {
public:
	container_init(STLT & s) : stlt(s) {}

	container_init<T, STLT> operator,(const T & x)
	{
		stlt.push_back(x);
		return container_init<T, STLT>(stlt);
	}

	// The following specializations produce much tighter code than the
	// general case above

	container_init<T, STLT> operator,(int x)
	{
		stlt.push_back(x);
		return container_init<T, STLT>(stlt);
	}

	container_init<T, STLT> operator,(unsigned int x)
	{
		stlt.push_back(x);
		return container_init<T, STLT>(stlt);
	}

	container_init<T, STLT> operator,(long x)
	{
		stlt.push_back(x);
		return container_init<T, STLT>(stlt);
	}

	container_init<T, STLT> operator,(unsigned long x)
	{
		stlt.push_back(x);
		return container_init<T, STLT>(stlt);
	}

	container_init<T, STLT> operator,(double x)
	{
		stlt.push_back(x);
		return container_init<T, STLT>(stlt);
	}

	container_init<T, STLT> operator,(const symbol & x)
	{
		stlt.push_back(T(x));
		return container_init<T, STLT>(stlt);
	}

private:
	container_init();
	STLT & stlt;
};

/** Wrapper template for making GiNaC classes out of STL containers. */
template <template <class T, class = std::allocator<T>> class C>
class container : public basic, public container_storage<C> {
	GINAC_DECLARE_REGISTERED_CLASS(container, basic)
protected:
	typedef typename container_storage<C>::STLT STLT;

public:
	typedef typename STLT::const_iterator const_iterator;
	typedef typename STLT::const_reverse_iterator const_reverse_iterator;

protected:
	// helpers
	static unsigned get_default_flags() { return 0; }
	static char get_open_delim() { return '('; }
	static char get_close_delim() { return ')'; }

	// constructors
public:
	container(STLT const & s)
	{
		setflag(get_default_flags());
		this->seq = s;
	}

	explicit container(STLT && v)
	{
		setflag(get_default_flags());
		this->seq.swap(v);
	}

	container(exvector::const_iterator b, exvector::const_iterator e)
	 : container_storage<C>(b, e)
	{
		setflag(get_default_flags());
	}

	container(std::initializer_list<ex> il)
	 : container_storage<C>(il)
	{
		setflag(get_default_flags());
	}

	explicit container(const ex & p1) attribute_deprecated;
	container(const ex & p1, const ex & p2) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10, const ex & p11) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10, const ex & p11, const ex & p12) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13, const ex & p14) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13, const ex & p14, const ex & p15) attribute_deprecated;
	container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
	          const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13, const ex & p14, const ex & p15, const ex & p16) attribute_deprecated;

	// First step of initialization of container with a comma-separated
	// sequence of expressions. Subsequent steps are handled by
	// container_init<>::operator,().
	container_init<ex, STLT> operator=(const ex & x) attribute_deprecated;

	// functions overriding virtual functions from base classes
public:
	bool info(unsigned inf) const override { return inherited::info(inf); }
	unsigned precedence() const override { return 10; }
	size_t nops() const override { return this->seq.size(); }
	ex op(size_t i) const override;
	ex & let_op(size_t i) override;
	ex subs(const exmap & m, unsigned options = 0) const override;

	void read_archive(const archive_node &n, lst &sym_lst) override
	{
		inherited::read_archive(n, sym_lst);
		setflag(get_default_flags());

		auto range =  n.find_property_range("seq", "seq");
		this->reserve(this->seq, range.end - range.begin);
		for (archive_node::archive_node_cit i=range.begin; i<range.end; ++i) {
			ex e;
			n.find_ex_by_loc(i, e, sym_lst);
			this->seq.emplace_back(e);
		}
	}

	/** Archive the object. */
	void archive(archive_node &n) const override
	{
		inherited::archive(n);
		for (auto & i : this->seq) {
			n.add_ex("seq", i);
		}
	}

protected:
	ex conjugate() const override
	{
		STLT *newcont = nullptr;
		for (const_iterator i=this->seq.begin(); i!=this->seq.end(); ++i) {
			if (newcont) {
				newcont->push_back(i->conjugate());
				continue;
			}
			ex x = i->conjugate();
			if (are_ex_trivially_equal(x, *i)) {
				continue;
			}
			newcont = new STLT;
			this->reserve(*newcont, this->seq.size());
			for (const_iterator j=this->seq.begin(); j!=i; ++j) {
				newcont->push_back(*j);
			}
			newcont->push_back(x);
		}
		if (newcont) {
			ex result = thiscontainer(*newcont);
			delete newcont;
			return result;
		}
		return *this;
	}

	ex real_part() const override
	{
		STLT cont;
		this->reserve(cont, nops());
		const_iterator b = begin();
		const_iterator e = end();
		for(const_iterator i=b; i!=e; ++i)
			cont.push_back(i->real_part());
		return thiscontainer(cont);
	}

	ex imag_part() const override
	{
		STLT cont;
		this->reserve(cont, nops());
		const_iterator b = begin();
		const_iterator e = end();
		for(const_iterator i=b; i!=e; ++i)
			cont.push_back(i->imag_part());
		return thiscontainer(cont);
	}

	bool is_equal_same_type(const basic & other) const override;

	// new virtual functions which can be overridden by derived classes
protected:
	/** Similar to duplicate(), but with a preset sequence. Must be
	 *  overridden by derived classes. */
	virtual ex thiscontainer(const STLT & v) const { return container(v); }

	/** Similar to duplicate(), but with a preset sequence (which gets
	 *  pilfered). Must be overridden by derived classes. */
	virtual ex thiscontainer(STLT && v) const { return container(std::move(v)); }

	virtual void printseq(const print_context & c, char openbracket, char delim,
	                      char closebracket, unsigned this_precedence,
	                      unsigned upper_precedence = 0) const;

	// non-virtual functions in this class
private:
	void sort_(std::random_access_iterator_tag)
	{
		std::sort(this->seq.begin(), this->seq.end(), ex_is_less());
	}

	void sort_(std::input_iterator_tag)
	{
		this->seq.sort(ex_is_less());
	}

	void unique_()
	{
		typename STLT::iterator p = std::unique(this->seq.begin(), this->seq.end(), ex_is_equal());
		this->seq.erase(p, this->seq.end());
	}

public:
	container & prepend(const ex & b);
	container & append(const ex & b);
	container & remove_first();
	container & remove_last();
	container & remove_all();
	container & sort();
	container & unique();

	const_iterator begin() const {return this->seq.begin();}
	const_iterator end() const {return this->seq.end();}
	const_reverse_iterator rbegin() const {return this->seq.rbegin();}
	const_reverse_iterator rend() const {return this->seq.rend();}

protected:
	void do_print(const print_context & c, unsigned level) const;
	void do_print_tree(const print_tree & c, unsigned level) const;
	void do_print_python(const print_python & c, unsigned level) const;
	void do_print_python_repr(const print_python_repr & c, unsigned level) const;
	STLT subschildren(const exmap & m, unsigned options = 0) const;
};

/** Default constructor */
template <template <class T, class = std::allocator<T>> class C>
container<C>::container()
{
	setflag(get_default_flags());
}

/** Deprecatd constructors (prefer initializer list) */
template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1)
  : container_storage<C>(1, p1) { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2)
  : container_storage<C>{p1, p2} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3)
  : container_storage<C>{p1, p2, p3} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4)
  : container_storage<C>{p1, p2, p3, p4} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5)
  : container_storage<C>{p1, p2, p3, p4, p5} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6)
  : container_storage<C>{p1, p2, p3, p4, p5, p6} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10, const ex & p11)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10, const ex & p11, const ex & p12)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13, const ex & p14)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13, const ex & p14, const ex & p15)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15} { setflag(get_default_flags()); }
template <template <class T, class = std::allocator<T>> class C>
container<C>::container(const ex & p1, const ex & p2, const ex & p3, const ex & p4, const ex & p5, const ex & p6, const ex & p7, const ex & p8,
                        const ex & p9, const ex & p10, const ex & p11, const ex & p12, const ex & p13, const ex & p14, const ex & p15, const ex & p16)
  : container_storage<C>{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16} { setflag(get_default_flags()); }

template <template <class T, class = std::allocator<T>> class C>
container_init<ex, typename container_storage<C>::STLT> container<C>::operator=(const ex & x)
{
	this->seq.push_back(x);
	return container_init<ex, typename container_storage<C>::STLT>(this->seq);
}

template <template <class T, class = std::allocator<T>> class C>
void container<C>::do_print(const print_context & c, unsigned level) const
{
	// always print brackets around seq, ignore upper_precedence
	printseq(c, get_open_delim(), ',', get_close_delim(), precedence(), precedence()+1);
}

template <template <class T, class = std::allocator<T>> class C>
void container<C>::do_print_tree(const print_tree & c, unsigned level) const
{
	c.s << std::string(level, ' ') << class_name() << " @" << this
	    << std::hex << ", hash=0x" << hashvalue << ", flags=0x" << flags << std::dec
	    << ", nops=" << nops()
	    << std::endl;
	const_iterator i = this->seq.begin(), end = this->seq.end();
	while (i != end) {
		i->print(c, level + c.delta_indent);
		++i;
	}
	c.s << std::string(level + c.delta_indent,' ') << "=====" << std::endl;
}

template <template <class T, class = std::allocator<T>> class C>
void container<C>::do_print_python(const print_python & c, unsigned level) const
{
	printseq(c, '[', ',', ']', precedence(), precedence()+1);
}

template <template <class T, class = std::allocator<T>> class C>
void container<C>::do_print_python_repr(const print_python_repr & c, unsigned level) const
{
	c.s << class_name();
	printseq(c, '(', ',', ')', precedence(), precedence()+1);
}

template <template <class T, class = std::allocator<T>> class C>
ex container<C>::op(size_t i) const
{
	GINAC_ASSERT(i < nops());

	const_iterator it = this->seq.begin();
	advance(it, i);
	return *it;
}

template <template <class T, class = std::allocator<T>> class C>
ex & container<C>::let_op(size_t i)
{
	GINAC_ASSERT(i < nops());

	ensure_if_modifiable();
	typename STLT::iterator it = this->seq.begin();
	advance(it, i);
	return *it;
}

template <template <class T, class = std::allocator<T>> class C>
ex container<C>::subs(const exmap & m, unsigned options) const
{
	// After having subs'ed all children, this method subs'es one final
	// level, but only if the intermediate result is a container! This is
	// because if the intermediate result has eval'ed to a non-container a
	// last level substitution would be wrong, as this example involving a
	// function f and its inverse f^-1 shows:
	// f(x).subs(x==f^-1(x))
	//   -> f(f^-1(x))  [subschildren]
	//   -> x           [eval]   /* must not subs(x==f^-1(x))! */
	STLT subsed = subschildren(m, options);
	if (!subsed.empty()) {
		ex result(thiscontainer(subsed));
		if (is_a<container<C>>(result))
			return ex_to<basic>(result).subs_one_level(m, options);
		else
			return result;
	} else {
		if (is_a<container<C>>(*this))
			return subs_one_level(m, options);
		else
			return *this;
	}
}

/** Compare two containers of the same type. */
template <template <class T, class = std::allocator<T>> class C>
int container<C>::compare_same_type(const basic & other) const
{
	GINAC_ASSERT(is_a<container>(other));
	const container & o = static_cast<const container &>(other);

	const_iterator it1 = this->seq.begin(), it1end = this->seq.end(),
	               it2 = o.seq.begin(), it2end = o.seq.end();

	while (it1 != it1end && it2 != it2end) {
		int cmpval = it1->compare(*it2);
		if (cmpval)
			return cmpval;
		++it1; ++it2;
	}

	return (it1 == it1end) ? (it2 == it2end ? 0 : -1) : 1;
}

template <template <class T, class = std::allocator<T>> class C>
bool container<C>::is_equal_same_type(const basic & other) const
{
	GINAC_ASSERT(is_a<container>(other));
	const container & o = static_cast<const container &>(other);

	if (this->seq.size() != o.seq.size())
		return false;

	const_iterator it1 = this->seq.begin(), it1end = this->seq.end(), it2 = o.seq.begin();
	while (it1 != it1end) {
		if (!it1->is_equal(*it2))
			return false;
		++it1; ++it2;
	}

	return true;
}

/** Add element at front. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::prepend(const ex & b)
{
	ensure_if_modifiable();
	this->seq.push_front(b);
	return *this;
}

/** Add element at back. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::append(const ex & b)
{
	ensure_if_modifiable();
	this->seq.push_back(b);
	return *this;
}

/** Remove first element. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::remove_first()
{
	ensure_if_modifiable();
	this->seq.pop_front();
	return *this;
}

/** Remove last element. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::remove_last()
{
	ensure_if_modifiable();
	this->seq.pop_back();
	return *this;
}

/** Remove all elements. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::remove_all()
{
	ensure_if_modifiable();
	this->seq.clear();
	return *this;
}

/** Sort elements. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::sort()
{
	ensure_if_modifiable();
	sort_(typename std::iterator_traits<typename STLT::iterator>::iterator_category());
	return *this;
}

/** Specialization of container::unique_() for std::list. */
template<> inline void container<std::list>::unique_()
{
	this->seq.unique(ex_is_equal());
}

/** Remove adjacent duplicate elements. */
template <template <class T, class = std::allocator<T>> class C>
container<C> & container<C>::unique()
{
	ensure_if_modifiable();
	unique_();
	return *this;
}

/** Print sequence of contained elements. */
template <template <class T, class = std::allocator<T>> class C>
void container<C>::printseq(const print_context & c, char openbracket, char delim,
                            char closebracket, unsigned this_precedence,
                            unsigned upper_precedence) const
{
	if (this_precedence <= upper_precedence)
		c.s << openbracket;

	if (!this->seq.empty()) {
		const_iterator it = this->seq.begin(), itend = this->seq.end();
		--itend;
		while (it != itend) {
			it->print(c, this_precedence);
			c.s << delim;
			++it;
		}
		it->print(c, this_precedence);
	}

	if (this_precedence <= upper_precedence)
		c.s << closebracket;
}

template <template <class T, class = std::allocator<T>> class C>
typename container<C>::STLT container<C>::subschildren(const exmap & m, unsigned options) const
{
	// returns an empty container if nothing had to be substituted
	// returns a STLT with substituted elements otherwise

	const_iterator cit = this->seq.begin(), end = this->seq.end();
	while (cit != end) {
		const ex & subsed_ex = cit->subs(m, options);
		if (!are_ex_trivially_equal(*cit, subsed_ex)) {

			// copy first part of seq which hasn't changed
			STLT s(this->seq.begin(), cit);
			this->reserve(s, this->seq.size());

			// insert changed element
			s.push_back(subsed_ex);
			++cit;

			// copy rest
			while (cit != end) {
				s.push_back(cit->subs(m, options));
				++cit;
			}

			return s;
		}

		++cit;
	}
	
	return STLT(); // nothing has changed
}

} // namespace GiNaC

#endif // ndef GINAC_CONTAINER_H
