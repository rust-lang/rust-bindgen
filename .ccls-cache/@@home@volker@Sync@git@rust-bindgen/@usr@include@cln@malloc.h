// User modifiable memory allocator.

#ifndef _CL_MALLOC_H
#define _CL_MALLOC_H

#include <cstdlib>

namespace cln {

// Function like malloc() which returns aligned memory of size (> 0) bytes.
extern void* (*malloc_hook) (size_t size);
// Function like free() which makes available for reuse such memory.
extern void (*free_hook) (void* ptr);

}  // namespace cln

#endif /* _CL_MALLOC_H */
