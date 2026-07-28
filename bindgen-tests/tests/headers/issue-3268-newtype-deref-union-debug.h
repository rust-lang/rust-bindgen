// bindgen-flags: --impl-debug --default-alias-style=new_type_deref

union Union {
  unsigned char bytes[4];
  unsigned int word;
};

typedef union Union UnionAlias;

struct StructContainingUnionAlias {
  UnionAlias ua;
};
