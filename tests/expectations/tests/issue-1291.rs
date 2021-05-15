#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct RTCRay {
    pub org: [f32; 3usize],
    pub align0: f32,
    pub dir: [f32; 3usize],
    pub align1: f32,
    pub tnear: f32,
    pub tfar: f32,
    pub time: f32,
    pub mask: ::std::os::raw::c_uint,
    pub Ng: [f32; 3usize],
    pub align2: f32,
    pub u: f32,
    pub v: f32,
    pub geomID: ::std::os::raw::c_uint,
    pub primID: ::std::os::raw::c_uint,
    pub instID: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_RTCRay() {
    assert_eq!(
        ::std::mem::size_of::<RTCRay>(),
        96usize,
        concat!("Size of: ", stringify!(RTCRay))
    );
    assert_eq!(
        ::std::mem::align_of::<RTCRay>(),
        16usize,
        concat!("Alignment of ", stringify!(RTCRay))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.org);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(org)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.align0);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(align0)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.dir);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(dir)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.align1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(align1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.tnear);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(tnear)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.tfar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(tfar)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.time);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.mask);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.Ng);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(Ng)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.align2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(align2)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.u);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        64usize,
        concat!("Offset of field: ", stringify!(RTCRay), "::", stringify!(u))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.v);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        68usize,
        concat!("Offset of field: ", stringify!(RTCRay), "::", stringify!(v))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.geomID);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(geomID)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.primID);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(primID)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<RTCRay>() };
            let struct_ptr = &struct_instance as *const RTCRay;
            let field_ptr = std::ptr::addr_of!(struct_instance.instID);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(instID)
        )
    );
}
