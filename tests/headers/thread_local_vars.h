// bindgen-flags: --generate-tls-vars
// bindgen-generate-bindings-on-linux-only

__thread int i;
extern __thread struct state s;
static __thread char *p;
