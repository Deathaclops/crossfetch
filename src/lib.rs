

mod wasm; use wasm::*;
mod native; use native::*;
use wasm_bindgen::prelude::*;
use std::str;
use web_sys::console;
use web_sys::TextDecoder;
use tokio::runtime::Builder;
use std::path::Path;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
type defetchpre = wasm::defetch_wasm::DefetchPre; 

#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
type defetchpre = native::defetch_native::DefetchPre;

pub struct Defetch {
	pub bytes: Vec<u8>,
	pub status: i32,
	pub filename: String,
	pub extension: String,
	pub url: String
}

impl Defetch {

	pub async fn defetch ( url: String ) -> Self {
		
		let contents = defetchpre::defetch ( url.clone() ).await;
		let filename = Path::new(&url).file_name().unwrap().to_str().unwrap();
		let extension = Path::new(&url).extension().unwrap().to_str().unwrap();

		return Defetch {
			bytes: contents.0,
			status: contents.1,
			filename: filename.to_string(),
			extension: extension.to_string(),
			url
		}

	}

	pub fn is_ok (&self) -> bool {
		return self.status == 200;
	}

	pub fn text (&self) -> String {
		return String::from_utf8 (self.bytes.clone()).unwrap();
	}

}