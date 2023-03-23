// bindgen-flags: --no-layout-tests

/**
 * This is Struct 
 */
typedef struct {
    /** 
     * This is field
     */
    int field;
} Struct;

/**
 * This is AliasToStruct 
 */
typedef Struct AliasToStruct;

/**
 * This is AliasToInt
 */
typedef int AliasToInt;

/**
 * This is AliasToAliasToInt
 */
typedef AliasToInt AliasToAliasToInt;
