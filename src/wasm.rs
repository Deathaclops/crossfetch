
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub mod crossfetch_wasm {

	use wasm_bindgen::prelude::*;
	use wasm_bindgen_futures::JsFuture;
	use web_sys::{Request, RequestInit, RequestMode, Response};
	pub struct CrossFetchPre;

	impl CrossFetchPre {

		pub async fn crossfetch ( path: String ) -> (Option<Vec<u8>>, i32) {

			let mut opts = RequestInit::new();
			opts.method("GET");
			opts.mode(RequestMode::NoCors);
			let request = Request::new_with_str_and_init(&path, &opts).unwrap();
			let window = web_sys::window().unwrap();
			let response_value = JsFuture::from(window.fetch_with_request(&request)).await;
			let checked_value = match response_value {
				Ok (response_valid) => response_valid,
				Err (err) => { panic!("Error getting array buffer: {:?}", err); }
			};
			// `resp_value` is a `Response` object.
			assert!(checked_value.is_instance_of::<Response>());
			let response: Response = checked_value.dyn_into().unwrap();
			if response.status() >= 400 && response.status() < 600
			{ return (None, response.status() as i32); }
			let response_data = JsFuture::from(response.array_buffer().unwrap()).await.unwrap();
			let array_buffer = js_sys::Uint8Array::new (&response_data);
			let byte_length = js_sys::Reflect::get(&array_buffer, &JsValue::from_str("byteLength"))
				.unwrap()
				.as_f64()
				.unwrap() as usize;
			let uint8_array = js_sys::Uint8Array::new(&array_buffer);
			let mut vec = vec![0; byte_length];
			uint8_array.copy_to(&mut vec);
			return (Some(vec), response.status() as i32);

		} // end async fn crossfetch
	} // end impl CrossFetchPre

}
