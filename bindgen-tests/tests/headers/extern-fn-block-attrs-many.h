// bindgen-flags: --extern-fn-block-attrs '#[allow(dead_code)]' --extern-fn-block-attrs '#[cfg_attr(not(windows), link(wasm_import_module = "test-module"))]'

void test_function();