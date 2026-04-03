// bindgen-parse-callbacks: struct-field-rename

struct RenameMe {
    int plain;
    int renamed_member;
    char a;
    int bitfield_uGlyName: 1;
    int bitfieldWorse_name: 1;
};
