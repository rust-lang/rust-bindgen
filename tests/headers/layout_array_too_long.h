// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq --rustified-enum .*
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

#define RTE_CACHE_LINE_SIZE 64

/**
 * Force alignment
 */
#define __rte_aligned(a) __attribute__((__aligned__(a)))

/**
 * Force alignment to cache line.
 */
#define __rte_cache_aligned __rte_aligned(RTE_CACHE_LINE_SIZE)

#define RTE_LIBRTE_IP_FRAG_MAX_FRAG 4

enum {
	IP_LAST_FRAG_IDX,    /**< index of last fragment */
	IP_FIRST_FRAG_IDX,   /**< index of first fragment */
	IP_MIN_FRAG_NUM,     /**< minimum number of fragments */
	IP_MAX_FRAG_NUM = RTE_LIBRTE_IP_FRAG_MAX_FRAG,
	/**< maximum number of fragments per packet */
};

/** @internal fragmented mbuf */
struct ip_frag {
	uint16_t ofs;          /**< offset into the packet */
	uint16_t len;          /**< length of fragment */
	struct rte_mbuf *mb;   /**< fragment mbuf */
};

/** @internal <src addr, dst_addr, id> to uniquely indetify fragmented datagram. */
struct ip_frag_key {
	uint64_t src_dst[4];      /**< src address, first 8 bytes used for IPv4 */
	uint32_t id;           /**< dst address */
	uint32_t key_len;      /**< src/dst key length */
};

#define	TAILQ_ENTRY(type)						                    \
struct {								                            \
	struct type *tqe_next;	/* next element */			            \
	struct type **tqe_prev;	/* address of previous next element */	\
}

/**
 * @internal Fragmented packet to reassemble.
 * First two entries in the frags[] array are for the last and first fragments.
 */
struct ip_frag_pkt {
	TAILQ_ENTRY(ip_frag_pkt) lru;   /**< LRU list */
	struct ip_frag_key key;           /**< fragmentation key */
	uint64_t             start;       /**< creation timestamp */
	uint32_t             total_size;  /**< expected reassembled size */
	uint32_t             frag_size;   /**< size of fragments received */
	uint32_t             last_idx;    /**< index of next entry to fill */
	struct ip_frag       frags[IP_MAX_FRAG_NUM]; /**< fragments */
} __rte_cache_aligned;
