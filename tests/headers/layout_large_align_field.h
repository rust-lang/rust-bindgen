// bindgen-flags: --rustified-enum .*

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

/*
 * Tail queue declarations.
 */
#define	TAILQ_HEAD(name, type)						            \
struct name {								                    \
	struct type *tqh_first;	/* first element */			        \
	struct type **tqh_last;	/* addr of last next element */		\
}


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

TAILQ_HEAD(ip_pkt_list, ip_frag_pkt); /**< @internal fragments tailq */

/** fragmentation table statistics */
struct ip_frag_tbl_stat {
	uint64_t find_num;      /**< total # of find/insert attempts. */
	uint64_t add_num;       /**< # of add ops. */
	uint64_t del_num;       /**< # of del ops. */
	uint64_t reuse_num;     /**< # of reuse (del/add) ops. */
	uint64_t fail_total;    /**< total # of add failures. */
	uint64_t fail_nospace;  /**< # of 'no space' add failures. */
} __rte_cache_aligned;

/** fragmentation table */
struct rte_ip_frag_tbl {
	uint64_t             max_cycles;      /**< ttl for table entries. */
	uint32_t             entry_mask;      /**< hash value mask. */
	uint32_t             max_entries;     /**< max entries allowed. */
	uint32_t             use_entries;     /**< entries in use. */
	uint32_t             bucket_entries;  /**< hash assocaitivity. */
	uint32_t             nb_entries;      /**< total size of the table. */
	uint32_t             nb_buckets;      /**< num of associativity lines. */
	struct ip_frag_pkt *last;         /**< last used entry. */
	struct ip_pkt_list lru;           /**< LRU list for table entries. */
	struct ip_frag_tbl_stat stat;     /**< statistics counters. */
	__extension__ struct ip_frag_pkt pkt[0]; /**< hash table. */
};
