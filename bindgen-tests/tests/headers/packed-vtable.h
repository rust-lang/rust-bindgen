// bindgen-flags: --raw-line '#![cfg(feature = "nightly")]' --rust-target 1.33 -- -x c++ -std=c++11

#pragma pack(1)

// This should be packed.
struct PackedVtable {
  virtual ~PackedVtable();
};

#pragma pack()
