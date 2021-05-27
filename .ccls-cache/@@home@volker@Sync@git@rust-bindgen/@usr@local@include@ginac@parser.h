/** @file parser.h
 *
 *  Interface to the parser. */

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

#ifndef GINAC_PARSER_H
#define GINAC_PARSER_H

#include "parse_context.h"
#include "ex.h"

#include <stdexcept>

namespace GiNaC {

class lexer;

class parse_error : public std::invalid_argument
{
public:
	const std::size_t line;
	const std::size_t column;
	parse_error(const std::string& what_,
		    const std::size_t line_ = 0,
		    const std::size_t column_ = 0) throw () :
		std::invalid_argument(what_), line(line_), column(column_)
	{ }
};

/**
 * Recursive descent parser for GiNaC expressions.
 */
class parser
{
	// The actual parser rules (in EBNF-alike notation):

	/// expression: primary binoprhs
	ex parse_expression();

	/// primary: indentifier_expr | number_expr | paren_expr | unary_expr
	ex parse_primary();

	/// binoprhs: ([+*/^-] primary)*
	ex parse_binop_rhs(int, ex&);

	/// identifier_expr: identifier |
	///                  identifier '(' expression (',' expression)* ')'
	ex parse_identifier_expr();

	/// paren_expr: '(' expression ')'
	ex parse_paren_expr();

	/// lst_expr: '{' expression { ',' expression } '}'
	ex parse_lst_expr();

	/// number_expr: number
	ex parse_number_expr();

	/// unary_expr: [+-] expression
	ex parse_unary_expr();

	/// literal_expr: 'I' | 'Pi' | 'Euler' | 'Catalan'
	ex parse_literal_expr();

public:
	/**
	 * @param syms_ symbol table.
	 * @param funcs_ function/ctors table.
	 * @param strict_ if true, throw an exception if unknown
	 *        symbol is encountered.
	 */
	parser(const symtab& syms_ = symtab(),
		const bool strict_ = false,
		const prototype_table& funcs_ = get_default_reader());
	~parser();

	/// parse the stream @a input
	ex operator()(std::istream& input);
	/// parse the string @a input
	ex operator()(const std::string& input);

	/// report the symbol table used by parser
	symtab get_syms() const 
	{ 
		return syms; 
	}
	/// read/write access to the symbol table
	symtab& get_syms()
	{
		return syms;
	}

	/// If true, throw an exception if an unknown symbol is encountered.
	bool strict;
private:
	/**
	 * Function/ctor table, maps a prototype (which is a name and number
	 * arguments) to a C++ function. Used for parsing identifier_expr's
	 * (see parse_identifier_expr). If expression contains unknown
	 * prototype, an exception will be thrown.
	 */
	const prototype_table funcs;
	/**
	 * Symbol (variable) table. Used for parsing identifier_expr's
	 * (see parse_identifier_expr). If parser is strict, exception is
	 * thrown if an unknown symbol is encountered. Non-strict parser
	 * appends unknown symbols to the symbol table.
	 */
	symtab syms;
	/// token scanner
	lexer* scanner;
	/// current token the parser is looking at
	int token;
	/// read the next token from the scanner
	int get_next_tok();
};

} // namespace GiNaC

#endif // ndef GINAC_PARSER_H
