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
    const UNINIT: ::std::mem::MaybeUninit<RTCRay> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).org) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(org)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).align0) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(align0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dir) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(dir)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).align1) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(align1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tnear) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(tnear)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tfar) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(tfar)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mask) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ng) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(Ng)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).align2) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(align2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        64usize,
        concat!("Offset of field: ", stringify!(RTCRay), "::", stringify!(u))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        68usize,
        concat!("Offset of field: ", stringify!(RTCRay), "::", stringify!(v))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).geomID) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(geomID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).primID) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(primID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).instID) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(RTCRay),
            "::",
            stringify!(instID)
        )
    );
}
