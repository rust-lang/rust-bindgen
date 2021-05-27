/** @file archive.h
 *
 *  Archiving of GiNaC expressions. */

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

#ifndef GINAC_ARCHIVE_H
#define GINAC_ARCHIVE_H

#include "ex.h"

#include <iosfwd>
#include <map>
#include <string>
#include <vector>

namespace GiNaC {

class archive;


/** Numerical ID value to refer to an archive_node. */
typedef unsigned archive_node_id;

/** Numerical ID value to refer to a string. */
typedef unsigned archive_atom;


/** This class stores all properties needed to record/retrieve the state
 *  of one object of class basic (or a derived class). Each property is
 *  addressed by its name and data type. */
class archive_node
{
	friend std::ostream &operator<<(std::ostream &os, const archive_node &ar);
	friend std::istream &operator>>(std::istream &is, archive_node &ar);

public:
	/** Property data types */
	enum property_type {
		PTYPE_BOOL,
		PTYPE_UNSIGNED,
		PTYPE_STRING,
		PTYPE_NODE
	};

	/** Information about a stored property. A vector of these structures
	 *  is returned by get_properties().
	 *  @see get_properties */
	struct property_info {
		property_info() {}
		property_info(property_type t, const std::string &n, unsigned c = 1) : type(t), name(n), count(c) {}

		property_type type; /**< Data type of property. */
		std::string name;   /**< Name of property. */
		unsigned count;     /**< Number of occurrences. */
	};
	typedef std::vector<property_info> propinfovector;

	/** Archived property (data type, name and associated data) */
	struct property {
		property() {}
		property(archive_atom n, property_type t, unsigned v) : type(t), name(n), value(v) {}

		property_type type; /**< Data type of property. */
		archive_atom name;  /**< Name of property. */
		unsigned value;     /**< Stored value. */
	};
	typedef std::vector<property>::const_iterator archive_node_cit;
	struct archive_node_cit_range {
		archive_node_cit begin, end;
	};

	archive_node(archive &ar) : a(ar), has_expression(false) {}
	archive_node(archive &ar, const ex &expr);

	const archive_node &operator=(const archive_node &other);

	/** Add property of type "bool" to node. */
	void add_bool(const std::string &name, bool value);

	/** Add property of type "unsigned int" to node. */
	void add_unsigned(const std::string &name, unsigned value);

	/** Add property of type "string" to node. */
	void add_string(const std::string &name, const std::string &value);

	/** Add property of type "ex" to node. */
	void add_ex(const std::string &name, const ex &value);

	/** Retrieve property of type "bool" from node.
	 *  @return "true" if property was found, "false" otherwise */
	bool find_bool(const std::string &name, bool &ret, unsigned index = 0) const;

	/** Retrieve property of type "unsigned" from node.
	 *  @return "true" if property was found, "false" otherwise */
	bool find_unsigned(const std::string &name, unsigned &ret, unsigned index = 0) const;

	/** Retrieve property of type "string" from node.
	 *  @return "true" if property was found, "false" otherwise */
	bool find_string(const std::string &name, std::string &ret, unsigned index = 0) const;

	/** Find the location in the vector of properties of the first/last
	 *  property with a given name. */
	archive_node_cit find_first(const std::string &name) const;
	archive_node_cit find_last(const std::string &name) const;

	/** Find a range of locations in the vector of properties. The result
	 *  begins at the first property with name1 and ends one past the last
	 *  property with name2. */
	archive_node_cit_range find_property_range(const std::string &name1, const std::string &name2) const;

	/** Retrieve property of type "ex" from node.
	 *  @return "true" if property was found, "false" otherwise */
	bool find_ex(const std::string &name, ex &ret, lst &sym_lst, unsigned index = 0) const;

	/** Retrieve property of type "ex" from the node if it is known
	 *  that this node in fact contains such a property at the given
	 *  location. This is much more efficient than the preceding function. */
	void find_ex_by_loc(archive_node_cit loc, ex &ret, lst &sym_lst) const;

	/** Retrieve property of type "ex" from node, returning the node of
	 *  the sub-expression. */
	const archive_node &find_ex_node(const std::string &name, unsigned index = 0) const;

	/** Return vector of properties stored in node. */
	void get_properties(propinfovector &v) const;

	ex unarchive(lst &sym_lst) const;
	bool has_same_ex_as(const archive_node &other) const;
	bool has_ex() const {return has_expression;}
	ex get_ex() const {return e;}

	void forget();
	void printraw(std::ostream &os) const;

private:
	/** Reference to the archive to which this node belongs. */
	archive &a;

	/** Vector of stored properties. */
	std::vector<property> props;

	/** Flag indicating whether a cached unarchived representation of this node exists. */
	mutable bool has_expression;

