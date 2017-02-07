typedef unsigned short uint16_t;
typedef unsigned int uint32_t;

int x __attribute__ ((aligned (16))) = 0;

struct foo { int x[2] __attribute__ ((unused, aligned (8), deprecated)); };

extern int old_var __attribute__ ((deprecated ("dont' use it")));

enum E {
    oldval __attribute__((deprecated)),
    newval
} __attribute__ ((unused));

int foo __attribute__ ((vector_size (16)));

int old_fn () __attribute__ ((deprecated, cold));

extern int
my_printf (void *my_object, const char *my_format, ...)
    __attribute__ ((format (printf, 2, 3)));

void fatal () __attribute__ ((noreturn));

/**
 * Force alignment
 */
#define __rte_aligned(a) __attribute__((__aligned__(a)))

/**
 * Force a structure to be packed
 */
#define __rte_packed __attribute__((__packed__))

#define RTE_CACHE_LINE_SIZE 64	/**< Minimum Cache line size. */

/**
 * Force alignment to cache line.
 */
#define __rte_cache_aligned __rte_aligned(RTE_CACHE_LINE_SIZE)

/**
 * The generic rte_mbuf, containing a packet mbuf.
 */
struct rte_mbuf {
	/** Timesync flags for use with IEEE1588. */
	uint16_t timesync;
} __rte_cache_aligned;

/**
 * Physical memory segment descriptor.
 */
struct rte_memseg {
	uint32_t nchannel;          /**< Number of channels. */
	uint32_t nrank;             /**< Number of ranks. */
} __rte_packed;

struct align {
	int foo;
} __attribute__((aligned(2 * sizeof(long long))));