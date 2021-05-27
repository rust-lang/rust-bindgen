// Abstract class of integers.

#ifndef _CL_INTEGER_CLASS_H
#define _CL_INTEGER_CLASS_H

#include "cln/number.h"
#include "cln/rational_class.h"

namespace cln {

class cl_I : public cl_RA {
public:
// Default constructor.
	cl_I ();
// Copy constructor.
	cl_I (const cl_I&);
// Assignment operators.
	cl_I& operator= (const cl_I&);
// Constructors and assignment operators from C numeric types.
	cl_I (const int);		// |argument| must be < 2^29
	cl_I (const unsigned int);	// argument must be < 2^29
	cl_I (const long);
	cl_I (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_I (const long long);
	cl_I (const unsigned long long);
#endif
	cl_I& operator= (const int);		// |argument| must be < 2^29
	cl_I& operator= (const unsigned int);	// argument must be < 2^29
	cl_I& operator= (const long);
	cl_I& operator= (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_I& operator= (const long long);
	cl_I& operator= (const unsigned long long);
#endif
// Other constructors.
	cl_I (const char *);
// Private constructor.
	cl_I (cl_private_thing);
	cl_I (struct cl_fixnum * /* NULL! */, cl_uint);
	cl_I (struct cl_heap_bignum *);
public:	// Ability to place an object at a given address.
	void* operator new (size_t size) { return malloc_hook(size); }
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void operator delete (void* ptr) { free_hook(ptr); }
};

// Private constructors.
inline cl_I::cl_I (cl_private_thing ptr) : cl_RA (ptr) {}
// The assignment operators:
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_I, cl_I)
// The default constructors.
inline cl_I::cl_I ()
	: cl_RA ((cl_private_thing) cl_combine(cl_FN_tag,0)) {}
// The copy constructors.
CL_DEFINE_COPY_CONSTRUCTOR2(cl_I,cl_RA)
// Constructors and assignment operators from C numeric types.
CL_DEFINE_INT_CONSTRUCTORS(cl_I)
CL_DEFINE_INT_ASSIGNMENT_OPERATORS(cl_I)
CL_DEFINE_LONG_CONSTRUCTORS(cl_I)
CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(cl_I)
#ifdef HAVE_LONGLONG
CL_DEFINE_LONGLONG_CONSTRUCTORS(cl_I)
CL_DEFINE_LONGLONG_ASSIGNMENT_OPERATORS(cl_I)
#endif

}  // namespace cln

#endif /* _CL_INTEGER_CLASS_H */
