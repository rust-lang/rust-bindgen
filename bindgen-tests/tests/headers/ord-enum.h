// bindgen-flags: --rustified-enum ".*" --with-derive-ord

enum A {
	A0 = 0,
	A1 = 1,
	A2 = 2,
	A3 = A0 - 1,
};

enum B {
	B0 = 1,
	B1 = B0 + 3,
	B2 = B0 + 2,
	B3 = B0 - 2,
};
