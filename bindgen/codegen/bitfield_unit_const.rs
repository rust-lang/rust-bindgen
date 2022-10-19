impl<const N: usize> __BindgenBitfieldUnit<[u8; N]>
{
    #[inline]
    #[must_use]
    pub const fn set_bit_const(mut self, index: usize, val: bool) -> Self {
        debug_assert!(index / 8 < self.storage.len());

        let byte_index = index / 8;

        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;
        if val {
            self.storage[byte_index] |= mask;
        } else {
            self.storage[byte_index] &= !mask;
        }
        
        self
    }

    #[inline]
    #[must_use]
    pub const fn set_const(mut self, bit_offset: usize, bit_width: u8, val: u64) -> Self {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.len()
        );

        let mut i = 0;
        while i < bit_width as usize {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self = self.set_bit_const(index + bit_offset, val_bit_is_set);

            i += 1;
        }
        
        self
    }
}
