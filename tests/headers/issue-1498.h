typedef unsigned long uint64_t;
typedef uint64_t size_t;
typedef unsigned uint32_t;
typedef int int32_t;

struct rte_memseg {
    uint64_t phys_addr;      /**< Start physical address. */
    union {
        void *addr;         /**< Start virtual address. */
        uint64_t addr_64;   /**< Makes sure addr is always 64 bits */
    };
    size_t len;               /**< Length of the segment. */
    uint64_t hugepage_sz;       /**< The pagesize of underlying memory */
    int32_t socket_id;          /**< NUMA socket ID. */
    uint32_t nchannel;          /**< Number of channels. */
    uint32_t nrank;             /**< Number of ranks. */
} __attribute__((__packed__));
