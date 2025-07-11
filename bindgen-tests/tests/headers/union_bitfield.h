// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq

union U4 {
	unsigned int               derp:1;             /*     0: 0  4 */
};

union B {
	unsigned int               foo:31;             /*     0: 0  4 */
	unsigned char              bar:1;              /*     0: 0  1 */
};
