use std::ptr;

pub fn to_bytes32(data: Vec<u8>) -> [u8; 32] {
	if data.len() != 32 {
		panic!("expected bytes32");
	}
	let mut result = [0u8; 32];
	unsafe {
		ptr::copy(data.as_ptr(), result.as_mut_ptr(), 32);
	}
	result
}
