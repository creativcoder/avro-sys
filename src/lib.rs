#![feature(core_intrinsics)]

extern crate libc;
use libc::c_void;

mod ffi;

pub fn avro_calloc(n_bytes: usize, sz: usize) -> *const c_void {
	let c = unsafe {
		ffi::avro_calloc(n_bytes, sz)
	};
	c
}