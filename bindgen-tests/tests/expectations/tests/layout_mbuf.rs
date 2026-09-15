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
        let Self { storage } = self;
        let storage = storage.as_ref();
        unsafe { Self::get_bits(storage.as_ptr(), storage.len(), bit_offset, bit_width) }
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        unsafe {
            Self::get_bits(
                core::ptr::addr_of!((*this).storage).cast(),
                core::mem::size_of::<Storage>(),
                bit_offset,
                bit_width,
            )
        }
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        let Self { storage } = self;
        let storage = storage.as_mut();
        unsafe {
            Self::set_bits(
                storage.as_mut_ptr(),
                storage.len(),
                bit_offset,
                bit_width,
                val,
            );
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        unsafe {
            Self::set_bits(
                core::ptr::addr_of_mut!((*this).storage).cast(),
                core::mem::size_of::<Storage>(),
                bit_offset,
                bit_width,
                val,
            );
        }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    const fn field_layout(
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
    ) -> (usize, usize, usize) {
        debug_assert!(bit_width <= 64);
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        debug_assert!(start_byte < storage_len);
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        assert!(start_byte + bytes_needed <= storage_len);
        (start_byte, bit_shift, bytes_needed)
    }
    #[inline]
    const unsafe fn get_bits(
        storage: *const u8,
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
    ) -> u64 {
        let (start_byte, bit_shift, bytes_needed) = Self::field_layout(
            storage_len,
            bit_offset,
            bit_width,
        );
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let mut i = 0;
        while i < bytes_needed && i < 8 {
            let byte = unsafe { *storage.add(start_byte + i) };
            let byte = if cfg!(target_endian = "big") {
                byte.reverse_bits()
            } else {
                byte
            };
            val |= (byte as u64) << (i * 8);
            i += 1;
        }
        val >>= bit_shift;
        if bytes_needed == 9 {
            let byte = unsafe { *storage.add(start_byte + 8) };
            let byte = if cfg!(target_endian = "big") {
                byte.reverse_bits()
            } else {
                byte
            };
            val |= (byte as u64) << (64 - bit_shift);
        }
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        val
    }
    #[inline]
    unsafe fn set_bits(
        storage: *mut u8,
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
        mut val: u64,
    ) {
        let (start_byte, bit_shift, bytes_needed) = Self::field_layout(
            storage_len,
            bit_offset,
            bit_width,
        );
        if bit_width == 0 {
            return;
        }
        if bit_width == 64 && bit_shift == 0 {
            unsafe { storage.add(start_byte).cast::<u64>().write_unaligned(val) };
            return;
        }
        let field_mask = if bit_width == 64 { !0u64 } else { (1u64 << bit_width) - 1 };
        val &= field_mask;
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        if bytes_needed == 9 {
            let shift = 64 - bit_shift;
            unsafe {
                Self::set_byte(
                    storage.add(start_byte + 8),
                    (val >> shift) as u8,
                    (field_mask >> shift) as u8,
                );
            }
        }
        let val = val << bit_shift;
        let field_mask = field_mask << bit_shift;
        for i in 0..bytes_needed.min(8) {
            unsafe {
                Self::set_byte(
                    storage.add(start_byte + i),
                    (val >> (i * 8)) as u8,
                    (field_mask >> (i * 8)) as u8,
                );
            }
        }
    }
    #[inline]
    unsafe fn set_byte(byte: *mut u8, val: u8, mask: u8) {
        let (val, mask) = if cfg!(target_endian = "big") {
            (val.reverse_bits(), mask.reverse_bits())
        } else {
            (val, mask)
        };
        unsafe { *byte = (*byte & !mask) | (val & mask) };
    }
}
/// Const-generic methods for bitfield access when offset and width are known
/// at compile time. Inlining exposes these constants to the shared implementation.
impl<const N: usize> __BindgenBitfieldUnit<[u8; N]> {
    /// Get a field using const generics for compile-time optimization.
    #[inline]
    pub const fn get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&self) -> u64 {
        let Self { storage } = self;
        unsafe { Self::get_bits(storage.as_ptr(), N, BIT_OFFSET, BIT_WIDTH) }
    }
    /// Set a field using const generics for compile-time optimization.
    #[inline]
    pub fn set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&mut self, val: u64) {
        self.set(BIT_OFFSET, BIT_WIDTH, val);
    }
    /// Raw pointer get using const generics for compile-time optimization.
    #[inline]
    pub const unsafe fn raw_get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *const Self,
    ) -> u64 {
        unsafe {
            Self::get_bits(
                core::ptr::addr_of!((*this).storage).cast(),
                N,
                BIT_OFFSET,
                BIT_WIDTH,
            )
        }
    }
    /// Raw pointer set using const generics for compile-time optimization.
    #[inline]
    pub unsafe fn raw_set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *mut Self,
        val: u64,
    ) {
        unsafe { Self::raw_set(this, BIT_OFFSET, BIT_WIDTH, val) };
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
    #[allow(unnecessary_transmutes)]
    pub fn l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_l2_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<4usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<4usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_l3_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                4usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<8usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<8usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_l4_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                8usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn tun_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<12usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_tun_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<12usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_tun_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                12usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn inner_l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_inner_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_inner_l2_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                16usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn inner_l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<20usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_inner_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<20usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_inner_l3_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                20usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn inner_l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 4u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_inner_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<24usize, 4u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_inner_l4_type_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                24usize,
                4u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
                let l2_type: u32 = unsafe { ::std::mem::transmute(l2_type) };
                l2_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                4usize,
                4u8,
            >({
                let l3_type: u32 = unsafe { ::std::mem::transmute(l3_type) };
                l3_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                4u8,
            >({
                let l4_type: u32 = unsafe { ::std::mem::transmute(l4_type) };
                l4_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                12usize,
                4u8,
            >({
                let tun_type: u32 = unsafe { ::std::mem::transmute(tun_type) };
                tun_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                4u8,
            >({
                let inner_l2_type: u32 = unsafe { ::std::mem::transmute(inner_l2_type) };
                inner_l2_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                20usize,
                4u8,
            >({
                let inner_l3_type: u32 = unsafe { ::std::mem::transmute(inner_l3_type) };
                inner_l3_type as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                4u8,
            >({
                let inner_l4_type: u32 = unsafe { ::std::mem::transmute(inner_l4_type) };
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
    #[allow(unnecessary_transmutes)]
    pub fn l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 7u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 7u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_l2_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                0usize,
                7u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<7usize, 9u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<7usize, 9u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_l3_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                7usize,
                9u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn l4_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 8u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_l4_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 8u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_l4_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                16usize,
                8u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn tso_segsz(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 16u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_tso_segsz(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<24usize, 16u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_tso_segsz_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                24usize,
                16u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn outer_l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<40usize, 9u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_outer_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<40usize, 9u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_outer_l3_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                40usize,
                9u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn outer_l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<49usize, 7u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_outer_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<49usize, 7u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_outer_l2_len_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 7usize],
            >>::raw_set_const::<
                49usize,
                7u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
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
                let l2_len: u64 = unsafe { ::std::mem::transmute(l2_len) };
                l2_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                9u8,
            >({
                let l3_len: u64 = unsafe { ::std::mem::transmute(l3_len) };
                l3_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                8u8,
            >({
                let l4_len: u64 = unsafe { ::std::mem::transmute(l4_len) };
                l4_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                16u8,
            >({
                let tso_segsz: u64 = unsafe { ::std::mem::transmute(tso_segsz) };
                tso_segsz as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                40usize,
                9u8,
            >({
                let outer_l3_len: u64 = unsafe { ::std::mem::transmute(outer_l3_len) };
                outer_l3_len as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                49usize,
                7u8,
            >({
                let outer_l2_len: u64 = unsafe { ::std::mem::transmute(outer_l2_len) };
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
