struct rte_ring {
  struct rte_memzone *memzone;

  struct prod {
    unsigned watermark;
  } prod;

  struct cons {
    unsigned sc_dequeue;
  } cons;

  void *ring[];
};
