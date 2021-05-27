// Abstract class of complex numbers.

#ifndef _CL_COMPLEX_CLASS_H
#define _CL_COMPLEX_CLASS_H

#include "cln/number.h"

namespace cln {

class cl_N : public cl_number {
public:
// Default constructor.
	cl_N ();
// Copy constructor.
	cl_N (const cl_N&);
// Converters.
// Assignment operators.
	cl_N& operator= (const cl_N&);
// Constructors and assignment operators from C numeric types.
	cl_N (const int);		// |argument| must be < 2^29
	cl_N (const unsigned int);	// argument must be < 2^29
	cl_N (const long);
	cl_N (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_N (const long long);
	cl_N (const unsigned long long);
#endif
	cl_N (const float);
	cl_N (const double);
	cl_N& operator= (const int);		// |argument| must be < 2^29
	cl_N& operator= (const unsigned int);	// argument must be < 2^29
	cl_N& operator= (const long);
	cl_N& operator= (const unsigned long);
	cl_N& operator= (const float);
	cl_N& operator= (const double);
#ifdef HAVE_LONGLONG
	cl_N& operator= (const long long);
	cl_N& operator= (const unsigned long long);
#endif
// Other constructors.
	cl_N (const char *);
// Private constructor.
	cl_N (cl_private_thing);
	cl_N (struct cl_heap_complex *);
public:	// Ability to place an object at a given address.
	void* operator new (size_t size) { return malloc_hook(size); }
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void operator delete (void* ptr) { free_hook(ptr); }
private:
// Friend declarations. They are for the compiler. Just ignore them.
};

// Private constructors.
inline cl_N::cl_N (cl_private_thing ptr) : cl_number (ptr) {}
// The assignment operators:
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_N, cl_N)
// The default constructors.
inline cl_N::cl_N ()
	: cl_number ((cl_private_thing) cl_combine(cl_FN_tag,0)) {}
// The copy constructors.
CL_DEFINE_COPY_CONSTRUCTOR2(cl_N,cl_number)
// Constructors and assignment operators from C numeric types.
CL_DEFINE_INT_CONSTRUCTORS(cl_N)
CL_DEFINE_INT_ASSIGNMENT_OPERATORS(cl_N)
CL_DEFINE_LONG_CONSTRUCTORS(cl_N)
CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(cl_N)
#ifdef HAVE_LONGLONG
CL_DEFINE_LONGLONG_CONSTRUCTORS(cl_N)
CL_DEFINE_LONGLONG_ASSIGNMENT_OPERATORS(cl_N)
#endif
CL_DEFINE_FLOAT_CONSTRUCTOR(cl_N)
CL_DEFINE_DOUBLE_CONSTRUCTOR(cl_N)

}  // namespace cln

#endif /* _CL_COMPLEX_CLASS_H */
