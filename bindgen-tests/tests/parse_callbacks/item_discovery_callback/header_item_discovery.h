// Unions
void function_using_anonymous_struct(struct {} arg0);

struct NamedStruct {
};

typedef struct NamedStruct AliasOfNamedStruct;


// Unions
void function_using_anonymous_union(union {} arg0);

union NamedUnion {
};

typedef union NamedUnion AliasOfNamedUnion;