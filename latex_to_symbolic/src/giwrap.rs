#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![allow(improper_ctypes)]

#![allow(dead_code)]


include!("../modbind.rs");

use std::ffi::CString;
//use std::os::raw::c_char;

struct ginac_symbol {
	ptr: *mut GiNaC_symbol
}
impl ginac_symbol {
	fn new(s: &str) -> ginac_symbol {
		let cstring = CString::new(s).expect("CString::new failed");
		let cptr = unsafe {
			wrap_symbol_construct(cstring.as_ptr())
		};
		ginac_symbol{ptr: cptr}
	}
}
impl Drop for ginac_symbol {
	fn drop(&mut self) {
		unsafe {
			wrap_symbol_destruct(self.ptr);
		}
	}
}
pub struct ginac_ex {
	ptr: *mut GiNaC_ex
}
impl ginac_ex {
	pub fn from_symbol_str(s: &str) -> ginac_ex {
		let cstring = CString::new(s).expect("CString::new failed");
		let cptr = unsafe {
			wrap_ex_construct(cstring.as_ptr())
		};
		ginac_ex{ptr: cptr}
	}
	pub fn from_copy(ex: &ginac_ex) -> ginac_ex {
		let cptr = unsafe {
			wrap_ex_construct_copy(ex.ptr)
		};
		ginac_ex{ptr: cptr}
	}
	pub fn from_float(f: f64) -> ginac_ex {
		let cptr = unsafe {
			wrap_ex_construct_float(f)
		};
		ginac_ex{ptr: cptr}
	}
	pub fn from_integer(i: i32) -> ginac_ex {
		let cptr = unsafe {
			wrap_ex_construct_integer(i)
		};
		ginac_ex{ptr: cptr}
	}
	pub fn from_power(base: &ginac_ex, expon: &ginac_ex) -> ginac_ex {
		let cptr = unsafe {
			wrap_ex_construct_power(base.ptr, expon.ptr)
		};
		ginac_ex{ptr: cptr}
	}
	pub fn print(&self) {
		unsafe {
			GiNaC_ex_dbgprint(self.ptr);
		}
	}
	pub fn print_tree(&self) {
		unsafe {
			GiNaC_ex_dbgprinttree(self.ptr);
		}
	}
	pub fn powex(&self, expon: &ginac_ex) -> ginac_ex {
		ginac_ex::from_power(self, expon)
	}
}
impl Drop for ginac_ex {
	fn drop(&mut self) {
		unsafe {
			wrap_ex_destruct(self.ptr);
		}
	}
}
impl std::ops::AddAssign for ginac_ex {
	fn add_assign(&mut self, right: Self) {
		unsafe {
			wrap_add_assign(self.ptr, right.ptr);
		}
	}
}
impl std::ops::MulAssign for ginac_ex {
	fn mul_assign(&mut self, right: Self) {
		unsafe {
			wrap_mul_assign(self.ptr, right.ptr);
		}
	}
}
impl std::ops::Add for ginac_ex {
	type Output = ginac_ex;
	fn add(self, right: Self) -> ginac_ex{
		let mut ret = ginac_ex::from_copy(&self);
		ret += right;
		ret
	}
}
impl std::ops::Mul for ginac_ex {
	type Output = ginac_ex;
	fn mul(self, right: Self) -> ginac_ex {
		let mut ret = ginac_ex::from_copy(&self);
		ret *= right;
		ret
	}
}
impl PartialEq for ginac_ex {
	fn eq(&self, other: &Self) -> bool {
		unsafe {
			wrap_equal(self.ptr, other.ptr)
		}
	}
}
impl Eq for ginac_ex {}
