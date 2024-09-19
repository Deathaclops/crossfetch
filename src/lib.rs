
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
	fn to_string (&self) -> String;
	fn as_str (&self) -> &str;
} // end trait Example

impl UnicodeBytes for Vec<u8> {
	fn to_string (&self) -> String { return String::from_utf8(self.clone()).unwrap(); }
	fn as_str (&self) -> &str { return std::str::from_utf8(self).unwrap(); }
} // end impl Example

