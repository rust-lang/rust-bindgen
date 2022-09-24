// bindgen-flags: --impl-debug
struct perf_event_attr {

    /*
     * Major type: hardware/software/tracepoint/etc.
     */
    unsigned int			type;

    float a;
    union {
        int b;
        int c;
    };
};
