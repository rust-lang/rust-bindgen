/* Minimal stand-in for a platform SDK's objc/objc.h: a typedef that
 * shadows a compiler builtin ObjC type by aliasing its own underlying
 * representation. Included via `-isystem` so clang treats it as a system
 * header, matching how real SDKs provide it. */
typedef struct objc_class *Class;
struct objc_object {
    Class isa;
};
typedef struct objc_object *id;
