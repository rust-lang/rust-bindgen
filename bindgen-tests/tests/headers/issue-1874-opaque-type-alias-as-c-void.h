// bindgen-flags: --opaque-type WINDOW --opaque-type-alias-as-c-void

typedef struct _WINDOW WINDOW;

int wgetch(WINDOW *win);
