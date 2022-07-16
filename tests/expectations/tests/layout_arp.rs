#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const ETHER_ADDR_LEN: u32 = 6;
pub const ARP_HRD_ETHER: u32 = 1;
pub const ARP_OP_REQUEST: u32 = 1;
pub const ARP_OP_REPLY: u32 = 2;
pub const ARP_OP_REVREQUEST: u32 = 3;
pub const ARP_OP_REVREPLY: u32 = 4;
pub const ARP_OP_INVREQUEST: u32 = 8;
pub const ARP_OP_INVREPLY: u32 = 9;
/// Ethernet address:
/// A universally administered address is uniquely assigned to a device by its
/// manufacturer. The first three octets (in transmission order) contain the
/// Organizationally Unique Identifier (OUI). The following three (MAC-48 and
/// EUI-48) octets are assigned by that organization with the only constraint
/// of uniqueness.
/// A locally administered address is assigned to a device by a network
/// administrator and does not contain OUIs.
/// See http://standards.ieee.org/regauth/groupmac/tutorial.html
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ether_addr {
    ///< Addr bytes in tx order
    pub addr_bytes: [u8; 6usize],
}
#[test]
fn bindgen_test_layout_ether_addr() {
    const UNINIT: ::std::mem::MaybeUninit<ether_addr> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ether_addr>(),
        6usize,
        concat!("Size of: ", stringify!(ether_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<ether_addr>(),
        1usize,
        concat!("Alignment of ", stringify!(ether_addr))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).addr_bytes) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ether_addr),
            "::",
            stringify!(addr_bytes)
        )
    );
}
/// ARP header IPv4 payload.
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct arp_ipv4 {
    ///< sender hardware address
    pub arp_sha: ether_addr,
    ///< sender IP address
    pub arp_sip: u32,
    ///< target hardware address
    pub arp_tha: ether_addr,
    ///< target IP address
    pub arp_tip: u32,
}
#[test]
fn bindgen_test_layout_arp_ipv4() {
    const UNINIT: ::std::mem::MaybeUninit<arp_ipv4> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<arp_ipv4>(),
        20usize,
        concat!("Size of: ", stringify!(arp_ipv4))
    );
    assert_eq!(
        ::std::mem::align_of::<arp_ipv4>(),
        1usize,
        concat!("Alignment of ", stringify!(arp_ipv4))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_sha) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_sha)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_sip) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_sip)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_tha) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_tha)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_tip) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_tip)
        )
    );
}
/// ARP header.
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct arp_hdr {
    pub arp_hrd: u16,
    pub arp_pro: u16,
    pub arp_hln: u8,
    pub arp_pln: u8,
    pub arp_op: u16,
    pub arp_data: arp_ipv4,
}
#[test]
fn bindgen_test_layout_arp_hdr() {
    const UNINIT: ::std::mem::MaybeUninit<arp_hdr> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<arp_hdr>(),
        28usize,
        concat!("Size of: ", stringify!(arp_hdr))
    );
    assert_eq!(
        ::std::mem::align_of::<arp_hdr>(),
        1usize,
        concat!("Alignment of ", stringify!(arp_hdr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_hrd) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_hrd)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_pro) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_pro)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_hln) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_hln)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_pln) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_pln)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arp_op) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_op)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).arp_data) as usize - ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_data)
        )
    );
}
