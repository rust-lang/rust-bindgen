// bindgen-flags: --macro-const-use-ctypes

#ifndef ISSUE_923_H
#define ISSUE_923_H

#define ULONG_ZERO 0UL
#define ULONGLONG_ZERO 0ULL
#define CHAR_ZERO ((char)'\0')
#define FLOAT_LIT 0.f
#define DOUBLE_LIT ((double)0.f)
#define UINT_LIT 0x80000000

#endif
