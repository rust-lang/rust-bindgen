// bindgen-flags: --translate-function-macros

// Redefined macro: the last definition should win.
#define F(x) ((x) + 1)
#undef F
#define F(x) ((x) + 2)
