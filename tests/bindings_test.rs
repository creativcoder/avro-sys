
extern crate avro_sys;
extern crate libc;

use std::ptr;

#[test]
fn test_bindings_avro_calloc() {
	let c = avro_sys::avro_calloc(3,6);
	assert!(!c.is_null());
}