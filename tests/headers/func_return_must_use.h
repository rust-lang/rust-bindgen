// bindgen-flags: --must-use-type 'MustUse.*'

typedef int MustUseInt;

MustUseInt return_int();

struct MustUseStruct;

struct MustUseStruct return_struct();

/**
 * <div rustbindgen mustusetype></div>
 */
typedef int AnnotatedInt;

AnnotatedInt return_annotated_int();

int return_plain_int();

/**
 * <div rustbindgen mustusetype></div>
 */
struct AnnotatedStruct {};

struct AnnotatedStruct return_annotated_struct();

struct PlainStruct {};

/**
 * <div rustbindgen mustusetype></div>
 */
typedef struct PlainStruct TypedefPlainStruct;

struct PlainStruct return_plain_struct();

TypedefPlainStruct return_typedef_struct();
