#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

include!(concat!(env!("OUT_DIR"), "/bindings_2.rs"));
//include!(concat!(env!("OUT_DIR"), "/bindings_1.rs"));

fn main() {
	let obj = Box_Base::new();
	// unsafe{
	// 	deleter(obj.ptr as *mut Base);
	// }
}
