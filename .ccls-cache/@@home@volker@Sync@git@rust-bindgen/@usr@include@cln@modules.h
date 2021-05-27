// Macros for correct module ordering.

#ifndef _CL_MODULES_H
#define _CL_MODULES_H

// global constructor/destructor naming.
#include "cln/config.h"

// Concatenation of macroexpanded tokens.
// Equivalent to CL_CONCAT in src/base/cl_macros.h which we do not want
// to expose, however.
#define CL_CONCATENATE_(xxx,yyy)  xxx##yyy
#define CL_CONCATENATE(xxx,yyy)  CL_CONCATENATE_(xxx,yyy)

// Sometimes a link time dependency is needed, but without requirements
// on initialization order.
//
// CL_FORCE_LINK(dummy,external_variable)
// forces a link time reference to the external_variable.
#include <cstdlib>
#if 0
// This definition does not work.  It gets optimized away by g++ 3.1.
#define CL_FORCE_LINK(dummy,external_variable) \
  static const void* const dummy[] = { &dummy, &external_variable };
#else
#define CL_FORCE_LINK(dummy,external_variable) \
  static const								\
  struct dummy {							\
    inline dummy () {							\
      if ((void*) &external_variable == (void*) this)			\
        abort();							\
    }									\
  }									\
  CL_CONCATENATE(dummy,_instance);
#endif

#endif /* _CL_MODULES_H */
