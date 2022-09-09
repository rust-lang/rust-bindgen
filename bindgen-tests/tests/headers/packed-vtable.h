// bindgen-flags: --raw-line '#![cfg(feature = "nightly")]' --rust-target 1.33 --cxx -- -std=c++11

#pragma pack(1)

// This should be packed.
struct PackedVtable {
  virtual ~PackedVtable();
};

#pragma pack()
