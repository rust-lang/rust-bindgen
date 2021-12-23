#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
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
///  A set of values to identify what method is to be used to route
///  packets to multiple queues.
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
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_rxmode {
    /// The multi-queue packet distribution mode to be used, e.g. RSS.
    pub mq_mode: rte_eth_rx_mq_mode,
    ///< Only used if jumbo_frame enabled.
    pub max_rx_pkt_len: u32,
    ///< hdr buf size (header_split enabled).
    pub split_hdr_size: u16,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
}
#[test]
fn bindgen_test_layout_rte_eth_rxmode() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_rxmode>(),
        12usize,
        concat!("Size of: ", stringify!(rte_eth_rxmode))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_rxmode>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_rxmode))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_rxmode>())).mq_mode as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_rxmode),
            "::",
            stringify!(mq_mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_rxmode>())).max_rx_pkt_len
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_rxmode),
            "::",
            stringify!(max_rx_pkt_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_rxmode>())).split_hdr_size
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_rxmode),
            "::",
            stringify!(split_hdr_size)
        )
    );
}
impl Clone for rte_eth_rxmode {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_rxmode {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl rte_eth_rxmode {
    #[inline]
    pub fn header_split(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_header_split(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_ip_checksum(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_hw_ip_checksum(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_filter(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_hw_vlan_filter(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_strip(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_hw_vlan_strip(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_extend(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_hw_vlan_extend(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn jumbo_frame(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_jumbo_frame(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_strip_crc(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_hw_strip_crc(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enable_scatter(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_enable_scatter(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enable_lro(&self) -> u16 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_enable_lro(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
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
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let header_split: u16 =
                unsafe { ::std::mem::transmute(header_split) };
            header_split as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let hw_ip_checksum: u16 =
                unsafe { ::std::mem::transmute(hw_ip_checksum) };
            hw_ip_checksum as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let hw_vlan_filter: u16 =
                unsafe { ::std::mem::transmute(hw_vlan_filter) };
            hw_vlan_filter as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let hw_vlan_strip: u16 =
                unsafe { ::std::mem::transmute(hw_vlan_strip) };
            hw_vlan_strip as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let hw_vlan_extend: u16 =
                unsafe { ::std::mem::transmute(hw_vlan_extend) };
            hw_vlan_extend as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let jumbo_frame: u16 =
                unsafe { ::std::mem::transmute(jumbo_frame) };
            jumbo_frame as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let hw_strip_crc: u16 =
                unsafe { ::std::mem::transmute(hw_strip_crc) };
            hw_strip_crc as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let enable_scatter: u16 =
                unsafe { ::std::mem::transmute(enable_scatter) };
            enable_scatter as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let enable_lro: u16 = unsafe { ::std::mem::transmute(enable_lro) };
            enable_lro as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(u32)]
/// A set of values to identify what method is to be used to transmit
/// packets using multi-TCs.
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
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_txmode {
    ///< TX multi-queues mode.
    pub mq_mode: rte_eth_tx_mq_mode,
    pub pvid: u16,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: u8,
}
#[test]
fn bindgen_test_layout_rte_eth_txmode() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_txmode>(),
        8usize,
        concat!("Size of: ", stringify!(rte_eth_txmode))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_txmode>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_txmode))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_txmode>())).mq_mode as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_txmode),
            "::",
            stringify!(mq_mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_txmode>())).pvid as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_txmode),
            "::",
            stringify!(pvid)
        )
    );
}
impl Clone for rte_eth_txmode {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_txmode {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl rte_eth_txmode {
    #[inline]
    pub fn hw_vlan_reject_tagged(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_hw_vlan_reject_tagged(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_reject_untagged(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_hw_vlan_reject_untagged(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hw_vlan_insert_pvid(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_hw_vlan_insert_pvid(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        hw_vlan_reject_tagged: u8,
        hw_vlan_reject_untagged: u8,
        hw_vlan_insert_pvid: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let hw_vlan_reject_tagged: u8 =
                unsafe { ::std::mem::transmute(hw_vlan_reject_tagged) };
            hw_vlan_reject_tagged as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let hw_vlan_reject_untagged: u8 =
                unsafe { ::std::mem::transmute(hw_vlan_reject_untagged) };
            hw_vlan_reject_untagged as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let hw_vlan_insert_pvid: u8 =
                unsafe { ::std::mem::transmute(hw_vlan_insert_pvid) };
            hw_vlan_insert_pvid as u64
        });
        __bindgen_bitfield_unit
    }
}
/// A structure used to configure the Receive Side Scaling (RSS) feature
/// of an Ethernet port.
/// If not NULL, the *rss_key* pointer of the *rss_conf* structure points
/// to an array holding the RSS key to use for hashing specific header
/// fields of received packets. The length of this array should be indicated
/// by *rss_key_len* below. Otherwise, a default random hash key is used by
/// the device driver.
///
/// The *rss_key_len* field of the *rss_conf* structure indicates the length
/// in bytes of the array pointed by *rss_key*. To be compatible, this length
/// will be checked in i40e only. Others assume 40 bytes to be used as before.
///
/// The *rss_hf* field of the *rss_conf* structure indicates the different
/// types of IPv4/IPv6 packets to which the RSS hashing must be applied.
/// Supplying an *rss_hf* equal to zero disables the RSS feature.
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_rss_conf {
    ///< If not NULL, 40-byte hash key.
    pub rss_key: *mut u8,
    ///< hash key length in bytes.
    pub rss_key_len: u8,
    ///< Hash functions to apply - see below.
    pub rss_hf: u64,
}
#[test]
fn bindgen_test_layout_rte_eth_rss_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_rss_conf>(),
        24usize,
        concat!("Size of: ", stringify!(rte_eth_rss_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_rss_conf>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_eth_rss_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_rss_conf>())).rss_key as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_rss_conf),
            "::",
            stringify!(rss_key)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_rss_conf>())).rss_key_len as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_rss_conf),
            "::",
            stringify!(rss_key_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_rss_conf>())).rss_hf as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_rss_conf),
            "::",
            stringify!(rss_hf)
        )
    );
}
impl Clone for rte_eth_rss_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_rss_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(u32)]
/// This enum indicates the possible number of traffic classes
/// in DCB configratioins
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rte_eth_nb_tcs {
    ///< 4 TCs with DCB.
    ETH_4_TCS = 4,
    ///< 8 TCs with DCB.
    ETH_8_TCS = 8,
}
#[repr(u32)]
/// This enum indicates the possible number of queue pools
/// in VMDQ configurations.
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
/// A structure used to configure the VMDQ+DCB feature
/// of an Ethernet port.
///
/// Using this feature, packets are routed to a pool of queues, based
/// on the vlan id in the vlan tag, and then to a specific queue within
/// that pool, using the user priority vlan tag field.
///
/// A default pool may be used, if desired, to route all traffic which
/// does not match the vlan filter rules.
#[repr(C)]
#[derive(Copy)]
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
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_dcb_conf__bindgen_ty_1 {
    ///< The vlan id of the received frame
    pub vlan_id: u16,
    ///< Bitmask of pools for packet rx
    pub pools: u64,
}
#[test]
fn bindgen_test_layout_rte_eth_vmdq_dcb_conf__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_vmdq_dcb_conf__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(rte_eth_vmdq_dcb_conf__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_vmdq_dcb_conf__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(rte_eth_vmdq_dcb_conf__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf__bindgen_ty_1>()))
                .vlan_id as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf__bindgen_ty_1),
            "::",
            stringify!(vlan_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf__bindgen_ty_1>()))
                .pools as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf__bindgen_ty_1),
            "::",
            stringify!(pools)
        )
    );
}
impl Clone for rte_eth_vmdq_dcb_conf__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_eth_vmdq_dcb_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_vmdq_dcb_conf>(),
        1040usize,
        concat!("Size of: ", stringify!(rte_eth_vmdq_dcb_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_vmdq_dcb_conf>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_eth_vmdq_dcb_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf>())).nb_queue_pools
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf),
            "::",
            stringify!(nb_queue_pools)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf>()))
                .enable_default_pool as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf),
            "::",
            stringify!(enable_default_pool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf>())).default_pool
                as *const _ as usize
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf),
            "::",
            stringify!(default_pool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf>())).nb_pool_maps
                as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf),
            "::",
            stringify!(nb_pool_maps)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf>())).pool_map
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf),
            "::",
            stringify!(pool_map)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_conf>())).dcb_tc as *const _
                as usize
        },
        1032usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_conf),
            "::",
            stringify!(dcb_tc)
        )
    );
}
impl Clone for rte_eth_vmdq_dcb_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_vmdq_dcb_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_dcb_rx_conf {
    ///< Possible DCB TCs, 4 or 8 TCs
    pub nb_tcs: rte_eth_nb_tcs,
    /// Traffic class each UP mapped to.
    pub dcb_tc: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_rte_eth_dcb_rx_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_dcb_rx_conf>(),
        12usize,
        concat!("Size of: ", stringify!(rte_eth_dcb_rx_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_dcb_rx_conf>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_dcb_rx_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_dcb_rx_conf>())).nb_tcs as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_dcb_rx_conf),
            "::",
            stringify!(nb_tcs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_dcb_rx_conf>())).dcb_tc as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_dcb_rx_conf),
            "::",
            stringify!(dcb_tc)
        )
    );
}
impl Clone for rte_eth_dcb_rx_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_dcb_rx_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_dcb_tx_conf {
    ///< With DCB, 16 or 32 pools.
    pub nb_queue_pools: rte_eth_nb_pools,
    /// Traffic class each UP mapped to.
    pub dcb_tc: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_rte_eth_vmdq_dcb_tx_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_vmdq_dcb_tx_conf>(),
        12usize,
        concat!("Size of: ", stringify!(rte_eth_vmdq_dcb_tx_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_vmdq_dcb_tx_conf>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_vmdq_dcb_tx_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_tx_conf>())).nb_queue_pools
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_tx_conf),
            "::",
            stringify!(nb_queue_pools)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_dcb_tx_conf>())).dcb_tc
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_dcb_tx_conf),
            "::",
            stringify!(dcb_tc)
        )
    );
}
impl Clone for rte_eth_vmdq_dcb_tx_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_vmdq_dcb_tx_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_dcb_tx_conf {
    ///< Possible DCB TCs, 4 or 8 TCs.
    pub nb_tcs: rte_eth_nb_tcs,
    /// Traffic class each UP mapped to.
    pub dcb_tc: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_rte_eth_dcb_tx_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_dcb_tx_conf>(),
        12usize,
        concat!("Size of: ", stringify!(rte_eth_dcb_tx_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_dcb_tx_conf>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_dcb_tx_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_dcb_tx_conf>())).nb_tcs as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_dcb_tx_conf),
            "::",
            stringify!(nb_tcs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_dcb_tx_conf>())).dcb_tc as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_dcb_tx_conf),
            "::",
            stringify!(dcb_tc)
        )
    );
}
impl Clone for rte_eth_dcb_tx_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_dcb_tx_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_tx_conf {
    ///< VMDq mode, 64 pools.
    pub nb_queue_pools: rte_eth_nb_pools,
}
#[test]
fn bindgen_test_layout_rte_eth_vmdq_tx_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_vmdq_tx_conf>(),
        4usize,
        concat!("Size of: ", stringify!(rte_eth_vmdq_tx_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_vmdq_tx_conf>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_vmdq_tx_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_tx_conf>())).nb_queue_pools
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_tx_conf),
            "::",
            stringify!(nb_queue_pools)
        )
    );
}
impl Clone for rte_eth_vmdq_tx_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_vmdq_tx_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Copy)]
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
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_vmdq_rx_conf__bindgen_ty_1 {
    ///< The vlan id of the received frame
    pub vlan_id: u16,
    ///< Bitmask of pools for packet rx
    pub pools: u64,
}
#[test]
fn bindgen_test_layout_rte_eth_vmdq_rx_conf__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_vmdq_rx_conf__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(rte_eth_vmdq_rx_conf__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_vmdq_rx_conf__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(rte_eth_vmdq_rx_conf__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf__bindgen_ty_1>()))
                .vlan_id as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf__bindgen_ty_1),
            "::",
            stringify!(vlan_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf__bindgen_ty_1>())).pools
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf__bindgen_ty_1),
            "::",
            stringify!(pools)
        )
    );
}
impl Clone for rte_eth_vmdq_rx_conf__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_eth_vmdq_rx_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_vmdq_rx_conf>(),
        1040usize,
        concat!("Size of: ", stringify!(rte_eth_vmdq_rx_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_vmdq_rx_conf>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_eth_vmdq_rx_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).nb_queue_pools
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(nb_queue_pools)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).enable_default_pool
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(enable_default_pool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).default_pool
                as *const _ as usize
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(default_pool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).enable_loop_back
                as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(enable_loop_back)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).nb_pool_maps
                as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(nb_pool_maps)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).rx_mode as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(rx_mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_vmdq_rx_conf>())).pool_map
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_vmdq_rx_conf),
            "::",
            stringify!(pool_map)
        )
    );
}
impl Clone for rte_eth_vmdq_rx_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_vmdq_rx_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
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
///  Memory space that can be configured to store Flow Director filters
///  in the board memory.
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
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
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
#[test]
fn bindgen_test_layout_rte_eth_ipv4_flow() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_ipv4_flow>(),
        12usize,
        concat!("Size of: ", stringify!(rte_eth_ipv4_flow))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_ipv4_flow>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_ipv4_flow))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv4_flow>())).src_ip as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv4_flow),
            "::",
            stringify!(src_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv4_flow>())).dst_ip as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv4_flow),
            "::",
            stringify!(dst_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv4_flow>())).tos as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv4_flow),
            "::",
            stringify!(tos)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv4_flow>())).ttl as *const _
                as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv4_flow),
            "::",
            stringify!(ttl)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv4_flow>())).proto as *const _
                as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv4_flow),
            "::",
            stringify!(proto)
        )
    );
}
impl Clone for rte_eth_ipv4_flow {
    fn clone(&self) -> Self {
        *self
    }
}
/// A structure used to define the input for IPV6 flow
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
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
#[test]
fn bindgen_test_layout_rte_eth_ipv6_flow() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_ipv6_flow>(),
        36usize,
        concat!("Size of: ", stringify!(rte_eth_ipv6_flow))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_ipv6_flow>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_ipv6_flow))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv6_flow>())).src_ip as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv6_flow),
            "::",
            stringify!(src_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv6_flow>())).dst_ip as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv6_flow),
            "::",
            stringify!(dst_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv6_flow>())).tc as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv6_flow),
            "::",
            stringify!(tc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv6_flow>())).proto as *const _
                as usize
        },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv6_flow),
            "::",
            stringify!(proto)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_ipv6_flow>())).hop_limits as *const _
                as usize
        },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_ipv6_flow),
            "::",
            stringify!(hop_limits)
        )
    );
}
impl Clone for rte_eth_ipv6_flow {
    fn clone(&self) -> Self {
        *self
    }
}
///  A structure used to configure FDIR masks that are used by the device
///  to match the various fields of RX packet headers.
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
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
    /// 6 bit mask for proper 6 bytes of Mac address, bit 0 matches the
    ///first byte on the wire
    pub mac_addr_byte_mask: u8,
    /// Bit mask for tunnel ID in big endian.
    pub tunnel_id_mask: u32,
    ///< 1 - Match tunnel type,
    ///0 - Ignore tunnel type.
    pub tunnel_type_mask: u8,
}
#[test]
fn bindgen_test_layout_rte_eth_fdir_masks() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_fdir_masks>(),
        68usize,
        concat!("Size of: ", stringify!(rte_eth_fdir_masks))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_fdir_masks>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_fdir_masks))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).vlan_tci_mask
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(vlan_tci_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).ipv4_mask as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(ipv4_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).ipv6_mask as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(ipv6_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).src_port_mask
                as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(src_port_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).dst_port_mask
                as *const _ as usize
        },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(dst_port_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).mac_addr_byte_mask
                as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(mac_addr_byte_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).tunnel_id_mask
                as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(tunnel_id_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_masks>())).tunnel_type_mask
                as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_masks),
            "::",
            stringify!(tunnel_type_mask)
        )
    );
}
impl Clone for rte_eth_fdir_masks {
    fn clone(&self) -> Self {
        *self
    }
}
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
/// A structure used to select bytes extracted from the protocol layers to
/// flexible payload for filter
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_flex_payload_cfg {
    ///< Payload type
    pub type_: rte_eth_payload_type,
    pub src_offset: [u16; 16usize],
}
#[test]
fn bindgen_test_layout_rte_eth_flex_payload_cfg() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_flex_payload_cfg>(),
        36usize,
        concat!("Size of: ", stringify!(rte_eth_flex_payload_cfg))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_flex_payload_cfg>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_flex_payload_cfg))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_flex_payload_cfg>())).type_
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_flex_payload_cfg),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_flex_payload_cfg>())).src_offset
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_flex_payload_cfg),
            "::",
            stringify!(src_offset)
        )
    );
}
impl Clone for rte_eth_flex_payload_cfg {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_flex_payload_cfg {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
/// A structure used to define FDIR masks for flexible payload
/// for each flow type
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_fdir_flex_mask {
    pub flow_type: u16,
    pub mask: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_rte_eth_fdir_flex_mask() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_fdir_flex_mask>(),
        18usize,
        concat!("Size of: ", stringify!(rte_eth_fdir_flex_mask))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_fdir_flex_mask>(),
        2usize,
        concat!("Alignment of ", stringify!(rte_eth_fdir_flex_mask))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_flex_mask>())).flow_type
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_flex_mask),
            "::",
            stringify!(flow_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_flex_mask>())).mask as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_flex_mask),
            "::",
            stringify!(mask)
        )
    );
}
impl Clone for rte_eth_fdir_flex_mask {
    fn clone(&self) -> Self {
        *self
    }
}
/// A structure used to define all flexible payload related setting
/// include flex payload and flex mask
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_fdir_flex_conf {
    ///< The number of following payload cfg
    pub nb_payloads: u16,
    ///< The number of following mask
    pub nb_flexmasks: u16,
    pub flex_set: [rte_eth_flex_payload_cfg; 8usize],
    pub flex_mask: [rte_eth_fdir_flex_mask; 22usize],
}
#[test]
fn bindgen_test_layout_rte_eth_fdir_flex_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_fdir_flex_conf>(),
        688usize,
        concat!("Size of: ", stringify!(rte_eth_fdir_flex_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_fdir_flex_conf>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_fdir_flex_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_flex_conf>())).nb_payloads
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_flex_conf),
            "::",
            stringify!(nb_payloads)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_flex_conf>())).nb_flexmasks
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_flex_conf),
            "::",
            stringify!(nb_flexmasks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_flex_conf>())).flex_set
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_flex_conf),
            "::",
            stringify!(flex_set)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_fdir_flex_conf>())).flex_mask
                as *const _ as usize
        },
        292usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_fdir_flex_conf),
            "::",
            stringify!(flex_mask)
        )
    );
}
impl Clone for rte_eth_fdir_flex_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_fdir_flex_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
/// A structure used to configure the Flow Director (FDIR) feature
/// of an Ethernet port.
///
/// If mode is RTE_FDIR_DISABLE, the pballoc value is ignored.
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
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
#[test]
fn bindgen_test_layout_rte_fdir_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_fdir_conf>(),
        772usize,
        concat!("Size of: ", stringify!(rte_fdir_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_fdir_conf>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_fdir_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_fdir_conf>())).mode as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_fdir_conf),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_fdir_conf>())).pballoc as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_fdir_conf),
            "::",
            stringify!(pballoc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_fdir_conf>())).status as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_fdir_conf),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_fdir_conf>())).drop_queue as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_fdir_conf),
            "::",
            stringify!(drop_queue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_fdir_conf>())).mask as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_fdir_conf),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_fdir_conf>())).flex_conf as *const _
                as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_fdir_conf),
            "::",
            stringify!(flex_conf)
        )
    );
}
impl Clone for rte_fdir_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_fdir_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
/// A structure used to enable/disable specific device interrupts.
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_intr_conf {
    /// enable/disable lsc interrupt. 0 (default) - disable, 1 enable
    pub lsc: u16,
    /// enable/disable rxq interrupt. 0 (default) - disable, 1 enable
    pub rxq: u16,
}
#[test]
fn bindgen_test_layout_rte_intr_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_intr_conf>(),
        4usize,
        concat!("Size of: ", stringify!(rte_intr_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_intr_conf>(),
        2usize,
        concat!("Alignment of ", stringify!(rte_intr_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_intr_conf>())).lsc as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_intr_conf),
            "::",
            stringify!(lsc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_intr_conf>())).rxq as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_intr_conf),
            "::",
            stringify!(rxq)
        )
    );
}
impl Clone for rte_intr_conf {
    fn clone(&self) -> Self {
        *self
    }
}
/// A structure used to configure an Ethernet port.
/// Depending upon the RX multi-queue mode, extra advanced
/// configuration settings may be needed.
#[repr(C)]
#[derive(Copy)]
pub struct rte_eth_conf {
    ///< bitmap of ETH_LINK_SPEED_XXX of speeds to be
    ///used. ETH_LINK_SPEED_FIXED disables link
    ///autonegotiation, and a unique speed shall be
    ///set. Otherwise, the bitmap defines the set of
    ///speeds to be advertised. If the special value
    ///ETH_LINK_SPEED_AUTONEG (0) is used, all speeds
    ///supported are advertised.
    pub link_speeds: u32,
    ///< Port RX configuration.
    pub rxmode: rte_eth_rxmode,
    ///< Port TX configuration.
    pub txmode: rte_eth_txmode,
    ///< Loopback operation mode. By default the value
    ///is 0, meaning the loopback mode is disabled.
    ///Read the datasheet of given ethernet controller
    ///for details. The possible values of this field
    ///are defined in implementation of each driver.
    pub lpbk_mode: u32,
    ///< Port RX filtering configuration (union).
    pub rx_adv_conf: rte_eth_conf__bindgen_ty_1,
    ///< Port TX DCB configuration (union).
    pub tx_adv_conf: rte_eth_conf__bindgen_ty_2,
    /// Currently,Priority Flow Control(PFC) are supported,if DCB with PFC
    ///is needed,and the variable must be set ETH_DCB_PFC_SUPPORT.
    pub dcb_capability_en: u32,
    ///< FDIR configuration.
    pub fdir_conf: rte_fdir_conf,
    ///< Interrupt mode configuration.
    pub intr_conf: rte_intr_conf,
}
#[repr(C)]
#[derive(Copy)]
pub struct rte_eth_conf__bindgen_ty_1 {
    ///< Port RSS configuration
    pub rss_conf: rte_eth_rss_conf,
    pub vmdq_dcb_conf: rte_eth_vmdq_dcb_conf,
    pub dcb_rx_conf: rte_eth_dcb_rx_conf,
    pub vmdq_rx_conf: rte_eth_vmdq_rx_conf,
}
#[test]
fn bindgen_test_layout_rte_eth_conf__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_conf__bindgen_ty_1>(),
        2120usize,
        concat!("Size of: ", stringify!(rte_eth_conf__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_conf__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_eth_conf__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_1>())).rss_conf
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_1),
            "::",
            stringify!(rss_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_1>())).vmdq_dcb_conf
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_1),
            "::",
            stringify!(vmdq_dcb_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_1>())).dcb_rx_conf
                as *const _ as usize
        },
        1064usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_1),
            "::",
            stringify!(dcb_rx_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_1>())).vmdq_rx_conf
                as *const _ as usize
        },
        1080usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_1),
            "::",
            stringify!(vmdq_rx_conf)
        )
    );
}
impl Clone for rte_eth_conf__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_conf__bindgen_ty_1 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_eth_conf__bindgen_ty_2 {
    pub vmdq_dcb_tx_conf: __BindgenUnionField<rte_eth_vmdq_dcb_tx_conf>,
    pub dcb_tx_conf: __BindgenUnionField<rte_eth_dcb_tx_conf>,
    pub vmdq_tx_conf: __BindgenUnionField<rte_eth_vmdq_tx_conf>,
    pub bindgen_union_field: [u32; 3usize],
}
#[test]
fn bindgen_test_layout_rte_eth_conf__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_conf__bindgen_ty_2>(),
        12usize,
        concat!("Size of: ", stringify!(rte_eth_conf__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_conf__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_eth_conf__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_2>()))
                .vmdq_dcb_tx_conf as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_2),
            "::",
            stringify!(vmdq_dcb_tx_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_2>())).dcb_tx_conf
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_2),
            "::",
            stringify!(dcb_tx_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf__bindgen_ty_2>())).vmdq_tx_conf
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf__bindgen_ty_2),
            "::",
            stringify!(vmdq_tx_conf)
        )
    );
}
impl Clone for rte_eth_conf__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_eth_conf() {
    assert_eq!(
        ::std::mem::size_of::<rte_eth_conf>(),
        2944usize,
        concat!("Size of: ", stringify!(rte_eth_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_conf>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_eth_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).link_speeds as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(link_speeds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).rxmode as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(rxmode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).txmode as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(txmode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).lpbk_mode as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(lpbk_mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).rx_adv_conf as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(rx_adv_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).tx_adv_conf as *const _
                as usize
        },
        2152usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(tx_adv_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).dcb_capability_en
                as *const _ as usize
        },
        2164usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(dcb_capability_en)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).fdir_conf as *const _
                as usize
        },
        2168usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(fdir_conf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_eth_conf>())).intr_conf as *const _
                as usize
        },
        2940usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_conf),
            "::",
            stringify!(intr_conf)
        )
    );
}
impl Clone for rte_eth_conf {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_eth_conf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
