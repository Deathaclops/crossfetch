

mod wasm; mod native;
use std::path::Path;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
type crossfetch_pre = wasm::crossfetch_wasm::CrossFetchPre; 

#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
type crossfetch_pre = native::crossfetch_native::CrossFetchPre;

pub struct CrossFetch {
	pub bytes: Result<Vec<u8>, i32>,
	pub response: i32,
	pub filename: String,
	pub extension: String,
	pub path: String
}

impl CrossFetch {

	pub async fn fetch ( path: String ) -> Self {
		
		let contents = crossfetch_pre::crossfetch ( path.clone() ).await;
		let filename = Path::new(&path).file_name().unwrap().to_str().unwrap();
		let extension = Path::new(&path).extension().unwrap().to_str().unwrap();
		let response = if contents.0.is_some() { Ok ( contents.0.unwrap() ) } else { Err ( contents.1 ) };

		return CrossFetch {
			bytes: response,
			response: contents.1,
			filename: filename.to_string(),
			extension: extension.to_string(),
			path
		};

	} // end async fn fetch

	pub fn is_ok (&self) -> bool {
		return self.bytes.is_ok();
	} // end fn is_ok

	pub fn text (&self) -> Option<String> {
		if self.bytes.is_err() { return None; }
		else { return Some(String::from_utf8 (self.bytes.clone().unwrap()).unwrap()); }
	} // end fn text

}