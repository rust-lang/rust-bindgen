struct inner1 {
	char a;
	char b;
} __attribute__((aligned(2)));

struct inner2 {
	struct inner1 a;
	struct inner1 b;
} __attribute__((aligned(2)));

struct outer1 {
	short a;
	struct inner1 b;
} __attribute__((packed, aligned(2)));

struct outer2 {
	short a;
	struct inner2 b;
} __attribute__((packed, aligned(2)));
