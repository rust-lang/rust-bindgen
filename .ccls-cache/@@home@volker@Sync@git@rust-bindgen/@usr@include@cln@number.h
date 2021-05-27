// Basic definitions of numbers


#ifndef _CL_NUMBER_H
#define _CL_NUMBER_H


#include "cln/object.h"
#include "cln/malloc.h"

// Type hierachy:
// Number (N) =
//    Real (R) =
//       Float (F) =
//          Short float (SF)
//          Single float (FF)
//          Double float (DF)
//          Long float (LF)
//       Rational (RA) =
//          Integer (I) =
//             Fixnum (FN)
//             Bignum (BN)
//          Ratio (RT)
//    Complex (C)

// Constructors and assignment operators from C numeric types.

#ifdef _MSC_VER
// Workaround to force MSVC to tag the symbol with the cln:: namespace
// When declaring inside an inlined function the symbol is placed in the 
// global namespace!
namespace cln {
extern cl_private_thing cl_I_constructor_from_L (sint32 wert);
extern cl_private_thing cl_I_constructor_from_UL (uint32 wert);
extern cl_private_thing cl_I_constructor_from_Q (sint64 wert);
extern cl_private_thing cl_I_constructor_from_UQ (uint64 wert);
}
#endif

#define CL_DEFINE_INT_CONSTRUCTOR(_class_,_type_)  \
inline _class_::_class_ (const _type_ wert)				\
{									\
	word = cl_combine(cl_FN_tag,wert);				\
}
#define CL_DEFINE_INT_CONSTRUCTORS(_class_)  \
CL_DEFINE_INT_CONSTRUCTOR(_class_, int)					\
CL_DEFINE_INT_CONSTRUCTOR(_class_, unsigned int)

#define CL_DEFINE_INT_ASSIGNMENT_OPERATOR(_class_,_type_)  \
inline _class_& _class_::operator= (const _type_ wert)			\
{									\
	cl_dec_refcount(*this);						\
	word = cl_combine(cl_FN_tag,wert);				\
	return *this;							\
}
#define CL_DEFINE_INT_ASSIGNMENT_OPERATORS(_class_)  \
CL_DEFINE_INT_ASSIGNMENT_OPERATOR(_class_, int)				\
CL_DEFINE_INT_ASSIGNMENT_OPERATOR(_class_, unsigned int)

#if (long_bitsize==32)
// `long' == `sintL', `unsigned long' == `uintL'.
#define CL_DEFINE_LONG_CONSTRUCTORS(_class_)  \
inline _class_::_class_ (const long wert)				\
{									\
	extern cl_private_thing cl_I_constructor_from_L (sint32 wert);	\
	pointer = cl_I_constructor_from_L(wert);			\
}									\
inline _class_::_class_ (const unsigned long wert)			\
{									\
	extern cl_private_thing cl_I_constructor_from_UL (uint32 wert);	\
	pointer = cl_I_constructor_from_UL(wert);			\
}
#elif (long_bitsize==64)
// `long' == `sintQ', `unsigned long' == `uintQ'.
#define CL_DEFINE_LONG_CONSTRUCTORS(_class_)  \
inline _class_::_class_ (const long wert)				\
{									\
	extern cl_private_thing cl_I_constructor_from_Q (sint64 wert);	\
	pointer = cl_I_constructor_from_Q(wert);			\
}									\
inline _class_::_class_ (const unsigned long wert)			\
{									\
	extern cl_private_thing cl_I_constructor_from_UQ (uint64 wert);	\
	pointer = cl_I_constructor_from_UQ(wert);			\
}
#endif

