// General object definitions: pointers, reference counting, garbage collection.

#ifndef _CL_OBJECT_H
#define _CL_OBJECT_H

#include "cln/types.h"
#include "cln/modules.h"
#include <cstdlib>

namespace cln {

// We don't have to deal with circular structures, so normal reference counting
// is sufficient. Is also has the advantage of being mostly non-interrupting.


// An object is either a pointer to heap allocated data
//              or immediate data.

// It is possible to distinguish these because pointers are aligned.
// cl_word_alignment is the guaranteed alignment of a `void*' or `long'
// in memory. Must be > 1.
#if defined(__m68k__)
  #define cl_word_alignment  2
#endif
#if defined(__i386__) || (defined(__mips__) && !defined(__LP64__)) || (defined(__sparc__) && !defined(__arch64__)) || defined(__hppa__) || defined(__arm__) || defined(__rs6000__) || defined(__m88k__) || defined(__convex__) || (defined(__s390__) && !defined(__s390x__)) || defined(__sh__) || (defined(__x86_64__) && defined(__ILP32__))
  #define cl_word_alignment  4
#endif
#if defined(__alpha__) || defined(__ia64__) || defined(__mips64__) || defined(__powerpc64__) || (defined(__sparc__) && defined(__arch64__)) || (defined(__x86_64__) && !defined(__ILP32__)) || defined(__s390x__) || defined(__aarch64__) || (defined(__riscv) && __riscv_xlen == 64) || defined(__e2k__)
  #define cl_word_alignment  8
#endif
#if !defined(cl_word_alignment)
  #error "Define cl_word_alignment for your CPU!"
#endif


// Four basic classes are introduced:
//
//   gcobject      rcobject
//
//   gcpointer     rcpointer
//
// `gcobject' = garbage collectible object (pointer or immediate),
// `gcpointer' = garbage collectible pointer,
// `rcobject' = reference counted object (pointer or immediate),
// `rcpointer' = reference counted pointer.
//
// "garbage collectible" means that a reference count is maintained, and
// when the reference count drops to 0, the object is freed. This is useful
// for all kind of short- or long-lived objects.
// "reference counted" means that a reference count is maintained, which
// cannot drop to 0. This is useful for objects which are registered in a
// global cache table, in order to know which objects can be thrown away
// when the cache is cleaned. (If the cache were never cleaned, its objects
// would never be freed, and we could get away with normal C pointers.)
//
// It is permissible to treat a `rcobject' as a `gcobject', and a `rcpointer'
// as a `gcpointer', but this just increases the destructor and copy-constructor
// overhead.
// It is also permissible to treat a `gcpointer' as a `gcobject', and a
// `rcpointer' as a `rcobject', but this just increases the destructor and
// copy-constructor overhead.


// Immediate data is a word, as wide as a pointer.
typedef sintP  cl_sint;
typedef uintP  cl_uint;  // This ought to be called `cl_word'.
#define cl_pointer_size intPsize
// NB: (cl_pointer_size==64) implies defined(HAVE_FAST_LONGLONG)
#if (cl_pointer_size==64)
  #define CL_WIDE_POINTERS
#endif

// Distinguish immediate data from pointers.
inline bool cl_pointer_p (cl_uint word)
{
	return (word & (cl_word_alignment-1)) == 0;
}
inline bool cl_immediate_p (cl_uint word)
{
	return (word & (cl_word_alignment-1)) != 0;
}

// Immediate data: Fixnum, Short Float, maybe Single Float.
// They have type tags.
//   |...............................|......|
//               cl_value             cl_tag

// Number of bits reserved for tagging information:
#if (cl_word_alignment <= 4)
  #define cl_tag_len	2
#else
  #define cl_tag_len	3
#endif
#define cl_tag_shift	0
#define cl_value_shift  (cl_tag_len+cl_tag_shift)
#define cl_value_len	(cl_pointer_size - cl_value_shift)
#define cl_tag_mask	((((cl_uint)1 << cl_tag_len) - 1) << cl_tag_shift)
#define cl_value_mask	((((cl_uint)1 << cl_value_len) - 1) << cl_value_shift)

// Return the tag of a word.
inline cl_uint cl_tag (cl_uint word)
{
	return (word & cl_tag_mask) >> cl_tag_shift;
}

// Return the value (unsigned) of a word.
inline cl_uint cl_value (cl_uint word)
{
	// This assumes cl_value_shift + cl_value_len == cl_pointer_size.
	return word >> cl_value_shift;
}

// Return a word, combining a value and a tag.
inline cl_uint cl_combine_impl (cl_uint tag, cl_uint value)
{
	return (value << cl_value_shift) + (tag << cl_tag_shift);
}
inline cl_uint cl_combine_impl (cl_uint tag, cl_sint value)
{
	// This assumes cl_value_shift + cl_value_len == cl_pointer_size.
	return (value << cl_value_shift) + (tag << cl_tag_shift);
}
inline cl_uint cl_combine (cl_uint tag, unsigned int value)
{ return cl_combine_impl(tag, (cl_uint)value); }
inline cl_uint cl_combine (cl_uint tag, int value)
{ return cl_combine_impl(tag, (cl_sint)value); }
inline cl_uint cl_combine (cl_uint tag, unsigned long value)
{ return cl_combine_impl(tag, (cl_uint)value); }
inline cl_uint cl_combine (cl_uint tag, long value)
{ return cl_combine_impl(tag, (cl_sint)value); }
#ifdef HAVE_LONGLONG
inline cl_uint cl_combine (cl_uint tag, unsigned long long value)
{ return cl_combine_impl(tag, (cl_uint)value); }
inline cl_uint cl_combine (cl_uint tag, long long value)
{ return cl_combine_impl(tag, (cl_uint)value); }
#endif

// Definition of the tags.
#if !defined(CL_WIDE_POINTERS)
  #if (cl_word_alignment == 2)
    #define cl_FN_tag	1
    #define cl_SF_tag	3	// must satisfy the cl_immediate_p predicate!
  #endif
  #if (cl_word_alignment == 4)
    #define cl_FN_tag	1
    #define cl_SF_tag	2
  #endif
#else // CL_WIDE_POINTERS
  // Single Floats are immediate as well.
  #define cl_FN_tag	1
  #define cl_SF_tag	2
  #define cl_FF_tag	3
#endif

// Corresponding classes.
extern const struct cl_class * cl_immediate_classes [1<<cl_tag_len];


// Heap allocated data contains a header, for two purposes:
// - dynamic typing,
// - reference count (a portable alternative to garbage collection,
//   or the basis for a portable and interoperable garbage collection).
struct cl_heap {
	int refcount;			// reference count
	const struct cl_class * type;	// type tag
};

// Function to destroy the contents of a heap object.
typedef void (*cl_heap_destructor_function) (cl_heap* pointer);
// Flags, may be ORed together.
#define cl_class_flags_subclass_complex   1  // all instances belong to cl_N
#define cl_class_flags_subclass_real      2  // all instances belong to cl_R
#define cl_class_flags_subclass_float     4  // all instances belong to cl_F
#define cl_class_flags_subclass_rational  8  // all instances belong to cl_RA
#define cl_class_flags_number_ring       16  // all instances are rings whose
                                             // elements belong to cl_number
#define cl_class_flags_modint_ring       32  // all instances are rings whose
                                             // elements belong to cl_MI
#define cl_class_flags_univpoly_ring     64  // all instances are rings whose
                                             // elements belong to cl_UP
// Function to print an object for debugging, to cerr.
typedef void (*cl_heap_dprint_function) (cl_heap* pointer);

struct cl_class {
	cl_heap_destructor_function destruct;
	int flags;
	cl_heap_dprint_function dprint;
};

// Free an object on heap.
extern void cl_free_heap_object (cl_heap* pointer);

// Debugging support for dynamic typing: Register a debugging print function.
#define cl_register_type_printer(type,printer)  \
  { extern cl_class type; type.dprint = (printer); }


// cl_private_thing: An immediate value or a pointer into the heap.
// This must be as wide as a `cl_uint'.
// (Actually, this ought to be a  union { void*; cl_uint; }, but using
// a pointer type generates better code.)
// Never throw away a cl_private_thing, or reference counts will be wrong!
typedef struct cl_anything * cl_private_thing;

// Increment the reference count.
inline void cl_inc_pointer_refcount (cl_heap* pointer)
{
	pointer->refcount++;
}

// Decrement the reference count of a garbage collected pointer.
inline void cl_gc_dec_pointer_refcount (cl_heap* pointer)
{
	if (--pointer->refcount == 0)
		cl_free_heap_object(pointer);
}
// Decrement the reference count of a reference counted pointer.
inline void cl_rc_dec_pointer_refcount (cl_heap* pointer)
{
	--pointer->refcount;
}

// Increment the reference count.
// This must be a macro, not an inline function, because pointer_p() and
// inc_pointer_refcount() are non-virtual member functions, so that the
// compiler can optimize it.
#define cl_inc_refcount(x)  \
	if ((x).pointer_p())					\
		(x).inc_pointer_refcount();			\

// Decrement the reference count.
// This must be a macro, not an inline function, because pointer_p() and
// dec_pointer_refcount() are non-virtual member functions, so that the
// compiler can optimize it.
#define cl_dec_refcount(x)  \
	if ((x).pointer_p())					\
		(x).dec_pointer_refcount();			\

// The declaration of a copy constructor.
// Restriction: The base class's default constructor must do nothing or
// initialize `pointer' to a constant expression.
#define CL_DEFINE_COPY_CONSTRUCTOR1(_class_)			\
	_CL_DEFINE_COPY_CONSTRUCTOR1(_class_,_class_)
#define _CL_DEFINE_COPY_CONSTRUCTOR1(_class_,_classname_)	\
inline _class_::_classname_ (const _class_& x)			\
{								\
	cl_uint x_word = x.word;				\
	cl_inc_refcount(x);					\
	this->word = x_word;					\
}

// The declaration of a copy constructor.
// Restriction: The base class must have the usual `cl_private_thing'
// constructor. Drawback: The base class must be known here.
#define CL_DEFINE_COPY_CONSTRUCTOR2(_class_,_baseclass_)		\
	_CL_DEFINE_COPY_CONSTRUCTOR2(_class_,_class_,_baseclass_)
#define _CL_DEFINE_COPY_CONSTRUCTOR2(_class_,_classname_,_baseclass_) \
inline _class_::_classname_ (const _class_& x)			\
	: _baseclass_ (as_cl_private_thing(x)) {}

// The declaration of an assignment operator.
#define CL_DEFINE_ASSIGNMENT_OPERATOR(dest_class,src_class)	\
inline dest_class& dest_class::operator= (const src_class& x)	\
{								\
	/* Be careful, we might be assigning x to itself. */	\
	cl_uint x_word = x.word;				\
	cl_inc_refcount(x);					\
	cl_dec_refcount(*this);					\
	this->word = x_word;					\
	return *this;						\
}

// We have a small problem with destructors: The specialized destructor
// of a leaf class such as `cl_SF' should be more efficient than the
// general destructor for `cl_N'. Since (by C++ specs) destructing a cl_SF
// would run the destructors for cl_SF, cl_F, cl_R, cl_N (in that order),
// and in the last step the compiler does not know any more that the object
// actually is a cl_SF, there is no way to optimize the destructor!
// ("progn-reversed" method combination is evil.)
// And if we define "mirror"/"shadow" classes with no destructors (such
// that `cl_F' inherits from `cl_F_no_destructor' buts adds a destructor)
// then we need to add explicit conversion operators cl_SF -> cl_F -> cl_R ...,
// with the effect that calling an overloaded function like `as_cl_F'
// (which has two signatures `as_cl_F(cl_number)' and `as_cl_F(cl_F)')
// with a cl_SF argument gives an "call of overloaded function is ambiguous"
// error.
// There is no help: If we want overloaded functions to be callable in a way
// that makes sense, `cl_SF' has to be a subclass of `cl_F', and then the
// destructor of `cl_SF' will do at least as much computation as the `cl_F'
// destructor. Praise C++ ! :-((
// (Even making `pointer_p()' a virtual function would not help.)


// This is obnoxious.
template <class key1_type, class value_type> struct cl_htentry1;

// The four concrete classes of all objects.

class cl_gcobject {
public: /* ugh */
	union {
		void*   pointer;
		cl_heap* heappointer;
		cl_uint word;
	};
public:
// Default constructor. (Used for objects with no initializer.)
	cl_gcobject ();
// Destructor. (Used when a variable goes out of scope.)
	~cl_gcobject ();
// Copy constructor.
	cl_gcobject (const cl_gcobject&);
// Assignment operator.
	cl_gcobject& operator= (const cl_gcobject&);
// Distinguish immediate data from pointer.
	bool pointer_p() const
		{ return cl_pointer_p(word); }
// Reference counting.
	void inc_pointer_refcount () const
		{ cl_inc_pointer_refcount(heappointer); }
	void dec_pointer_refcount () const
		{ cl_gc_dec_pointer_refcount(heappointer); }
// Return the type tag of an immediate number.
	cl_uint nonpointer_tag () const
		{ return cl_tag(word); }
// Return the type tag of a heap-allocated number.
	const cl_class * pointer_type () const
		{ return heappointer->type; }
// Private pointer manipulations.
	cl_private_thing _as_cl_private_thing () const;
// Private constructor.
	cl_gcobject (cl_private_thing p)
		: pointer (p) {}
// Debugging output.
	void debug_print () const;
// Ability to place an object at a given address.
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void* operator new (size_t size) { return ::operator new (size); }
};
inline cl_gcobject::cl_gcobject () {}
inline cl_gcobject::~cl_gcobject () { cl_dec_refcount(*this); }
CL_DEFINE_COPY_CONSTRUCTOR1(cl_gcobject)
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_gcobject,cl_gcobject)

