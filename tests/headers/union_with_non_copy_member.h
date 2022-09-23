// bindgen-flags: --bindgen-wrapper-union 'WithBindgenGeneratedWrapper' --manually-drop-union 'WithManuallyDrop' --no-copy 'NonCopyType'

struct NonCopyType {
  int foo;
};

union WithBindgenGeneratedWrapper {
  struct NonCopyType non_copy_type;
  int bar;
};

union WithManuallyDrop {
  struct NonCopyType non_copy_type;
  int bar;
};

union WithDefaultWrapper {
  struct NonCopyType non_copy_type;
  int bar;
};