#if (long_bitsize==32)
// `long' == `sintL', `unsigned long' == `uintL'.
#define CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(_class_)  \
inline _class_& _class_::operator= (const long wert)			\
{									\
	extern cl_private_thing cl_I_constructor_from_L (sint32 wert);	\
	cl_dec_refcount(*this);						\
	pointer = cl_I_constructor_from_L(wert);			\
	return *this;							\
}									\
inline _class_& _class_::operator= (const unsigned long wert)		\
{									\
	extern cl_private_thing cl_I_constructor_from_UL (uint32 wert);	\
	cl_dec_refcount(*this);						\
	pointer = cl_I_constructor_from_UL(wert);			\
	return *this;							\
}
#elif (long_bitsize==64)
// `long' == `sintQ', `unsigned long' == `uintQ'.
#define CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(_class_)  \
inline _class_& _class_::operator= (const long wert)			\
{									\
	extern cl_private_thing cl_I_constructor_from_Q (sint64 wert);	\
	cl_dec_refcount(*this);						\
	pointer = cl_I_constructor_from_Q(wert);			\
	return *this;							\
}									\
inline _class_& _class_::operator= (const unsigned long wert)		\
{									\
	extern cl_private_thing cl_I_constructor_from_UQ (uint64 wert);	\
	cl_dec_refcount(*this);						\
	pointer = cl_I_constructor_from_UQ(wert);			\
	return *this;							\
}
#endif

#ifdef HAVE_LONGLONG
#if (long_long_bitsize==64)
// `long' == `sintQ', `unsigned long' == `uintQ'.
#define CL_DEFINE_LONGLONG_CONSTRUCTORS(_class_)  \
inline _class_::_class_ (const long long wert)				\
{									\
	extern cl_private_thing cl_I_constructor_from_Q (sint64 wert);	\
	pointer = cl_I_constructor_from_Q(wert);			\
}									\
inline _class_::_class_ (const unsigned long long wert)			\
{									\
	extern cl_private_thing cl_I_constructor_from_UQ (uint64 wert);	\
	pointer = cl_I_constructor_from_UQ(wert);			\
}
#define CL_DEFINE_LONGLONG_ASSIGNMENT_OPERATORS(_class_)			\
inline _class_& _class_::operator= (const long long wert)		\
{									\
	extern cl_private_thing cl_I_constructor_from_Q (sint64 wert);	\
	cl_dec_refcount(*this);						\
	pointer = cl_I_constructor_from_Q(wert);			\
	return *this;							\
}									\
inline _class_& _class_::operator= (const unsigned long long wert)	\
{									\
	extern cl_private_thing cl_I_constructor_from_UQ (uint64 wert);	\
	cl_dec_refcount(*this);						\
	pointer = cl_I_constructor_from_UQ(wert);			\
	return *this;							\
}
#endif
#endif

namespace cln {

// Constructors and assignment operators from C numeric types.

// from `float':
extern cl_private_thing cl_float_to_FF_pointer (const float val);

#define CL_DEFINE_FLOAT_CONSTRUCTOR(_class_)				\
inline _class_ :: _class_ (const float x)				\
{									\
	pointer = cl_float_to_FF_pointer(x);				\
}									\
inline _class_& _class_::operator= (const float x)			\
{									\
	cl_dec_refcount(*this);						\
	pointer = cl_float_to_FF_pointer(x);				\
	return *this;							\
}

// from `double':
extern struct cl_heap_dfloat * cl_double_to_DF_pointer (const double val);

#define CL_DEFINE_DOUBLE_CONSTRUCTOR(_class_)				\
inline _class_::_class_ (const double x)				\
{									\
	pointer = cl_double_to_DF_pointer(x);				\
}									\
inline _class_& _class_::operator= (const double x)			\
{									\
	cl_dec_refcount(*this);						\
	pointer = cl_double_to_DF_pointer(x);				\
	return *this;							\
}


// Abstract class of all numbers.

class cl_number : public cl_gcobject {
public:
// Default constructor. (Used for objects with no initializer.)
	cl_number ();
// Copy constructor. (Used for function argument passing and function
// return value, and of course for objects with initializers of the same type.)
	cl_number (const cl_number& x);
// Converters. (Used for function argument passing and function return values.)
// Assignment operators. (Used for assignments.)
	cl_number& operator= (const cl_number&);
// Constructors and assignment operators from C numeric types.
	cl_number (const int);		// |argument| must be < 2^29
	cl_number (const unsigned int);	// argument must be < 2^29
	cl_number (const long);
	cl_number (const unsigned long);
#ifdef HAVE_LONGLONG
	cl_number (const long long);
	cl_number (const unsigned long long);
#endif
	cl_number (const float);
	cl_number (const double);
	cl_number& operator= (const int);	// |argument| must be < 2^29
	cl_number& operator= (const unsigned int); // argument must be < 2^29
	cl_number& operator= (const long);
	cl_number& operator= (const unsigned long);
	cl_number& operator= (const float);
	cl_number& operator= (const double);
#ifdef HAVE_LONGLONG
	cl_number& operator= (const long long);
	cl_number& operator= (const unsigned long long);
#endif
// Other constructors.
//	cl_number (const char *);
// Private pointer manipulations.
	cl_number (cl_private_thing);
};

// Private constructors.
inline cl_number::cl_number (cl_private_thing ptr) : cl_gcobject (ptr) {}
// The assignment operators:
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_number, cl_number)
// The default constructors.
inline cl_number::cl_number ()
	: cl_gcobject ((cl_private_thing) cl_combine(cl_FN_tag,0)) {}
