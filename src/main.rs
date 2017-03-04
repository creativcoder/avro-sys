#![feature(core_intrinsics)]

extern crate libc;

mod ffi;

use ffi::type_of;

fn main() {
	// Test
	unsafe {
		let c = ffi::avro_calloc(3,6);
		type_of(&c);
	}
}