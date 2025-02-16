// Structs
void function_using_anonymous_struct(struct {} arg0);

struct NamedStruct {
};

typedef struct NamedStruct AliasOfNamedStruct;


// Unions
void function_using_anonymous_union(union {} arg0);

union NamedUnion {
};

typedef union NamedUnion AliasOfNamedUnion;

// Enums

// We don't include an anonymous enum because such enums
// are not visible outside the function, and thus tend not
// to be useful - bindgen doesn't handle them for this reason.

enum NamedEnum {
    Fish,
};

typedef enum NamedEnum AliasOfNamedEnum;