// The copy constructors.
CL_DEFINE_COPY_CONSTRUCTOR2(cl_number,cl_gcobject)
// Constructors and assignment operators from C numeric types.
CL_DEFINE_INT_CONSTRUCTORS(cl_number)
CL_DEFINE_INT_ASSIGNMENT_OPERATORS(cl_number)
CL_DEFINE_LONG_CONSTRUCTORS(cl_number)
CL_DEFINE_LONG_ASSIGNMENT_OPERATORS(cl_number)
#ifdef HAVE_LONGLONG
CL_DEFINE_LONGLONG_CONSTRUCTORS(cl_number)
CL_DEFINE_LONGLONG_ASSIGNMENT_OPERATORS(cl_number)
#endif
CL_DEFINE_FLOAT_CONSTRUCTOR(cl_number)
CL_DEFINE_DOUBLE_CONSTRUCTOR(cl_number)


// Hack section.

// Conversions to subtypes without checking, template version:
// the<cl_I>(x) converts x to a cl_I, without change of representation.
template<class type>
inline const type& the(const cl_number& x)
{
	// check that sizeof(type)==sizeof(cl_number)
	static_assert(sizeof(type)==sizeof(cl_number),
	              "sizeof(type)!=sizeof(cl_number)");
	return *(const type *) &x;
}
// Conversions to subtypes without checking, macro version:
// The(cl_I)(x) converts x to a cl_I, without change of representation.
  #define The(type)  *(const type *) & cl_identity
// This inline function is for type checking purposes only.
  inline const cl_number& cl_identity (const cl_number& x) { return x; }

}  // namespace cln


// Conversions to subtypes:
// As(cl_I)(x) returns x as a cl_I. It first checks that x is a cl_I
// and then returns it without change of representation.
#if 0 // no debug information  
  #define As(type)  type##_As
  #define CL_DEFINE_AS_CONVERSION(_class_)				\
    extern const _class_& _class_##_As (const cl_number& x);		\
    inline const _class_& _class_##_As (const _class_& x) { return x; }
#else // Line number information for ease of debugging.
  #define As(type)  type##_As cl_as_aux
  #define cl_as_aux(expr)  (expr,__FILE__,__LINE__)
  #define CL_DEFINE_AS_CONVERSION(_class_)				\
    extern const _class_& _class_##_As (const cl_number& x, const char * filename, int line); \
    inline const _class_& _class_##_As (const _class_& x, const char * filename, int line) { (void)filename; (void)line; return x; }
#endif

// Mutable(type,x);
// x should be a variable `const type x' or `const type& x'.
// This macro introduces a new variable `type& x' whose value can be
// modified. Useful for modifying the argument of a function which takes
// a `const type &x'.
// Warning: To apply this to a function's formal parameter, a block { ... }
// must be inserted.
  #define Mutable(type,x)  \
    type __copied_##x = x;						\
    type& x = __copied_##x;

// DeclareType(type,x);
// x should be a variable of some subtype of `cl_number'. type should be
// a subtype of `cl_number'. A new variable of the given type is declared,
// with name x and which refers to x (by reference, with const attribute).
  #define DeclareType(type,x)  \
    const type& __tmp_##x = *(const type*) &x;				\
    const type& x = __tmp_##x;

#endif /* _CL_NUMBER_H */
