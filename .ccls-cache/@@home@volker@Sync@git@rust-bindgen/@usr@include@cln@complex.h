// Public complex number operations.

#ifndef _CL_COMPLEX_H
#define _CL_COMPLEX_H

#include "cln/number.h"
#include "cln/complex_class.h"
#include "cln/real_class.h"
#include "cln/integer_class.h"

namespace cln {

CL_DEFINE_AS_CONVERSION(cl_N)


// zerop(x) testet, ob (= x 0).
extern bool zerop (const cl_N& x);


// Liefert zu reellen Zahlen a und b die komplexe Zahl a+bi.
// complex(a,b)
extern const cl_N complex (const cl_R& a, const cl_R& b);

// realpart(x) liefert den Realteil der Zahl x.
extern const cl_R realpart (const cl_N& x);

// imagpart(x) liefert den Imaginärteil der Zahl x.
extern const cl_R imagpart (const cl_N& x);

// conjugate(x) liefert die konjugiert komplexe Zahl zur Zahl x.
extern const cl_N conjugate (const cl_N& x);


// Liefert (- x), wo x eine Zahl ist.
extern const cl_N operator- (const cl_N& x);

// Liefert (+ x y), wo x und y Zahlen sind.
extern const cl_N operator+ (const cl_N& x, const cl_N& y);

// Liefert (- x y), wo x und y Zahlen sind.
extern const cl_N operator- (const cl_N& x, const cl_N& y);

// Liefert (* x y), wo x und y Zahlen sind.
extern const cl_N operator* (const cl_N& x, const cl_N& y);

// Liefert (* x x), wo x eine Zahl ist.
extern const cl_N square (const cl_N& x);

// Liefert (/ x y), wo x und y Zahlen sind.
extern const cl_N operator/ (const cl_N& x, const cl_N& y);

// Liefert (abs x), wo x eine Zahl ist.
extern const cl_R abs (const cl_N& x);

// recip(x) liefert (/ x), wo x eine Zahl ist.
extern const cl_N recip (const cl_N& x);

// (1+ x), wo x eine Zahl ist.
extern const cl_N plus1 (const cl_N& x);

// (1- x), wo x eine Zahl ist.
extern const cl_N minus1 (const cl_N& x);

// signum(x) liefert (signum x), wo x eine Zahl ist.
extern const cl_N signum (const cl_N& x);

// sqrt(x) = (sqrt x) zieht die Wurzel aus einer Zahl x.
extern const cl_N sqrt (const cl_N& x);

// equal(x,y) vergleicht zwei Zahlen x und y auf Gleichheit.
extern bool equal (const cl_N& x, const cl_N& y);
// equal_hashcode(x) liefert einen equal-invarianten Hashcode für x.
extern uint32 equal_hashcode (const cl_N& x);

inline bool operator== (const cl_N& x, const cl_N& y)
        { return equal(x,y); }
inline bool operator!= (const cl_N& x, const cl_N& y)
        { return !equal(x,y); }

// phase(x) liefert (phase x), wo x eine Zahl ist.
// Ergebnis rational nur wenn (= x 0) oder wenn x reell und >0.
extern const cl_R phase (const cl_N& x);

// exp(x) liefert (exp x), wo x eine Zahl ist.
extern const cl_N exp (const cl_N& x);

// log(x) liefert (log x), wo x eine Zahl ist.
extern const cl_N log (const cl_N& x);

// log(a,b) liefert (log a b), wo a und b Zahlen sind.
extern const cl_N log (const cl_N& a, const cl_N& b);

// (expt x y), wo x eine Zahl und y ein Integer ist.
extern const cl_N expt (const cl_N& x, sintL y);
extern const cl_N expt (const cl_N& x, const cl_I& y);

// (expt x y), wo x und y Zahlen sind.
extern const cl_N expt (const cl_N& x, const cl_N& y);

// sin(x) liefert (sin x), wo x eine Zahl ist.
extern const cl_N sin (const cl_N& x);

// cos(x) liefert (cos x), wo x eine Zahl ist.
extern const cl_N cos (const cl_N& x);

// tan(x) liefert (tan x), wo x eine Zahl ist.
extern const cl_N tan (const cl_N& x);

// cis(x) liefert (cis x), wo x eine Zahl ist.
extern const cl_N cis (const cl_R& x);
extern const cl_N cis (const cl_N& x);

// sinh(x) liefert (sinh x), wo x eine Zahl ist.
extern const cl_N sinh (const cl_N& x);

// cosh(x) liefert (cosh x), wo x eine Zahl ist.
extern const cl_N cosh (const cl_N& x);

// tanh(x) liefert (tanh x), wo x eine Zahl ist.
extern const cl_N tanh (const cl_N& x);

// atan(z) liefert den Arctan einer Zahl z.
extern const cl_N atan (const cl_N& z);

// atanh(z) liefert den Artanh einer Zahl z.
extern const cl_N atanh (const cl_N& z);

// asin(z) liefert den Arcsin einer Zahl z.
extern const cl_N asin (const cl_N& z);

// asinh(z) liefert den Arsinh einer Zahl z.
extern const cl_N asinh (const cl_N& z);

// acos(z) liefert den Arccos einer Zahl z.
extern const cl_N acos (const cl_N& z);

// acosh(z) liefert den Arcosh einer Zahl z.
extern const cl_N acosh (const cl_N& z);


// This could be optimized to use in-place operations.
inline cl_N& operator+= (cl_N& x, const cl_N& y) { return x = x + y; }
inline cl_N& operator++ /* prefix */ (cl_N& x) { return x = plus1(x); }
inline void operator++ /* postfix */ (cl_N& x, int dummy) { (void)dummy; x = plus1(x); }
inline cl_N& operator-= (cl_N& x, const cl_N& y) { return x = x - y; }
inline cl_N& operator-- /* prefix */ (cl_N& x) { return x = minus1(x); }
inline void operator-- /* postfix */ (cl_N& x, int dummy) { (void)dummy; x = minus1(x); }
inline cl_N& operator*= (cl_N& x, const cl_N& y) { return x = x * y; }
inline cl_N& operator/= (cl_N& x, const cl_N& y) { return x = x / y; }


// Runtime typing support.
extern cl_class cl_class_complex;

}  // namespace cln

#endif /* _CL_COMPLEX_H */