class cl_gcpointer {
public: /* ugh */
	union {
		void*   pointer;
		cl_heap* heappointer;
		cl_uint word;
	};
public:
// Default constructor. (Used for objects with no initializer.)
	cl_gcpointer ();
// Destructor. (Used when a variable goes out of scope.)
	~cl_gcpointer ();
// Copy constructor.
	cl_gcpointer (const cl_gcpointer&);
// Assignment operator.
	cl_gcpointer& operator= (const cl_gcpointer&);
// Distinguish immediate data from pointer.
	bool pointer_p() const
		{ return true; }
// Reference counting.
	void inc_pointer_refcount () const
		{ cl_inc_pointer_refcount(heappointer); }
	void dec_pointer_refcount () const
		{ cl_gc_dec_pointer_refcount(heappointer); }
// Return the type tag of an immediate number.
	cl_uint nonpointer_tag () const
		{ return cl_tag(word); }
// Return the type tag of a heap-allocated number.
	const cl_class * pointer_type () const
		{ return heappointer->type; }
// Private pointer manipulations.
	cl_private_thing _as_cl_private_thing () const;
// Private constructor.
	cl_gcpointer (cl_private_thing p)
		: pointer (p) {}
// Debugging output.
	void debug_print () const;
// Ability to place an object at a given address.
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void* operator new (size_t size) { return ::operator new (size); }
};
inline cl_gcpointer::cl_gcpointer () {}
inline cl_gcpointer::~cl_gcpointer () { cl_dec_refcount(*this); }
CL_DEFINE_COPY_CONSTRUCTOR1(cl_gcpointer)
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_gcpointer,cl_gcpointer)

