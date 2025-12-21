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
pub const ETH_MQ_RX_RSS_FLAG: u32 = 1;
pub const ETH_MQ_RX_DCB_FLAG: u32 = 2;
pub const ETH_MQ_RX_VMDQ_FLAG: u32 = 4;
pub const ETH_VMDQ_MAX_VLAN_FILTERS: u32 = 64;
pub const ETH_DCB_NUM_USER_PRIORITIES: u32 = 8;
pub const ETH_VMDQ_DCB_NUM_QUEUES: u32 = 128;
pub const ETH_DCB_NUM_QUEUES: u32 = 128;
pub const RTE_ETH_FDIR_MAX_FLEXLEN: u32 = 16;
pub const RTE_ETH_INSET_SIZE_MAX: u32 = 128;
pub const RTE_ETH_FLOW_UNKNOWN: u32 = 0;
pub const RTE_ETH_FLOW_RAW: u32 = 1;
pub const RTE_ETH_FLOW_IPV4: u32 = 2;
pub const RTE_ETH_FLOW_FRAG_IPV4: u32 = 3;
pub const RTE_ETH_FLOW_NONFRAG_IPV4_TCP: u32 = 4;
pub const RTE_ETH_FLOW_NONFRAG_IPV4_UDP: u32 = 5;
pub const RTE_ETH_FLOW_NONFRAG_IPV4_SCTP: u32 = 6;
pub const RTE_ETH_FLOW_NONFRAG_IPV4_OTHER: u32 = 7;
pub const RTE_ETH_FLOW_IPV6: u32 = 8;
pub const RTE_ETH_FLOW_FRAG_IPV6: u32 = 9;
pub const RTE_ETH_FLOW_NONFRAG_IPV6_TCP: u32 = 10;
pub const RTE_ETH_FLOW_NONFRAG_IPV6_UDP: u32 = 11;
pub const RTE_ETH_FLOW_NONFRAG_IPV6_SCTP: u32 = 12;
pub const RTE_ETH_FLOW_NONFRAG_IPV6_OTHER: u32 = 13;
pub const RTE_ETH_FLOW_L2_PAYLOAD: u32 = 14;
pub const RTE_ETH_FLOW_IPV6_EX: u32 = 15;
pub const RTE_ETH_FLOW_IPV6_TCP_EX: u32 = 16;
pub const RTE_ETH_FLOW_IPV6_UDP_EX: u32 = 17;
pub const RTE_ETH_FLOW_PORT: u32 = 18;
pub const RTE_ETH_FLOW_VXLAN: u32 = 19;
pub const RTE_ETH_FLOW_GENEVE: u32 = 20;
pub const RTE_ETH_FLOW_NVGRE: u32 = 21;
pub const RTE_ETH_FLOW_MAX: u32 = 22;
#[repr(u32)]
/**  A set of values to identify what method is to be used to route
  packets to multiple queues.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_eth_rx_mq_mode {
    /// None of DCB,RSS or VMDQ mode
    ETH_MQ_RX_NONE = 0,
    /// For RX side, only RSS is on
    ETH_MQ_RX_RSS = 1,
    /// For RX side,only DCB is on.
    ETH_MQ_RX_DCB = 2,
    /// Both DCB and RSS enable
    ETH_MQ_RX_DCB_RSS = 3,
    /// Only VMDQ, no RSS nor DCB
    ETH_MQ_RX_VMDQ_ONLY = 4,
    /// RSS mode with VMDQ
    ETH_MQ_RX_VMDQ_RSS = 5,
    /// Use VMDQ+DCB to route traffic to queues
    ETH_MQ_RX_VMDQ_DCB = 6,
    /// Enable both VMDQ and DCB in VMDq
    ETH_MQ_RX_VMDQ_DCB_RSS = 7,
}
/// A structure used to configure the RX features of an Ethernet port.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_rxmode {
    /// The multi-queue packet distribution mode to be used, e.g. RSS.
    pub mq_mode: rte_eth_rx_mq_mode,
    ///< Only used if jumbo_frame enabled.
    pub max_rx_pkt_len: u32,
    ///< hdr buf size (header_split enabled).
    pub split_hdr_size: u16,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_eth_rxmode"][::std::mem::size_of::<rte_eth_rxmode>() - 12usize];
    ["Alignment of rte_eth_rxmode"][::std::mem::align_of::<rte_eth_rxmode>() - 4usize];
    [
        "Offset of field: rte_eth_rxmode::mq_mode",
    ][::std::mem::offset_of!(rte_eth_rxmode, mq_mode) - 0usize];
    [
        "Offset of field: rte_eth_rxmode::max_rx_pkt_len",
    ][::std::mem::offset_of!(rte_eth_rxmode, max_rx_pkt_len) - 4usize];
    [
        "Offset of field: rte_eth_rxmode::split_hdr_size",
    ][::std::mem::offset_of!(rte_eth_rxmode, split_hdr_size) - 8usize];
};
impl Default for rte_eth_rxmode {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl rte_eth_rxmode {
    #[inline]
    pub fn header_split(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_header_split(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<0usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn header_split_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    0usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_header_split_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                0usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_ip_checksum(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<1usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_hw_ip_checksum(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<1usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_ip_checksum_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    1usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_ip_checksum_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                1usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_filter(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<2usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_hw_vlan_filter(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<2usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_vlan_filter_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    2usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_vlan_filter_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                2usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_strip(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_hw_vlan_strip(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<3usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_vlan_strip_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    3usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_vlan_strip_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                3usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_extend(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<4usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_hw_vlan_extend(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<4usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_vlan_extend_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    4usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_vlan_extend_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                4usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn jumbo_frame(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<5usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_jumbo_frame(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<5usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn jumbo_frame_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    5usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_jumbo_frame_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                5usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_strip_crc(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<6usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_hw_strip_crc(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<6usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_strip_crc_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    6usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_strip_crc_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                6usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn enable_scatter(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<7usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_enable_scatter(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<7usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn enable_scatter_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    7usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_enable_scatter_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                7usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn enable_lro(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<8usize, 1u8>() as u16)
        }
    }
    #[inline]
    pub fn set_enable_lro(&mut self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            self._bitfield_1.set_const::<8usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn enable_lro_raw(this: *const Self) -> u16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    8usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_enable_lro_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                8usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        header_split: u16,
        hw_ip_checksum: u16,
        hw_vlan_filter: u16,
        hw_vlan_strip: u16,
        hw_vlan_extend: u16,
        jumbo_frame: u16,
        hw_strip_crc: u16,
        enable_scatter: u16,
        enable_lro: u16,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                1u8,
            >({
                let header_split: u16 = header_split as _;
                header_split as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                1usize,
                1u8,
            >({
                let hw_ip_checksum: u16 = hw_ip_checksum as _;
                hw_ip_checksum as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                2usize,
                1u8,
            >({
                let hw_vlan_filter: u16 = hw_vlan_filter as _;
                hw_vlan_filter as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                1u8,
            >({
                let hw_vlan_strip: u16 = hw_vlan_strip as _;
                hw_vlan_strip as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                4usize,
                1u8,
            >({
                let hw_vlan_extend: u16 = hw_vlan_extend as _;
                hw_vlan_extend as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                5usize,
                1u8,
            >({
                let jumbo_frame: u16 = jumbo_frame as _;
                jumbo_frame as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                6usize,
                1u8,
            >({
                let hw_strip_crc: u16 = hw_strip_crc as _;
                hw_strip_crc as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                1u8,
            >({
                let enable_scatter: u16 = enable_scatter as _;
                enable_scatter as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                1u8,
            >({
                let enable_lro: u16 = enable_lro as _;
                enable_lro as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(u32)]
/** A set of values to identify what method is to be used to transmit
 packets using multi-TCs.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_eth_tx_mq_mode {
    ///< It is in neither DCB nor VT mode.
    ETH_MQ_TX_NONE = 0,
    ///< For TX side,only DCB is on.
    ETH_MQ_TX_DCB = 1,
    ///< For TX side,both DCB and VT is on.
    ETH_MQ_TX_VMDQ_DCB = 2,
    ///< Only VT on, no DCB
    ETH_MQ_TX_VMDQ_ONLY = 3,
}
/// A structure used to configure the TX features of an Ethernet port.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_txmode {
    ///< TX multi-queues mode.
    pub mq_mode: rte_eth_tx_mq_mode,
    pub pvid: u16,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_eth_txmode"][::std::mem::size_of::<rte_eth_txmode>() - 8usize];
    ["Alignment of rte_eth_txmode"][::std::mem::align_of::<rte_eth_txmode>() - 4usize];
    [
        "Offset of field: rte_eth_txmode::mq_mode",
    ][::std::mem::offset_of!(rte_eth_txmode, mq_mode) - 0usize];
    [
        "Offset of field: rte_eth_txmode::pvid",
    ][::std::mem::offset_of!(rte_eth_txmode, pvid) - 4usize];
};
impl Default for rte_eth_txmode {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl rte_eth_txmode {
    #[inline]
    pub fn hw_vlan_reject_tagged(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_hw_vlan_reject_tagged(&mut self, val: u8) {
        unsafe {
            let val: u8 = val as _;
            self._bitfield_1.set_const::<0usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_vlan_reject_tagged_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get_const::<
                    0usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_vlan_reject_tagged_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set_const::<
                0usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_reject_untagged(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<1usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_hw_vlan_reject_untagged(&mut self, val: u8) {
        unsafe {
            let val: u8 = val as _;
            self._bitfield_1.set_const::<1usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_vlan_reject_untagged_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get_const::<
                    1usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_vlan_reject_untagged_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set_const::<
                1usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_insert_pvid(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<2usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_hw_vlan_insert_pvid(&mut self, val: u8) {
        unsafe {
            let val: u8 = val as _;
            self._bitfield_1.set_const::<2usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn hw_vlan_insert_pvid_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get_const::<
                    2usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_hw_vlan_insert_pvid_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set_const::<
                2usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        hw_vlan_reject_tagged: u8,
        hw_vlan_reject_untagged: u8,
        hw_vlan_insert_pvid: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                1u8,
            >({
                let hw_vlan_reject_tagged: u8 = hw_vlan_reject_tagged as _;
                hw_vlan_reject_tagged as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                1usize,
                1u8,
            >({
                let hw_vlan_reject_untagged: u8 = hw_vlan_reject_untagged as _;
                hw_vlan_reject_untagged as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                2usize,
                1u8,
            >({
                let hw_vlan_insert_pvid: u8 = hw_vlan_insert_pvid as _;
                hw_vlan_insert_pvid as u64
            });
        __bindgen_bitfield_unit
    }
}
/** A structure used to configure the Receive Side Scaling (RSS) feature
 of an Ethernet port.
 If not NULL, the *rss_key* pointer of the *rss_conf* structure points
 to an array holding the RSS key to use for hashing specific header
 fields of received packets. The length of this array should be indicated
 by *rss_key_len* below. Otherwise, a default random hash key is used by
 the device driver.

 The *rss_key_len* field of the *rss_conf* structure indicates the length
 in bytes of the array pointed by *rss_key*. To be compatible, this length
 will be checked in i40e only. Others assume 40 bytes to be used as before.

 The *rss_hf* field of the *rss_conf* structure indicates the different
 types of IPv4/IPv6 packets to which the RSS hashing must be applied.
 Supplying an *rss_hf* equal to zero disables the RSS feature.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_rss_conf {
    ///< If not NULL, 40-byte hash key.
    pub rss_key: *mut u8,
    ///< hash key length in bytes.
    pub rss_key_len: u8,
    ///< Hash functions to apply - see below.
    pub rss_hf: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_eth_rss_conf"][::std::mem::size_of::<rte_eth_rss_conf>() - 24usize];
    [
        "Alignment of rte_eth_rss_conf",
    ][::std::mem::align_of::<rte_eth_rss_conf>() - 8usize];
    [
        "Offset of field: rte_eth_rss_conf::rss_key",
    ][::std::mem::offset_of!(rte_eth_rss_conf, rss_key) - 0usize];
    [
        "Offset of field: rte_eth_rss_conf::rss_key_len",
    ][::std::mem::offset_of!(rte_eth_rss_conf, rss_key_len) - 8usize];
    [
        "Offset of field: rte_eth_rss_conf::rss_hf",
    ][::std::mem::offset_of!(rte_eth_rss_conf, rss_hf) - 16usize];
};
impl Default for rte_eth_rss_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(u32)]
/** This enum indicates the possible number of traffic classes
 in DCB configratioins*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_eth_nb_tcs {
    ///< 4 TCs with DCB.
    ETH_4_TCS = 4,
    ///< 8 TCs with DCB.
    ETH_8_TCS = 8,
}
#[repr(u32)]
/** This enum indicates the possible number of queue pools
 in VMDQ configurations.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_eth_nb_pools {
    ///< 8 VMDq pools.
    ETH_8_POOLS = 8,
    ///< 16 VMDq pools.
    ETH_16_POOLS = 16,
    ///< 32 VMDq pools.
    ETH_32_POOLS = 32,
    ///< 64 VMDq pools.
    ETH_64_POOLS = 64,
}
/** A structure used to configure the VMDQ+DCB feature
 of an Ethernet port.

 Using this feature, packets are routed to a pool of queues, based
 on the vlan ID in the vlan tag, and then to a specific queue within
 that pool, using the user priority vlan tag field.

 A default pool may be used, if desired, to route all traffic which
 does not match the vlan filter rules.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_dcb_conf {
    ///< With DCB, 16 or 32 pools
    pub nb_queue_pools: rte_eth_nb_pools,
    ///< If non-zero, use a default pool
    pub enable_default_pool: u8,
    ///< The default pool, if applicable
    pub default_pool: u8,
    ///< We can have up to 64 filters/mappings
    pub nb_pool_maps: u8,
    ///< VMDq vlan pool maps.
    pub pool_map: [rte_eth_vmdq_dcb_conf__bindgen_ty_1; 64usize],
    pub dcb_tc: [u8; 8usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_dcb_conf__bindgen_ty_1 {
    ///< The vlan ID of the received frame
    pub vlan_id: u16,
    ///< Bitmask of pools for packet rx
    pub pools: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_vmdq_dcb_conf__bindgen_ty_1",
    ][::std::mem::size_of::<rte_eth_vmdq_dcb_conf__bindgen_ty_1>() - 16usize];
    [
        "Alignment of rte_eth_vmdq_dcb_conf__bindgen_ty_1",
    ][::std::mem::align_of::<rte_eth_vmdq_dcb_conf__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf__bindgen_ty_1::vlan_id",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf__bindgen_ty_1, vlan_id) - 0usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf__bindgen_ty_1::pools",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf__bindgen_ty_1, pools) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_vmdq_dcb_conf",
    ][::std::mem::size_of::<rte_eth_vmdq_dcb_conf>() - 1040usize];
    [
        "Alignment of rte_eth_vmdq_dcb_conf",
    ][::std::mem::align_of::<rte_eth_vmdq_dcb_conf>() - 8usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf::nb_queue_pools",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf, nb_queue_pools) - 0usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf::enable_default_pool",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf, enable_default_pool) - 4usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf::default_pool",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf, default_pool) - 5usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf::nb_pool_maps",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf, nb_pool_maps) - 6usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf::pool_map",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf, pool_map) - 8usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_conf::dcb_tc",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_conf, dcb_tc) - 1032usize];
};
impl Default for rte_eth_vmdq_dcb_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_dcb_rx_conf {
    ///< Possible DCB TCs, 4 or 8 TCs
    pub nb_tcs: rte_eth_nb_tcs,
    /// Traffic class each UP mapped to.
    pub dcb_tc: [u8; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_dcb_rx_conf",
    ][::std::mem::size_of::<rte_eth_dcb_rx_conf>() - 12usize];
    [
        "Alignment of rte_eth_dcb_rx_conf",
    ][::std::mem::align_of::<rte_eth_dcb_rx_conf>() - 4usize];
    [
        "Offset of field: rte_eth_dcb_rx_conf::nb_tcs",
    ][::std::mem::offset_of!(rte_eth_dcb_rx_conf, nb_tcs) - 0usize];
    [
        "Offset of field: rte_eth_dcb_rx_conf::dcb_tc",
    ][::std::mem::offset_of!(rte_eth_dcb_rx_conf, dcb_tc) - 4usize];
};
impl Default for rte_eth_dcb_rx_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_dcb_tx_conf {
    ///< With DCB, 16 or 32 pools.
    pub nb_queue_pools: rte_eth_nb_pools,
    /// Traffic class each UP mapped to.
    pub dcb_tc: [u8; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_vmdq_dcb_tx_conf",
    ][::std::mem::size_of::<rte_eth_vmdq_dcb_tx_conf>() - 12usize];
    [
        "Alignment of rte_eth_vmdq_dcb_tx_conf",
    ][::std::mem::align_of::<rte_eth_vmdq_dcb_tx_conf>() - 4usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_tx_conf::nb_queue_pools",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_tx_conf, nb_queue_pools) - 0usize];
    [
        "Offset of field: rte_eth_vmdq_dcb_tx_conf::dcb_tc",
    ][::std::mem::offset_of!(rte_eth_vmdq_dcb_tx_conf, dcb_tc) - 4usize];
};
impl Default for rte_eth_vmdq_dcb_tx_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_dcb_tx_conf {
    ///< Possible DCB TCs, 4 or 8 TCs.
    pub nb_tcs: rte_eth_nb_tcs,
    /// Traffic class each UP mapped to.
    pub dcb_tc: [u8; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_dcb_tx_conf",
    ][::std::mem::size_of::<rte_eth_dcb_tx_conf>() - 12usize];
    [
        "Alignment of rte_eth_dcb_tx_conf",
    ][::std::mem::align_of::<rte_eth_dcb_tx_conf>() - 4usize];
    [
        "Offset of field: rte_eth_dcb_tx_conf::nb_tcs",
    ][::std::mem::offset_of!(rte_eth_dcb_tx_conf, nb_tcs) - 0usize];
    [
        "Offset of field: rte_eth_dcb_tx_conf::dcb_tc",
    ][::std::mem::offset_of!(rte_eth_dcb_tx_conf, dcb_tc) - 4usize];
};
impl Default for rte_eth_dcb_tx_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_tx_conf {
    ///< VMDq mode, 64 pools.
    pub nb_queue_pools: rte_eth_nb_pools,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_vmdq_tx_conf",
    ][::std::mem::size_of::<rte_eth_vmdq_tx_conf>() - 4usize];
    [
        "Alignment of rte_eth_vmdq_tx_conf",
    ][::std::mem::align_of::<rte_eth_vmdq_tx_conf>() - 4usize];
    [
        "Offset of field: rte_eth_vmdq_tx_conf::nb_queue_pools",
    ][::std::mem::offset_of!(rte_eth_vmdq_tx_conf, nb_queue_pools) - 0usize];
};
impl Default for rte_eth_vmdq_tx_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_rx_conf {
    ///< VMDq only mode, 8 or 64 pools
    pub nb_queue_pools: rte_eth_nb_pools,
    ///< If non-zero, use a default pool
    pub enable_default_pool: u8,
    ///< The default pool, if applicable
    pub default_pool: u8,
    ///< Enable VT loop back
    pub enable_loop_back: u8,
    ///< We can have up to 64 filters/mappings
    pub nb_pool_maps: u8,
    ///< Flags from ETH_VMDQ_ACCEPT_*
    pub rx_mode: u32,
    ///< VMDq vlan pool maps.
    pub pool_map: [rte_eth_vmdq_rx_conf__bindgen_ty_1; 64usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_rx_conf__bindgen_ty_1 {
    ///< The vlan ID of the received frame
    pub vlan_id: u16,
    ///< Bitmask of pools for packet rx
    pub pools: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_vmdq_rx_conf__bindgen_ty_1",
    ][::std::mem::size_of::<rte_eth_vmdq_rx_conf__bindgen_ty_1>() - 16usize];
    [
        "Alignment of rte_eth_vmdq_rx_conf__bindgen_ty_1",
    ][::std::mem::align_of::<rte_eth_vmdq_rx_conf__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf__bindgen_ty_1::vlan_id",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf__bindgen_ty_1, vlan_id) - 0usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf__bindgen_ty_1::pools",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf__bindgen_ty_1, pools) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_vmdq_rx_conf",
    ][::std::mem::size_of::<rte_eth_vmdq_rx_conf>() - 1040usize];
    [
        "Alignment of rte_eth_vmdq_rx_conf",
    ][::std::mem::align_of::<rte_eth_vmdq_rx_conf>() - 8usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::nb_queue_pools",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, nb_queue_pools) - 0usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::enable_default_pool",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, enable_default_pool) - 4usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::default_pool",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, default_pool) - 5usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::enable_loop_back",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, enable_loop_back) - 6usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::nb_pool_maps",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, nb_pool_maps) - 7usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::rx_mode",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, rx_mode) - 8usize];
    [
        "Offset of field: rte_eth_vmdq_rx_conf::pool_map",
    ][::std::mem::offset_of!(rte_eth_vmdq_rx_conf, pool_map) - 16usize];
};
impl Default for rte_eth_vmdq_rx_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(u32)]
///  Flow Director setting modes: none, signature or perfect.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_fdir_mode {
    ///< Disable FDIR support.
    RTE_FDIR_MODE_NONE = 0,
    ///< Enable FDIR signature filter mode.
    RTE_FDIR_MODE_SIGNATURE = 1,
    ///< Enable FDIR perfect filter mode.
    RTE_FDIR_MODE_PERFECT = 2,
    ///< Enable FDIR filter mode - MAC VLAN.
    RTE_FDIR_MODE_PERFECT_MAC_VLAN = 3,
    ///< Enable FDIR filter mode - tunnel.
    RTE_FDIR_MODE_PERFECT_TUNNEL = 4,
}
#[repr(u32)]
/**  Memory space that can be configured to store Flow Director filters
  in the board memory.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_fdir_pballoc_type {
    ///< 64k.
    RTE_FDIR_PBALLOC_64K = 0,
    ///< 128k.
    RTE_FDIR_PBALLOC_128K = 1,
    ///< 256k.
    RTE_FDIR_PBALLOC_256K = 2,
}
#[repr(u32)]
///  Select report mode of FDIR hash information in RX descriptors.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_fdir_status_mode {
    ///< Never report FDIR hash.
    RTE_FDIR_NO_REPORT_STATUS = 0,
    ///< Only report FDIR hash for matching pkts.
    RTE_FDIR_REPORT_STATUS = 1,
    ///< Always report FDIR hash.
    RTE_FDIR_REPORT_STATUS_ALWAYS = 2,
}
/// A structure used to define the input for IPV4 flow
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_ipv4_flow {
    ///< IPv4 source address in big endian.
    pub src_ip: u32,
    ///< IPv4 destination address in big endian.
    pub dst_ip: u32,
    ///< Type of service to match.
    pub tos: u8,
    ///< Time to live to match.
    pub ttl: u8,
    ///< Protocol, next header in big endian.
    pub proto: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_eth_ipv4_flow"][::std::mem::size_of::<rte_eth_ipv4_flow>() - 12usize];
    [
        "Alignment of rte_eth_ipv4_flow",
    ][::std::mem::align_of::<rte_eth_ipv4_flow>() - 4usize];
    [
        "Offset of field: rte_eth_ipv4_flow::src_ip",
    ][::std::mem::offset_of!(rte_eth_ipv4_flow, src_ip) - 0usize];
    [
        "Offset of field: rte_eth_ipv4_flow::dst_ip",
    ][::std::mem::offset_of!(rte_eth_ipv4_flow, dst_ip) - 4usize];
    [
        "Offset of field: rte_eth_ipv4_flow::tos",
    ][::std::mem::offset_of!(rte_eth_ipv4_flow, tos) - 8usize];
    [
        "Offset of field: rte_eth_ipv4_flow::ttl",
    ][::std::mem::offset_of!(rte_eth_ipv4_flow, ttl) - 9usize];
    [
        "Offset of field: rte_eth_ipv4_flow::proto",
    ][::std::mem::offset_of!(rte_eth_ipv4_flow, proto) - 10usize];
};
/// A structure used to define the input for IPV6 flow
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_ipv6_flow {
    ///< IPv6 source address in big endian.
    pub src_ip: [u32; 4usize],
    ///< IPv6 destination address in big endian.
    pub dst_ip: [u32; 4usize],
    ///< Traffic class to match.
    pub tc: u8,
    ///< Protocol, next header to match.
    pub proto: u8,
    ///< Hop limits to match.
    pub hop_limits: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_eth_ipv6_flow"][::std::mem::size_of::<rte_eth_ipv6_flow>() - 36usize];
    [
        "Alignment of rte_eth_ipv6_flow",
    ][::std::mem::align_of::<rte_eth_ipv6_flow>() - 4usize];
    [
        "Offset of field: rte_eth_ipv6_flow::src_ip",
    ][::std::mem::offset_of!(rte_eth_ipv6_flow, src_ip) - 0usize];
    [
        "Offset of field: rte_eth_ipv6_flow::dst_ip",
    ][::std::mem::offset_of!(rte_eth_ipv6_flow, dst_ip) - 16usize];
    [
        "Offset of field: rte_eth_ipv6_flow::tc",
    ][::std::mem::offset_of!(rte_eth_ipv6_flow, tc) - 32usize];
    [
        "Offset of field: rte_eth_ipv6_flow::proto",
    ][::std::mem::offset_of!(rte_eth_ipv6_flow, proto) - 33usize];
    [
        "Offset of field: rte_eth_ipv6_flow::hop_limits",
    ][::std::mem::offset_of!(rte_eth_ipv6_flow, hop_limits) - 34usize];
};
/**  A structure used to configure FDIR masks that are used by the device
  to match the various fields of RX packet headers.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_fdir_masks {
    ///< Bit mask for vlan_tci in big endian
    pub vlan_tci_mask: u16,
    /// Bit mask for ipv4 flow in big endian.
    pub ipv4_mask: rte_eth_ipv4_flow,
    /// Bit maks for ipv6 flow in big endian.
    pub ipv6_mask: rte_eth_ipv6_flow,
    /// Bit mask for L4 source port in big endian.
    pub src_port_mask: u16,
    /// Bit mask for L4 destination port in big endian.
    pub dst_port_mask: u16,
    /** 6 bit mask for proper 6 bytes of Mac address, bit 0 matches the
first byte on the wire*/
    pub mac_addr_byte_mask: u8,
    /// Bit mask for tunnel ID in big endian.
    pub tunnel_id_mask: u32,
    /**< 1 - Match tunnel type,
0 - Ignore tunnel type.*/
    pub tunnel_type_mask: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_fdir_masks",
    ][::std::mem::size_of::<rte_eth_fdir_masks>() - 68usize];
    [
        "Alignment of rte_eth_fdir_masks",
    ][::std::mem::align_of::<rte_eth_fdir_masks>() - 4usize];
    [
        "Offset of field: rte_eth_fdir_masks::vlan_tci_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, vlan_tci_mask) - 0usize];
    [
        "Offset of field: rte_eth_fdir_masks::ipv4_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, ipv4_mask) - 4usize];
    [
        "Offset of field: rte_eth_fdir_masks::ipv6_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, ipv6_mask) - 16usize];
    [
        "Offset of field: rte_eth_fdir_masks::src_port_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, src_port_mask) - 52usize];
    [
        "Offset of field: rte_eth_fdir_masks::dst_port_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, dst_port_mask) - 54usize];
    [
        "Offset of field: rte_eth_fdir_masks::mac_addr_byte_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, mac_addr_byte_mask) - 56usize];
    [
        "Offset of field: rte_eth_fdir_masks::tunnel_id_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, tunnel_id_mask) - 60usize];
    [
        "Offset of field: rte_eth_fdir_masks::tunnel_type_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_masks, tunnel_type_mask) - 64usize];
};
#[repr(u32)]
/// Payload type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_eth_payload_type {
    RTE_ETH_PAYLOAD_UNKNOWN = 0,
    RTE_ETH_RAW_PAYLOAD = 1,
    RTE_ETH_L2_PAYLOAD = 2,
    RTE_ETH_L3_PAYLOAD = 3,
    RTE_ETH_L4_PAYLOAD = 4,
    RTE_ETH_PAYLOAD_MAX = 8,
}
/** A structure used to select bytes extracted from the protocol layers to
 flexible payload for filter*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_flex_payload_cfg {
    ///< Payload type
    pub type_: rte_eth_payload_type,
    pub src_offset: [u16; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_flex_payload_cfg",
    ][::std::mem::size_of::<rte_eth_flex_payload_cfg>() - 36usize];
    [
        "Alignment of rte_eth_flex_payload_cfg",
    ][::std::mem::align_of::<rte_eth_flex_payload_cfg>() - 4usize];
    [
        "Offset of field: rte_eth_flex_payload_cfg::type_",
    ][::std::mem::offset_of!(rte_eth_flex_payload_cfg, type_) - 0usize];
    [
        "Offset of field: rte_eth_flex_payload_cfg::src_offset",
    ][::std::mem::offset_of!(rte_eth_flex_payload_cfg, src_offset) - 4usize];
};
impl Default for rte_eth_flex_payload_cfg {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** A structure used to define FDIR masks for flexible payload
 for each flow type*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_fdir_flex_mask {
    pub flow_type: u16,
    pub mask: [u8; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_fdir_flex_mask",
    ][::std::mem::size_of::<rte_eth_fdir_flex_mask>() - 18usize];
    [
        "Alignment of rte_eth_fdir_flex_mask",
    ][::std::mem::align_of::<rte_eth_fdir_flex_mask>() - 2usize];
    [
        "Offset of field: rte_eth_fdir_flex_mask::flow_type",
    ][::std::mem::offset_of!(rte_eth_fdir_flex_mask, flow_type) - 0usize];
    [
        "Offset of field: rte_eth_fdir_flex_mask::mask",
    ][::std::mem::offset_of!(rte_eth_fdir_flex_mask, mask) - 2usize];
};
/** A structure used to define all flexible payload related setting
 include flex payload and flex mask*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_fdir_flex_conf {
    ///< The number of following payload cfg
    pub nb_payloads: u16,
    ///< The number of following mask
    pub nb_flexmasks: u16,
    pub flex_set: [rte_eth_flex_payload_cfg; 8usize],
    pub flex_mask: [rte_eth_fdir_flex_mask; 22usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_fdir_flex_conf",
    ][::std::mem::size_of::<rte_eth_fdir_flex_conf>() - 688usize];
    [
        "Alignment of rte_eth_fdir_flex_conf",
    ][::std::mem::align_of::<rte_eth_fdir_flex_conf>() - 4usize];
    [
        "Offset of field: rte_eth_fdir_flex_conf::nb_payloads",
    ][::std::mem::offset_of!(rte_eth_fdir_flex_conf, nb_payloads) - 0usize];
    [
        "Offset of field: rte_eth_fdir_flex_conf::nb_flexmasks",
    ][::std::mem::offset_of!(rte_eth_fdir_flex_conf, nb_flexmasks) - 2usize];
    [
        "Offset of field: rte_eth_fdir_flex_conf::flex_set",
    ][::std::mem::offset_of!(rte_eth_fdir_flex_conf, flex_set) - 4usize];
    [
        "Offset of field: rte_eth_fdir_flex_conf::flex_mask",
    ][::std::mem::offset_of!(rte_eth_fdir_flex_conf, flex_mask) - 292usize];
};
impl Default for rte_eth_fdir_flex_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** A structure used to configure the Flow Director (FDIR) feature
 of an Ethernet port.

 If mode is RTE_FDIR_DISABLE, the pballoc value is ignored.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_fdir_conf {
    ///< Flow Director mode.
    pub mode: rte_fdir_mode,
    ///< Space for FDIR filters.
    pub pballoc: rte_fdir_pballoc_type,
    ///< How to report FDIR hash.
    pub status: rte_fdir_status_mode,
    /// RX queue of packets matching a "drop" filter in perfect mode.
    pub drop_queue: u8,
    pub mask: rte_eth_fdir_masks,
    pub flex_conf: rte_eth_fdir_flex_conf,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_fdir_conf"][::std::mem::size_of::<rte_fdir_conf>() - 772usize];
    ["Alignment of rte_fdir_conf"][::std::mem::align_of::<rte_fdir_conf>() - 4usize];
    [
        "Offset of field: rte_fdir_conf::mode",
    ][::std::mem::offset_of!(rte_fdir_conf, mode) - 0usize];
    [
        "Offset of field: rte_fdir_conf::pballoc",
    ][::std::mem::offset_of!(rte_fdir_conf, pballoc) - 4usize];
    [
        "Offset of field: rte_fdir_conf::status",
    ][::std::mem::offset_of!(rte_fdir_conf, status) - 8usize];
    [
        "Offset of field: rte_fdir_conf::drop_queue",
    ][::std::mem::offset_of!(rte_fdir_conf, drop_queue) - 12usize];
    [
        "Offset of field: rte_fdir_conf::mask",
    ][::std::mem::offset_of!(rte_fdir_conf, mask) - 16usize];
    [
        "Offset of field: rte_fdir_conf::flex_conf",
    ][::std::mem::offset_of!(rte_fdir_conf, flex_conf) - 84usize];
};
impl Default for rte_fdir_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// A structure used to enable/disable specific device interrupts.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_intr_conf {
    /// enable/disable lsc interrupt. 0 (default) - disable, 1 enable
    pub lsc: u16,
    /// enable/disable rxq interrupt. 0 (default) - disable, 1 enable
    pub rxq: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_intr_conf"][::std::mem::size_of::<rte_intr_conf>() - 4usize];
    ["Alignment of rte_intr_conf"][::std::mem::align_of::<rte_intr_conf>() - 2usize];
    [
        "Offset of field: rte_intr_conf::lsc",
    ][::std::mem::offset_of!(rte_intr_conf, lsc) - 0usize];
    [
        "Offset of field: rte_intr_conf::rxq",
    ][::std::mem::offset_of!(rte_intr_conf, rxq) - 2usize];
};
/** A structure used to configure an Ethernet port.
 Depending upon the RX multi-queue mode, extra advanced
 configuration settings may be needed.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rte_eth_conf {
    /**< bitmap of ETH_LINK_SPEED_XXX of speeds to be
used. ETH_LINK_SPEED_FIXED disables link
autonegotiation, and a unique speed shall be
set. Otherwise, the bitmap defines the set of
speeds to be advertised. If the special value
ETH_LINK_SPEED_AUTONEG (0) is used, all speeds
supported are advertised.*/
    pub link_speeds: u32,
    ///< Port RX configuration.
    pub rxmode: rte_eth_rxmode,
    ///< Port TX configuration.
    pub txmode: rte_eth_txmode,
    /**< Loopback operation mode. By default the value
is 0, meaning the loopback mode is disabled.
Read the datasheet of given ethernet controller
for details. The possible values of this field
are defined in implementation of each driver.*/
    pub lpbk_mode: u32,
    ///< Port RX filtering configuration (union).
    pub rx_adv_conf: rte_eth_conf__bindgen_ty_1,
    ///< Port TX DCB configuration (union).
    pub tx_adv_conf: rte_eth_conf__bindgen_ty_2,
    /** Currently,Priority Flow Control(PFC) are supported,if DCB with PFC
is needed,and the variable must be set ETH_DCB_PFC_SUPPORT.*/
    pub dcb_capability_en: u32,
    ///< FDIR configuration.
    pub fdir_conf: rte_fdir_conf,
    ///< Interrupt mode configuration.
    pub intr_conf: rte_intr_conf,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_eth_conf__bindgen_ty_1 {
    ///< Port RSS configuration
    pub rss_conf: rte_eth_rss_conf,
    pub vmdq_dcb_conf: rte_eth_vmdq_dcb_conf,
    pub dcb_rx_conf: rte_eth_dcb_rx_conf,
    pub vmdq_rx_conf: rte_eth_vmdq_rx_conf,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_conf__bindgen_ty_1",
    ][::std::mem::size_of::<rte_eth_conf__bindgen_ty_1>() - 2120usize];
    [
        "Alignment of rte_eth_conf__bindgen_ty_1",
    ][::std::mem::align_of::<rte_eth_conf__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_1::rss_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_1, rss_conf) - 0usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_1::vmdq_dcb_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_1, vmdq_dcb_conf) - 24usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_1::dcb_rx_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_1, dcb_rx_conf) - 1064usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_1::vmdq_rx_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_1, vmdq_rx_conf) - 1080usize];
};
impl Default for rte_eth_conf__bindgen_ty_1 {
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
pub union rte_eth_conf__bindgen_ty_2 {
    pub vmdq_dcb_tx_conf: rte_eth_vmdq_dcb_tx_conf,
    pub dcb_tx_conf: rte_eth_dcb_tx_conf,
    pub vmdq_tx_conf: rte_eth_vmdq_tx_conf,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_eth_conf__bindgen_ty_2",
    ][::std::mem::size_of::<rte_eth_conf__bindgen_ty_2>() - 12usize];
    [
        "Alignment of rte_eth_conf__bindgen_ty_2",
    ][::std::mem::align_of::<rte_eth_conf__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_2::vmdq_dcb_tx_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_2, vmdq_dcb_tx_conf) - 0usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_2::dcb_tx_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_2, dcb_tx_conf) - 0usize];
    [
        "Offset of field: rte_eth_conf__bindgen_ty_2::vmdq_tx_conf",
    ][::std::mem::offset_of!(rte_eth_conf__bindgen_ty_2, vmdq_tx_conf) - 0usize];
};
impl Default for rte_eth_conf__bindgen_ty_2 {
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
    ["Size of rte_eth_conf"][::std::mem::size_of::<rte_eth_conf>() - 2944usize];
    ["Alignment of rte_eth_conf"][::std::mem::align_of::<rte_eth_conf>() - 8usize];
    [
        "Offset of field: rte_eth_conf::link_speeds",
    ][::std::mem::offset_of!(rte_eth_conf, link_speeds) - 0usize];
    [
        "Offset of field: rte_eth_conf::rxmode",
    ][::std::mem::offset_of!(rte_eth_conf, rxmode) - 4usize];
    [
        "Offset of field: rte_eth_conf::txmode",
    ][::std::mem::offset_of!(rte_eth_conf, txmode) - 16usize];
    [
        "Offset of field: rte_eth_conf::lpbk_mode",
    ][::std::mem::offset_of!(rte_eth_conf, lpbk_mode) - 24usize];
    [
        "Offset of field: rte_eth_conf::rx_adv_conf",
    ][::std::mem::offset_of!(rte_eth_conf, rx_adv_conf) - 32usize];
    [
        "Offset of field: rte_eth_conf::tx_adv_conf",
    ][::std::mem::offset_of!(rte_eth_conf, tx_adv_conf) - 2152usize];
    [
        "Offset of field: rte_eth_conf::dcb_capability_en",
    ][::std::mem::offset_of!(rte_eth_conf, dcb_capability_en) - 2164usize];
    [
        "Offset of field: rte_eth_conf::fdir_conf",
    ][::std::mem::offset_of!(rte_eth_conf, fdir_conf) - 2168usize];
    [
        "Offset of field: rte_eth_conf::intr_conf",
    ][::std::mem::offset_of!(rte_eth_conf, intr_conf) - 2940usize];
};
impl Default for rte_eth_conf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