	/** The cached unarchived representation of this node (if any). */
	mutable ex e;
};

typedef basic* (*synthesize_func)();
typedef std::map<std::string, synthesize_func> unarchive_map_t;

class unarchive_table_t
{
	static int usecount;
	static unarchive_map_t* unarch_map;
public:
	unarchive_table_t();
	~unarchive_table_t();
	synthesize_func find(const std::string& classname) const;
	void insert(const std::string& classname, synthesize_func f);
};
static unarchive_table_t unarch_table_instance;

/** Helper macros to register a class with (un)archiving (a.k.a.
 * (de)serialization).
 *
 * Usage: put 
 *
 * GINAC_DECLARE_UNARCHIVER(myclass);
 *
 * into the header file (in the global or namespace scope), and
 *
 * GINAC_BIND_UNARCHIVER(myclass);
 *
 * into the source file.
 *
 * Effect: the `myclass' (being a class derived directly or indirectly
 * from GiNaC::basic) can be archived and unarchived.
 *
 * Note: you need to use GINAC_{DECLARE,BIND}_UNARCHIVER incantations
 * in order to make your class (un)archivable _even if your class does
 * not overload `read_archive' method_. Sorry for inconvenience.
 *
 * How it works: 
 *
 * The `basic' class has a `read_archive' virtual method which reads an
 * expression from archive. Derived classes can overload that method.
 * There's a small problem, though. On unarchiving all we have is a set
 * of named byte streams. In C++ the class name (as written in the source
 * code) has nothing to do with its actual type. Thus, we need establish
 * a correspondence ourselves. To do so we maintain a `class_name' =>
 * `function_pointer' table (see the unarchive_table_t class above).
 * Every function in this table is supposed to create a new object of
 * the `class_name' type. The `archive_node' class uses that table to
 * construct an object of correct type. Next it invokes read_archive
 * virtual method of newly created object, which does the actual job.
 *
 * Note: this approach is very simple-minded (it does not handle classes
 * with same names from different namespaces, multiple inheritance, etc),
 * but it happens to work surprisingly well.
 */
#define GINAC_DECLARE_UNARCHIVER(classname)			\
class classname ## _unarchiver					\
{								\
	static int usecount;					\
public:								\
	static GiNaC::basic* create();				\
	classname ## _unarchiver();				\
	~ classname ## _unarchiver();				\
};								\
static classname ## _unarchiver classname ## _unarchiver_instance

#define GINAC_BIND_UNARCHIVER(classname)			\
classname ## _unarchiver::classname ## _unarchiver()		\
{								\
	static GiNaC::unarchive_table_t table;			\
	if (usecount++ == 0) {					\
		table.insert(std::string(#classname),		\
			&(classname ## _unarchiver::create));	\
	}							\
}								\
GiNaC::basic* classname ## _unarchiver::create()		\
{								\
	return new classname();					\
}								\
classname ## _unarchiver::~ classname ## _unarchiver() { }	\
int classname ## _unarchiver::usecount = 0


/** This class holds archived versions of GiNaC expressions (class ex).
 *  An archive can be constructed from an expression and then written to
 *  a stream; or it can be read from a stream and then unarchived, yielding
 *  back the expression. Archives can hold multiple expressions which can
 *  be referred to by name or index number. The main component of the
 *  archive class is a vector of archive_nodes which each store one object
 *  of class basic (or a derived class). */
class archive
{
	friend std::ostream &operator<<(std::ostream &os, const archive &ar);
	friend std::istream &operator>>(std::istream &is, archive &ar);

public:
	archive() {}
	~archive() {}

	/** Construct archive from expression using the default name "ex". */
	archive(const ex &e) {archive_ex(e, "ex");}

	/** Construct archive from expression using the specified name. */
	archive(const ex &e, const char *n) {archive_ex(e, n);}

	/** Archive an expression.
	 *  @param e the expression to be archived
	 *  @param name name under which the expression is stored */
	void archive_ex(const ex &e, const char *name);

	/** Retrieve expression from archive by name.
	 *  @param sym_lst list of pre-defined symbols
	 *  @param name name of expression */
	ex unarchive_ex(const lst &sym_lst, const char *name) const;

	/** Retrieve expression from archive by index.
	 *  @param sym_lst list of pre-defined symbols
	 *  @param index index of expression
	 *  @see count_expressions */
	ex unarchive_ex(const lst &sym_lst, unsigned index = 0) const;

	/** Retrieve expression and its name from archive by index.
	 *  @param sym_lst list of pre-defined symbols
	 *  @param name receives the name of the expression
	 *  @param index index of expression
	 *  @see count_expressions */
	ex unarchive_ex(const lst &sym_lst, std::string &name, unsigned index = 0) const;

	/** Return number of archived expressions. */
	unsigned num_expressions() const;

	/** Return reference to top node of an expression specified by index. */
	const archive_node &get_top_node(unsigned index = 0) const;

	/** Clear all archived expressions. */
	void clear();

	archive_node_id add_node(const archive_node &n);
	archive_node &get_node(archive_node_id id);

	void forget();
	void printraw(std::ostream &os) const;

private:
	/** Vector of archived nodes. */
	std::vector<archive_node> nodes;

	/** Archived expression descriptor. */
	struct archived_ex {
		archived_ex() {}
		archived_ex(archive_atom n, archive_node_id node) : name(n), root(node) {}

		archive_atom name;     /**< Name of expression. */
		archive_node_id root;  /**< ID of root node. */
	};

	/** Vector of archived expression descriptors. */
	std::vector<archived_ex> exprs;

public:
	archive_atom atomize(const std::string &s) const;
	const std::string &unatomize(archive_atom id) const;

private:
	/** Vector of atomized strings (using a vector allows faster unarchiving). */
	mutable std::vector<std::string> atoms;
	/** The map of from strings to indices of the atoms vectors allows for
	 *  faster archiving.
	 */
	typedef std::map<std::string, archive_atom>::const_iterator inv_at_cit;
	mutable std::map<std::string, archive_atom> inverse_atoms;

	/** Map of stored expressions to nodes for faster archiving */
	mutable std::map<ex, archive_node_id, ex_is_less> exprtable;
};


std::ostream &operator<<(std::ostream &os, const archive &ar);
std::istream &operator>>(std::istream &is, archive &ar);

} // namespace GiNaC

#endif // ndef GINAC_ARCHIVE_H
