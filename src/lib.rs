
mod wasm; mod native;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub use wasm::crossfetch_wasm::*;

#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub use native::crossfetch_native::*;

#[derive(Debug)]
pub enum CrossFetchError {
	JSError(wasm_bindgen::JsValue),
	IOError(std::io::Error),
	ReqwestError(reqwest::Error),
} // end enum CrossFetchError

pub trait UnicodeBytes {
	fn as_str(&self) -> Option<&str>;
	fn to_string(&self) -> Option<String>;
	fn to_string_cloneless(self) -> Option<String>;
	fn to_string_from_utf16(&self) -> Option<String>;
} // end trait Example

impl UnicodeBytes for Vec<u8> {
	fn as_str(&self) -> Option<&str> { return std::str::from_utf8(self).ok(); }
	fn to_string(&self) -> Option<String> { return String::from_utf8(self.clone()).ok(); }
	fn to_string_cloneless(self) -> Option<String> { return String::from_utf8(self).ok(); }
	fn to_string_from_utf16(&self) -> Option<String> {
		if self.len() % 2 != 0 { return None; } // UTF-16 requires even number of bytes
		let u16_vec: Vec<u16> = self
			.chunks_exact(2) // Iterate over chunks of 2 bytes
			.map(|chunk| {
				// Convert each 2-byte chunk into a [u8; 2] array
				let array: [u8; 2] = [chunk[0], chunk[1]];
				// Interpret the 2 bytes as a u16 using native endianness
				u16::from_ne_bytes(array)
			}).collect();
		return String::from_utf16(&u16_vec).ok();
	} // end fn to_string_from_utf16
} // end impl Example

