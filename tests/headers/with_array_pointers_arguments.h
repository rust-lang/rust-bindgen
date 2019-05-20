// bindgen-flags: --use-array-pointers-in-arguments

int test_fn(float a, int arr[20]);

int test_fn2(const float arr[20], int b);