class cl_rcobject {
public: /* ugh */
	union {
		void*   pointer;
		cl_heap* heappointer;
		cl_uint word;
	};
public:
// Default constructor. (Used for objects with no initializer.)
	cl_rcobject ();
// Destructor. (Used when a variable goes out of scope.)
	~cl_rcobject ();
// Copy constructor.
	cl_rcobject (const cl_rcobject&);
// Assignment operator.
	cl_rcobject& operator= (const cl_rcobject&);
// Distinguish immediate data from pointer.
	bool pointer_p() const
		{ return cl_pointer_p(word); }
// Reference counting.
	void inc_pointer_refcount () const
		{ cl_inc_pointer_refcount(heappointer); }
	void dec_pointer_refcount () const
		{ cl_rc_dec_pointer_refcount(heappointer); }
// Return the type tag of an immediate number.
	cl_uint nonpointer_tag () const
		{ return cl_tag(word); }
// Return the type tag of a heap-allocated number.
	const cl_class * pointer_type () const
		{ return heappointer->type; }
// Private pointer manipulations.
	cl_private_thing _as_cl_private_thing () const;
// Private constructor.
	cl_rcobject (cl_private_thing p)
		: pointer (p) {}
// Debugging output.
	void debug_print () const;
// Ability to place an object at a given address.
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void* operator new (size_t size) { return ::operator new (size); }
};
inline cl_rcobject::cl_rcobject () {}
inline cl_rcobject::~cl_rcobject () { cl_dec_refcount(*this); }
CL_DEFINE_COPY_CONSTRUCTOR1(cl_rcobject)
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_rcobject,cl_rcobject)

