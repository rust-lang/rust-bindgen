/* Integers of type char have 8 bits. */
#define char_bitsize 8

/* Integers of type short have 16 bits. */
#define short_bitsize 16

/* Integers of type int have 32 bits. */
#define int_bitsize 32

/* Integers of type long have 64 bits. */
#define long_bitsize 64

/* Integers of type long long have 64 bits. */
#define long_long_bitsize 64

/* Integers of type unsigned char have 8 bits. */

/* Integers of type unsigned short have 16 bits. */

/* Integers of type unsigned int have 32 bits. */

/* Integers of type unsigned long have 64 bits. */

/* Integers of type unsigned long long have 64 bits. */

/* Integer types char and unsigned char have the same binary representation. */
/* Integer types short and unsigned short have the same binary representation. */
/* Integer types int and unsigned int have the same binary representation. */
/* Integer types long and unsigned long have the same binary representation. */
/* Integer types long long and unsigned long long have the same binary representation. */

/* Pointers of type char * have 64 bits. */
#define pointer_bitsize 64

/* Casts from long * to char * is OK (does nothing). */
/* Casts from char * to long * is OK (does nothing). */
/* Casts from function * to char * is OK (does nothing). */
/* Casts from char * to function * is OK (does nothing). */

/* Type char has sizeof = 1 and alignment = 1. */
#define sizeof_char 1
#define alignment_char 1

/* Type unsigned char has sizeof = 1 and alignment = 1. */

/* Type short has sizeof = 2 and alignment = 2. */
#define sizeof_short 2
#define alignment_short 2

/* Type unsigned short has sizeof = 2 and alignment = 2. */

/* Type int has sizeof = 4 and alignment = 4. */
#define sizeof_int 4
#define alignment_int 4

/* Type unsigned int has sizeof = 4 and alignment = 4. */

/* Type long has sizeof = 8 and alignment = 8. */
#define sizeof_long 8
#define alignment_long 8

/* Type unsigned long has sizeof = 8 and alignment = 8. */

/* Type long long has sizeof = 8 and alignment = 8. */
#define sizeof_long_long 8
#define alignment_long_long 8

/* Type unsigned long long has sizeof = 8 and alignment = 8. */

/* Type float has sizeof = 4 and alignment = 4. */
#define sizeof_float 4
#define alignment_float 4

/* Type double has sizeof = 8 and alignment = 8. */
#define sizeof_double 8
#define alignment_double 8

/* Type char * has sizeof = 8 and alignment = 8. */

/* Type long * has sizeof = 8 and alignment = 8. */

/* Type function * has sizeof = 8 and alignment = 8. */

/* Type unsigned short is stored LITTLE-ENDIAN in memory (i.e. like Z80 or VAX). */
#define short_little_endian
/* Type unsigned int is stored LITTLE-ENDIAN in memory (i.e. like Z80 or VAX). */
#define int_little_endian
/* Type unsigned long is stored LITTLE-ENDIAN in memory (i.e. like Z80 or VAX). */
#define long_little_endian
/* Type unsigned long long is stored LITTLE-ENDIAN in memory (i.e. like Z80 or VAX). */
#define long_long_little_endian

/* Stack grows up, ca. 1 bytes per function call. */
#define stack_grows_up
