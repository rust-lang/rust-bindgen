// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq

struct rte_mbuf {
    union {};
} __attribute__((__aligned__(64)));