class cl_rcpointer {
public: /* ugh */
	union {
		void*   pointer;
		cl_heap* heappointer;
		cl_uint word;
	};
public:
// Default constructor. (Used for objects with no initializer.)
	cl_rcpointer ();
// Destructor. (Used when a variable goes out of scope.)
	~cl_rcpointer ();
// Copy constructor.
	cl_rcpointer (const cl_rcpointer&);
// Assignment operator.
	cl_rcpointer& operator= (const cl_rcpointer&);
// Distinguish immediate data from pointer.
	bool pointer_p() const
		{ return true; }
// Reference counting.
	void inc_pointer_refcount () const
		{ cl_inc_pointer_refcount(heappointer); }
	void dec_pointer_refcount () const
		{ cl_rc_dec_pointer_refcount(heappointer); }
// Return the type tag of an immediate number.
	cl_uint nonpointer_tag () const
		{ return cl_tag(word); }
// Return the type tag of a heap-allocated number.
	const cl_class * pointer_type () const
		{ return heappointer->type; }
// Private pointer manipulations.
	cl_private_thing _as_cl_private_thing () const;
// Private constructor.
	cl_rcpointer (cl_private_thing p)
		: pointer (p) {}
// Debugging output.
	void debug_print () const;
// Ability to place an object at a given address.
	void* operator new (size_t size, void* ptr) { (void)size; return ptr; }
	void* operator new (size_t size) { return ::operator new (size); }
};
inline cl_rcpointer::cl_rcpointer () {}
inline cl_rcpointer::~cl_rcpointer () { cl_dec_refcount(*this); }
CL_DEFINE_COPY_CONSTRUCTOR1(cl_rcpointer)
CL_DEFINE_ASSIGNMENT_OPERATOR(cl_rcpointer,cl_rcpointer)

