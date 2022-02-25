// bindgen-flags: --dynamic-loading TestLib --dynamic-link-require-all --enable-function-attribute-detection
/**
 * @brief A function
 * 
 * @param x 
 * @param y 
 * @return int 
 */
__attribute__((warn_unused_result))
int foo(int x, int y);
int baz() ;
