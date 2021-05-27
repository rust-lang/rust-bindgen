// Abstract class of real numbers.

#ifndef _CL_REAL_CLASS_H
#define _CL_REAL_CLASS_H

#include "cln/number.h"
#include "cln/complex_class.h"

namespace cln {

class cl_R : public cl_N {
public:
// Default constructor.
	cl_R ();
// Copy constructor.
	cl_R (const cl_R&);
// Converters.
// Assignment operators.
	cl_R& operator= (const cl_R&);
// Constructors and assignment operators from C numeric types.
	cl_R (const int);		// |argument| must be < 2^29
	cl_R (const unsigned int);	// argument must be < 2^29
	cl_R (const long);
	cl_R (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_R (const long long);
	cl_R (const unsigned long long);
#endif
	cl_R (const float);
	cl_R (const double);
	cl_R& operator= (const int);		// |argument| must be < 2^29
	cl_R& operator= (const unsigned int);	// argument must be < 2^29
	cl_R& operator= (const long);
	cl_R& operator= (const unsigned long);
	cl_R& operator= (const float);
	cl_R& operator= (const double);
#ifdef HAVE_LONGLONG
	cl_R& operator= (const long long);
	cl_R& operator= (const unsigned long long);
#endif
// Other constructors.
	cl_R (const char *);
// Private constructor.
	cl_R (cl_private_thing);
public:	// Ability to place an object at a given address.
	void* operator new (size_t size) { return malloc_hook(size); }
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void operator delete (void* ptr) { free_hook(ptr); }
private:
// Friend declarations. They are for the compiler. Just ignore them.
};

// Private constructors.
inline cl_R::cl_R (cl_private_thing ptr) : cl_N (ptr) {}
// The assignment operators:
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_R, cl_R)
// The default constructors.
inline cl_R::cl_R ()
	: cl_N ((cl_private_thing) cl_combine(cl_FN_tag,0)) {}
// The copy constructors.
CL_DEFINE_COPY_CONSTRUCTOR2(cl_R,cl_N)
// Constructors and assignment operators from C numeric types.
CL_DEFINE_INT_CONSTRUCTORS(cl_R)
CL_DEFINE_INT_ASSIGNMENT_OPERATORS(cl_R)
CL_DEFINE_LONG_CONSTRUCTORS(cl_R)
CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(cl_R)
#ifdef HAVE_LONGLONG
CL_DEFINE_LONGLONG_CONSTRUCTORS(cl_R)
CL_DEFINE_LONGLONG_ASSIGNMENT_OPERATORS(cl_R)
#endif
CL_DEFINE_FLOAT_CONSTRUCTOR(cl_R)
CL_DEFINE_DOUBLE_CONSTRUCTOR(cl_R)

}  // namespace cln

#endif /* _CL_REAL_CLASS_H */
