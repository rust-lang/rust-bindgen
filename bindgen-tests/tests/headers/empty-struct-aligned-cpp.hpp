struct empty_aligned {
} __attribute__((aligned(64)));

struct contains_empty_aligned {
	int before;
	empty_aligned empty;
	long long after;
};
