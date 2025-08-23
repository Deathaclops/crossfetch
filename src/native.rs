
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub mod crossfetch_native {
	
	use crate::CrossFetchError;
	use std::fs;

	pub async fn fetch(url: impl Into<String>) -> Result<Vec<u8>, CrossFetchError> {
		let url_str = url.into();
		if url_str.len() > 4 && &url_str[0..4] == "http" {
			return cf_remote_adapter(url_str).await;
		} else { return cf_local_adapter(url_str); }
	} // end fn fetch

	fn cf_local_adapter(path: String) -> Result<Vec<u8>, CrossFetchError> {
		let receiver: Result<Vec<u8>, std::io::Error> = cf_local(path);
		if receiver.is_err() {
			return Err(CrossFetchError::IOError(receiver.err().unwrap()));
		} else { return Ok(receiver.unwrap()); }
	} // end fn local

	fn cf_local(path: String) -> Result<Vec<u8>, std::io::Error> { return fs::read(path); }

	async fn cf_remote_adapter(url: String) -> Result<Vec<u8>, CrossFetchError> {
		let receiver: Result<Vec<u8>, reqwest::Error> = cf_remote(url).await;
		if receiver.is_err() {
			return Err(CrossFetchError::ReqwestError(receiver.err().unwrap()));
		} else { return Ok(receiver.unwrap()); }
	} // end fn remote

	async fn cf_remote(url: String) -> Result<Vec<u8>, reqwest::Error> {
		let response = reqwest::get(url).await?;
		let bytes = response.bytes().await?;
		return Ok(bytes.to_vec());
	} // end fn fetch_remote

} // end mod crossfetch_native