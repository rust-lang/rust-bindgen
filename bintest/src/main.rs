#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
	let mut a = MyClass{val: 27, other:0};
	let b = MyClass{val: 13, other:0};
	a += &b; // should call the overloaded C++ operator+=
	let a = &a + &b; // should call the overloaded C++ operator+
	let a = -&a; // should call the overloaded C++ operator- (unary)
	let a = &a - &b; // should call the overloaded C++ operator- (binary)
	assert!(a == MyClass{val: -66, other:0}); // should call the overloaded C++ operator==
}
