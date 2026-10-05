#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ether_addr"][::std::mem::size_of::<ether_addr>() - 6usize];
    ["Alignment of ether_addr"][::std::mem::align_of::<ether_addr>() - 1usize];
    [
        "Offset of field: ether_addr::addr_bytes",
    ][::std::mem::offset_of!(ether_addr, addr_bytes) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arp_ipv4"][::std::mem::size_of::<arp_ipv4>() - 20usize];
    ["Alignment of arp_ipv4"][::std::mem::align_of::<arp_ipv4>() - 1usize];
    [
        "Offset of field: arp_ipv4::arp_sha",
    ][::std::mem::offset_of!(arp_ipv4, arp_sha) - 0usize];
    [
        "Offset of field: arp_ipv4::arp_sip",
    ][::std::mem::offset_of!(arp_ipv4, arp_sip) - 6usize];
    [
        "Offset of field: arp_ipv4::arp_tha",
    ][::std::mem::offset_of!(arp_ipv4, arp_tha) - 10usize];
    [
        "Offset of field: arp_ipv4::arp_tip",
    ][::std::mem::offset_of!(arp_ipv4, arp_tip) - 16usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arp_hdr"][::std::mem::size_of::<arp_hdr>() - 28usize];
    ["Alignment of arp_hdr"][::std::mem::align_of::<arp_hdr>() - 1usize];
    [
        "Offset of field: arp_hdr::arp_hrd",
    ][::std::mem::offset_of!(arp_hdr, arp_hrd) - 0usize];
    [
        "Offset of field: arp_hdr::arp_pro",
    ][::std::mem::offset_of!(arp_hdr, arp_pro) - 2usize];
    [
        "Offset of field: arp_hdr::arp_hln",
    ][::std::mem::offset_of!(arp_hdr, arp_hln) - 4usize];
    [
        "Offset of field: arp_hdr::arp_pln",
    ][::std::mem::offset_of!(arp_hdr, arp_pln) - 5usize];
    [
        "Offset of field: arp_hdr::arp_op",
    ][::std::mem::offset_of!(arp_hdr, arp_op) - 6usize];
    [
        "Offset of field: arp_hdr::arp_data",
    ][::std::mem::offset_of!(arp_hdr, arp_data) - 8usize];
};
