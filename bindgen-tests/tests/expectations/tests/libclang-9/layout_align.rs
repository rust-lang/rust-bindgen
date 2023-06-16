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
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
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
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct rte_kni_fifo {
    ///< Next position to be written
    pub write: ::std::os::raw::c_uint,
    ///< Next position to be read
    pub read: ::std::os::raw::c_uint,
    ///< Circular buffer length
    pub len: ::std::os::raw::c_uint,
    ///< Pointer size - for 32/64 bit OS
    pub elem_size: ::std::os::raw::c_uint,
    ///< The buffer contains mbuf pointers
    pub buffer: __IncompleteArrayField<*mut ::std::os::raw::c_void>,
}
#[test]
fn bindgen_test_layout_rte_kni_fifo() {
    const UNINIT: ::std::mem::MaybeUninit<rte_kni_fifo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_kni_fifo>(),
        16usize,
        concat!("Size of: ", stringify!(rte_kni_fifo)),
    );
    assert_eq!(
        ::std::mem::align_of::<rte_kni_fifo>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_kni_fifo)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).write) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(rte_kni_fifo), "::", stringify!(write)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).read) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(rte_kni_fifo), "::", stringify!(read)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(rte_kni_fifo), "::", stringify!(len)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).elem_size) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_fifo),
            "::",
            stringify!(elem_size),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        16usize,
        concat!("Offset of field: ", stringify!(rte_kni_fifo), "::", stringify!(buffer)),
    );
}
impl Default for rte_kni_fifo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_eth_link {
    ///< ETH_SPEED_NUM_
    pub link_speed: u32,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_rte_eth_link() {
    const UNINIT: ::std::mem::MaybeUninit<rte_eth_link> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_eth_link>(),
        8usize,
        concat!("Size of: ", stringify!(rte_eth_link)),
    );
    assert_eq!(
        ::std::mem::align_of::<rte_eth_link>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_eth_link)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).link_speed) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_eth_link),
            "::",
            stringify!(link_speed),
        ),
    );
}
impl rte_eth_link {
    #[inline]
    pub fn link_duplex(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_link_duplex(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn link_autoneg(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_link_autoneg(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn link_status(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_link_status(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        link_duplex: u16,
        link_autoneg: u16,
        link_status: u16,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let link_duplex: u16 = unsafe { ::std::mem::transmute(link_duplex) };
                    link_duplex as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                1u8,
                {
                    let link_autoneg: u16 = unsafe {
                        ::std::mem::transmute(link_autoneg)
                    };
                    link_autoneg as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                2usize,
                1u8,
                {
                    let link_status: u16 = unsafe { ::std::mem::transmute(link_status) };
                    link_status as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
