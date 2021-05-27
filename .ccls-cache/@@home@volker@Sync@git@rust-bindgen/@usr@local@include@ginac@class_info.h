/** @file class_info.h
 *
 *  Helper templates to provide per-class information for class hierarchies. */

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

#ifndef GINAC_CLASS_INFO_H
#define GINAC_CLASS_INFO_H

#include <cstddef> // for size_t
#include <cstring>
#include <iomanip>
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

namespace GiNaC {

// OPT is the class that stores the actual per-class data. It must provide
// get_name(), get_parent_name() and get_id() members.

template <class OPT>
class class_info {
public:
	class_info(const OPT & o) : options(o), next(first), parent(nullptr)
	{
		first = this;
		parents_identified = false;
	}

	/** Get pointer to class_info of parent class (or nullptr). */
	class_info *get_parent() const
	{
		identify_parents();
		return parent;
	}

	/** Find class_info by name. */
	static const class_info *find(const std::string &class_name);

	/** Dump class hierarchy to std::cout. */
	static void dump_hierarchy(bool verbose = false);

	OPT options;

private:
	struct tree_node {
		tree_node(class_info *i) : info(i) {}
		void add_child(tree_node *n) { children.push_back(n); }

		std::vector<tree_node *> children;
		class_info *info;
	};

	static void dump_tree(tree_node *n, const std::string & prefix, bool verbose);
	static void identify_parents();

	static class_info *first;
	class_info *next;
	mutable class_info *parent;

	static bool parents_identified;
};

template <class OPT>
const class_info<OPT> *class_info<OPT>::find(const std::string &class_name)
{
	// Use a map for faster lookup. The registered_class_info list doesn't
	// change at run-time, so it's sufficient to construct the map once
	// on the first trip through this function.
	typedef std::map<std::string, const class_info *> name_map_type;
	static name_map_type name_map;
	static bool name_map_initialized = false;

	if (!name_map_initialized) {
		// Construct map
		const class_info *p = first;
		while (p) {
			name_map[p->options.get_name()] = p;
			p = p->next;
		}
		name_map_initialized = true;
	}

	typename name_map_type::const_iterator it = name_map.find(class_name);
	if (it == name_map.end())
		throw (std::runtime_error("class '" + class_name + "' not registered"));
	else
		return it->second;
}

template <class OPT>
void class_info<OPT>::dump_tree(tree_node *n, const std::string & prefix, bool verbose)
{
	std::string name = n->info->options.get_name();
	std::cout << name;
	if (verbose)
		std::cout << " [ID 0x" << std::hex << std::setw(8) << std::setfill('0') << n->info->options.get_id() << std::dec << "]" << std::endl;

	size_t num_children = n->children.size();
	if (num_children) {
		for (size_t i = 0; i < num_children; ++i) {
			if (verbose) {
				std::cout << prefix << " +- ";
				if (i == num_children - 1)
					dump_tree(n->children[i], prefix + "    ", verbose);
				else
					dump_tree(n->children[i], prefix + " |  ", verbose);
			} else {
				std::string spaces(name.size(), ' ');
				if (i > 0)
					std::cout << prefix << spaces;
				if (num_children == 1)
					std::cout << " --- ";
				else if (i > 0)
					std::cout << "  +- ";
				else
					std::cout << " -+- ";
				if (i == num_children - 1)
					dump_tree(n->children[i], prefix + spaces + "     ", verbose);
				else
					dump_tree(n->children[i], prefix + spaces + "  |  ", verbose);
			}
		}
	} else if (!verbose)
		std::cout << std::endl;
}

template <class OPT>
void class_info<OPT>::dump_hierarchy(bool verbose)
{
	identify_parents();

	// Create tree nodes for all class_infos
	std::vector<tree_node> tree;
	for (class_info *p = first; p; p = p->next)
		tree.push_back(tree_node(p));

	// Identify children for all nodes and find the root
	tree_node *root = nullptr;
	for (typename std::vector<tree_node>::iterator i = tree.begin(); i != tree.end(); ++i) {
		class_info *p = i->info->get_parent();
		if (p) {
			for (typename std::vector<tree_node>::iterator j = tree.begin(); j != tree.end(); ++j) {
				if (j->info == p) {
					j->add_child(&*i);
					break;
				}
			}
		} else
			root = &*i;
	}

	// Print hierarchy tree starting at the root
	dump_tree(root, "", verbose);
}

template <class OPT>
void class_info<OPT>::identify_parents()
{
	if (!parents_identified) {
		for (class_info *p = first; p; p = p->next) {
			const char *parent_name = p->options.get_parent_name();
			for (class_info *q = first; q; q = q->next) {
				if (std::strcmp(q->options.get_name(), parent_name) == 0) {
					p->parent = q;
					break;
				}
			}
		}
		parents_identified = true;
	}
}

template <class OPT> class_info<OPT> *class_info<OPT>::first = nullptr;
template <class OPT> bool class_info<OPT>::parents_identified = false;

} // namespace GiNaC

#endif // ndef GINAC_CLASS_INFO_H
