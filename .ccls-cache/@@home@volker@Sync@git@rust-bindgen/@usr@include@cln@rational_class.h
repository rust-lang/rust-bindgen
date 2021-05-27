// Abstract class of rational numbers.

#ifndef _CL_RATIONAL_CLASS_H
#define _CL_RATIONAL_CLASS_H

#include "cln/number.h"
#include "cln/real_class.h"

namespace cln {

class cl_RA : public cl_R {
public:
// Default constructor.
	cl_RA ();
// Copy constructor.
	cl_RA (const cl_RA&);
// Converters.
// Assignment operators.
	cl_RA& operator= (const cl_RA&);
// Constructors and assignment operators from C numeric types.
	cl_RA (const int);		// |argument| must be < 2^29
	cl_RA (const unsigned int);	// argument must be < 2^29
	cl_RA (const long);
	cl_RA (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_RA (const long long);
	cl_RA (const unsigned long long);
#endif
	cl_RA& operator= (const int);		// |argument| must be < 2^29
	cl_RA& operator= (const unsigned int);	// argument must be < 2^29
	cl_RA& operator= (const long);
	cl_RA& operator= (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_RA& operator= (const long long);
	cl_RA& operator= (const unsigned long long);
#endif
// Other constructors.
	cl_RA (const char *);
// Private constructor.
	cl_RA (cl_private_thing);
	cl_RA (struct cl_heap_ratio *);
public:	// Ability to place an object at a given address.
	void* operator new (size_t size) { return malloc_hook(size); }
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void operator delete (void* ptr) { free_hook(ptr); }
private:
// Friend declarations. They are for the compiler. Just ignore them.
};

// Private constructors.
inline cl_RA::cl_RA (cl_private_thing ptr) : cl_R (ptr) {}
// The assignment operators:
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_RA, cl_RA)
// The default constructors.
inline cl_RA::cl_RA ()
	: cl_R ((cl_private_thing) cl_combine(cl_FN_tag,0)) {}
// The copy constructors.
CL_DEFINE_COPY_CONSTRUCTOR2(cl_RA,cl_R)
// Constructors and assignment operators from C numeric types.
CL_DEFINE_INT_CONSTRUCTORS(cl_RA)
CL_DEFINE_INT_ASSIGNMENT_OPERATORS(cl_RA)
CL_DEFINE_LONG_CONSTRUCTORS(cl_RA)
CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(cl_RA)
#ifdef HAVE_LONGLONG
CL_DEFINE_LONGLONG_CONSTRUCTORS(cl_RA)
CL_DEFINE_LONGLONG_ASSIGNMENT_OPERATORS(cl_RA)
#endif

}  // namespace cln

#endif /* _CL_RATIONAL_CLASS_H */
