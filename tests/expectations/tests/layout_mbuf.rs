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
    pub const fn new(storage: Storage) -> Self {
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
#[test]
fn bindgen_test_layout_rte_atomic16_t() {
    assert_eq!(
        ::std::mem::size_of::<rte_atomic16_t>(),
        2usize,
        concat!("Size of: ", stringify!(rte_atomic16_t))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_atomic16_t>(),
        2usize,
        concat!("Alignment of ", stringify!(rte_atomic16_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_atomic16_t>())).cnt as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_atomic16_t),
            "::",
            stringify!(cnt)
        )
    );
}
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
    /// Size of the application private data. In case of an indirect
    /// mbuf, it stores the direct mbuf private data size.
    pub priv_size: u16,
    /// Timesync flags for use with IEEE1588.
    pub timesync: u16,
}
/// 16-bit Reference counter.
/// It should only be accessed using the following functions:
/// rte_mbuf_refcnt_update(), rte_mbuf_refcnt_read(), and
/// rte_mbuf_refcnt_set(). The functionality of these functions (atomic,
/// or non-atomic) is controlled by the CONFIG_RTE_MBUF_REFCNT_ATOMIC
/// config option.
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_mbuf__bindgen_ty_1 {
    ///< Atomically accessed refcnt
    pub refcnt_atomic: rte_atomic16_t,
    ///< Non-atomically accessed refcnt
    pub refcnt: u16,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_1>(),
        2usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_1>())).refcnt_atomic
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_1),
            "::",
            stringify!(refcnt_atomic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_1>())).refcnt
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_1),
            "::",
            stringify!(refcnt)
        )
    );
}
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
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_2__bindgen_ty_1)
        )
    );
}
impl rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    #[inline]
    pub fn l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(8usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn tun_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_tun_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn inner_l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_inner_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn inner_l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_inner_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn inner_l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(24usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_inner_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 4u8, val as u64)
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
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let l2_type: u32 = unsafe { ::std::mem::transmute(l2_type) };
            l2_type as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let l3_type: u32 = unsafe { ::std::mem::transmute(l3_type) };
            l3_type as u64
        });
        __bindgen_bitfield_unit.set(8usize, 4u8, {
            let l4_type: u32 = unsafe { ::std::mem::transmute(l4_type) };
            l4_type as u64
        });
        __bindgen_bitfield_unit.set(12usize, 4u8, {
            let tun_type: u32 = unsafe { ::std::mem::transmute(tun_type) };
            tun_type as u64
        });
        __bindgen_bitfield_unit.set(16usize, 4u8, {
            let inner_l2_type: u32 =
                unsafe { ::std::mem::transmute(inner_l2_type) };
            inner_l2_type as u64
        });
        __bindgen_bitfield_unit.set(20usize, 4u8, {
            let inner_l3_type: u32 =
                unsafe { ::std::mem::transmute(inner_l3_type) };
            inner_l3_type as u64
        });
        __bindgen_bitfield_unit.set(24usize, 4u8, {
            let inner_l4_type: u32 =
                unsafe { ::std::mem::transmute(inner_l4_type) };
            inner_l4_type as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_2>())).packet_type
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_2),
            "::",
            stringify!(packet_type)
        )
    );
}
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
    pub __bindgen_anon_1:
        rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    pub lo: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub hash: u16,
    pub id: u16,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1(
) {
    assert_eq ! (:: std :: mem :: size_of :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > () , 4usize , concat ! ("Size of: " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1)));
    assert_eq ! (:: std :: mem :: align_of :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > () , 2usize , concat ! ("Alignment of " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1)));
    assert_eq ! (unsafe { & (* (:: std :: ptr :: null :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > ())) . hash as * const _ as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1) , "::" , stringify ! (hash)));
    assert_eq ! (unsafe { & (* (:: std :: ptr :: null :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > ())) . id as * const _ as usize } , 2usize , concat ! ("Offset of field: " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1) , "::" , stringify ! (id)));
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1>(
        ),
        4usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<
            rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
        >(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
            >()))
            .lo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(lo)
        )
    );
}
impl Default for rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>())).hi
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1),
            "::",
            stringify!(hi)
        )
    );
}
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
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>())).lo
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2),
            "::",
            stringify!(lo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>())).hi
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2),
            "::",
            stringify!(hi)
        )
    );
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3>(),
        8usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_3>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_3))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3>())).rss as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3),
            "::",
            stringify!(rss)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3>())).fdir as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3),
            "::",
            stringify!(fdir)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3>())).sched as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3),
            "::",
            stringify!(sched)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_3>())).usr as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_3),
            "::",
            stringify!(usr)
        )
    );
}
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
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_4>(),
        8usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_4>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_4))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_4>())).userdata
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_4),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_4>())).udata64
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_4),
            "::",
            stringify!(udata64)
        )
    );
}
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
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 7usize]>,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_5__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_5__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_5__bindgen_ty_1)
        )
    );
}
impl rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    #[inline]
    pub fn l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u64)
        }
    }
    #[inline]
    pub fn set_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(7usize, 9u8) as u64)
        }
    }
    #[inline]
    pub fn set_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 9u8, val as u64)
        }
    }
    #[inline]
    pub fn l4_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u64)
        }
    }
    #[inline]
    pub fn set_l4_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn tso_segsz(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(24usize, 16u8) as u64)
        }
    }
    #[inline]
    pub fn set_tso_segsz(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn outer_l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(40usize, 9u8) as u64)
        }
    }
    #[inline]
    pub fn set_outer_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(40usize, 9u8, val as u64)
        }
    }
    #[inline]
    pub fn outer_l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(49usize, 7u8) as u64)
        }
    }
    #[inline]
    pub fn set_outer_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(49usize, 7u8, val as u64)
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
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 7usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let l2_len: u64 = unsafe { ::std::mem::transmute(l2_len) };
            l2_len as u64
        });
        __bindgen_bitfield_unit.set(7usize, 9u8, {
            let l3_len: u64 = unsafe { ::std::mem::transmute(l3_len) };
            l3_len as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let l4_len: u64 = unsafe { ::std::mem::transmute(l4_len) };
            l4_len as u64
        });
        __bindgen_bitfield_unit.set(24usize, 16u8, {
            let tso_segsz: u64 = unsafe { ::std::mem::transmute(tso_segsz) };
            tso_segsz as u64
        });
        __bindgen_bitfield_unit.set(40usize, 9u8, {
            let outer_l3_len: u64 =
                unsafe { ::std::mem::transmute(outer_l3_len) };
            outer_l3_len as u64
        });
        __bindgen_bitfield_unit.set(49usize, 7u8, {
            let outer_l2_len: u64 =
                unsafe { ::std::mem::transmute(outer_l2_len) };
            outer_l2_len as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_5>(),
        8usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_5))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_5>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_5))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf__bindgen_ty_5>())).tx_offload
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf__bindgen_ty_5),
            "::",
            stringify!(tx_offload)
        )
    );
}
impl Default for rte_mbuf__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf>(),
        128usize,
        concat!("Size of: ", stringify!(rte_mbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf>(),
        64usize,
        concat!("Alignment of ", stringify!(rte_mbuf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).cacheline0 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(cacheline0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).buf_addr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(buf_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).buf_physaddr as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(buf_physaddr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).buf_len as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(buf_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).rearm_data as *const _ as usize
        },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(rearm_data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).data_off as *const _ as usize
        },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(data_off)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).nb_segs as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(nb_segs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).port as *const _ as usize
        },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).ol_flags as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(ol_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).rx_descriptor_fields1
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(rx_descriptor_fields1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).pkt_len as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(pkt_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).data_len as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(data_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).vlan_tci as *const _ as usize
        },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(vlan_tci)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).hash as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).seqn as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(seqn)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).vlan_tci_outer as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(vlan_tci_outer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).cacheline1 as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(cacheline1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).pool as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(pool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).next as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).priv_size as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(priv_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_mbuf>())).timesync as *const _ as usize
        },
        98usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mbuf),
            "::",
            stringify!(timesync)
        )
    );
}
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
