
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub mod crossfetch_wasm {

	use wasm_bindgen::prelude::*;
	use wasm_bindgen_futures::JsFuture;
	use web_sys::{Request, RequestInit, RequestMode, Response};
	use crate::CrossFetchError;
	
	pub async fn fetch<S: AsRef<str>> ( url: S ) -> Result<Vec<u8>, CrossFetchError> {

		let url_str = url.as_ref().to_string();
		let local_fetch = fetch_web(url_str).await;
		if local_fetch.is_err() {
			return Err(CrossFetchError::JSError(local_fetch.err().unwrap()));
		} else { return Ok(local_fetch.unwrap()); }

	} // end fn fetch

	async fn fetch_web ( path: String ) -> Result<Vec<u8>, JsValue> {

		let opts: RequestInit = RequestInit::new();
		opts.set_method("GET");
		opts.set_mode(RequestMode::Cors);
		let request: Request = Request::new_with_str_and_init(&path, &opts)?;
	
		let window: web_sys::Window = web_sys::window().unwrap();
		let response_value: JsValue = JsFuture::from(window.fetch_with_request(&request)).await?;

		// `resp_value` is a `Response` object.
		assert!(response_value.is_instance_of::<Response>());
		let response: Response = response_value.dyn_into().unwrap();

		let response_data: JsValue = JsFuture::from(response.array_buffer().unwrap()).await?;
		let array_buffer: js_sys::Uint8Array = js_sys::Uint8Array::new(&response_data);
		let out: Vec<u8> = array_buffer.to_vec();

		return Ok(out);
		
	} // end fn fetch_local

	

}