// Private pointer manipulations.

inline cl_private_thing cl_gcobject::_as_cl_private_thing () const
{
	cl_private_thing p = (cl_private_thing) pointer;
	cl_inc_refcount(*this);
	return p;
}
inline cl_private_thing as_cl_private_thing (const cl_gcobject& x)
{
	return x._as_cl_private_thing();
}

inline cl_private_thing cl_gcpointer::_as_cl_private_thing () const
{
	cl_private_thing p = (cl_private_thing) pointer;
	cl_inc_refcount(*this);
	return p;
}
inline cl_private_thing as_cl_private_thing (const cl_gcpointer& x)
{
	return x._as_cl_private_thing();
}

inline cl_private_thing cl_rcobject::_as_cl_private_thing () const
{
	cl_private_thing p = (cl_private_thing) pointer;
	cl_inc_refcount(*this);
	return p;
}
inline cl_private_thing as_cl_private_thing (const cl_rcobject& x)
{
	return x._as_cl_private_thing();
}

inline cl_private_thing cl_rcpointer::_as_cl_private_thing () const
{
	cl_private_thing p = (cl_private_thing) pointer;
	cl_inc_refcount(*this);
	return p;
}
inline cl_private_thing as_cl_private_thing (const cl_rcpointer& x)
{
	return x._as_cl_private_thing();
}

// Note: When we define a function that returns a class object by value,
// we normally return it as const value. The declarations
//            T func (...);                    (A)
// and
//            const T func (...);              (B)
// behave identically and generate identical code, except that the code
//            func(...) = foo;
// compiles fine with (A) but is an error (and yields a warning) with (B).
// We want this warning.

// Define a conversion operator from one object to another object of the
// same size.
  #define CL_DEFINE_CONVERTER(target_class)  \
    operator const target_class & () const				\
    {									\
      int (*dummy1)(int assert1 [2*(sizeof(target_class)==sizeof(*this))-1]); (void)dummy1; \
      return * (const target_class *) (void*) this;			\
    }

}  // namespace cln

#endif /* _CL_OBJECT_H */
