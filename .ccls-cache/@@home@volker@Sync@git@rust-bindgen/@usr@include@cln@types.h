// Basic type definitions

#ifndef _CL_TYPES_H
#define _CL_TYPES_H

// CPU and other
#include "cln/config.h"

// char_bitsize, short_bitsize, long_bitsize, long_long_bitsize, pointer_bitsize
#include "cln/intparam.h"

// intptr_t, uintptr_t
#include <stdint.h>

// Elementary arithmetic types of given width:
  // 8 bits
  #if (char_bitsize==8)
    typedef signed char    sint8;
    typedef unsigned char  uint8;
  #else
    #error "No 8 bit integer type?"
  #endif
  // 16 bits
  #if (short_bitsize==16)
    typedef short           sint16;
    typedef unsigned short  uint16;
  #else
    #error "No 16 bit integer type?"
  #endif
  // 32 bits
  #if (long_bitsize==32)
    typedef long           sint32;
    typedef unsigned long  uint32;
  #elif (int_bitsize==32)
    typedef int            sint32;
    typedef unsigned int   uint32;
  #else
    #error "No 32 bit integer type?"
  #endif
  // 64 bits
  #if (long_bitsize==64)
    typedef long           sint64;
    typedef unsigned long  uint64;
    #undef HAVE_LONGLONG
    #define HAVE_LONGLONG
  #elif defined(HAVE_LONGLONG)
   #if defined(long_long_bitsize) && (long_long_bitsize==64)
    typedef long long           sint64;
    typedef unsigned long long  uint64;
   #else // unusable type
    #undef HAVE_LONGLONG
   #endif
  #endif
  #if defined(HAVE_LONGLONG) && (defined(__alpha__) || defined(__ia64__) || defined(__mips64__) || defined(__powerpc64__) || defined(__s390x__) || (defined(__sparc__) && defined(__arch64__)) || (defined(__x86_64__) || defined(_M_AMD64)) || defined(__aarch64__) || (defined(__riscv) && __riscv_xlen == 64)) || defined(__e2k__)
    // 64 bit registers in hardware
    #define HAVE_FAST_LONGLONG
  #endif
// Synonyms
  #define intBsize 8
  typedef sint8  sintB;
  typedef uint8  uintB;
  #define intWsize 16
  typedef sint16 sintW;
  typedef uint16 uintW;
  #define intLsize 32
  typedef sint32 sintL;
  typedef uint32 uintL;
  #ifdef HAVE_LONGLONG
    #define intQsize 64
    typedef sint64 sintQ;
    typedef uint64 uintQ;
  #endif

// Type for three values (0, +1, -1).
  typedef int  cl_signean;
  #define signean_plus  1
  #define signean_null  0
  #define signean_minus -1

// Integer type used for counters.
// Constraint: sizeof(uintC) >= sizeof(uintL)
  #if (defined(HAVE_FAST_LONGLONG) && (defined(__alpha__) || defined(__ia64__) || defined(__powerpc64__) || defined(__s390x__) || (defined(__sparc__) && defined(__arch64__)) || defined(__x86_64__) || defined(__aarch64__) || defined(__mips64__) || (defined(__riscv) && __riscv_xlen == 64) || defined(__e2k__)))
    #define intCsize long_bitsize
    typedef long           sintC;
    typedef unsigned long  uintC;
  #else
    #define intCsize int_bitsize
    typedef int           sintC;
    typedef unsigned int  uintC;
  #endif

// Integer type used for lfloat exponents.
// Constraint: sizeof(uintE) >= sizeof(uintC)
  #if (defined(HAVE_LONGLONG) && (defined(__alpha__) || defined(__ia64__) || defined(__powerpc64__) || defined(__s390x__) || (defined(__sparc__) && defined(__arch64__)) || defined(__x86_64__) || defined(__i386__) || defined(__mips__) || defined(__rs6000__) || defined(__aarch64__) || (defined(__riscv) && __riscv_xlen == 64) || defined(__e2k__)))
    #define intEsize 64
    typedef sint64  sintE;
    typedef uint64  uintE;
  #else
    #define intEsize 32
    typedef sint32  sintE;
    typedef uint32  uintE;
  #endif

// Integer type as large as a pointer.
// Assumption: sizeof(intptr_t) == sizeof(void*)
  #define intPsize pointer_bitsize
  typedef intptr_t   sintP;
  typedef uintptr_t  uintP;

// Integer type used for the value of a fixnum.
  // It must be like this, because in a couple of places we assume
  // cl_value_shift + cl_value_len == cl_pointer_size.
  #define intVsize intPsize
  typedef sintP  sintV;
  typedef uintP  uintV;

// Numbers in the heap are stored as "digit" sequences.
// A digit is an unsigned int with intDsize bits.
// intDsize should be 8 or 16 or 32 or 64 and it should match mp_limb_t,
// if CLN is sitting on top of GMP.
  #if defined(GMP_DEMANDS_UINTD_LONG_LONG)
    #define HAVE_FAST_LONGLONG
    #define intDsize long_long_bitsize
    typedef long long sintD;
    typedef unsigned long long uintD;
  #elif defined(GMP_DEMANDS_UINTD_LONG)
    #define intDsize long_bitsize
    typedef long sintD;
    typedef unsigned long uintD;
  #elif defined(GMP_DEMANDS_UINTD_INT)
    #define intDsize int_bitsize
    typedef int sintD;
    typedef unsigned int uintD;
  #else  // we are not using GMP, so just guess something reasonable
    #if (defined(HAVE_FAST_LONGLONG) && (defined(__alpha__) || defined(__ia64__) || defined(__powerpc64__) || (defined(__sparc__) && defined(__arch64__)) || defined(__s390x__) || defined(__x86_64__) || defined(__aarch64__) || defined(__mips64__) || (defined(__riscv) && __riscv_xlen == 64) || defined(__e2k__)))
      #define intDsize 64
      typedef sint64  sintD;
      typedef uint64  uintD;
    #else
      #define intDsize 32
      typedef sint32  sintD;
      typedef uint32  uintD;
    #endif
  #endif
  #if (intDsize==64)
    #define intDDsize 128    // = 2*intDsize
    #define log2_intDsize 6  // = log2(intDsize)
  #elif (intDsize==32)
    #define intDDsize 64     // = 2*intDsize
    #define log2_intDsize 5  // = log2(intDsize)
  #elif (intDsize==16)
    #define intDDsize 32     // = 2*intDsize
    #define log2_intDsize 4  // = log2(intDsize)
  #elif (intDsize==8)
    #define intDDsize 16     // = 2*intDsize
    #define log2_intDsize 3  // = log2(intDsize)
  #else
    #error "What is intDsize again?"
  #endif
// HAVE_DD means that there are unsigned ints with 2*intDsize bits.
  #if (intDDsize <= (defined(HAVE_FAST_LONGLONG) ? 64 : 32))
    #define HAVE_DD 1
    #if (intDDsize==16)
      typedef sint16  sintDD;
      typedef uint16  uintDD;
    #endif
    #if (intDDsize==32)
      typedef sint32  sintDD;
      typedef uint32  uintDD;
    #endif
    #if (intDDsize==64)
      typedef sint64  sintDD;
      typedef uint64  uintDD;
    #endif
  #else
    #define HAVE_DD 0
  #endif

// Largest integer type which can be manipulated as efficiently as a pointer.
// This is normally the same as the hardware register size.
// Assumption: cl_word_size >= intPsize
  #ifdef HAVE_FAST_LONGLONG
    #define cl_word_size  64
  #else
    #define cl_word_size  32
  #endif

#endif /* _CL_TYPES_H */
