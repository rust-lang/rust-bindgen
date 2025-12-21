#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            *(core::ptr::addr_of!((*this).storage) as *const u8)
                .offset(byte_index as isize)
        };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
                .offset(byte_index as isize)
        };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let storage = self.storage.as_ref();
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        if cfg!(target_endian = "big") {
            for i in 0..bytes_needed {
                val |= (storage[start_byte + i].reverse_bits() as u64) << (i * 8);
            }
        } else {
            for i in 0..bytes_needed {
                val |= (storage[start_byte + i] as u64) << (i * 8);
            }
        }
        val >>= bit_shift;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        let storage_ptr = unsafe { core::ptr::addr_of!((*this).storage) as *const u8 };
        if cfg!(target_endian = "big") {
            for i in 0..bytes_needed {
                let byte = unsafe { *storage_ptr.add(start_byte + i) };
                val |= (byte.reverse_bits() as u64) << (i * 8);
            }
        } else {
            for i in 0..bytes_needed {
                let byte = unsafe { *storage_ptr.add(start_byte + i) };
                val |= (byte as u64) << (i * 8);
            }
        }
        val >>= bit_shift;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        if bit_width == 0 {
            return;
        }
        let mut val = val;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        let storage = self.storage.as_mut();
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        val <<= bit_shift;
        let field_mask = if bit_width as usize + bit_shift >= 64 {
            !0u64 << bit_shift
        } else {
            ((1u64 << bit_width) - 1) << bit_shift
        };
        for i in 0..bytes_needed {
            let byte_val = (val >> (i * 8)) as u8;
            let byte_mask = (field_mask >> (i * 8)) as u8;
            if cfg!(target_endian = "big") {
                let byte = storage[start_byte + i].reverse_bits();
                let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                storage[start_byte + i] = new_byte.reverse_bits();
            } else {
                storage[start_byte + i] = (storage[start_byte + i] & !byte_mask)
                    | (byte_val & byte_mask);
            }
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        if bit_width == 0 {
            return;
        }
        let mut val = val;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        val <<= bit_shift;
        let field_mask = if bit_width as usize + bit_shift >= 64 {
            !0u64 << bit_shift
        } else {
            ((1u64 << bit_width) - 1) << bit_shift
        };
        let storage_ptr = unsafe { core::ptr::addr_of_mut!((*this).storage) as *mut u8 };
        for i in 0..bytes_needed {
            let byte_val = (val >> (i * 8)) as u8;
            let byte_mask = (field_mask >> (i * 8)) as u8;
            let byte_ptr = unsafe { storage_ptr.add(start_byte + i) };
            if cfg!(target_endian = "big") {
                let byte = unsafe { (*byte_ptr).reverse_bits() };
                let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                unsafe { *byte_ptr = new_byte.reverse_bits() };
            } else {
                unsafe { *byte_ptr = (*byte_ptr & !byte_mask) | (byte_val & byte_mask) };
            }
        }
    }
}
/// Const-generic methods for efficient bitfield access when offset and width
/// are known at compile time.
impl<const N: usize> __BindgenBitfieldUnit<[u8; N]> {
    /// Get a field using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub const fn get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&self) -> u64 {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return 0;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = 0usize;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    val
                        |= (self.storage[start_byte + i].reverse_bits() as usize)
                            << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i] as usize) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val as u64
        } else {
            let mut val = 0u64;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    val
                        |= (self.storage[start_byte + i].reverse_bits() as u64)
                            << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i] as u64) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val
        }
    }
    /// Set a field using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub fn set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&mut self, val: u64) {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = val as usize;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = ((1usize << BIT_WIDTH) - 1) << bit_shift;
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                if cfg!(target_endian = "big") {
                    let byte = self.storage[start_byte + i].reverse_bits();
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    self.storage[start_byte + i] = new_byte.reverse_bits();
                } else {
                    self.storage[start_byte + i] = (self.storage[start_byte + i]
                        & !byte_mask) | (byte_val & byte_mask);
                }
                i += 1;
            }
        } else {
            let mut val = val;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = if BIT_WIDTH as usize + bit_shift >= 64 {
                !0u64 << bit_shift
            } else {
                ((1u64 << BIT_WIDTH) - 1) << bit_shift
            };
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                if cfg!(target_endian = "big") {
                    let byte = self.storage[start_byte + i].reverse_bits();
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    self.storage[start_byte + i] = new_byte.reverse_bits();
                } else {
                    self.storage[start_byte + i] = (self.storage[start_byte + i]
                        & !byte_mask) | (byte_val & byte_mask);
                }
                i += 1;
            }
        }
    }
    /// Raw pointer get using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub const unsafe fn raw_get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *const Self,
    ) -> u64 {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return 0;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        let storage_ptr = unsafe { core::ptr::addr_of!((*this).storage) as *const u8 };
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = 0usize;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte.reverse_bits() as usize) << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte as usize) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val as u64
        } else {
            let mut val = 0u64;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte.reverse_bits() as u64) << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte as u64) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val
        }
    }
    /// Raw pointer set using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub unsafe fn raw_set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *mut Self,
        val: u64,
    ) {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        let storage_ptr = this.cast::<[u8; N]>().cast::<u8>();
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = val as usize;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = ((1usize << BIT_WIDTH) - 1) << bit_shift;
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                let byte_ptr = unsafe { storage_ptr.add(start_byte + i) };
                if cfg!(target_endian = "big") {
                    let byte = unsafe { (*byte_ptr).reverse_bits() };
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    unsafe { *byte_ptr = new_byte.reverse_bits() };
                } else {
                    unsafe {
                        *byte_ptr = (*byte_ptr & !byte_mask) | (byte_val & byte_mask)
                    };
                }
                i += 1;
            }
        } else {
            let mut val = val;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = if BIT_WIDTH as usize + bit_shift >= 64 {
                !0u64 << bit_shift
            } else {
                ((1u64 << BIT_WIDTH) - 1) << bit_shift
            };
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                let byte_ptr = unsafe { storage_ptr.add(start_byte + i) };
                if cfg!(target_endian = "big") {
                    let byte = unsafe { (*byte_ptr).reverse_bits() };
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    unsafe { *byte_ptr = new_byte.reverse_bits() };
                } else {
                    unsafe {
                        *byte_ptr = (*byte_ptr & !byte_mask) | (byte_val & byte_mask)
                    };
                }
                i += 1;
            }
        }
    }
}
pub const RTE_CACHE_LINE_MIN_SIZE: u32 = 64;
pub const RTE_CACHE_LINE_SIZE: u32 = 64;
pub type phys_addr_t = u64;
pub type MARKER = [*mut ::std::os::raw::c_void; 0usize];
pub type MARKER8 = [u8; 0usize];
pub type MARKER64 = [u64; 0usize];
/// The atomic counter structure.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_atomic16_t {
    ///< An internal counter value.
    pub cnt: i16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_atomic16_t"][::std::mem::size_of::<rte_atomic16_t>() - 2usize];
    ["Alignment of rte_atomic16_t"][::std::mem::align_of::<rte_atomic16_t>() - 2usize];
    [
        "Offset of field: rte_atomic16_t::cnt",
    ][::std::mem::offset_of!(rte_atomic16_t, cnt) - 0usize];
};
/// The generic rte_mbuf, containing a packet mbuf.
#[repr(C)]
#[repr(align(64))]
pub struct rte_mbuf {
    pub cacheline0: MARKER,
    ///< Virtual address of segment buffer.
    pub buf_addr: *mut ::std::os::raw::c_void,
    ///< Physical address of segment buffer.
    pub buf_physaddr: phys_addr_t,
    ///< Length of segment buffer.
    pub buf_len: u16,
    pub rearm_data: MARKER8,
    pub data_off: u16,
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_1,
    ///< Number of segments.
    pub nb_segs: u8,
    ///< Input port.
    pub port: u8,
    ///< Offload features.
    pub ol_flags: u64,
    pub rx_descriptor_fields1: MARKER,
    pub __bindgen_anon_2: rte_mbuf__bindgen_ty_2,
    ///< Total pkt len: sum of all segments.
    pub pkt_len: u32,
    ///< Amount of data in segment buffer.
    pub data_len: u16,
    /// VLAN TCI (CPU order), valid if PKT_RX_VLAN_STRIPPED is set.
    pub vlan_tci: u16,
    ///< hash information
    pub hash: rte_mbuf__bindgen_ty_3,
    ///< Sequence number. See also rte_reorder_insert()
    pub seqn: u32,
    /// Outer VLAN TCI (CPU order), valid if PKT_RX_QINQ_STRIPPED is set.
    pub vlan_tci_outer: u16,
    pub cacheline1: MARKER,
    pub __bindgen_anon_3: rte_mbuf__bindgen_ty_4,
    ///< Pool from which mbuf was allocated.
    pub pool: *mut rte_mempool,
    ///< Next segment of scattered packet.
    pub next: *mut rte_mbuf,
    pub __bindgen_anon_4: rte_mbuf__bindgen_ty_5,
    /** Size of the application private data. In case of an indirect
 mbuf, it stores the direct mbuf private data size.*/
    pub priv_size: u16,
    /// Timesync flags for use with IEEE1588.
    pub timesync: u16,
}
/** 16-bit Reference counter.
 It should only be accessed using the following functions:
 rte_mbuf_refcnt_update(), rte_mbuf_refcnt_read(), and
 rte_mbuf_refcnt_set(). The functionality of these functions (atomic,
 or non-atomic) is controlled by the CONFIG_RTE_MBUF_REFCNT_ATOMIC
 config option.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_1 {
    ///< Atomically accessed refcnt
    pub refcnt_atomic: rte_atomic16_t,
    ///< Non-atomically accessed refcnt
    pub refcnt: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_1",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_1>() - 2usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_1",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_1>() - 2usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_1::refcnt_atomic",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_1, refcnt_atomic) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_1::refcnt",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_1, refcnt) - 0usize];
};
impl Default for rte_mbuf__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_2 {
    ///< L2/L3/L4 and tunnel information.
    pub packet_type: u32,
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    pub _bindgen_align: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_2__bindgen_ty_1",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>() - 4usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_2__bindgen_ty_1",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>() - 4usize];
};
impl rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    #[inline]
    pub fn l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<0usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn l2_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    0usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_l2_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<4usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<4usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn l3_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    4usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_l3_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                4usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<8usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<8usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn l4_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    8usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_l4_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                8usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn tun_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<12usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_tun_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<12usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn tun_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    12usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_tun_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                12usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn inner_l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_inner_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<16usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn inner_l2_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    16usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_inner_l2_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                16usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn inner_l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<20usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_inner_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<20usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn inner_l3_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    20usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_inner_l3_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                20usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn inner_l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 4u8>() as u32)
        }
    }
    #[inline]
    pub fn set_inner_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<24usize, 4u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn inner_l4_type_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    24usize,
                    4u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_inner_l4_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                24usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        l2_type: u32,
        l3_type: u32,
        l4_type: u32,
        tun_type: u32,
        inner_l2_type: u32,
        inner_l3_type: u32,
        inner_l4_type: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                4u8,
            >({
                let l2_type: u32 = l2_type as _;
                l2_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                4usize,
                4u8,
            >({
                let l3_type: u32 = l3_type as _;
                l3_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                4u8,
            >({
                let l4_type: u32 = l4_type as _;
                l4_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                12usize,
                4u8,
            >({
                let tun_type: u32 = tun_type as _;
                tun_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                4u8,
            >({
                let inner_l2_type: u32 = inner_l2_type as _;
                inner_l2_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                20usize,
                4u8,
            >({
                let inner_l3_type: u32 = inner_l3_type as _;
                inner_l3_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                4u8,
            >({
                let inner_l4_type: u32 = inner_l4_type as _;
                inner_l4_type as u64
            });
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_2",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_2>() - 4usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_2",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_2::packet_type",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_2, packet_type) - 0usize];
};
impl Default for rte_mbuf__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_3 {
    ///< RSS hash result if RSS enabled
    pub rss: u32,
    ///< Filter identifier if FDIR enabled
    pub fdir: rte_mbuf__bindgen_ty_3__bindgen_ty_1,
    ///< Hierarchical scheduler
    pub sched: rte_mbuf__bindgen_ty_3__bindgen_ty_2,
    ///< User defined tags. See rte_distributor_process()
    pub usr: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1 {
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
    pub hi: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    pub lo: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub hash: u16,
    pub id: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<
        rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    >() - 4usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<
        rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    >() - 2usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1::hash",
    ][::std::mem::offset_of!(
        rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1, hash
    ) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1::id",
    ][::std::mem::offset_of!(
        rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1, id
    ) - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1>()
        - 4usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1>()
        - 4usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1::lo",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1, lo)
        - 0usize];
};
impl Default for rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_3__bindgen_ty_1",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>() - 8usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_3__bindgen_ty_1",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3__bindgen_ty_1::hi",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3__bindgen_ty_1, hi) - 4usize];
};
impl Default for rte_mbuf__bindgen_ty_3__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_2 {
    pub lo: u32,
    pub hi: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_3__bindgen_ty_2",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>() - 8usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_3__bindgen_ty_2",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3__bindgen_ty_2::lo",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3__bindgen_ty_2, lo) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3__bindgen_ty_2::hi",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3__bindgen_ty_2, hi) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_3",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_3>() - 8usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_3",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_3>() - 4usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3::rss",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3, rss) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3::fdir",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3, fdir) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3::sched",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3, sched) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_3::usr",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_3, usr) - 0usize];
};
impl Default for rte_mbuf__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_4 {
    ///< Can be used for external metadata
    pub userdata: *mut ::std::os::raw::c_void,
    ///< Allow 8-byte userdata on 32-bit
    pub udata64: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_4",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_4>() - 8usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_4",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_4>() - 8usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_4::userdata",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_4, userdata) - 0usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_4::udata64",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_4, udata64) - 0usize];
};
impl Default for rte_mbuf__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_5 {
    ///< combined for easy fetch
    pub tx_offload: u64,
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    pub _bindgen_align: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 7usize]>,
    pub __bindgen_padding_0: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_5__bindgen_ty_1",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>() - 8usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_5__bindgen_ty_1",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>() - 8usize];
};
impl rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    #[inline]
    pub fn l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 7u8>() as u64)
        }
    }
    #[inline]
    pub fn set_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            self._bitfield_1.set_const::<0usize, 7u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn l2_len_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 7usize],
                >>::raw_get_const::<
                    0usize,
                    7u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_l2_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                0usize,
                7u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<7usize, 9u8>() as u64)
        }
    }
    #[inline]
    pub fn set_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            self._bitfield_1.set_const::<7usize, 9u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn l3_len_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 7usize],
                >>::raw_get_const::<
                    7usize,
                    9u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_l3_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                7usize,
                9u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn l4_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 8u8>() as u64)
        }
    }
    #[inline]
    pub fn set_l4_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            self._bitfield_1.set_const::<16usize, 8u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn l4_len_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 7usize],
                >>::raw_get_const::<
                    16usize,
                    8u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_l4_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                16usize,
                8u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn tso_segsz(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 16u8>() as u64)
        }
    }
    #[inline]
    pub fn set_tso_segsz(&mut self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            self._bitfield_1.set_const::<24usize, 16u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn tso_segsz_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 7usize],
                >>::raw_get_const::<
                    24usize,
                    16u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_tso_segsz_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                24usize,
                16u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn outer_l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<40usize, 9u8>() as u64)
        }
    }
    #[inline]
    pub fn set_outer_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            self._bitfield_1.set_const::<40usize, 9u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn outer_l3_len_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 7usize],
                >>::raw_get_const::<
                    40usize,
                    9u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_outer_l3_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                40usize,
                9u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn outer_l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<49usize, 7u8>() as u64)
        }
    }
    #[inline]
    pub fn set_outer_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            self._bitfield_1.set_const::<49usize, 7u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn outer_l2_len_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 7usize],
                >>::raw_get_const::<
                    49usize,
                    7u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_outer_l2_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                49usize,
                7u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        l2_len: u64,
        l3_len: u64,
        l4_len: u64,
        tso_segsz: u64,
        outer_l3_len: u64,
        outer_l2_len: u64,
    ) -> __BindgenBitfieldUnit<[u8; 7usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 7usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                7u8,
            >({
                let l2_len: u64 = l2_len as _;
                l2_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                9u8,
            >({
                let l3_len: u64 = l3_len as _;
                l3_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                8u8,
            >({
                let l4_len: u64 = l4_len as _;
                l4_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                16u8,
            >({
                let tso_segsz: u64 = tso_segsz as _;
                tso_segsz as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                40usize,
                9u8,
            >({
                let outer_l3_len: u64 = outer_l3_len as _;
                outer_l3_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                49usize,
                7u8,
            >({
                let outer_l2_len: u64 = outer_l2_len as _;
                outer_l2_len as u64
            });
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_5",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_5>() - 8usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_5",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_5>() - 8usize];
    [
        "Offset of field: rte_mbuf__bindgen_ty_5::tx_offload",
    ][::std::mem::offset_of!(rte_mbuf__bindgen_ty_5, tx_offload) - 0usize];
};
impl Default for rte_mbuf__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_mbuf"][::std::mem::size_of::<rte_mbuf>() - 128usize];
    ["Alignment of rte_mbuf"][::std::mem::align_of::<rte_mbuf>() - 64usize];
    [
        "Offset of field: rte_mbuf::cacheline0",
    ][::std::mem::offset_of!(rte_mbuf, cacheline0) - 0usize];
    [
        "Offset of field: rte_mbuf::buf_addr",
    ][::std::mem::offset_of!(rte_mbuf, buf_addr) - 0usize];
    [
        "Offset of field: rte_mbuf::buf_physaddr",
    ][::std::mem::offset_of!(rte_mbuf, buf_physaddr) - 8usize];
    [
        "Offset of field: rte_mbuf::buf_len",
    ][::std::mem::offset_of!(rte_mbuf, buf_len) - 16usize];
    [
        "Offset of field: rte_mbuf::rearm_data",
    ][::std::mem::offset_of!(rte_mbuf, rearm_data) - 18usize];
    [
        "Offset of field: rte_mbuf::data_off",
    ][::std::mem::offset_of!(rte_mbuf, data_off) - 18usize];
    [
        "Offset of field: rte_mbuf::nb_segs",
    ][::std::mem::offset_of!(rte_mbuf, nb_segs) - 22usize];
    [
        "Offset of field: rte_mbuf::port",
    ][::std::mem::offset_of!(rte_mbuf, port) - 23usize];
    [
        "Offset of field: rte_mbuf::ol_flags",
    ][::std::mem::offset_of!(rte_mbuf, ol_flags) - 24usize];
    [
        "Offset of field: rte_mbuf::rx_descriptor_fields1",
    ][::std::mem::offset_of!(rte_mbuf, rx_descriptor_fields1) - 32usize];
    [
        "Offset of field: rte_mbuf::pkt_len",
    ][::std::mem::offset_of!(rte_mbuf, pkt_len) - 36usize];
    [
        "Offset of field: rte_mbuf::data_len",
    ][::std::mem::offset_of!(rte_mbuf, data_len) - 40usize];
    [
        "Offset of field: rte_mbuf::vlan_tci",
    ][::std::mem::offset_of!(rte_mbuf, vlan_tci) - 42usize];
    [
        "Offset of field: rte_mbuf::hash",
    ][::std::mem::offset_of!(rte_mbuf, hash) - 44usize];
    [
        "Offset of field: rte_mbuf::seqn",
    ][::std::mem::offset_of!(rte_mbuf, seqn) - 52usize];
    [
        "Offset of field: rte_mbuf::vlan_tci_outer",
    ][::std::mem::offset_of!(rte_mbuf, vlan_tci_outer) - 56usize];
    [
        "Offset of field: rte_mbuf::cacheline1",
    ][::std::mem::offset_of!(rte_mbuf, cacheline1) - 64usize];
    [
        "Offset of field: rte_mbuf::pool",
    ][::std::mem::offset_of!(rte_mbuf, pool) - 72usize];
    [
        "Offset of field: rte_mbuf::next",
    ][::std::mem::offset_of!(rte_mbuf, next) - 80usize];
    [
        "Offset of field: rte_mbuf::priv_size",
    ][::std::mem::offset_of!(rte_mbuf, priv_size) - 96usize];
    [
        "Offset of field: rte_mbuf::timesync",
    ][::std::mem::offset_of!(rte_mbuf, timesync) - 98usize];
};
impl Default for rte_mbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
///< Pool from which mbuf was allocated.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mempool {
    pub _address: u8,
}
