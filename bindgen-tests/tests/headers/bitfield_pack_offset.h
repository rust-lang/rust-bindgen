struct A {
	const unsigned char        name[7];              /*     0     7 */
	unsigned char              firmness:4;           /*     7: 0  1 */
	unsigned char              color:4;              /*     7: 4  1 */
	unsigned short             weedsBonus:3;         /*     8: 0  2 */
	unsigned short             pestsBonus:3;         /*     8: 3  2 */
	unsigned short             size:10;              /*     8: 6  2 */
	unsigned char              maxYield;             /*    10     1 */
	unsigned char              minYield:4;           /*    11: 0  1 */
	unsigned char              waterBonus:4;         /*    11: 4  1 */

	/* XXX 4 bytes hole, try to pack */

	const unsigned char  *     description1;         /*    16     8 */

	/* size: 24, cachelines: 1, members: 10 */
	/* sum members: 16, holes: 1, sum holes: 4 */
	/* sum bitfield members: 32 bits (4 bytes) */
	/* last cacheline: 24 bytes */
};

