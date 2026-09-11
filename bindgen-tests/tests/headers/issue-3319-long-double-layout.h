// Test for issue #3319: Layout mismatch when Rust can't represent C type exactly
//
// This test documents the scenario where layout tests fail due to:
// - C types that have different sizes on different platforms (e.g., long double)
// - Rust type mapping producing different sizes than libclang reports
//
// On FreeBSD i386:
// - long double is 12 bytes (x87 extended precision)
// - max_align_t has size 20 bytes
// - Rust maps long double to an integer type (not f64)
// - Layout test fails: Rust_size - libclang_size = overflow
//
// On macOS/Linux x86_64:
// - long double is 8 bytes (macOS) or 16 bytes (Linux)
// - This test passes because sizes match

// Test case: Struct with long double field
// This will have different layouts on different platforms
struct test_long_double_layout {
  long double value;
};

// Test case: Struct simulating max_align_t on Linux/FreeBSD
// The aligned attributes are recognized, not unknown
struct test_max_align_sim {
  long long field1 __attribute__((__aligned__(__alignof__(long long))));
  long double field2 __attribute__((__aligned__(__alignof__(long double))));
};
