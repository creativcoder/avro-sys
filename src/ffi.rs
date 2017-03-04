
use libc::size_t;
use libc::c_void;

#[repr(C)]
struct AvroRawArray<T> {
	element_size: size_t,
	element_count: size_t,
	allocated_size: size_t,
	data: *mut T
}

pub fn type_of<T>(_: &T) {
    println!("{}", unsafe { ::std::intrinsics::type_name::<T>() });
}

#[link(name="avro", kind="static")]
extern {
	pub fn avro_calloc(count: size_t, size: size_t)  -> *const c_void;
	//fn print_hi();
}